#!/bin/bash
# Script to run database migrations manually

echo "Running database migrations..."
cd "$(dirname "$0")/.."

# Build and run the migration binary
cargo run --bin migrate

if [ $? -eq 0 ]; then
    echo "✅ Migration completed successfully."
else
    echo "❌ Migration failed."
    exit 1
fi
