# ElysiaJS API Testing Script (PowerShell Version)
# Usage: .\test-all-api.ps1
# Usage with auto-start server: .\test-all-api.ps1 -s
# Optional: $env:API_URL = "http://localhost:4092"; .\test-all-api.ps1

param(
    [string]$BaseUrl = "http://localhost:4092",
    [switch]$s
)

# Use environment variable if set
if ($env:API_URL) {
    $BaseUrl = $env:API_URL
}

# Variable to track if we started the server
$script:ServerStarted = $false
$script:ServerProcess = $null

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
            Write-Host "[PASS] " -ForegroundColor Green -NoNewline
            Write-Host $Message
            $script:PassedTests++
        }
        "FAIL" {
            Write-Host "[FAIL] " -ForegroundColor Red -NoNewline
            Write-Host $Message
            $script:FailedTests++
        }
        "INFO" {
            Write-Host "[INFO] " -ForegroundColor Blue -NoNewline
            Write-Host $Message
        }
        "WARN" {
            Write-Host "[WARN] " -ForegroundColor Yellow -NoNewline
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
            return $content
        }
        else {
            Write-Status -Status "FAIL" -Message "$Description (Expected: $ExpectedStatus, Got: $statusCode)"
            Write-Host "Response: $content" -ForegroundColor Gray
            return $null
        }
    }
    catch {
        $statusCode = 0
        $errorBody = $_.Exception.Message

        if ($_.Exception.Response) {
            $statusCode = [int]$_.Exception.Response.StatusCode
            try {
                $reader = New-Object System.IO.StreamReader($_.Exception.Response.GetResponseStream())
                $errorBody = $reader.ReadToEnd()
                $reader.Close()
            }
            catch {
                $errorBody = $_.Exception.Message
            }
        }

        if ($statusCode -eq $ExpectedStatus) {
            Write-Status -Status "PASS" -Message "$Description (HTTP $statusCode)"
            return $errorBody
        }
        else {
            Write-Status -Status "FAIL" -Message "$Description (Expected: $ExpectedStatus, Got: $statusCode)"
            Write-Host "Error: $errorBody" -ForegroundColor Gray
            return $null
        }
    }
}
# Function to start server
function Start-Server {
    Write-Host "`n>>> Starting ElysiaJS server..." -ForegroundColor Cyan

    try {
        # Start server in background
        $script:ServerProcess = Start-Process -FilePath "bun" -ArgumentList "run", "dev" -PassThru -NoNewWindow -RedirectStandardOutput "server-output.log" -RedirectStandardError "server-error.log"
        $script:ServerStarted = $true

        Write-Host "Server started with PID: $($script:ServerProcess.Id)" -ForegroundColor Green
        Write-Host "Waiting for server to be ready..." -ForegroundColor Yellow

        # Wait for server to be ready (max 30 seconds)
        $maxRetries = 30
        $retryCount = 0
        $serverReady = $false

        while ($retryCount -lt $maxRetries -and -not $serverReady) {
            Start-Sleep -Seconds 1
            try {
                $response = Invoke-WebRequest -Uri "$BaseUrl/health" -Method GET -TimeoutSec 2 -ErrorAction SilentlyContinue
                if ($response.StatusCode -eq 200) {
                    $serverReady = $true
                    Write-Host "Server is ready!" -ForegroundColor Green
                }
            }
            catch {
                $retryCount++
                Write-Host "." -NoNewline
            }
        }

        if (-not $serverReady) {
            Write-Host "`nServer failed to start within 30 seconds" -ForegroundColor Red

            # Show error logs if available
            if (Test-Path "server-error.log") {
                $errorContent = Get-Content "server-error.log" -Tail 10 -ErrorAction SilentlyContinue
                if ($errorContent) {
                    Write-Host "`nServer Error Log (last 10 lines):" -ForegroundColor Red
                    $errorContent | ForEach-Object { Write-Host $_ -ForegroundColor Gray }
                    Write-Host "`nCommon issues:" -ForegroundColor Yellow
                    Write-Host "  - Redis not running (ECONNREFUSED 127.0.0.1:6379)" -ForegroundColor Yellow
                    Write-Host "  - Database not accessible" -ForegroundColor Yellow
                    Write-Host "  - Port 4092 already in use" -ForegroundColor Yellow
                }
            }

            Stop-Server
            exit 1
        }

        Write-Host ""
    }
    catch {
        Write-Host "Failed to start server: $($_.Exception.Message)" -ForegroundColor Red
        exit 1
    }
}
# Function to stop server
function Stop-Server {
    if ($script:ServerStarted -and $script:ServerProcess) {
        Write-Host "`n>>> Stopping server..." -ForegroundColor Cyan
        try {
            Stop-Process -Id $script:ServerProcess.Id -Force -ErrorAction SilentlyContinue
            Write-Host "Server stopped (PID: $($script:ServerProcess.Id))" -ForegroundColor Green
        }
        catch {
            Write-Host "Failed to stop server: $($_.Exception.Message)" -ForegroundColor Yellow
        }
    }
}

