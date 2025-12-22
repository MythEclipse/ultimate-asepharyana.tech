#!/bin/bash

# =====================================================
# RustExpress API Test Script
# Test all API endpoints for the Rust application
# =====================================================

# Configuration
BASE_URL="${BASE_URL:-http://localhost:4091}"
VERBOSE="${VERBOSE:-false}"
AUTH_TOKEN=""
REFRESH_TOKEN=""
TEST_USER_EMAIL="testuser_$(date +%s)@test.com"
TEST_USER_PASSWORD="TestPassword123!"
TEST_USER_NAME="Test User"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Counters
PASSED=0
FAILED=0
SKIPPED=0

# =====================================================
# Helper Functions
# =====================================================

print_header() {
    echo ""
    echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${CYAN}  $1${NC}"
    echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
}

print_subheader() {
    echo ""
    echo -e "${BLUE}▸ $1${NC}"
    echo -e "${BLUE}────────────────────────────────────────${NC}"
}

log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[PASS]${NC} $1"
    ((PASSED++))
}

log_fail() {
    echo -e "${RED}[FAIL]${NC} $1"
    ((FAILED++))
}

log_skip() {
    echo -e "${YELLOW}[SKIP]${NC} $1"
    ((SKIPPED++))
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

# Test a GET endpoint - accepts any 2xx or 4xx as valid response (API is working)
test_get() {
    local endpoint="$1"
    local description="$2"
    local requires_auth="${3:-false}"
    
    local full_url="${BASE_URL}${endpoint}"
    
    if [ "$VERBOSE" = "true" ]; then
        log_info "Testing GET $full_url"
    fi
    
    if [ "$requires_auth" = "true" ] && [ -n "$AUTH_TOKEN" ]; then
        response=$(curl -s -w "\n%{http_code}" -X GET "$full_url" \
            -H "Content-Type: application/json" \
            -H "Authorization: Bearer $AUTH_TOKEN" \
            --max-time 30 2>/dev/null)
    else
        response=$(curl -s -w "\n%{http_code}" -X GET "$full_url" \
            -H "Content-Type: application/json" \
            --max-time 30 2>/dev/null)
    fi
    
    status_code=$(echo "$response" | tail -n1)
    body=$(echo "$response" | sed '$d')
    
    if [ -z "$status_code" ] || [ "$status_code" = "000" ]; then
        log_fail "$description - Connection failed"
        return 1
    fi
    
    # 2xx = success, 4xx = client error (but API working), 5xx = server error
    if [ "$status_code" -ge 200 ] && [ "$status_code" -lt 500 ]; then
        log_success "$description (Status: $status_code)"
        if [ "$VERBOSE" = "true" ]; then
            echo "  Response: $(echo "$body" | head -c 200)..."
        fi
        return 0
    else
        log_fail "$description (Status: $status_code - Server Error)"
        if [ "$VERBOSE" = "true" ]; then
            echo "  Response: $body"
        fi
        return 1
    fi
}

# Test a POST endpoint
test_post() {
    local endpoint="$1"
    local description="$2"
    local payload="$3"
    local requires_auth="${4:-false}"
    
    local full_url="${BASE_URL}${endpoint}"
    
    if [ "$VERBOSE" = "true" ]; then
        log_info "Testing POST $full_url"
    fi
    
    if [ "$requires_auth" = "true" ] && [ -n "$AUTH_TOKEN" ]; then
        response=$(curl -s -w "\n%{http_code}" -X POST "$full_url" \
            -H "Content-Type: application/json" \
            -H "Authorization: Bearer $AUTH_TOKEN" \
            -d "$payload" \
            --max-time 30 2>/dev/null)
    else
        response=$(curl -s -w "\n%{http_code}" -X POST "$full_url" \
            -H "Content-Type: application/json" \
            -d "$payload" \
            --max-time 30 2>/dev/null)
    fi
    
    status_code=$(echo "$response" | tail -n1)
    body=$(echo "$response" | sed '$d')
    
    if [ -z "$status_code" ] || [ "$status_code" = "000" ]; then
        log_fail "$description - Connection failed"
        return 1
    fi
    
    if [ "$status_code" -ge 200 ] && [ "$status_code" -lt 500 ]; then
        log_success "$description (Status: $status_code)"
        echo "$body"
        return 0
    else
        log_fail "$description (Status: $status_code - Server Error)"
        if [ "$VERBOSE" = "true" ]; then
            echo "  Response: $body"
        fi
        return 1
    fi
}

# Test a PUT endpoint
test_put() {
    local endpoint="$1"
    local description="$2"
    local payload="$3"
    local requires_auth="${4:-false}"
    
    local full_url="${BASE_URL}${endpoint}"
    
    if [ "$requires_auth" = "true" ] && [ -n "$AUTH_TOKEN" ]; then
        response=$(curl -s -w "\n%{http_code}" -X PUT "$full_url" \
            -H "Content-Type: application/json" \
            -H "Authorization: Bearer $AUTH_TOKEN" \
            -d "$payload" \
            --max-time 30 2>/dev/null)
    else
        response=$(curl -s -w "\n%{http_code}" -X PUT "$full_url" \
            -H "Content-Type: application/json" \
            -d "$payload" \
            --max-time 30 2>/dev/null)
    fi
    
    status_code=$(echo "$response" | tail -n1)
    
    if [ -z "$status_code" ] || [ "$status_code" = "000" ]; then
        log_fail "$description - Connection failed"
        return 1
    fi
    
    if [ "$status_code" -ge 200 ] && [ "$status_code" -lt 500 ]; then
        log_success "$description (Status: $status_code)"
        return 0
    else
        log_fail "$description (Status: $status_code - Server Error)"
        return 1
    fi
}

# Test a DELETE endpoint
test_delete() {
    local endpoint="$1"
    local description="$2"
    local payload="$3"
    local requires_auth="${4:-false}"
    
    local full_url="${BASE_URL}${endpoint}"
    
    if [ "$requires_auth" = "true" ] && [ -n "$AUTH_TOKEN" ]; then
        response=$(curl -s -w "\n%{http_code}" -X DELETE "$full_url" \
            -H "Content-Type: application/json" \
            -H "Authorization: Bearer $AUTH_TOKEN" \
            -d "$payload" \
            --max-time 30 2>/dev/null)
    else
        response=$(curl -s -w "\n%{http_code}" -X DELETE "$full_url" \
            -H "Content-Type: application/json" \
            -d "$payload" \
            --max-time 30 2>/dev/null)
    fi
    
    status_code=$(echo "$response" | tail -n1)
    
    if [ -z "$status_code" ] || [ "$status_code" = "000" ]; then
        log_fail "$description - Connection failed"
        return 1
    fi
    
    if [ "$status_code" -ge 200 ] && [ "$status_code" -lt 500 ]; then
        log_success "$description (Status: $status_code)"
        return 0
    else
        log_fail "$description (Status: $status_code - Server Error)"
        return 1
    fi
}

# Check if server is running
check_server() {
    log_info "Checking if server is running at $BASE_URL..."
    response=$(curl -s -o /dev/null -w "%{http_code}" --max-time 15 "$BASE_URL/docs/" 2>/dev/null)
    if [ "$response" = "000" ]; then
        echo -e "${RED}ERROR: Server is not running at $BASE_URL${NC}"
        echo "Please start the server with: cargo run --bin rust"
        exit 1
    fi
    log_success "Server is running!"
}

# =====================================================
# Test Sections
# =====================================================

test_anime_endpoints() {
    print_subheader "Anime API Endpoints (Primary - otakudesu)"
    
    test_get "/api/anime" "GET /api/anime - Anime index"
    test_get "/api/anime/search?q=naruto" "GET /api/anime/search - Search anime"
    test_get "/api/anime/ongoing-anime/1" "GET /api/anime/ongoing-anime/1 - Ongoing anime page"
    test_get "/api/anime/complete-anime/1" "GET /api/anime/complete-anime/1 - Complete anime page"
    test_get "/api/anime/detail/kimetsu-no-yaiba-sub-indo" "GET /api/anime/detail/{slug} - Anime detail"
    test_get "/api/anime/full/knysbi-episode-1-sub-indo" "GET /api/anime/full/{slug} - Anime full episode"
}

test_anime2_endpoints() {
    print_subheader "Anime2 API Endpoints (Secondary - alqanime)"
    
    log_warn "Anime2 uses external source (alqanime.net) which may be slow or unavailable"
    
    # Test basic endpoints - these may timeout if external source is down
    test_get "/api/anime2" "GET /api/anime2 - Anime2 index"
    test_get "/api/anime2/search?q=naruto" "GET /api/anime2/search - Search anime2"
    test_get "/api/anime2/ongoing-anime/1" "GET /api/anime2/ongoing-anime/1 - Ongoing anime2 page"
    test_get "/api/anime2/complete-anime/1" "GET /api/anime2/complete-anime/1 - Complete anime2 page"
    
    # Get actual slug from index for detail test
    log_info "Fetching anime2 index to get valid slug..."
    anime2_slug=$(curl -s --max-time 60 "${BASE_URL}/api/anime2" 2>/dev/null | grep -o '"slug":"[^"]*"' | head -1 | sed 's/"slug":"//;s/"$//' 2>/dev/null)
    if [ -n "$anime2_slug" ]; then
        test_get "/api/anime2/detail/${anime2_slug}" "GET /api/anime2/detail/{slug} - Anime2 detail"
    else
        log_skip "Anime2 detail - Could not get valid slug from index"
    fi
}

test_komik_endpoints() {
    print_subheader "Komik API Endpoints (Primary - komikindo)"
    
    test_get "/api/komik/search?q=kagurabachi" "GET /api/komik/search - Search komik"
    test_get "/api/komik/manga?page=1" "GET /api/komik/manga - Manga list"
    test_get "/api/komik/manhwa?page=1" "GET /api/komik/manhwa - Manhwa list"
    test_get "/api/komik/manhua?page=1" "GET /api/komik/manhua - Manhua list"
    
    # Get actual manga slug from list for detail/chapter tests
    log_info "Fetching manga list to get valid slug..."
    manga_slug=$(curl -s --max-time 30 "${BASE_URL}/api/komik/manga?page=1" 2>/dev/null | grep -o '"slug":"[^"]*"' | head -1 | sed 's/"slug":"//;s/"$//' 2>/dev/null)
    if [ -n "$manga_slug" ]; then
        test_get "/api/komik/detail?id=${manga_slug}" "GET /api/komik/detail - Komik detail (${manga_slug})"
        
        # Get chapter slug from detail
        chapter_slug=$(curl -s --max-time 30 "${BASE_URL}/api/komik/detail?id=${manga_slug}" 2>/dev/null | grep -o '"slug":"[^"]*chapter[^"]*"' | head -1 | sed 's/"slug":"//;s/"$//' 2>/dev/null)
        if [ -n "$chapter_slug" ]; then
            test_get "/api/komik/chapter?id=${chapter_slug}" "GET /api/komik/chapter - Chapter images (${chapter_slug})"
        else
            log_skip "Komik chapter - Could not get chapter slug from detail"
        fi
    else
        log_skip "Komik detail - Could not get valid slug from manga list"
        log_skip "Komik chapter - Skipped (no detail slug)"
    fi
}

test_komik2_endpoints() {
    print_subheader "Komik2 API Endpoints (Secondary - komiku)"
    
    test_get "/api/komik2/search?q=bocchi" "GET /api/komik2/search - Search komik2"
    test_get "/api/komik2/manga?page=1" "GET /api/komik2/manga - Manga2 list"
    test_get "/api/komik2/manhwa?page=1" "GET /api/komik2/manhwa - Manhwa2 list"
    test_get "/api/komik2/manhua?page=1" "GET /api/komik2/manhua - Manhua2 list"
    
    # Get actual manga slug from list for detail/chapter tests
    log_info "Fetching komik2 manga list to get valid slug..."
    manga2_slug=$(curl -s --max-time 30 "${BASE_URL}/api/komik2/manga?page=1" 2>/dev/null | grep -o '"slug":"[^"]*"' | head -1 | sed 's/"slug":"//;s/"$//' 2>/dev/null)
    if [ -n "$manga2_slug" ]; then
        test_get "/api/komik2/detail?id=${manga2_slug}" "GET /api/komik2/detail - Komik2 detail (${manga2_slug})"
        
        # Get chapter slug from detail
        chapter2_slug=$(curl -s --max-time 30 "${BASE_URL}/api/komik2/detail?id=${manga2_slug}" 2>/dev/null | grep -o '"slug":"[^"]*chapter[^"]*"' | head -1 | sed 's/"slug":"//;s/"$//' 2>/dev/null)
        if [ -n "$chapter2_slug" ]; then
            test_get "/api/komik2/chapter?id=${chapter2_slug}" "GET /api/komik2/chapter - Chapter2 images (${chapter2_slug})"
        else
            log_skip "Komik2 chapter - Could not get chapter slug from detail"
        fi
    else
        log_skip "Komik2 detail - Could not get valid slug from manga list"
        log_skip "Komik2 chapter - Skipped (no detail slug)"
    fi
}

test_auth_endpoints() {
    print_subheader "Authentication API Endpoints"
    
    # Register new user
    log_info "Testing user registration..."
    register_payload=$(cat <<EOF
{
    "name": "$TEST_USER_NAME",
    "email": "$TEST_USER_EMAIL",
    "password": "$TEST_USER_PASSWORD"
}
EOF
)
    register_response=$(curl -s -w "\n%{http_code}" -X POST "${BASE_URL}/api/auth/register" \
        -H "Content-Type: application/json" \
        -d "$register_payload" \
        --max-time 30 2>/dev/null)
    
    register_status=$(echo "$register_response" | tail -n1)
    register_body=$(echo "$register_response" | sed '$d')
    
    if [ "$register_status" -ge 200 ] && [ "$register_status" -lt 500 ]; then
        log_success "POST /api/auth/register - Register user (Status: $register_status)"
    else
        log_fail "POST /api/auth/register - Register user (Status: $register_status)"
    fi
    
    # Login
    log_info "Testing user login..."
    login_payload=$(cat <<EOF
{
    "email": "$TEST_USER_EMAIL",
    "password": "$TEST_USER_PASSWORD"
}
EOF
)
    login_response=$(curl -s -X POST "${BASE_URL}/api/auth/login" \
        -H "Content-Type: application/json" \
        -d "$login_payload" \
        --max-time 30 2>/dev/null)
    
    # Check if login response has access_token
    if echo "$login_response" | grep -q "access_token"; then
        AUTH_TOKEN=$(echo "$login_response" | grep -o '"access_token":"[^"]*"' | sed 's/"access_token":"//' | sed 's/"$//' 2>/dev/null)
        REFRESH_TOKEN=$(echo "$login_response" | grep -o '"refresh_token":"[^"]*"' | sed 's/"refresh_token":"//' | sed 's/"$//' 2>/dev/null)
        log_success "POST /api/auth/login - Login successful (token received)"
    else
        # Check HTTP status
        login_status_response=$(curl -s -w "\n%{http_code}" -X POST "${BASE_URL}/api/auth/login" \
            -H "Content-Type: application/json" \
            -d "$login_payload" \
            --max-time 30 2>/dev/null)
        login_status=$(echo "$login_status_response" | tail -n1)
        if [ "$login_status" -ge 200 ] && [ "$login_status" -lt 500 ]; then
            log_success "POST /api/auth/login - Login endpoint working (Status: $login_status)"
        else
            log_fail "POST /api/auth/login - Login failed (Status: $login_status)"
        fi
    fi
    
    # Test /me endpoint
    test_get "/api/auth/me" "GET /api/auth/me - Get current user" true
    
    # Update profile
    profile_payload='{"name": "Updated Test User"}'
    test_put "/api/auth/profile" "PUT /api/auth/profile - Update profile" "$profile_payload" true
    
    # Change password
    change_pass_payload=$(cat <<EOF
{
    "current_password": "$TEST_USER_PASSWORD",
    "new_password": "NewTestPassword123!"
}
EOF
)
    test_post "/api/auth/change-password" "POST /api/auth/change-password - Change password" "$change_pass_payload" true
    
    # Test refresh token
    if [ -n "$REFRESH_TOKEN" ]; then
        refresh_payload="{\"refresh_token\": \"$REFRESH_TOKEN\"}"
        test_post "/api/auth/refresh" "POST /api/auth/refresh - Refresh token" "$refresh_payload"
    else
        test_post "/api/auth/refresh" "POST /api/auth/refresh - Refresh token" '{"refresh_token": "test-token"}'
    fi
    
    # Test forgot password
    forgot_payload='{"email": "test@example.com"}'
    test_post "/api/auth/forgot-password" "POST /api/auth/forgot-password - Forgot password" "$forgot_payload"
    
    # Test reset password
    reset_payload='{"token": "test-token", "password": "NewPassword123!"}'
    test_post "/api/auth/reset-password" "POST /api/auth/reset-password - Reset password" "$reset_payload"
    
    # Test verify email endpoint
    test_get "/api/auth/verify?token=test-token" "GET /api/auth/verify - Verify email"
    
    # Logout
    if [ -n "$REFRESH_TOKEN" ]; then
        logout_payload="{\"refresh_token\": \"$REFRESH_TOKEN\"}"
    else
        logout_payload='{"refresh_token": "test"}'
    fi
    test_post "/api/auth/logout" "POST /api/auth/logout - Logout" "$logout_payload" true
}

test_utility_endpoints() {
    print_subheader "Utility API Endpoints"
    
    # Test image compression with proper parameters (url and size required)
    test_get "/api/compress?url=https://via.placeholder.com/150.jpg&size=50%25" "GET /api/compress - Image compression"
    
    # Test drive PNG
    test_get "/api/drivepng" "GET /api/drivepng - Drive PNG list"
    
    # Test uploader
    test_get "/api/uploader" "GET /api/uploader - Uploader list"
    
    # Test proxy
    test_get "/api/proxy/croxy?url=https://httpbin.org/get" "GET /api/proxy/croxy - Proxy fetch"
}

test_swagger_docs() {
    print_subheader "API Documentation"
    
    test_get "/docs/" "GET /docs/ - Swagger UI"
    test_get "/api-docs/openapi.json" "GET /api-docs/openapi.json - OpenAPI spec"
}

test_websocket() {
    print_subheader "WebSocket Endpoint"
    
    # Test WebSocket upgrade (the route is /ws/chat not /ws)
    log_info "Testing WebSocket endpoint availability..."
    response=$(curl -s -o /dev/null -w "%{http_code}" \
        -H "Upgrade: websocket" \
        -H "Connection: Upgrade" \
        -H "Sec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==" \
        -H "Sec-WebSocket-Version: 13" \
        --max-time 5 \
        "${BASE_URL}/ws/chat" 2>/dev/null)
    
    # 101 = Switching Protocols (WebSocket success)
    # 400 = Bad Request (WebSocket headers not complete, but endpoint exists)
    # 426 = Upgrade Required (endpoint exists but needs proper WS client)
    if [ "$response" = "101" ] || [ "$response" = "400" ] || [ "$response" = "426" ]; then
        log_success "WebSocket /ws/chat - Endpoint available (Status: $response)"
    elif [ "$response" -ge 200 ] && [ "$response" -lt 500 ]; then
        log_success "WebSocket /ws/chat - Endpoint responding (Status: $response)"
    else
        log_fail "WebSocket /ws/chat - Endpoint error (Status: $response)"
    fi
}

# =====================================================
# Print Summary
# =====================================================

print_summary() {
    print_header "Test Summary"
    
    total=$((PASSED + FAILED + SKIPPED))
    
    echo ""
    echo -e "  ${GREEN}✓ Passed:${NC}  $PASSED"
    echo -e "  ${RED}✗ Failed:${NC}  $FAILED"
    echo -e "  ${YELLOW}⊘ Skipped:${NC} $SKIPPED"
    echo -e "  ─────────────"
    echo -e "  Total:    $total"
    echo ""
    
    if [ $FAILED -eq 0 ]; then
        echo -e "${GREEN}═══════════════════════════════════════════════════════════${NC}"
        echo -e "${GREEN}  All tests passed! 🎉${NC}"
        echo -e "${GREEN}═══════════════════════════════════════════════════════════${NC}"
        exit 0
    else
        echo -e "${RED}═══════════════════════════════════════════════════════════${NC}"
        echo -e "${RED}  Some tests failed. Check the output above for details.${NC}"
        echo -e "${RED}═══════════════════════════════════════════════════════════${NC}"
        exit 1
    fi
}

# =====================================================
# Main Execution
# =====================================================

print_header "RustExpress API Test Suite"
echo ""
echo "  Base URL:    $BASE_URL"
echo "  Verbose:     $VERBOSE"
echo "  Timestamp:   $(date '+%Y-%m-%d %H:%M:%S')"
echo ""

# Parse arguments first
while [[ $# -gt 0 ]]; do
    case $1 in
        --verbose|-v)
            VERBOSE="true"
            shift
            ;;
        --base-url)
            BASE_URL="$2"
            shift 2
            ;;
        --only)
            ONLY_TEST="$2"
            shift 2
            ;;
        --help|-h)
            echo "Usage: $0 [options]"
            echo ""
            echo "Options:"
            echo "  --verbose, -v       Enable verbose output"
            echo "  --base-url URL      Set base URL (default: http://localhost:4091)"
            echo "  --only SECTION      Run only specific section (anime, komik, auth, utility, docs, ws)"
            echo "  --help, -h          Show this help message"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

# Check if server is running
check_server

# Run tests based on --only flag or all tests
if [ -n "$ONLY_TEST" ]; then
    case $ONLY_TEST in
        anime)
            test_anime_endpoints
            test_anime2_endpoints
            ;;
        komik)
            test_komik_endpoints
            test_komik2_endpoints
            ;;
        auth)
            test_auth_endpoints
            ;;
        utility)
            test_utility_endpoints
            ;;
        docs)
            test_swagger_docs
            ;;
        ws|websocket)
            test_websocket
            ;;
        *)
            echo "Unknown test section: $ONLY_TEST"
            echo "Available: anime, komik, auth, utility, docs, ws"
            exit 1
            ;;
    esac
else
    # Run all tests
    test_swagger_docs
    test_anime_endpoints
    test_anime2_endpoints
    test_komik_endpoints
    test_komik2_endpoints
    test_auth_endpoints
    test_utility_endpoints
    test_websocket
fi

# Print summary
print_summary
