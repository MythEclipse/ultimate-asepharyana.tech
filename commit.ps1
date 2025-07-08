#!/usr/bin/env pwsh

# Jalankan npm format
bun run format
if ($LASTEXITCODE -ne 0) {
    Write-Error "Please fix format issues"
    exit 1
}

# Jalankan lint
bun run lint
if ($LASTEXITCODE -ne 0) {
    Write-Host "Linting failed, attempting to fix issues..."
    bun run lint:fix

    # Periksa kembali linting setelah menjalankan lint:fix
    bun run lint
    if ($LASTEXITCODE -ne 0) {
        Write-Error "Linting still failed after auto-fix. Please fix manually."
        exit 1
    }

    # Periksa apakah ada perubahan setelah lint:fix
    if (-not ([string]::IsNullOrWhiteSpace($(git status --porcelain)))) {
        Write-Host "Changes made by lint:fix. Adding changes to commit..."
        git add .
        Write-Host "All changes added. Please proceed with your commit."
    } else {
        Write-Error "No changes made by lint:fix. Exiting."
        exit 1
    }
}

# Jika linting berhasil, lanjutkan
Write-Host "Linting passed."
Write-Host "All checks passed."

# Tambahkan semua perubahan
git add .

# Commit dengan pesan otomatis
git commit -m "Malas isi commit message"

# Push ke repository
git push