# Cleanup on exit
$scriptBlock = {
    Stop-Server
}
Register-EngineEvent -SourceIdentifier PowerShell.Exiting -Action $scriptBlock | Out-Null

Write-Host "================================" -ForegroundColor Blue
Write-Host "  ElysiaJS API Testing Script" -ForegroundColor Blue
Write-Host "================================" -ForegroundColor Blue
Write-Host "Base URL: $BaseUrl" -ForegroundColor Blue
Write-Host "Test Email: $script:TestEmail" -ForegroundColor Blue
Write-Host "Auto-start server: $(if ($s) { 'Yes' } else { 'No' })" -ForegroundColor Blue
Write-Host ""

# Start server if -s flag is provided
if ($s) {
    Start-Server
}

# ========================
# 1. Health & Basic Tests
# ========================
Write-Host "`n=== Health & Basic Endpoints ===" -ForegroundColor Yellow

$null = Test-Api -Method "GET" -Endpoint "/" -ExpectedStatus 200 -Description "Root endpoint"
$null = Test-Api -Method "GET" -Endpoint "/health" -ExpectedStatus 200 -Description "Health check endpoint"
$null = Test-Api -Method "GET" -Endpoint "/api/hello/World" -ExpectedStatus 200 -Description "Hello endpoint with parameter"
$null = Test-Api -Method "POST" -Endpoint "/api/echo" -Body '{"test":"data"}' -ExpectedStatus 200 -Description "Echo endpoint"

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
$null = Test-Api -Method "GET" -Endpoint "/api/auth/me" -ExpectedStatus 200 -Description "Get current user info" -AuthToken $script:AccessToken

# Refresh token
Write-Host "`n>>> Refreshing token..." -ForegroundColor Blue
$refreshBody = @{
    refresh_token = $script:RefreshToken
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
        Write-Host "Response: $($refreshResponse | ConvertTo-Json -Compress)" -ForegroundColor Gray
    }
}
catch {
    Write-Status -Status "FAIL" -Message "Token refresh"
    Write-Host "Error: $($_.Exception.Message)" -ForegroundColor Gray
    if ($_.ErrorDetails.Message) {
        Write-Host "Details: $($_.ErrorDetails.Message)" -ForegroundColor Gray
    }
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

    if ($postResponse.post -and $postResponse.post.id) {
        Write-Status -Status "PASS" -Message "Create post"
        $script:PostId = $postResponse.post.id
        Write-Host "Post ID: $($script:PostId)" -ForegroundColor Gray
    }
    else {
        Write-Status -Status "FAIL" -Message "Create post"
        Write-Host "Response: $($postResponse | ConvertTo-Json -Compress)" -ForegroundColor Gray
    }
}
catch {
    Write-Status -Status "FAIL" -Message "Create post"
    Write-Host "Error: $($_.Exception.Message)" -ForegroundColor Gray
    if ($_.ErrorDetails.Message) {
        Write-Host "Details: $($_.ErrorDetails.Message)" -ForegroundColor Gray
    }
}
# Get all posts
Write-Host "`n>>> Getting all posts..." -ForegroundColor Blue
$null = Test-Api -Method "GET" -Endpoint "/api/sosmed/posts" -ExpectedStatus 200 -Description "Get all posts" -AuthToken $script:AccessToken

# Update post
if ($script:PostId) {
    Write-Host "`n>>> Updating post..." -ForegroundColor Blue
    $updateBody = @{
        content = "Updated test post content"
    } | ConvertTo-Json
    $null = Test-Api -Method "PUT" -Endpoint "/api/sosmed/posts/$($script:PostId)" -Body $updateBody -ExpectedStatus 200 -Description "Update post" -AuthToken $script:AccessToken
}

# Like post
if ($script:PostId) {
    Write-Host "`n>>> Liking post..." -ForegroundColor Blue
    $null = Test-Api -Method "POST" -Endpoint "/api/sosmed/posts/$($script:PostId)/like" -ExpectedStatus 200 -Description "Like post" -AuthToken $script:AccessToken
}

