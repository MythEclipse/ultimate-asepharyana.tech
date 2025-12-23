#!/usr/bin/env bash
# Auto-fix warnings and lint Rust code

set -e

echo "🔧 Auto-fixing Rust warnings and linting..."

# Format code
echo "📝 Running rustfmt..."
cargo fmt

# Fix clippy warnings automatically
echo "🔍 Running clippy auto-fix..."
cargo clippy --fix --allow-dirty --allow-staged --all-targets

# Check for remaining issues
echo "✅ Running final check..."
cargo clippy --all-targets -- -D warnings

echo "✨ Done! Code is formatted and linted."
