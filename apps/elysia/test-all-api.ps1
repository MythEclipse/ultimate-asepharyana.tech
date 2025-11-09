# ElysiaJS API Testing Script (PowerShell Version)
# Usage: .\test-all-api.ps1
# Optional: $env:API_URL = "http://localhost:4092"; .\test-all-api.ps1

param(
    [string]$BaseUrl = $env:API_URL ?? "http://localhost:4092"
)

# Test counters
$script:TotalTests = 0
$script:PassedTests = 0
$script:FailedTests = 0

# Global variables for tokens and IDs
$script:AccessToken = ""
$script:RefreshToken = ""
$script:UserId = ""
$script:PostId = ""
$script:CommentId = ""
$script:RoomId = ""
$script:MessageId = ""
$script:TestEmail = "test_$(Get-Date -Format 'yyyyMMddHHmmss')@example.com"
$script:TestPassword = "TestPassword123!"
$script:TestUsername = "testuser_$(Get-Date -Format 'yyyyMMddHHmmss')"

# Function to print colored output
function Write-Status {
    param(
        [Parameter(Mandatory=$true)]
        [ValidateSet("PASS", "FAIL", "INFO", "WARN")]
        [string]$Status,

        [Parameter(Mandatory=$true)]
        [string]$Message
    )

    $script:TotalTests++

    switch ($Status) {
        "PASS" {
            Write-Host "✓ PASS: " -ForegroundColor Green -NoNewline
            Write-Host $Message
            $script:PassedTests++
        }
        "FAIL" {
            Write-Host "✗ FAIL: " -ForegroundColor Red -NoNewline
            Write-Host $Message
            $script:FailedTests++
        }
        "INFO" {
            Write-Host "ℹ INFO: " -ForegroundColor Blue -NoNewline
            Write-Host $Message
        }
        "WARN" {
            Write-Host "⚠ WARN: " -ForegroundColor Yellow -NoNewline
            Write-Host $Message
        }
    }
}

# Function to make API request
function Test-Api {
    param(
        [Parameter(Mandatory=$true)]
        [string]$Method,

        [Parameter(Mandatory=$true)]
        [string]$Endpoint,

        [Parameter(Mandatory=$false)]
        [string]$Body = "",

        [Parameter(Mandatory=$true)]
        [int]$ExpectedStatus,

        [Parameter(Mandatory=$true)]
        [string]$Description,

        [Parameter(Mandatory=$false)]
        [string]$AuthToken = ""
    )

    $url = "$BaseUrl$Endpoint"
    $headers = @{
        "Content-Type" = "application/json"
    }

    if ($AuthToken) {
        $headers["Authorization"] = "Bearer $AuthToken"
    }

    try {
        $params = @{
            Uri = $url
            Method = $Method
            Headers = $headers
            ErrorAction = "Stop"
        }

        if ($Body -and $Method -ne "GET") {
            $params["Body"] = $Body
        }

        $response = Invoke-WebRequest @params
        $statusCode = $response.StatusCode
        $content = $response.Content

        if ($statusCode -eq $ExpectedStatus) {
            Write-Status -Status "PASS" -Message "$Description (HTTP $statusCode)"
            if ($content) {
                Write-Host $content -ForegroundColor Gray
            }
            return $true, $content
        }
        else {
            Write-Status -Status "FAIL" -Message "$Description (Expected: $ExpectedStatus, Got: $statusCode)"
            Write-Host "Response: $content" -ForegroundColor Gray
            return $false, $content
        }
    }
    catch {
        $statusCode = $_.Exception.Response.StatusCode.value__
        $errorBody = ""

        if ($_.Exception.Response) {
            $reader = New-Object System.IO.StreamReader($_.Exception.Response.GetResponseStream())
            $errorBody = $reader.ReadToEnd()
            $reader.Close()
        }

        if ($statusCode -eq $ExpectedStatus) {
            Write-Status -Status "PASS" -Message "$Description (HTTP $statusCode)"
            return $true, $errorBody
        }
        else {
            Write-Status -Status "FAIL" -Message "$Description (Expected: $ExpectedStatus, Got: $statusCode)"
            Write-Host "Error: $errorBody" -ForegroundColor Gray
            return $false, $errorBody
        }
    }
}

Write-Host "================================" -ForegroundColor Blue
Write-Host "  ElysiaJS API Testing Script" -ForegroundColor Blue
Write-Host "================================" -ForegroundColor Blue
Write-Host "Base URL: $BaseUrl" -ForegroundColor Blue
Write-Host "Test Email: $script:TestEmail" -ForegroundColor Blue
Write-Host ""

