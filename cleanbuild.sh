#!/bin/bash

set -e # Exit immediately if a command exits with a non-zero status

# Function to check if bun is available
command_exists () {
  type "$1" &> /dev/null ;
}

# Function to clean and build JavaScript/TypeScript projects
clean_and_build_js_project() {
    local project_path=$1
    local action_clean=$2
    local action_build=$3

    echo "Processing JS/TS project: $project_path"
    (cd "$project_path" && \
        if [ "$action_clean" = true ]; then \
            echo "Cleaning $project_path..." && \
            rm -rf node_modules dist .next build; \
        fi && \
        if [ "$action_build" = true ]; then \
            if command_exists bun; then \
                echo "Using bun for $project_path..." && \
                bun install && \
                if grep -q '"build":' package.json; then \
                    echo "Building $project_path with bun..." && \
                    bun run build; \
                else \
                    echo "No build script found for $project_path, skipping build."; \
                fi; \
            else \
                echo "Using npm for $project_path..." && \
                npm install && \
                if grep -q '"build":' package.json; then \
                    echo "Building $project_path with npm..." && \
                    npm run build; \
                else \
                    echo "No build script found for $project_path, skipping build."; \
                fi; \
            fi; \
        fi)
}

# Function to clean and build Rust projects
clean_and_build_rust_project() {
    local project_path=$1
    local action_clean=$2
    local action_build=$3

    echo "Processing Rust project: $project_path"
    (cd "$project_path" && \
        if [ "$action_clean" = true ]; then \
            echo "Cleaning $project_path..." && \
            cargo clean; \
        fi && \
        if [ "$action_build" = true ]; then \
            echo "Building $project_path..." && \
            cargo build; \
        fi)
}

# Determine action based on arguments
ACTION_CLEAN=false
ACTION_BUILD=false

if [ $# -eq 0 ]; then
    echo "No arguments provided. Defaulting to 'clean build'."
    ACTION_CLEAN=true
    ACTION_BUILD=true
else
    for arg in "$@"; do
        case "$arg" in
            clean)
                ACTION_CLEAN=true
                ;;
            build)
                ACTION_BUILD=true
                ;;
            "clean build" | "untuk" | "semua") # "untuk semua" will be passed as two separate args "untuk" and "semua"
                ACTION_CLEAN=true
                ACTION_BUILD=true
                ;;
            *)
                echo "Unknown argument: $arg"
                echo "Usage: $0 [clean|build|\"clean build\"|untuk semua]"
                exit 1
                ;;
        esac
    done
fi

echo "Starting clean and build process with clean=$ACTION_CLEAN and build=$ACTION_BUILD."

# Process apps
for dir in apps/*; do
    if [ -d "$dir" ]; then
        if [ -f "$dir/package.json" ]; then
            clean_and_build_js_project "$dir" "$ACTION_CLEAN" "$ACTION_BUILD"
        elif [ -f "$dir/Cargo.toml" ]; then # Assuming Cargo.toml for Rust projects
            clean_and_build_rust_project "$dir" "$ACTION_CLEAN" "$ACTION_BUILD"
        else
            echo "Skipping unknown project type in $dir"
        fi
    fi
done

# Process packages
for dir in packages/*; do
    if [ -d "$dir" ]; then
        if [ -f "$dir/package.json" ]; then
            clean_and_build_js_project "$dir" "$ACTION_CLEAN" "$ACTION_BUILD"
        else
            echo "Skipping unknown project type in $dir"
        fi
    fi
done

echo "Clean and build process completed."