# Create comment
if ($script:PostId) {
    Write-Host "`n>>> Creating comment..." -ForegroundColor Blue
    $commentBody = @{
        content = "This is a test comment!"
    } | ConvertTo-Json

    try {
        $commentResponse = Invoke-RestMethod -Uri "$BaseUrl/api/sosmed/posts/$($script:PostId)/comments" `
            -Method POST `
            -Headers @{
                "Authorization" = "Bearer $($script:AccessToken)"
                "Content-Type" = "application/json"
            } `
            -Body $commentBody

        if ($commentResponse.comment -and $commentResponse.comment.id) {
            Write-Status -Status "PASS" -Message "Create comment"
            $script:CommentId = $commentResponse.comment.id
            Write-Host "Comment ID: $($script:CommentId)" -ForegroundColor Gray
        }
        elseif ($commentResponse.id) {
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
    $null = Test-Api -Method "PUT" -Endpoint "/api/sosmed/comments/$($script:CommentId)" -Body $updateCommentBody -ExpectedStatus 200 -Description "Update comment" -AuthToken $script:AccessToken
}

# Delete comment
if ($script:CommentId) {
    Write-Host "`n>>> Deleting comment..." -ForegroundColor Blue
    $null = Test-Api -Method "DELETE" -Endpoint "/api/sosmed/comments/$($script:CommentId)" -ExpectedStatus 200 -Description "Delete comment" -AuthToken $script:AccessToken
}

# Unlike post
if ($script:PostId) {
    Write-Host "`n>>> Unliking post..." -ForegroundColor Blue
    $null = Test-Api -Method "DELETE" -Endpoint "/api/sosmed/posts/$($script:PostId)/like" -ExpectedStatus 200 -Description "Unlike post" -AuthToken $script:AccessToken
}

# Delete post
if ($script:PostId) {
    Write-Host "`n>>> Deleting post..." -ForegroundColor Blue
    $null = Test-Api -Method "DELETE" -Endpoint "/api/sosmed/posts/$($script:PostId)" -ExpectedStatus 200 -Description "Delete post" -AuthToken $script:AccessToken
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

    if ($roomResponse.room -and $roomResponse.room.id) {
        Write-Status -Status "PASS" -Message "Create chat room"
        $script:RoomId = $roomResponse.room.id
        Write-Host "Room ID: $($script:RoomId)" -ForegroundColor Gray
    }
    else {
        Write-Status -Status "FAIL" -Message "Create chat room"
        Write-Host "Response: $($roomResponse | ConvertTo-Json -Compress)" -ForegroundColor Gray
    }
}
catch {
    Write-Status -Status "FAIL" -Message "Create chat room"
    Write-Host "Error: $($_.Exception.Message)" -ForegroundColor Gray
    if ($_.ErrorDetails.Message) {
        Write-Host "Details: $($_.ErrorDetails.Message)" -ForegroundColor Gray
    }
}
# Get all rooms
Write-Host "`n>>> Getting all chat rooms..." -ForegroundColor Blue
$null = Test-Api -Method "GET" -Endpoint "/api/chat/rooms" -ExpectedStatus 200 -Description "Get all chat rooms" -AuthToken $script:AccessToken

# Join chat room
if ($script:RoomId) {
    Write-Host "`n>>> Joining chat room..." -ForegroundColor Blue
    $null = Test-Api -Method "POST" -Endpoint "/api/chat/rooms/$($script:RoomId)/join" -ExpectedStatus 200 -Description "Join chat room" -AuthToken $script:AccessToken
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

        if ($messageResponse.message -and $messageResponse.message.id) {
            Write-Status -Status "PASS" -Message "Send message"
            $script:MessageId = $messageResponse.message.id
            Write-Host "Message ID: $($script:MessageId)" -ForegroundColor Gray
        }
        elseif ($messageResponse.id) {
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
    $null = Test-Api -Method "GET" -Endpoint "/api/chat/rooms/$($script:RoomId)/messages" -ExpectedStatus 200 -Description "Get messages" -AuthToken $script:AccessToken
}

# Leave chat room
if ($script:RoomId) {
    Write-Host "`n>>> Leaving chat room..." -ForegroundColor Blue
    $null = Test-Api -Method "POST" -Endpoint "/api/chat/rooms/$($script:RoomId)/leave" -ExpectedStatus 200 -Description "Leave chat room" -AuthToken $script:AccessToken
}

# ========================
# 5. Logout
# ========================
Write-Host "`n=== Logout ===" -ForegroundColor Yellow
$null = Test-Api -Method "POST" -Endpoint "/api/auth/logout" -ExpectedStatus 200 -Description "User logout" -AuthToken $script:AccessToken
# ========================
# Test Summary
# ========================
Write-Host "`n================================" -ForegroundColor Blue
Write-Host "      Test Summary" -ForegroundColor Blue
Write-Host "================================" -ForegroundColor Blue
Write-Host "Total Tests: $($script:TotalTests)"
Write-Host "Passed: $($script:PassedTests)" -ForegroundColor Green
Write-Host "Failed: $($script:FailedTests)" -ForegroundColor Red

# Stop server if we started it
if ($script:ServerStarted) {
    Stop-Server
}

if ($script:FailedTests -eq 0) {
    Write-Host "`nAll tests passed!" -ForegroundColor Green
    exit 0
}
else {
    Write-Host "`nSome tests failed!" -ForegroundColor Red
    exit 1
}
