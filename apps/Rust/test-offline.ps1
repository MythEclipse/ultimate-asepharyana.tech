# PowerShell Test script for RustExpress (Offline - No Database Required)
Write-Host "Running RustExpress Unit Tests (Offline Mode)..." -ForegroundColor Blue

Write-Host "Running tests that don't require database connection..." -ForegroundColor Yellow
Write-Host ""

# Run only model tests (no database required)
Write-Host "Running ChatMessage model tests..." -ForegroundColor Cyan
cargo test models -- --nocapture

Write-Host ""
Write-Host "Test Summary:" -ForegroundColor Yellow
Write-Host "   Model tests: ChatMessage creation, serialization, validation" -ForegroundColor Green
Write-Host "   Database tests: Skipped (require MySQL connection)" -ForegroundColor Gray
Write-Host ""
Write-Host "To run database tests:" -ForegroundColor Yellow
Write-Host "   1. Setup MySQL server (localhost:3306)" -ForegroundColor Gray
Write-Host "   2. Create test database: test_rustexpress" -ForegroundColor Gray
Write-Host "   3. Run: .\test.ps1" -ForegroundColor Gray
Write-Host ""
Write-Host "Offline tests completed!" -ForegroundColor Green
