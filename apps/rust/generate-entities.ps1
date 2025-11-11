# Script untuk generate entities dari database menggunakan sea-orm-cli
# Install sea-orm-cli jika belum ada: cargo install sea-orm-cli

Write-Host "Generating entities from database..." -ForegroundColor Green

# Load environment variables from .env
if (Test-Path ".env") {
    Get-Content .env | ForEach-Object {
        if ($_ -match '^\s*([^#][^=]+)=(.*)$') {
            $name = $matches[1].Trim()
            $value = $matches[2].Trim()
            [Environment]::SetEnvironmentVariable($name, $value, "Process")
        }
    }
}

$DATABASE_URL = $env:DATABASE_URL
if (-not $DATABASE_URL) {
    Write-Host "DATABASE_URL not found in environment" -ForegroundColor Red
    exit 1
}

Write-Host "Using DATABASE_URL: $DATABASE_URL" -ForegroundColor Cyan

# Generate entities
sea-orm-cli generate entity `
    --database-url "$DATABASE_URL" `
    --output-dir src/entities `
    --with-serde both `
    --expanded-format

if ($LASTEXITCODE -eq 0) {
    Write-Host "Success: Entities generated successfully in src/entities/" -ForegroundColor Green
    Write-Host "You can now use these entities in your application" -ForegroundColor Cyan
} else {
    Write-Host "Error: Failed to generate entities" -ForegroundColor Red
    exit 1
}
