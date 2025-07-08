#!/bin/bash

# Test script for RustExpress
echo "🧪 Running RustExpress Unit Tests..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Set test environment variables
export TEST_DATABASE_URL="mysql://root:password@localhost:3307/test_rustexpress"
export RUST_LOG=info

print_status "Test Configuration:"
echo "   Database: $TEST_DATABASE_URL"
echo "   Log Level: $RUST_LOG"
echo ""

# Run tests
print_status "Running Unit Tests..."
echo ""

# Run model tests first (these don't need database)
print_status "Running model tests..."
cargo test models -- --nocapture --test-threads=1

echo ""

# Run chat service tests (these need database)
print_status "Running chat_service tests..."
cargo test chat_service -- --nocapture --test-threads=1

echo ""

# Run all tests
print_status "Running all tests..."
cargo test -- --nocapture --test-threads=1

# Test results
if [ $? -eq 0 ]; then
    print_success "All tests passed!"
else
    print_error "Some tests failed. Check the output above."
fi

echo ""
print_success "Test run completed!"
