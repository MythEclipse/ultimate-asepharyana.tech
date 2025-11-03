#!/usr/bin/env bash
set -euo pipefail

# Clean all node_modules directories with optional cache & lockfile cleanup.
# Usage:
#   ./scripts/clean-node-modules.sh [--include-cache] [--include-lock] [--prune-store] [--yes] [--dry-run]

INCLUDE_CACHE=false
INCLUDE_LOCK=false
PRUNE_STORE=false
YES=false
DRY_RUN=false

for arg in "$@"; do
  case "$arg" in
    --include-cache) INCLUDE_CACHE=true ;;
    --include-lock)  INCLUDE_LOCK=true  ;;
    --prune-store)   PRUNE_STORE=true   ;;
    --yes)           YES=true           ;;
    --dry-run)       DRY_RUN=true       ;;
    *) echo "Unknown option: $arg"; exit 2 ;;
  esac
done

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &> /dev/null && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

preview() {
  if [[ "$DRY_RUN" == true || "$YES" == false ]]; then echo 1; else echo 0; fi
}

section() { echo -e "\n==== $* ====\n"; }

DIRS_TO_DELETE=()
FILES_TO_DELETE=()

section "Scanning node_modules directories"
while IFS= read -r -d '' d; do DIRS_TO_DELETE+=("$d"); done < <(find "$REPO_ROOT" -type d -name node_modules -print0)
if [[ -d "$REPO_ROOT/node_modules" ]]; then DIRS_TO_DELETE+=("$REPO_ROOT/node_modules"); fi

section "Summary so far"
echo "Found ${#DIRS_TO_DELETE[@]} node_modules directories"

if [[ "$INCLUDE_CACHE" == true ]]; then
  section "Scanning cache/output directories"
  while IFS= read -r -d '' d; do DIRS_TO_DELETE+=("$d"); done < <(find "$REPO_ROOT" -type d \( \
    -name .next -o -name .turbo -o -name .vite -o -name .parcel-cache -o -name .cache -o \
    -name dist -o -name build -o -name coverage -o -name out -o -name storybook-static -o -name .wrangler \
  \) -print0)
fi

if [[ "$INCLUDE_LOCK" == true ]]; then
  section "Scanning lock files"
  while IFS= read -r -d '' f; do FILES_TO_DELETE+=("$f"); done < <(\
    find "$REPO_ROOT" -type f \( -name pnpm-lock.yaml -o -name package-lock.json -o -name yarn.lock \) -print0)
fi

section "Summary"
echo "Directories to delete: ${#DIRS_TO_DELETE[@]}"
echo "Files to delete:       ${#FILES_TO_DELETE[@]}"

if [[ $(preview) -eq 1 ]]; then
  echo "Preview mode (no deletions). Re-run with --yes to confirm."
  for d in "${DIRS_TO_DELETE[@]}"; do echo "[dir]  $d"; done
  for f in "${FILES_TO_DELETE[@]}"; do echo "[file] $f"; done
  exit 0
fi

section "Deleting directories"
for d in "${DIRS_TO_DELETE[@]}"; do rm -rf -- "$d" || true; done

if [[ ${#FILES_TO_DELETE[@]} -gt 0 ]]; then
  section "Deleting files"
  for f in "${FILES_TO_DELETE[@]}"; do rm -f -- "$f" || true; done
fi

if [[ "$PRUNE_STORE" == true ]]; then
  section "Pruning pnpm store"
  if command -v pnpm >/dev/null 2>&1; then pnpm store prune || true; else echo "pnpm not found; skipping"; fi
fi

echo "Done."
