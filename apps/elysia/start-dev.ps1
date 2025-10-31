# ElysiaJS Development Server Starter
# This script ensures a clean start by checking for port availability

$PORT = 3001
$APP_DIR = "C:\ultimate-asepharyana.cloud\apps\elysia"

Write-Host "🦊 Starting ElysiaJS Development Server..." -ForegroundColor Cyan

# Check if port is in use
$portInUse = Get-NetTCPConnection -LocalPort $PORT -ErrorAction SilentlyContinue

if ($portInUse) {
    Write-Host "⚠️  Port $PORT is already in use" -ForegroundColor Yellow
    Write-Host "Attempting to free port..." -ForegroundColor Yellow

    # Try to find and kill the process
    $processId = $portInUse.OwningProcess
    if ($processId) {
        Stop-Process -Id $processId -Force -ErrorAction SilentlyContinue
        Write-Host "✅ Freed port $PORT" -ForegroundColor Green
        Start-Sleep -Seconds 2
    }
}

# Change to app directory and run
Set-Location $APP_DIR
Write-Host "📂 Working directory: $APP_DIR" -ForegroundColor Gray
Write-Host "🚀 Starting server on port $PORT..." -ForegroundColor Green

bun run --watch src/index.ts
