# Script untuk reset database MySQL localhost
# Run: .\reset-db.ps1

Write-Host "Resetting MySQL database..." -ForegroundColor Yellow

# Drop dan create database sosmed
mysql -u root -e "DROP DATABASE IF EXISTS sosmed;"
mysql -u root -e "CREATE DATABASE sosmed CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;"

Write-Host "Database reset complete!" -ForegroundColor Green
Write-Host "Database 'sosmed' created fresh" -ForegroundColor Cyan

# Info
Write-Host ""
Write-Host "Next steps:" -ForegroundColor Cyan
Write-Host "   1. Run: cargo run --bin rust" -ForegroundColor White
Write-Host "   2. Migrations and seed will run automatically" -ForegroundColor White
