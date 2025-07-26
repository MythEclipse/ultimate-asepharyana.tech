#!/bin/bash
# Copy .env from root to all direct subprojects/packages (not nested, not build, not .git, not .github, not node_modules, not .turbo, not .next, not .vscode, not dist, not public, not src, not coverage, not logs, not .devcontainer, not .yarn, not target)

ROOT_ENV="/workspaces/ultimate-asepharyana.cloud/.env"

if [ ! -f "$ROOT_ENV" ]; then
  echo "Root .env file not found at $ROOT_ENV"
  exit 1
fi

find /workspaces/ultimate-asepharyana.cloud -mindepth 1 -maxdepth 2 -type d | while read -r dir; do
  # Only copy to direct subprojects/packages, skip unwanted dirs
  base=$(basename "$dir")
  parent=$(basename "$(dirname "$dir")")
  if [[ "$dir" == *node_modules* || "$base" =~ ^(\.git|\.github|node_modules|\.turbo|\.next|\.vscode|dist|public|src|coverage|logs|\.devcontainer|\.yarn|target)$ || "$parent" =~ ^(\.git|\.github|node_modules|\.turbo|\.next|\.vscode|dist|public|src|coverage|logs|\.devcontainer|\.yarn|target)$ ]]; then
    continue
  fi
  cp "$ROOT_ENV" "$dir/.env"
  echo "Copied .env to $dir/.env"
done