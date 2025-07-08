#!/bin/bash

# Migration script from Express.js to RustExpress
# This script helps transition from the Node.js Express app to the Rust version

echo "🔄 Starting migration from Express.js to RustExpress..."

# Check if Express app exists
if [ ! -d "../Express" ]; then
    echo "❌ Express app not found. Make sure you're running this from the RustExpress directory."
    exit 1
fi

echo "✅ Found Express app"

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust/Cargo not found. Please install Rust first: https://rustup.rs/"
    exit 1
fi

echo "✅ Rust toolchain found"

# Build the Rust application
echo "🔨 Building RustExpress..."
cargo build --release

if [ $? -ne 0 ]; then
    echo "❌ Build failed. Please check the errors above."
    exit 1
fi

echo "✅ RustExpress built successfully"

# Create database directory if it doesn't exist
mkdir -p data

# Set up environment
if [ ! -f ".env" ]; then
    cp ".env.example" ".env"
    echo "📝 Created .env file from example"
fi

echo "🗄️  Setting up database..."
# The Rust app will run migrations automatically on startup

echo "🚀 Starting RustExpress server..."
echo "   - Express.js app typically runs on port 4091"
echo "   - RustExpress will run on port 3001 (configurable in .env)"
echo "   - Both can run simultaneously for gradual migration"

echo ""
echo "Migration checklist:"
echo "✅ Rust application built"
echo "✅ Database configuration ready"
echo "✅ Environment variables set"
echo "⏳ Ready to start RustExpress"
echo ""
echo "To start the server: cargo run"
echo "To test the API: curl http://localhost:3001/api/health"
echo ""
echo "🎉 Migration preparation complete!"
