<#
.SYNOPSIS
  Clean all node_modules directories in the repository with optional cache and lockfile cleanup.

.DESCRIPTION
  This script recursively finds and removes all 'node_modules' directories starting at the repo root.
  Optionally, it can also remove build caches and lockfiles, and run 'pnpm store prune'.

.PARAMETER IncludeCache
  Also remove common cache/build output directories (e.g., .next, .turbo, .vite, node_modules/.cache, dist, build, coverage, out, storybook-static).

.PARAMETER IncludeLock
  Also remove lock files (pnpm-lock.yaml, package-lock.json, yarn.lock) in the repo.

.PARAMETER PruneStore
  After deletion, try to run 'pnpm store prune' if pnpm is installed.

.PARAMETER Yes
  Proceed without interactive confirmation (non-interactive mode).

.PARAMETER DryRun
  Show what would be removed without deleting anything.

.EXAMPLE
  # Preview what will be removed
  ./scripts/clean-node-modules.ps1 -DryRun

.EXAMPLE
  # Clean node_modules only, no prompt
  ./scripts/clean-node-modules.ps1 -Yes

.EXAMPLE
  # Deep clean including caches and lockfiles, and prune pnpm store
  ./scripts/clean-node-modules.ps1 -IncludeCache -IncludeLock -PruneStore -Yes
#>

[CmdletBinding(SupportsShouldProcess=$true)]
param(
  [switch] $IncludeCache,
  [switch] $IncludeLock,
  [switch] $PruneStore,
  [switch] $Yes,
  [switch] $DryRun
)

$ErrorActionPreference = 'Stop'

function Write-Section($text) {
  Write-Host "`n==== $text ====\n" -ForegroundColor Cyan
}

function Safe-RemoveDirectory {
  param(
    [Parameter(Mandatory=$true)][string] $Path,
    [switch] $Preview
  )
  if (-not (Test-Path -LiteralPath $Path)) { return }
  if ($Preview) { Write-Host "[dir]  $Path"; return }
  try {
    # Use cmd rmdir for better handling of read-only/long paths on Windows PowerShell 5.1
    & cmd.exe /c "rmdir /s /q \"$Path\"" | Out-Null
  } catch {
    try { Remove-Item -LiteralPath $Path -Recurse -Force -ErrorAction Stop } catch {
      Write-Warning "Failed to remove directory: $Path -> $($_.Exception.Message)"
    }
  }
}

function Safe-RemoveFile {
  param(
    [Parameter(Mandatory=$true)][string] $Path,
    [switch] $Preview
  )
  if (-not (Test-Path -LiteralPath $Path)) { return }
  if ($Preview) { Write-Host "[file] $Path"; return }
  try { Remove-Item -LiteralPath $Path -Force -ErrorAction Stop } catch {
    Write-Warning "Failed to remove file: $Path -> $($_.Exception.Message)"
  }
}

# Resolve repo root (this script lives in ./scripts)
$RepoRoot = (Resolve-Path (Join-Path $PSScriptRoot '..')).Path
Set-Location -LiteralPath $RepoRoot

Write-Host "Repo root: $RepoRoot" -ForegroundColor DarkGray

# Accumulators
$DirsToDelete  = New-Object System.Collections.Generic.List[string]
$FilesToDelete = New-Object System.Collections.Generic.List[string]

# 1) node_modules everywhere (including root)
Write-Section 'Scanning node_modules directories'
$nodeModulesDirs = Get-ChildItem -LiteralPath $RepoRoot -Directory -Recurse -Force -ErrorAction SilentlyContinue |
  Where-Object { $_.Name -eq 'node_modules' }

# Ensure root node_modules is included if present
$rootNM = Join-Path $RepoRoot 'node_modules'
if (Test-Path -LiteralPath $rootNM) {
  $DirsToDelete.Add($rootNM)
}
foreach ($d in $nodeModulesDirs) {
  if (-not $DirsToDelete.Contains($d.FullName)) { $DirsToDelete.Add($d.FullName) }
}
Write-Host ("Found {0} node_modules directory(ies)" -f $DirsToDelete.Count)

# 2) Optional caches/output folders
if ($IncludeCache) {
  Write-Section 'Scanning cache/output directories'
  $cacheNames = @(
    '.next', '.turbo', '.vite', '.parcel-cache', '.cache',
    'dist', 'build', 'coverage', 'out', 'storybook-static',
    '.wrangler'
  )
  # node_modules/.cache is common; include it via name match too
  $allDirs = Get-ChildItem -LiteralPath $RepoRoot -Directory -Recurse -Force -ErrorAction SilentlyContinue
  foreach ($dir in $allDirs) {
    if ($cacheNames -contains $dir.Name) {
      if (-not $DirsToDelete.Contains($dir.FullName)) { $DirsToDelete.Add($dir.FullName) }
    }
  }
  Write-Host ("Found {0} cache/output directory(ies)" -f ($DirsToDelete | Where-Object { Test-Path $_ }).Count)
}

# 3) Optional lockfiles
if ($IncludeLock) {
  Write-Section 'Scanning lock files'
  $lockGlobs = @('pnpm-lock.yaml', 'package-lock.json', 'yarn.lock')
  foreach ($glob in $lockGlobs) {
    $files = Get-ChildItem -LiteralPath $RepoRoot -Recurse -Force -File -Filter $glob -ErrorAction SilentlyContinue
    foreach ($f in $files) { if (-not $FilesToDelete.Contains($f.FullName)) { $FilesToDelete.Add($f.FullName) } }
  }
  Write-Host ("Found {0} lock file(s)" -f $FilesToDelete.Count)
}

# 4) Summary
Write-Section 'Summary'
Write-Host ("Directories to delete: {0}" -f $DirsToDelete.Count)
Write-Host ("Files to delete:       {0}" -f $FilesToDelete.Count)

$preview = $DryRun -or (-not $Yes)
if ($preview) {
  Write-Host "Preview mode (no deletions). Use -Yes to confirm, or pass -DryRun:$false to hide this list." -ForegroundColor Yellow
  foreach ($dir in $DirsToDelete) { Safe-RemoveDirectory -Path $dir -Preview }
  foreach ($fil in $FilesToDelete) { Safe-RemoveFile -Path $fil -Preview }
  if (-not $Yes) { Write-Host "\nRun again with -Yes to confirm deletion." -ForegroundColor Yellow }
  exit 0
}

# 5) Deletion
Write-Section 'Deleting directories'
foreach ($dir in $DirsToDelete) { Safe-RemoveDirectory -Path $dir }

if ($FilesToDelete.Count -gt 0) {
  Write-Section 'Deleting files'
  foreach ($fil in $FilesToDelete) { Safe-RemoveFile -Path $fil }
}

# 6) Optional pnpm store prune
if ($PruneStore) {
  Write-Section 'Pruning pnpm store'
  $pnpm = Get-Command pnpm -ErrorAction SilentlyContinue
  if ($null -ne $pnpm) {
    try { & pnpm store prune } catch { Write-Warning "pnpm store prune failed: $($_.Exception.Message)" }
  } else {
    Write-Warning "pnpm not found on PATH; skipping 'pnpm store prune'."
  }
}

Write-Host "\nDone." -ForegroundColor Green
