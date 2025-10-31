# ElysiaJS Authentication System Test Script
# Make sure the server is running before executing this script

$BASE_URL = "http://localhost:3002"
$EMAIL = "test@example.com"
$USERNAME = "testuser"
$PASSWORD = "Test123!@#"

Write-Host "🧪 Testing ElysiaJS Authentication System" -ForegroundColor Cyan
Write-Host "=========================================" -ForegroundColor Cyan
Write-Host ""

# Test 1: Health Check
Write-Host "1️⃣  Testing Health Check..." -ForegroundColor Yellow
try {
    $health = Invoke-RestMethod -Uri "$BASE_URL/health" -Method Get
    Write-Host "✅ Health check passed" -ForegroundColor Green
    Write-Host ($health | ConvertTo-Json) -ForegroundColor Gray
} catch {
    Write-Host "❌ Health check failed: $_" -ForegroundColor Red
    exit 1
}
Write-Host ""

# Test 2: Register
Write-Host "2️⃣  Testing User Registration..." -ForegroundColor Yellow
try {
    $registerBody = @{
        email = $EMAIL
        username = $USERNAME
        password = $PASSWORD
        full_name = "Test User"
    } | ConvertTo-Json

    $registerResponse = Invoke-RestMethod -Uri "$BASE_URL/api/auth/register" `
        -Method Post `
        -ContentType "application/json" `
        -Body $registerBody

    Write-Host "✅ Registration successful" -ForegroundColor Green
    Write-Host "   User ID: $($registerResponse.user.id)" -ForegroundColor Gray
    Write-Host "   Email: $($registerResponse.user.email)" -ForegroundColor Gray
    Write-Host "   Username: $($registerResponse.user.username)" -ForegroundColor Gray

    $VERIFICATION_TOKEN = $registerResponse.verification_token
} catch {
    if ($_.Exception.Response.StatusCode -eq 400) {
        Write-Host "⚠️  User might already exist, continuing..." -ForegroundColor Yellow
    } else {
        Write-Host "❌ Registration failed: $_" -ForegroundColor Red
        Write-Host $_.Exception.Message -ForegroundColor Red
    }
}
Write-Host ""

# Test 3: Login
Write-Host "3️⃣  Testing User Login..." -ForegroundColor Yellow
try {
    $loginBody = @{
        login = $EMAIL
        password = $PASSWORD
        remember_me = $false
    } | ConvertTo-Json

    $loginResponse = Invoke-RestMethod -Uri "$BASE_URL/api/auth/login" `
        -Method Post `
        -ContentType "application/json" `
        -Body $loginBody

    Write-Host "✅ Login successful" -ForegroundColor Green
    Write-Host "   Access Token: $($loginResponse.access_token.Substring(0, 20))..." -ForegroundColor Gray
    Write-Host "   Token Type: $($loginResponse.token_type)" -ForegroundColor Gray
    Write-Host "   Expires In: $($loginResponse.expires_in) seconds" -ForegroundColor Gray

    $ACCESS_TOKEN = $loginResponse.access_token
    $REFRESH_TOKEN = $loginResponse.refresh_token
} catch {
    Write-Host "❌ Login failed: $_" -ForegroundColor Red
    Write-Host $_.Exception.Message -ForegroundColor Red
    exit 1
}
Write-Host ""

# Test 4: Get Current User (Me)
Write-Host "4️⃣  Testing Get Current User..." -ForegroundColor Yellow
try {
    $headers = @{
        Authorization = "Bearer $ACCESS_TOKEN"
    }

    $meResponse = Invoke-RestMethod -Uri "$BASE_URL/api/auth/me" `
        -Method Get `
        -Headers $headers

    Write-Host "✅ Get current user successful" -ForegroundColor Green
    Write-Host "   User ID: $($meResponse.user.id)" -ForegroundColor Gray
    Write-Host "   Email: $($meResponse.user.email)" -ForegroundColor Gray
    Write-Host "   Username: $($meResponse.user.username)" -ForegroundColor Gray
    Write-Host "   Email Verified: $($meResponse.user.email_verified)" -ForegroundColor Gray
} catch {
    Write-Host "❌ Get current user failed: $_" -ForegroundColor Red
    Write-Host $_.Exception.Message -ForegroundColor Red
}
Write-Host ""

# Test 5: Refresh Token
Write-Host "5️⃣  Testing Refresh Token..." -ForegroundColor Yellow
try {
    $refreshBody = @{
        refresh_token = $REFRESH_TOKEN
    } | ConvertTo-Json

    $refreshResponse = Invoke-RestMethod -Uri "$BASE_URL/api/auth/refresh-token" `
        -Method Post `
        -ContentType "application/json" `
        -Body $refreshBody

    Write-Host "✅ Token refresh successful" -ForegroundColor Green
    Write-Host "   New Access Token: $($refreshResponse.access_token.Substring(0, 20))..." -ForegroundColor Gray

    $ACCESS_TOKEN = $refreshResponse.access_token
    $REFRESH_TOKEN = $refreshResponse.refresh_token
} catch {
    Write-Host "❌ Token refresh failed: $_" -ForegroundColor Red
    Write-Host $_.Exception.Message -ForegroundColor Red
}
Write-Host ""

# Test 6: Forgot Password
Write-Host "6️⃣  Testing Forgot Password..." -ForegroundColor Yellow
try {
    $forgotBody = @{
        email = $EMAIL
    } | ConvertTo-Json

    $forgotResponse = Invoke-RestMethod -Uri "$BASE_URL/api/auth/forgot-password" `
        -Method Post `
        -ContentType "application/json" `
        -Body $forgotBody

    Write-Host "✅ Forgot password request successful" -ForegroundColor Green
    Write-Host "   Message: $($forgotResponse.message)" -ForegroundColor Gray
} catch {
    Write-Host "❌ Forgot password failed: $_" -ForegroundColor Red
    Write-Host $_.Exception.Message -ForegroundColor Red
}
Write-Host ""

# Test 7: Logout
Write-Host "7️⃣  Testing Logout..." -ForegroundColor Yellow
try {
    $headers = @{
        Authorization = "Bearer $ACCESS_TOKEN"
    }

    $logoutResponse = Invoke-RestMethod -Uri "$BASE_URL/api/auth/logout" `
        -Method Post `
        -Headers $headers

    Write-Host "✅ Logout successful" -ForegroundColor Green
    Write-Host "   Message: $($logoutResponse.message)" -ForegroundColor Gray
} catch {
    Write-Host "❌ Logout failed: $_" -ForegroundColor Red
    Write-Host $_.Exception.Message -ForegroundColor Red
}
Write-Host ""

Write-Host "=========================================" -ForegroundColor Cyan
Write-Host "🎉 All tests completed!" -ForegroundColor Cyan
Write-Host ""
Write-Host "📝 Notes:" -ForegroundColor Yellow
Write-Host "   - Check server logs for email verification token (DEV mode)" -ForegroundColor Gray
Write-Host "   - Use the verification token to test email verification endpoint" -ForegroundColor Gray
Write-Host "   - Token is blacklisted after logout, subsequent /me calls will fail" -ForegroundColor Gray
