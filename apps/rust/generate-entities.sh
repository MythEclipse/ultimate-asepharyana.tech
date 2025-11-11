#!/bin/bash
# Script untuk generate entities dari database menggunakan sea-orm-cli
# Install sea-orm-cli jika belum ada: cargo install sea-orm-cli

set -e

echo "Generating entities from database..."

# Load environment variables from .env
if [ -f .env ]; then
    export $(cat .env | grep -v '^#' | xargs)
fi

if [ -z "$DATABASE_URL" ]; then
    echo "DATABASE_URL not found in environment"
    exit 1
fi

echo "Using DATABASE_URL: $DATABASE_URL"

# Generate entities
sea-orm-cli generate entity \
    --database-url "$DATABASE_URL" \
    --output-dir src/entities \
    --with-serde both \
    --expanded-format

echo "✓ Entities generated successfully in src/entities/"
echo "You can now use these entities in your application"