# ========================
# 1. Health & Basic Tests
# ========================
Write-Host "`n=== Health & Basic Endpoints ===" -ForegroundColor Yellow

Test-Api -Method "GET" -Endpoint "/" -ExpectedStatus 200 -Description "Root endpoint"
Test-Api -Method "GET" -Endpoint "/health" -ExpectedStatus 200 -Description "Health check endpoint"
Test-Api -Method "GET" -Endpoint "/api/hello/World" -ExpectedStatus 200 -Description "Hello endpoint with parameter"
Test-Api -Method "POST" -Endpoint "/api/echo" -Body '{"test":"data"}' -ExpectedStatus 200 -Description "Echo endpoint"

# ========================
# 2. Authentication Tests
# ========================
Write-Host "`n=== Authentication Endpoints ===" -ForegroundColor Yellow

# Register new user
Write-Host "`n>>> Registering new user..." -ForegroundColor Blue
$registerBody = @{
    email = $script:TestEmail
    password = $script:TestPassword
    name = "Test User"
    username = $script:TestUsername
} | ConvertTo-Json

try {
    $registerResponse = Invoke-RestMethod -Uri "$BaseUrl/api/auth/register" `
        -Method POST `
        -Headers @{"Content-Type"="application/json"} `
        -Body $registerBody

    if ($registerResponse.user) {
        Write-Status -Status "PASS" -Message "User registration"
        $script:UserId = $registerResponse.user.id
        Write-Host "User ID: $($script:UserId)" -ForegroundColor Gray
    }
    else {
        Write-Status -Status "FAIL" -Message "User registration"
        Write-Host "Response: $($registerResponse | ConvertTo-Json)" -ForegroundColor Gray
    }
}
catch {
    Write-Status -Status "FAIL" -Message "User registration"
    Write-Host "Error: $($_.Exception.Message)" -ForegroundColor Gray
}

# Login
Write-Host "`n>>> Logging in..." -ForegroundColor Blue
$loginBody = @{
    email = $script:TestEmail
    password = $script:TestPassword
} | ConvertTo-Json

try {
    $loginResponse = Invoke-RestMethod -Uri "$BaseUrl/api/auth/login" `
        -Method POST `
        -Headers @{"Content-Type"="application/json"} `
        -Body $loginBody

    if ($loginResponse.accessToken) {
        Write-Status -Status "PASS" -Message "User login"
        $script:AccessToken = $loginResponse.accessToken
        $script:RefreshToken = $loginResponse.refreshToken
        Write-Host "Access Token: $($script:AccessToken.Substring(0, [Math]::Min(50, $script:AccessToken.Length)))..." -ForegroundColor Gray
    }
    else {
        Write-Status -Status "FAIL" -Message "User login"
        Write-Host "Response: $($loginResponse | ConvertTo-Json)" -ForegroundColor Gray
        exit 1
    }
}
catch {
    Write-Status -Status "FAIL" -Message "User login"
    Write-Host "Error: $($_.Exception.Message)" -ForegroundColor Gray
    exit 1
}

# Get current user info
Write-Host "`n>>> Getting current user info..." -ForegroundColor Blue
Test-Api -Method "GET" -Endpoint "/api/auth/me" -ExpectedStatus 200 -Description "Get current user info" -AuthToken $script:AccessToken

# Refresh token
Write-Host "`n>>> Refreshing token..." -ForegroundColor Blue
$refreshBody = @{
    refreshToken = $script:RefreshToken
} | ConvertTo-Json

try {
    $refreshResponse = Invoke-RestMethod -Uri "$BaseUrl/api/auth/refresh-token" `
        -Method POST `
        -Headers @{"Content-Type"="application/json"} `
        -Body $refreshBody

    if ($refreshResponse.accessToken) {
        Write-Status -Status "PASS" -Message "Token refresh"
        $script:AccessToken = $refreshResponse.accessToken
    }
    else {
        Write-Status -Status "FAIL" -Message "Token refresh"
    }
}
catch {
    Write-Status -Status "FAIL" -Message "Token refresh"
    Write-Host "Error: $($_.Exception.Message)" -ForegroundColor Gray
}

# ========================
# 3. Social Media Tests
# ========================
Write-Host "`n=== Social Media Endpoints ===" -ForegroundColor Yellow

# Create post
Write-Host "`n>>> Creating a post..." -ForegroundColor Blue
$postBody = @{
    content = "This is a test post from API testing script!"
    imageUrl = "https://example.com/test-image.jpg"
} | ConvertTo-Json

try {
    $postResponse = Invoke-RestMethod -Uri "$BaseUrl/api/sosmed/posts" `
        -Method POST `
        -Headers @{
            "Authorization" = "Bearer $($script:AccessToken)"
            "Content-Type" = "application/json"
        } `
        -Body $postBody

    if ($postResponse.id) {
        Write-Status -Status "PASS" -Message "Create post"
        $script:PostId = $postResponse.id
        Write-Host "Post ID: $($script:PostId)" -ForegroundColor Gray
    }
    else {
        Write-Status -Status "FAIL" -Message "Create post"
    }
}
catch {
    Write-Status -Status "FAIL" -Message "Create post"
    Write-Host "Error: $($_.Exception.Message)" -ForegroundColor Gray
}

# Get all posts
Write-Host "`n>>> Getting all posts..." -ForegroundColor Blue
Test-Api -Method "GET" -Endpoint "/api/sosmed/posts" -ExpectedStatus 200 -Description "Get all posts" -AuthToken $script:AccessToken

# Update post
if ($script:PostId) {
    Write-Host "`n>>> Updating post..." -ForegroundColor Blue
    $updateBody = @{
        content = "Updated test post content"
    } | ConvertTo-Json
    Test-Api -Method "PUT" -Endpoint "/api/sosmed/posts/$($script:PostId)" -Body $updateBody -ExpectedStatus 200 -Description "Update post" -AuthToken $script:AccessToken
}

# Like post
if ($script:PostId) {
    Write-Host "`n>>> Liking post..." -ForegroundColor Blue
    Test-Api -Method "POST" -Endpoint "/api/sosmed/posts/$($script:PostId)/like" -ExpectedStatus 200 -Description "Like post" -AuthToken $script:AccessToken
}

# Create comment
if ($script:PostId) {
    Write-Host "`n>>> Creating comment..." -ForegroundColor Blue
    $commentBody = @{
        postId = $script:PostId
        content = "This is a test comment!"
    } | ConvertTo-Json

    try {
        $commentResponse = Invoke-RestMethod -Uri "$BaseUrl/api/sosmed/comments" `
            -Method POST `
            -Headers @{
                "Authorization" = "Bearer $($script:AccessToken)"
                "Content-Type" = "application/json"
            } `
            -Body $commentBody

        if ($commentResponse.id) {
            Write-Status -Status "PASS" -Message "Create comment"
            $script:CommentId = $commentResponse.id
            Write-Host "Comment ID: $($script:CommentId)" -ForegroundColor Gray
        }
        else {
            Write-Status -Status "FAIL" -Message "Create comment"
        }
    }
    catch {
        Write-Status -Status "FAIL" -Message "Create comment"
        Write-Host "Error: $($_.Exception.Message)" -ForegroundColor Gray
    }
}

# Update comment
if ($script:CommentId) {
    Write-Host "`n>>> Updating comment..." -ForegroundColor Blue
    $updateCommentBody = @{
        content = "Updated comment content"
    } | ConvertTo-Json
    Test-Api -Method "PUT" -Endpoint "/api/sosmed/comments/$($script:CommentId)" -Body $updateCommentBody -ExpectedStatus 200 -Description "Update comment" -AuthToken $script:AccessToken
}

# Delete comment
if ($script:CommentId) {
    Write-Host "`n>>> Deleting comment..." -ForegroundColor Blue
    Test-Api -Method "DELETE" -Endpoint "/api/sosmed/comments/$($script:CommentId)" -ExpectedStatus 200 -Description "Delete comment" -AuthToken $script:AccessToken
}

# Unlike post
if ($script:PostId) {
    Write-Host "`n>>> Unliking post..." -ForegroundColor Blue
    Test-Api -Method "DELETE" -Endpoint "/api/sosmed/posts/$($script:PostId)/like" -ExpectedStatus 200 -Description "Unlike post" -AuthToken $script:AccessToken
}

# Delete post
if ($script:PostId) {
    Write-Host "`n>>> Deleting post..." -ForegroundColor Blue
    Test-Api -Method "DELETE" -Endpoint "/api/sosmed/posts/$($script:PostId)" -ExpectedStatus 200 -Description "Delete post" -AuthToken $script:AccessToken
}

# ========================
# 4. Chat Tests
# ========================
Write-Host "`n=== Chat Endpoints ===" -ForegroundColor Yellow

# Create chat room
Write-Host "`n>>> Creating chat room..." -ForegroundColor Blue
$roomBody = @{
    name = "Test Chat Room"
    description = "A test chat room created by API testing script"
} | ConvertTo-Json

try {
    $roomResponse = Invoke-RestMethod -Uri "$BaseUrl/api/chat/rooms" `
        -Method POST `
        -Headers @{
            "Authorization" = "Bearer $($script:AccessToken)"
            "Content-Type" = "application/json"
        } `
        -Body $roomBody

    if ($roomResponse.id) {
        Write-Status -Status "PASS" -Message "Create chat room"
        $script:RoomId = $roomResponse.id
        Write-Host "Room ID: $($script:RoomId)" -ForegroundColor Gray
    }
    else {
        Write-Status -Status "FAIL" -Message "Create chat room"
    }
}
catch {
    Write-Status -Status "FAIL" -Message "Create chat room"
    Write-Host "Error: $($_.Exception.Message)" -ForegroundColor Gray
}

# Get all rooms
Write-Host "`n>>> Getting all chat rooms..." -ForegroundColor Blue
Test-Api -Method "GET" -Endpoint "/api/chat/rooms" -ExpectedStatus 200 -Description "Get all chat rooms" -AuthToken $script:AccessToken

# Join chat room
if ($script:RoomId) {
    Write-Host "`n>>> Joining chat room..." -ForegroundColor Blue
    Test-Api -Method "POST" -Endpoint "/api/chat/rooms/$($script:RoomId)/join" -ExpectedStatus 200 -Description "Join chat room" -AuthToken $script:AccessToken
}

# Send message
if ($script:RoomId) {
    Write-Host "`n>>> Sending message..." -ForegroundColor Blue
    $messageBody = @{
        content = "Hello! This is a test message from API testing script."
    } | ConvertTo-Json

    try {
        $messageResponse = Invoke-RestMethod -Uri "$BaseUrl/api/chat/rooms/$($script:RoomId)/messages" `
            -Method POST `
            -Headers @{
                "Authorization" = "Bearer $($script:AccessToken)"
                "Content-Type" = "application/json"
            } `
            -Body $messageBody

        if ($messageResponse.id) {
            Write-Status -Status "PASS" -Message "Send message"
            $script:MessageId = $messageResponse.id
            Write-Host "Message ID: $($script:MessageId)" -ForegroundColor Gray
        }
        else {
            Write-Status -Status "FAIL" -Message "Send message"
        }
    }
    catch {
        Write-Status -Status "FAIL" -Message "Send message"
        Write-Host "Error: $($_.Exception.Message)" -ForegroundColor Gray
    }
}

# Get messages
if ($script:RoomId) {
    Write-Host "`n>>> Getting messages..." -ForegroundColor Blue
    Test-Api -Method "GET" -Endpoint "/api/chat/rooms/$($script:RoomId)/messages" -ExpectedStatus 200 -Description "Get messages" -AuthToken $script:AccessToken
}

# Leave chat room
if ($script:RoomId) {
    Write-Host "`n>>> Leaving chat room..." -ForegroundColor Blue
    Test-Api -Method "POST" -Endpoint "/api/chat/rooms/$($script:RoomId)/leave" -ExpectedStatus 200 -Description "Leave chat room" -AuthToken $script:AccessToken
}

# ========================
# 5. Logout
# ========================
Write-Host "`n=== Logout ===" -ForegroundColor Yellow
Test-Api -Method "POST" -Endpoint "/api/auth/logout" -ExpectedStatus 200 -Description "User logout" -AuthToken $script:AccessToken

# ========================
# Test Summary
# ========================
Write-Host "`n================================" -ForegroundColor Blue
Write-Host "      Test Summary" -ForegroundColor Blue
Write-Host "================================" -ForegroundColor Blue
Write-Host "Total Tests: $($script:TotalTests)"
Write-Host "Passed: $($script:PassedTests)" -ForegroundColor Green
Write-Host "Failed: $($script:FailedTests)" -ForegroundColor Red

if ($script:FailedTests -eq 0) {
    Write-Host "`n✓ All tests passed!" -ForegroundColor Green
    exit 0
}
else {
    Write-Host "`n✗ Some tests failed!" -ForegroundColor Red
    exit 1
}
