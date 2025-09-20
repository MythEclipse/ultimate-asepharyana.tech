# Testing Guide for RustExpress

This document explains how to run unit tests for the RustExpress application.

## Prerequisites

- Docker and Docker Compose installed
- Rust and Cargo installed
- PowerShell (for Windows) or Bash (for Unix/Linux)

## Test Types

### 1. Model Tests (No Database Required)

These tests verify the ChatMessage model functionality:

- Message creation
- Serialization/Deserialization
- Field validation
- Cloning

### 2. Service Tests (Database Required)

These tests verify the chat service functionality:

- Saving messages to database
- Loading messages from database
- Message ordering
- Pagination
- Error handling

## Running Tests

### Option 1: Using Docker (Recommended)

This method automatically sets up a MySQL test database using Docker.

**Windows (PowerShell):**

```powershell
.\test-docker.ps1
```

**Unix/Linux (Bash):**

```bash
chmod +x test-docker.sh
./test-docker.sh
```

This script will:

1. Start a MySQL container on port 3307
2. Create the test database
3. Run all tests
4. Optionally clean up the container

### Option 2: Manual Database Setup

If you have a MySQL server running locally:

**Windows:**

```powershell
.\test.ps1
```

**Unix/Linux:**

```bash
chmod +x test.sh
./test.sh
```

Make sure your MySQL server is configured with:

- Host: localhost:3307 (or update the scripts)
- Username: root
- Password: password
- Database: test_rustexpress

### Option 3: Model Tests Only

To run only the model tests (no database required):

```bash
cargo test models -- --nocapture
```

### Option 4: All Tests with Cargo

```bash
# Set environment variable first
export TEST_DATABASE_URL="mysql://root:password@localhost:3307/test_rustexpress"

# Run all tests
cargo test -- --nocapture
```

## Test Configuration

### Environment Variables

- `TEST_DATABASE_URL`: MySQL connection string for tests
- `RUST_LOG`: Log level for tests (debug, info, warn, error)

### Database Schema

The tests require a `chat_messages` table. The schema is defined in:

```
migrations/20250101000000_create_chat_messages_table.sql
```

## Test Coverage

Current test coverage includes:

### ChatMessage Model Tests

- ✅ `test_chat_message_new()` - Message creation with all fields
- ✅ `test_chat_message_new_minimal()` - Message creation with minimal fields
- ✅ `test_chat_message_serialization()` - JSON serialization
- ✅ `test_chat_message_deserialization()` - JSON deserialization
- ✅ `test_chat_message_clone()` - Object cloning

### Chat Service Tests

- ✅ `test_save_message_success()` - Successful message saving
- ✅ `test_save_message_with_minimal_fields()` - Saving with optional fields
- ✅ `test_load_messages_empty()` - Loading from empty database
- ✅ `test_load_messages_with_data()` - Loading multiple messages
- ✅ `test_load_messages_with_limit()` - Pagination functionality
- ✅ `test_load_messages_order()` - Message ordering (newest first)
- ✅ `test_save_message_duplicate_id()` - Duplicate ID handling
- ✅ `test_save_message_validates_required_fields()` - Field validation

## Troubleshooting

### Common Issues

1. **Database Connection Failed**
   - Make sure Docker is running
   - Check if port 3307 is available
   - Verify MySQL container is healthy

2. **Tests Timeout**
   - Increase timeout in test scripts
   - Check system resources

3. **Permission Denied**
   - Make scripts executable: `chmod +x test-docker.sh`
   - Run PowerShell as administrator if needed

### Docker Commands

**Check container status:**

```bash
docker-compose -f docker-compose.test.yml ps
```

**View container logs:**

```bash
docker-compose -f docker-compose.test.yml logs mysql-test
```

**Stop test containers:**

```bash
docker-compose -f docker-compose.test.yml down
```

**Clean up volumes:**

```bash
docker-compose -f docker-compose.test.yml down -v
```

## CI/CD Integration

For automated testing in CI/CD pipelines, use the Docker-based approach:

```yaml
# Example GitHub Actions
steps:
  - name: Setup MySQL Test Database
    run: docker-compose -f apps/RustExpress/docker-compose.test.yml up -d

  - name: Wait for MySQL
    run: |
      timeout 60 bash -c 'while ! docker exec rustexpress-mysql-test mysqladmin ping -h"localhost" --silent; do sleep 2; done'

  - name: Run Tests
    env:
      TEST_DATABASE_URL: mysql://root:password@localhost:3307/test_rustexpress
    run: |
      cd apps/RustExpress
      cargo test -- --nocapture
```

## Performance Notes

- Model tests are fast (< 1 second)
- Service tests depend on database performance
- Docker startup adds ~10-30 seconds overhead
- Consider using testcontainers for isolated tests in production
