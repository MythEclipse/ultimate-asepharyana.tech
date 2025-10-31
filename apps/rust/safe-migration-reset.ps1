#!/usr/bin/env pwsh
# Safe migration reset and re-run script

Write-Host "=== Safe Migration Reset ===" -ForegroundColor Cyan
Write-Host ""

# Load .env file
if (Test-Path "apps/rust/.env") {
    Get-Content "apps/rust/.env" | ForEach-Object {
        if ($_ -match '^([^=]+)=(.*)$') {
            $name = $matches[1].Trim()
            $value = $matches[2].Trim()
            [Environment]::SetEnvironmentVariable($name, $value, "Process")
        }
    }
}

# Parse DATABASE_URL
$dbUrl = $env:DATABASE_URL
if ($dbUrl -match 'mysql://([^:]+):([^@]+)@([^:/]+)(?::(\d+))?/(.+)') {
    $dbUser = $matches[1]
    $dbPass = $matches[2]
    $dbHost = $matches[3]
    $dbPort = if ($matches[4]) { $matches[4] } else { "3306" }
    $dbName = $matches[5]

    Write-Host "Database: $dbName @ $dbHost:$dbPort" -ForegroundColor Yellow
    Write-Host "User: $dbUser" -ForegroundColor Yellow
    Write-Host ""

    # Confirm action
    $confirm = Read-Host "This will reset migration history. Continue? (yes/no)"
    if ($confirm -ne "yes") {
        Write-Host "Aborted." -ForegroundColor Red
        exit 0
    }

    Write-Host ""
    Write-Host "Step 1: Backing up current migration state..." -ForegroundColor Cyan

    # Create backup
    $backupFile = "migration_backup_$(Get-Date -Format 'yyyyMMdd_HHmmss').sql"
    $query = "SELECT * FROM _sqlx_migrations ORDER BY version;"

    try {
        # Show current state
        Write-Host "Current migrations:" -ForegroundColor Yellow
        & mysql -h $dbHost -P $dbPort -u $dbUser -p$dbPass $dbName -e $query

        Write-Host ""
        Write-Host "Step 2: Cleaning migration history..." -ForegroundColor Cyan

        # Run fix-migrations.sql
        Get-Content "apps/rust/fix-migrations.sql" | & mysql -h $dbHost -P $dbPort -u $dbUser -p$dbPass $dbName

        Write-Host ""
        Write-Host "Step 3: Verifying migration state..." -ForegroundColor Cyan
        & mysql -h $dbHost -P $dbPort -u $dbUser -p$dbPass $dbName -e $query

        Write-Host ""
        Write-Host "✓ Migration history cleaned successfully!" -ForegroundColor Green
        Write-Host ""
        Write-Host "Next steps:" -ForegroundColor Cyan
        Write-Host "1. Restart the Rust app: pm2 restart RustExpr" -ForegroundColor White
        Write-Host "2. Check logs: pm2 logs RustExpr" -ForegroundColor White

    } catch {
        Write-Host "Error: $_" -ForegroundColor Red
        exit 1
    }

} else {
    Write-Host "Invalid DATABASE_URL format!" -ForegroundColor Red
    Write-Host "Expected: mysql://user:pass@host:port/database" -ForegroundColor Yellow
    exit 1
}
