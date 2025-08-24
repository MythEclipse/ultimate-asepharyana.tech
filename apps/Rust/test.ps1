# P# Set test environment variables
$env:TEST_DATABASE_URL = "mysql://root:password@localhost:3307/test_rustexpress"
$env:RUST_LOG = "debug"rShell Test script for RustExpress
Write-Host "🧪 Running RustExpress Unit Tests..." -ForegroundColor Blue

# Set test environment variables
$env:TEST_DATABASE_URL = "mysql://root:@localhost:3306/test_rustexpress"
$env:RUST_LOG = "debug"

Write-Host "📝 Test Configuration:" -ForegroundColor Yellow
Write-Host "   Database: $env:TEST_DATABASE_URL" -ForegroundColor Gray
Write-Host "   Log Level: $env:RUST_LOG" -ForegroundColor Gray
Write-Host ""

# Run only model tests first (no database required)
Write-Host "🔍 Running model tests (no database required)..." -ForegroundColor Cyan
cargo test models -- --nocapture

Write-Host ""
Write-Host "🔍 Running all unit tests..." -ForegroundColor Cyan
cargo test -- --nocapture

Write-Host ""
Write-Host "📊 Test Coverage (if available)..." -ForegroundColor Yellow
# Uncomment if you have cargo-tarpaulin installed
# cargo tarpaulin --out Html --output-dir target/coverage

Write-Host ""
Write-Host "✅ Tests completed!" -ForegroundColor Green
