#!/usr/bin/env bash
# Pre-build hook to enforce best practices

set -e

echo "🔍 Running pre-build lint checks..."

# Format check
echo "📝 Checking code formatting..."
if ! cargo fmt -- --check; then
    echo "❌ Code not formatted. Run: cargo fmt"
    exit 1
fi

# Clippy check with denials
echo "🔧 Running clippy..."
cargo clippy --all-targets -- -D warnings

echo "✅ All lint checks passed!"
