#!/bin/bash
# Copy .env from root to all direct subprojects/packages (not nested, not build, not .git, not .github, not node_modules, not .turbo, not .next, not .vscode, not dist, not public, not src, not coverage, not logs, not .devcontainer, not .yarn, not target)
# Do NOT copy to /apps or /packages root, only to their subfolders.

ROOT_ENV="./.env"

if [ ! -f "$ROOT_ENV" ]; then
  echo "Root .env file not found at $ROOT_ENV"
  exit 1
fi

for parent in apps packages; do
  for dir in ./$parent/*/; do
    # Remove trailing slash
    dir="${dir%/}"
    base=$(basename "$dir")
    if [[ "$base" =~ ^(\.git|\.github|node_modules|\.turbo|\.next|\.vscode|dist|public|src|coverage|logs|\.devcontainer|\.yarn|target)$ ]]; then
      continue
    fi
    cp "$ROOT_ENV" "$dir/.env"
    echo "Copied .env to $dir/.env"
  done
done