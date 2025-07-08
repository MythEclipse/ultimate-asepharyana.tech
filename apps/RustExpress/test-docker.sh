#!/bin/bash

# Docker Test Setup for RustExpress
echo "🐳 Setting up MySQL Test Database with Docker..."

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

# Check if Docker is running
if ! docker info > /dev/null 2>&1; then
    print_error "Docker is not running. Please start Docker first."
    exit 1
fi

print_status "Docker is running ✓"

# Stop any existing test containers
print_status "Stopping any existing test containers..."
# Handle .env file that conflicts with Docker Compose
env_backup=""
if [ -f ".env" ]; then
    env_backup=$(cat .env)
    echo "" > .env  # Create empty .env temporarily
fi

docker-compose -f docker-compose.test.yml down -v

# Start MySQL test container
print_status "Starting MySQL test container..."
docker-compose -f docker-compose.test.yml up -d

# Restore .env file
if [ -n "$env_backup" ]; then
    echo "$env_backup" > .env
fi

# Wait for MySQL to be ready
print_status "Waiting for MySQL to be ready..."
timeout=60
counter=0

while ! docker exec rustexpress-mysql-test mysqladmin ping -h"localhost" --silent; do
    if [ $counter -eq $timeout ]; then
        print_error "MySQL failed to start within $timeout seconds"
        docker-compose -f docker-compose.test.yml logs mysql-test
        exit 1
    fi
    print_status "Waiting for MySQL... ($counter/$timeout)"
    sleep 2
    counter=$((counter + 2))
done

print_success "MySQL is ready!"

# Set test environment variables
export TEST_DATABASE_URL="mysql://root:password@localhost:3307/test_rustexpress"
export RUST_LOG=info

print_status "Test Configuration:"
echo "   Database: $TEST_DATABASE_URL"
echo "   Log Level: $RUST_LOG"
echo ""

# Run database migrations if needed
print_status "Running database setup..."
if [ -f "migrations/20250101000000_create_chat_messages_table.sql" ]; then
    print_status "Applying database migrations..."
    docker exec -i rustexpress-mysql-test mysql -uroot -ppassword test_rustexpress < migrations/20250101000000_create_chat_messages_table.sql
    if [ $? -eq 0 ]; then
        print_success "Database migrations applied successfully"
    else
        print_warning "Migration might have already been applied or failed"
    fi
else
    print_status "Creating database table manually..."
    docker exec -i rustexpress-mysql-test mysql -uroot -ppassword test_rustexpress << 'EOF'
CREATE TABLE IF NOT EXISTS chat_messages (
    id VARCHAR(36) PRIMARY KEY NOT NULL,
    user_id VARCHAR(255) NOT NULL,
    text TEXT NOT NULL,
    email VARCHAR(255),
    image_profile TEXT,
    image_message TEXT,
    role VARCHAR(50) NOT NULL,
    timestamp VARCHAR(50) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);
EOF
    if [ $? -eq 0 ]; then
        print_success "Database table created successfully"
    else
        print_warning "Failed to create database table"
    fi
fi

# Wait a moment for everything to settle
print_status "Waiting for database to be fully ready..."
sleep 5

# Run tests
print_status "Running Unit Tests..."
echo ""

# Run model tests first (these don't need database)
print_status "Running model tests..."
export TEST_DATABASE_URL="mysql://root:password@localhost:3307/test_rustexpress"
cargo test models -- --nocapture --test-threads=1

echo ""

# Run chat service tests (these need database)
print_status "Running chat_service tests..."
export TEST_DATABASE_URL="mysql://root:password@localhost:3307/test_rustexpress"
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
print_status "Test run completed."

# Ask if user wants to keep containers running
read -p "Keep MySQL container running for manual testing? (y/n): " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    print_status "Stopping test containers..."
    # Handle .env file for cleanup
    env_backup_cleanup=""
    if [ -f ".env" ]; then
        env_backup_cleanup=$(cat .env)
        echo "" > .env  # Create empty .env temporarily
    fi
    
    docker-compose -f docker-compose.test.yml down
    
    # Restore .env file
    if [ -n "$env_backup_cleanup" ]; then
        echo "$env_backup_cleanup" > .env
    fi
    
    print_success "Test containers stopped."
else
    print_status "MySQL container is still running on port 3307"
    print_status "To stop it later, run: docker-compose -f docker-compose.test.yml down"
fi

echo ""
print_success "Test setup complete!"
