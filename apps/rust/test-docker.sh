#!/bin/bash

# RustExpress Docker Test Setup

set -e

BLUE='\033[0;34m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

info()    { echo -e "${BLUE}[INFO]${NC} $1"; }
success() { echo -e "${GREEN}[SUCCESS]${NC} $1"; }
warn()    { echo -e "${YELLOW}[WARNING]${NC} $1"; }
error()   { echo -e "${RED}[ERROR]${NC} $1"; }

if ! docker info > /dev/null 2>&1; then
    error "Docker is not running. Please start Docker first."
    exit 1
fi

info "Docker is running ✓"
info "Stopping any existing test containers..."

env_backup=""
if [ -f ".env" ]; then
    env_backup=$(cat .env)
    echo "" > .env
fi

docker-compose -f docker-compose.test.yml down -v

info "Starting MySQL test container..."
docker-compose -f docker-compose.test.yml up -d

if [ -n "$env_backup" ]; then
    echo "$env_backup" > .env
fi

info "Waiting for MySQL to be ready..."
timeout=60
counter=0
while ! docker exec rustexpress-mysql-test mysqladmin ping -h"localhost" --silent; do
    if [ $counter -ge $timeout ]; then
        error "MySQL failed to start within $timeout seconds"
        docker-compose -f docker-compose.test.yml logs mysql-test
        exit 1
    fi
    info "Waiting for MySQL... ($counter/$timeout)"
    sleep 2
    counter=$((counter + 2))
done

success "MySQL is ready!"

export TEST_DATABASE_URL="mysql://root:password@localhost:3307/test_rustexpress"
export RUST_LOG=info

info "Test Configuration:"
echo "   Database: $TEST_DATABASE_URL"
echo "   Log Level: $RUST_LOG"
echo ""

info "Running database setup..."
if [ -f "migrations/20250101000000_create_chat_messages_table.sql" ]; then
    info "Applying database migrations..."
    docker exec -i rustexpress-mysql-test mysql -uroot -ppassword test_rustexpress < migrations/20250101000000_create_chat_messages_table.sql \
        && success "Database migrations applied successfully" \
        || warn "Migration might have already been applied or failed"
else
    info "Creating database table manually..."
    docker exec -i rustexpress-mysql-test mysql -uroot -ppassword test_rustexpress <<'EOF'
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
    [ $? -eq 0 ] && success "Database table created successfully" || warn "Failed to create database table"
fi

info "Waiting for database to be fully ready..."
sleep 5

info "Running Unit Tests..."
echo ""

info "Running model tests..."
cargo test models -- --nocapture --test-threads=1

echo ""
info "Running chat_service tests..."
cargo test chat_service -- --nocapture --test-threads=1

echo ""
info "Running all tests..."
cargo test -- --nocapture --test-threads=1

if [ $? -eq 0 ]; then
    success "All tests passed!"
else
    error "Some tests failed. Check the output above."
fi

echo ""
info "Test run completed."

read -p "Keep MySQL container running for manual testing? (y/n): " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    info "Stopping test containers..."
    env_backup_cleanup=""
    if [ -f ".env" ]; then
        env_backup_cleanup=$(cat .env)
        echo "" > .env
    fi
    docker-compose -f docker-compose.test.yml down
    if [ -n "$env_backup_cleanup" ]; then
        echo "$env_backup_cleanup" > .env
    fi
    success "Test containers stopped."
else
    info "MySQL container is still running on port 3307"
    info "To stop it later, run: docker-compose -f docker-compose.test.yml down"
fi

echo ""
success "Test setup complete!"
