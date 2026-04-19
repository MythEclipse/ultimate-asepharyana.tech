#!/bin/bash

# Cleanup script for old GitHub Container Registry (GHCR) images
# This script uses the 'gh' CLI to delete old package versions.
# Requires 'gh' CLI to be installed and authenticated with 'delete:packages' scope.

set -e

# Configuration
ORG="MythEclipse"
PACKAGE_NAMES=("rust-api" "solidjs-web" "leptos-web" "elysia-api" "nextjs-web")

echo "🚀 Starting GHCR cleanup for $ORG..."

for PACKAGE in "${PACKAGE_NAMES[@]}"; do
    echo "------------------------------------------------"
    echo "📦 Checking package: $PACKAGE"
    
    # List versions that are NOT 'latest' and DON'T match the current SHAs
    # This is a safe approach: list all versions and let the user decide or 
    # filter by date/tag patterns.
    
    # For simplicity and safety, this script will list versions and 
    # provide the command to delete them. 
    # To AUTOMATICALLY delete, uncomment the 'gh api' call below.
    
    echo "🔍 Fetching versions..."
    VERSIONS=$(gh api "/orgs/$ORG/packages/container/$PACKAGE/versions" --paginate -q '.[] | "\(.id) \(.metadata.container.tags[0] // "no-tag") \(.updated_at)"')
    
    if [ -z "$VERSIONS" ]; then
        echo "✅ No versions found for $PACKAGE"
        continue
    fi
    
    echo "$VERSIONS" | while read -r ID TAG DATE; do
        if [[ "$TAG" == "latest" ]]; then
            echo "✨ Skipping latest: $ID ($DATE)"
            continue
        fi
        
        # Example: only delete if the tag doesn't start with 'sha-' (adjust as needed)
        # Or delete very old ones.
        
        echo "🗑️ Found old version: $ID | Tag: $TAG | Date: $DATE"
        
        # UNCOMMENT THE LINE BELOW TO ENABLE AUTOMATIC DELETION
        # gh api -X DELETE "/orgs/$ORG/packages/container/$PACKAGE/versions/$ID"
        # echo "✅ Deleted $ID"
    done
done

echo "------------------------------------------------"
echo "✅ Cleanup script finished."
echo "💡 Note: Deletion is commented out by default for safety. Edit the script to enable it."
