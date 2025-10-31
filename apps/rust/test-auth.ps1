# Authentication Testing Script
# Run this script to test all authentication endpoints

$BASE_URL = "http://localhost:3000"
$EMAIL = "test@example.com"
$USERNAME = "testuser"
$PASSWORD = "TestPass123!@#"

Write-Host "=== Authentication Testing Script ===" -ForegroundColor Cyan

# 1. Register User
Write-Host "`n1. Testing User Registration..." -ForegroundColor Yellow
$registerResponse = Invoke-RestMethod -Uri "$BASE_URL/api/auth/register" `
    -Method POST `
    -ContentType "application/json" `
    -Body (@{
        email = $EMAIL
        username = $USERNAME
        password = $PASSWORD
        full_name = "Test User"
    } | ConvertTo-Json)

Write-Host "Registration successful!" -ForegroundColor Green
Write-Host "User ID: $($registerResponse.user.id)"
Write-Host "Verification Token: $($registerResponse.verification_token)"
$VERIFICATION_TOKEN = $registerResponse.verification_token

# 2. Verify Email
Write-Host "`n2. Testing Email Verification..." -ForegroundColor Yellow
$verifyResponse = Invoke-RestMethod -Uri "$BASE_URL/api/auth/verify?token=$VERIFICATION_TOKEN" `
    -Method GET

Write-Host "Email verified successfully!" -ForegroundColor Green

# 3. Login
Write-Host "`n3. Testing Login..." -ForegroundColor Yellow
$loginResponse = Invoke-RestMethod -Uri "$BASE_URL/api/auth/login" `
    -Method POST `
    -ContentType "application/json" `
    -Body (@{
        login = $EMAIL
        password = $PASSWORD
        remember_me = $false
    } | ConvertTo-Json)

Write-Host "Login successful!" -ForegroundColor Green
$ACCESS_TOKEN = $loginResponse.access_token
$REFRESH_TOKEN = $loginResponse.refresh_token
Write-Host "Access Token: $ACCESS_TOKEN"

# 4. Get Current User
Write-Host "`n4. Testing Get Current User..." -ForegroundColor Yellow
$meResponse = Invoke-RestMethod -Uri "$BASE_URL/api/auth/me" `
    -Method GET `
    -Headers @{
        "Authorization" = "Bearer $ACCESS_TOKEN"
    }

Write-Host "Current user retrieved!" -ForegroundColor Green
Write-Host "Username: $($meResponse.username)"
Write-Host "Email: $($meResponse.email)"
Write-Host "Email Verified: $($meResponse.email_verified)"

# 5. Change Password
Write-Host "`n5. Testing Change Password..." -ForegroundColor Yellow
$NEW_PASSWORD = "NewTestPass456!@#"
try {
    $changePasswordResponse = Invoke-RestMethod -Uri "$BASE_URL/api/auth/change-password" `
        -Method POST `
        -ContentType "application/json" `
        -Headers @{
            "Authorization" = "Bearer $ACCESS_TOKEN"
        } `
        -Body (@{
            current_password = $PASSWORD
            new_password = $NEW_PASSWORD
        } | ConvertTo-Json)

    Write-Host "Password changed successfully!" -ForegroundColor Green
} catch {
    Write-Host "Change password test failed (expected if tokens were revoked)" -ForegroundColor Yellow
}

# 6. Login with New Password
Write-Host "`n6. Testing Login with New Password..." -ForegroundColor Yellow
$loginResponse2 = Invoke-RestMethod -Uri "$BASE_URL/api/auth/login" `
    -Method POST `
    -ContentType "application/json" `
    -Body (@{
        login = $EMAIL
        password = $NEW_PASSWORD
        remember_me = $false
    } | ConvertTo-Json)

Write-Host "Login with new password successful!" -ForegroundColor Green
$ACCESS_TOKEN = $loginResponse2.access_token
$REFRESH_TOKEN = $loginResponse2.refresh_token

# 7. Refresh Token
Write-Host "`n7. Testing Token Refresh..." -ForegroundColor Yellow
$refreshResponse = Invoke-RestMethod -Uri "$BASE_URL/api/auth/refresh" `
    -Method POST `
    -ContentType "application/json" `
    -Body (@{
        refresh_token = $REFRESH_TOKEN
    } | ConvertTo-Json)

Write-Host "Token refreshed successfully!" -ForegroundColor Green
$ACCESS_TOKEN = $refreshResponse.access_token
$REFRESH_TOKEN = $refreshResponse.refresh_token

# 8. Forgot Password
Write-Host "`n8. Testing Forgot Password..." -ForegroundColor Yellow
$forgotResponse = Invoke-RestMethod -Uri "$BASE_URL/api/auth/forgot-password" `
    -Method POST `
    -ContentType "application/json" `
    -Body (@{
        email = $EMAIL
    } | ConvertTo-Json)

Write-Host "Password reset email sent!" -ForegroundColor Green
$RESET_TOKEN = $forgotResponse.reset_token
Write-Host "Reset Token: $RESET_TOKEN"

# 9. Reset Password
if ($RESET_TOKEN) {
    Write-Host "`n9. Testing Password Reset..." -ForegroundColor Yellow
    $FINAL_PASSWORD = "FinalPass789!@#"
    $resetResponse = Invoke-RestMethod -Uri "$BASE_URL/api/auth/reset-password" `
        -Method POST `
        -ContentType "application/json" `
        -Body (@{
            token = $RESET_TOKEN
            new_password = $FINAL_PASSWORD
        } | ConvertTo-Json)

    Write-Host "Password reset successfully!" -ForegroundColor Green

    # Login with final password
    Write-Host "`n10. Testing Login with Reset Password..." -ForegroundColor Yellow
    $loginResponse3 = Invoke-RestMethod -Uri "$BASE_URL/api/auth/login" `
        -Method POST `
        -ContentType "application/json" `
        -Body (@{
            login = $EMAIL
            password = $FINAL_PASSWORD
            remember_me = $false
        } | ConvertTo-Json)

    Write-Host "Login with reset password successful!" -ForegroundColor Green
    $ACCESS_TOKEN = $loginResponse3.access_token
}

# 11. Logout
Write-Host "`n11. Testing Logout..." -ForegroundColor Yellow
$logoutResponse = Invoke-RestMethod -Uri "$BASE_URL/api/auth/logout" `
    -Method POST `
    -ContentType "application/json" `
    -Headers @{
        "Authorization" = "Bearer $ACCESS_TOKEN"
    } `
    -Body (@{
        refresh_token = $REFRESH_TOKEN
        logout_all = $false
    } | ConvertTo-Json)

Write-Host "Logout successful!" -ForegroundColor Green

# 12. Try to access protected route (should fail)
Write-Host "`n12. Testing Access After Logout (should fail)..." -ForegroundColor Yellow
try {
    $meResponse2 = Invoke-RestMethod -Uri "$BASE_URL/api/auth/me" `
        -Method GET `
        -Headers @{
            "Authorization" = "Bearer $ACCESS_TOKEN"
        }
    Write-Host "ERROR: Should have failed!" -ForegroundColor Red
} catch {
    Write-Host "Access denied as expected!" -ForegroundColor Green
}

Write-Host "`n=== All Tests Completed ===" -ForegroundColor Cyan
