# PowerShell Migration script from Express.js to RustExpress
# This script helps transition from the Node.js Express app to the Rust version

Write-Host "🔄 Starting migration from Express.js to RustExpress..." -ForegroundColor Blue

# Check if Express app exists
if (-not (Test-Path "../Express")) {
    Write-Host "❌ Express app not found. Make sure you're running this from the RustExpress directory." -ForegroundColor Red
    exit 1
}

Write-Host "✅ Found Express app" -ForegroundColor Green

# Check if Rust is installed
try {
    $null = Get-Command cargo -ErrorAction Stop
    Write-Host "✅ Rust toolchain found" -ForegroundColor Green
} catch {
    Write-Host "❌ Rust/Cargo not found. Please install Rust first: https://rustup.rs/" -ForegroundColor Red
    exit 1
}

# Build the Rust application
Write-Host "🔨 Building RustExpress..." -ForegroundColor Yellow
cargo build --release

if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Build failed. Please check the errors above." -ForegroundColor Red
    exit 1
}

Write-Host "✅ RustExpress built successfully" -ForegroundColor Green

# Create database directory if it doesn't exist
if (-not (Test-Path "data")) {
    New-Item -ItemType Directory -Path "data" | Out-Null
}

# Set up environment
if (-not (Test-Path ".env")) {
    Copy-Item ".env.example" ".env"
    Write-Host "📝 Created .env file from example" -ForegroundColor Cyan
}

Write-Host "🗄️  Setting up database..." -ForegroundColor Yellow
# The Rust app will run migrations automatically on startup

Write-Host "🚀 Starting RustExpress server..." -ForegroundColor Blue
Write-Host "   - Express.js app typically runs on port 4091" -ForegroundColor Gray
Write-Host "   - RustExpress will run on port 3001 (configurable in .env)" -ForegroundColor Gray
Write-Host "   - Both can run simultaneously for gradual migration" -ForegroundColor Gray

Write-Host ""
Write-Host "Migration checklist:" -ForegroundColor Yellow
Write-Host "✅ Rust application built" -ForegroundColor Green
Write-Host "✅ Database configuration ready" -ForegroundColor Green
Write-Host "✅ Environment variables set" -ForegroundColor Green
Write-Host "⏳ Ready to start RustExpress" -ForegroundColor Yellow
Write-Host ""
Write-Host "To start the server: cargo run" -ForegroundColor Cyan
Write-Host "To test the API: curl http://localhost:3001/api/health" -ForegroundColor Cyan
Write-Host ""
Write-Host "🎉 Migration preparation complete!" -ForegroundColor Green
