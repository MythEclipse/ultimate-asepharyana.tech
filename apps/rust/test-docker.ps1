# RustExpress Docker Test Setup (PowerShell)

function Info($msg)    { Write-Host "[INFO] $msg" -ForegroundColor Blue }
function Success($msg) { Write-Host "[SUCCESS] $msg" -ForegroundColor Green }
function Warn($msg)    { Write-Host "[WARNING] $msg" -ForegroundColor Yellow }
function Error($msg)   { Write-Host "[ERROR] $msg" -ForegroundColor Red }

try {
    docker info | Out-Null
    Info "Docker is running ✓"
} catch {
    Error "Docker is not running. Please start Docker first."
    exit 1
}

Info "Stopping any existing test containers..."
$envBackup = $null
if (Test-Path ".env") {
    $envBackup = Get-Content ".env"
    Set-Content ".env" ""
}

docker-compose -f docker-compose.test.yml down -v

Info "Starting MySQL test container..."
docker-compose -f docker-compose.test.yml up -d

if ($null -ne $envBackup) {
    Set-Content ".env" $envBackup
}

Info "Waiting for MySQL to be ready..."
$timeout = 60
$counter = 0
do {
    if ($counter -ge $timeout) {
        Error "MySQL failed to start within $timeout seconds"
        docker-compose -f docker-compose.test.yml logs mysql-test
        exit 1
    }
    Info "Waiting for MySQL... ($counter/$timeout)"
    Start-Sleep -Seconds 2
    $counter += 2
    try {
        docker exec rustexpress-mysql-test mysqladmin ping -h"localhost" --silent
        $mysqlReady = $LASTEXITCODE -eq 0
    } catch {
        $mysqlReady = $false
    }
} while (-not $mysqlReady)

Success "MySQL is ready!"

$env:TEST_DATABASE_URL = "mysql://root:password@localhost:3307/test_rustexpress"
$env:RUST_LOG = "info"

Info "Test Configuration:"
Write-Host "   Database: $env:TEST_DATABASE_URL" -ForegroundColor Gray
Write-Host "   Log Level: $env:RUST_LOG" -ForegroundColor Gray
Write-Host ""

Info "Running database setup..."
if (Test-Path "migrations/20250101000000_create_chat_messages_table.sql") {
    Info "Applying database migrations..."
    Get-Content "migrations/20250101000000_create_chat_messages_table.sql" | docker exec -i rustexpress-mysql-test mysql -uroot -ppassword test_rustexpress
    if ($LASTEXITCODE -eq 0) {
        Success "Database migrations applied successfully"
    } else {
        Warn "Migration might have already been applied or failed"
    }
} else {
    Info "Creating database table manually..."
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
        Success "Database table created successfully"
    } else {
        Warn "Failed to create database table"
    }
}

Info "Waiting for database to be fully ready..."
Start-Sleep -Seconds 5

Info "Running Unit Tests..."
Write-Host ""

Info "Running model tests..."
cargo test models -- --nocapture --test-threads=1

Write-Host ""
Info "Running chat_service tests..."
cargo test chat_service -- --nocapture --test-threads=1

Write-Host ""
Info "Running all tests..."
cargo test -- --nocapture --test-threads=1

if ($LASTEXITCODE -eq 0) {
    Success "All tests passed!"
} else {
    Error "Some tests failed. Check the output above."
}

Write-Host ""
Info "Test run completed."

$keep = Read-Host "Keep MySQL container running for manual testing? (y/n)"
if ($keep -notmatch "^[Yy]") {
    Info "Stopping test containers..."
    $envBackupCleanup = $null
    if (Test-Path ".env") {
        $envBackupCleanup = Get-Content ".env"
        Set-Content ".env" ""
    }
    docker-compose -f docker-compose.test.yml down
    if ($null -ne $envBackupCleanup) {
        Set-Content ".env" $envBackupCleanup
    }
    Success "Test containers stopped."
} else {
    Info "MySQL container is still running on port 3307"
    Info "To stop it later, run: docker-compose -f docker-compose.test.yml down"
}

Write-Host ""
Success "Test setup complete!"
