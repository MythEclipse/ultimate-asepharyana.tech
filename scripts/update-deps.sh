#!/usr/bin/env bash
set -euo pipefail

# Define an array of directories to aggressively prune from the search tree
# This prevents exhaustive traversal into massive localized dependency stores, build caches,
# and anomalously created cache directories (e.g., literal '~').
PRUNE_DIRS=(
  "node_modules"
  "dist"
  ".next"
  ".bun"
  "~"
  "build"
  ".git"
  "target"
)

# Construct the prune expression dynamically to feed the find command
PRUNE_ARGS=()
for dir in "${PRUNE_DIRS[@]}"; do
  if [ ${#PRUNE_ARGS[@]} -gt 0 ]; then
    PRUNE_ARGS+=("-o")
  fi
  PRUNE_ARGS+=("-name" "$dir")
done

echo "Scanning for top-level and workspace package.json manifests..."

# Perform an optimized find:
# 1. -prune halts traversal immediately upon matching a PRUNE_DIR, achieving extreme I/O efficiency.
# 2. -print0 strictly streams zero-byte delimited paths, immunizing the loop against spaces/newlines.
find . \( "${PRUNE_ARGS[@]}" \) -prune -o -name "package.json" -type f -print0 | while IFS= read -r -d '' filename; do
  dir=$(dirname "$filename")
  
  echo "--------------------------------------------------------------------------------"
  echo "Initiating strict dependency update sequence in: $dir"
  
  # Spawn a tightly scoped subshell. This isolates environment state and traps internal pathing failures.
  (
    # Strict directory entry checks.
    cd "$dir" || {
      echo "CRITICAL: Directory transition failed for $dir. Process aborted." >&2
      exit 1
    }
    
    # Heuristic check: Ensure the manifest actually declares dependencies before thrashing the disk with ncu.
    if ! grep -Eq '"(dependencies|devDependencies|peerDependencies)"[[:space:]]*:' package.json; then
      echo "Notice: No valid dependency blocks detected in $dir/package.json. Bypassing node traversal."
      exit 0
    fi

    # Delegate command resolution to bunx. This obliterates the 'command not found' failure mode
    # by dynamically sourcing the npm-check-updates binary, completely ignoring global namespace pollution.
    echo "Executing constraint-free updates via bunx..."
    bunx --bun npm-check-updates -u
    
    echo "Commencing rigorous package installation phase..."
    bun install
  ) || {
    echo "WARNING: Subshell failure encountered in $dir. Subsystem continues." >&2
  }
done

echo "--------------------------------------------------------------------------------"
echo "SYSTEM STATE: All traversable dependencies aggressively updated."
