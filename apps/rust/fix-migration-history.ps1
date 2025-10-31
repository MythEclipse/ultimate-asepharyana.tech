#!/usr/bin/env pwsh
# Fix Rust migration history

Write-Host "Fixing Rust migration history..." -ForegroundColor Cyan

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
if ($dbUrl -match 'mysql://([^:]+):([^@]+)@([^:]+):(\d+)/(.+)') {
    $dbUser = $matches[1]
    $dbPass = $matches[2]
    $dbHost = $matches[3]
    $dbPort = $matches[4]
    $dbName = $matches[5]

    Write-Host "Database: $dbName@$dbHost" -ForegroundColor Yellow

    # Execute fix-migrations.sql
    if (Test-Path "apps/rust/fix-migrations.sql") {
        Write-Host "Cleaning up invalid migration records..." -ForegroundColor Yellow

        $mysqlCmd = "mysql -h$dbHost -P$dbPort -u$dbUser -p$dbPass $dbName"
        Get-Content "apps/rust/fix-migrations.sql" | & mysql -h $dbHost -P $dbPort -u $dbUser -p$dbPass $dbName

        Write-Host "Migration history fixed!" -ForegroundColor Green
    } else {
        Write-Host "fix-migrations.sql not found!" -ForegroundColor Red
        exit 1
    }
} else {
    Write-Host "Invalid DATABASE_URL format!" -ForegroundColor Red
    exit 1
}

Write-Host "`nDone! You can now restart the Rust app." -ForegroundColor Green
