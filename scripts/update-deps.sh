#!/bin/bash

# Find all package.json files in the current directory and subdirectories,
# excluding node_modules and dist directories.
find . -name "package.json" -not -path "*/node_modules/*" -not -path "*/dist/*" | while read filename; do
  dir=$(dirname "$filename")
  echo "Updating dependencies in $dir"
  (cd "$dir" && ncu -u && pnpm install)
done

echo "All dependencies updated."
