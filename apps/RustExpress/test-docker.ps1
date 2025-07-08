# PowerShell Docker Test Setup for RustExpress
Write-Host "Setting up MySQL Test Database with Docker..." -ForegroundColor Blue

# Function to print colored output
function Write-Status {
    param([string]$Message)
    Write-Host "[INFO] $Message" -ForegroundColor Blue
}

function Write-Success {
    param([string]$Message)
    Write-Host "[SUCCESS] $Message" -ForegroundColor Green
}

function Write-Warning {
    param([string]$Message)
    Write-Host "[WARNING] $Message" -ForegroundColor Yellow
}

function Write-Error {
    param([string]$Message)
    Write-Host "[ERROR] $Message" -ForegroundColor Red
}

# Check if Docker is running
try {
    docker info | Out-Null
    Write-Status "Docker is running (check)"
} catch {
    Write-Error "Docker is not running. Please start Docker first."
    exit 1
}

# Stop any existing test containers
Write-Status "Stopping any existing test containers..."
# Handle .env file that conflicts with Docker Compose
$envBackup = $null
if (Test-Path ".env") {
    $envBackup = Get-Content ".env"
    Set-Content ".env" ""  # Create empty .env temporarily
}

docker-compose -f docker-compose.test.yml down -v

# Start MySQL test container
Write-Status "Starting MySQL test container..."
docker-compose -f docker-compose.test.yml up -d

# Restore .env file
if ($null -ne $envBackup) {
    Set-Content ".env" $envBackup
}

# Wait for MySQL to be ready
Write-Status "Waiting for MySQL to be ready..."
$timeout = 60
$counter = 0

do {
    if ($counter -eq $timeout) {
        Write-Error "MySQL failed to start within $timeout seconds"
        docker-compose -f docker-compose.test.yml logs mysql-test
        exit 1
    }
    Write-Status "Waiting for MySQL... ($counter/$timeout)"
    Start-Sleep -Seconds 2
    $counter += 2
    
    try {
        docker exec rustexpress-mysql-test mysqladmin ping -h"localhost" --silent
        $mysqlReady = $LASTEXITCODE -eq 0
    } catch {
        $mysqlReady = $false
    }
} while (-not $mysqlReady)

Write-Success "MySQL is ready!"

# Set test environment variables
$env:TEST_DATABASE_URL = "mysql://root:password@localhost:3307/test_rustexpress"
$env:RUST_LOG = "info"

Write-Status "Test Configuration:"
Write-Host "   Database: $env:TEST_DATABASE_URL" -ForegroundColor Gray
Write-Host "   Log Level: $env:RUST_LOG" -ForegroundColor Gray
Write-Host ""

# Run database migrations if needed
Write-Status "Running database setup..."
if (Test-Path "migrations/20250101000000_create_chat_messages_table.sql") {
    Write-Status "Applying database migrations..."
    Get-Content "migrations/20250101000000_create_chat_messages_table.sql" | docker exec -i rustexpress-mysql-test mysql -uroot -ppassword test_rustexpress
    if ($LASTEXITCODE -eq 0) {
        Write-Success "Database migrations applied successfully"
    } else {
        Write-Warning "Migration might have already been applied or failed"
    }
} else {
    Write-Status "Creating database table manually..."
    $createTableSQL = @"
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
"@
    $createTableSQL | docker exec -i rustexpress-mysql-test mysql -uroot -ppassword test_rustexpress
    if ($LASTEXITCODE -eq 0) {
        Write-Success "Database table created successfully"
    } else {
        Write-Warning "Failed to create database table"
    }
}

# Wait a moment for everything to settle
Write-Status "Waiting for database to be fully ready..."
Start-Sleep -Seconds 5

# Run tests
Write-Status "Running Unit Tests..."
Write-Host ""

# Run model tests first (these don't need database)
Write-Status "Running model tests..."
$env:TEST_DATABASE_URL = "mysql://root:password@localhost:3307/test_rustexpress"
cargo test models -- --nocapture --test-threads=1

Write-Host ""

# Run chat service tests (these need database)
Write-Status "Running chat_service tests..."
$env:TEST_DATABASE_URL = "mysql://root:password@localhost:3307/test_rustexpress"
cargo test chat_service -- --nocapture --test-threads=1

Write-Host ""

# Run all tests
Write-Status "Running all tests..."
cargo test -- --nocapture --test-threads=1

# Test results
if ($LASTEXITCODE -eq 0) {
    Write-Success "All tests passed!"
} else {
    Write-Error "Some tests failed. Check the output above."
}

Write-Host ""
Write-Status "Test run completed."

# Ask if user wants to keep containers running
$keep = Read-Host "Keep MySQL container running for manual testing? (y/n)"
if ($keep -notmatch "^[Yy]") {
    Write-Status "Stopping test containers..."
    # Handle .env file for cleanup
    $envBackupCleanup = $null
    if (Test-Path ".env") {
        $envBackupCleanup = Get-Content ".env"
        Set-Content ".env" ""  # Create empty .env temporarily
    }
    
    docker-compose -f docker-compose.test.yml down
    
    # Restore .env file
    if ($null -ne $envBackupCleanup) {
        Set-Content ".env" $envBackupCleanup
    }
    
    Write-Success "Test containers stopped."
} else {
    Write-Status "MySQL container is still running on port 3307"
    Write-Status "To stop it later, run: docker-compose -f docker-compose.test.yml down"
}

Write-Host ""
Write-Success "Test setup complete!"
