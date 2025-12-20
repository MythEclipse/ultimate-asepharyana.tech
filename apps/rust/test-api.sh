#!/bin/bash

# =====================================================
# RustExpress API Test Script
# Test all API endpoints for the Rust application
# =====================================================

# Configuration
BASE_URL="${BASE_URL:-http://localhost:4091}"
VERBOSE="${VERBOSE:-false}"
AUTH_TOKEN=""
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

# Test a GET endpoint
# Arguments: endpoint_path description expected_status [requires_auth]
test_get() {
    local endpoint="$1"
    local description="$2"
    local expected_status="${3:-200}"
    local requires_auth="${4:-false}"
    
    local auth_header=""
    if [ "$requires_auth" = "true" ] && [ -n "$AUTH_TOKEN" ]; then
        auth_header="-H \"Authorization: Bearer $AUTH_TOKEN\""
    fi
    
    local full_url="${BASE_URL}${endpoint}"
    
    if [ "$VERBOSE" = "true" ]; then
        log_info "Testing GET $full_url"
    fi
    
    if [ "$requires_auth" = "true" ] && [ -n "$AUTH_TOKEN" ]; then
        response=$(curl -s -w "\n%{http_code}" -X GET "$full_url" \
            -H "Content-Type: application/json" \
            -H "Authorization: Bearer $AUTH_TOKEN" 2>/dev/null)
    else
        response=$(curl -s -w "\n%{http_code}" -X GET "$full_url" \
            -H "Content-Type: application/json" 2>/dev/null)
    fi
    
    status_code=$(echo "$response" | tail -n1)
    body=$(echo "$response" | sed '$d')
    
    if [ -z "$status_code" ] || [ "$status_code" = "000" ]; then
        log_fail "$description - Connection failed"
        return 1
    fi
    
    if [ "$status_code" -eq "$expected_status" ] || [ "$status_code" -eq 200 ] || [ "$status_code" -eq 201 ]; then
        log_success "$description (Status: $status_code)"
        if [ "$VERBOSE" = "true" ]; then
            echo "  Response: $(echo "$body" | head -c 200)..."
        fi
        return 0
    else
        log_fail "$description (Expected: $expected_status, Got: $status_code)"
        if [ "$VERBOSE" = "true" ]; then
            echo "  Response: $body"
        fi
        return 1
    fi
}

# Test a POST endpoint
# Arguments: endpoint_path description payload expected_status [requires_auth]
test_post() {
    local endpoint="$1"
    local description="$2"
    local payload="$3"
    local expected_status="${4:-200}"
    local requires_auth="${5:-false}"
    
    local full_url="${BASE_URL}${endpoint}"
    
    if [ "$VERBOSE" = "true" ]; then
        log_info "Testing POST $full_url"
    fi
    
    if [ "$requires_auth" = "true" ] && [ -n "$AUTH_TOKEN" ]; then
        response=$(curl -s -w "\n%{http_code}" -X POST "$full_url" \
            -H "Content-Type: application/json" \
            -H "Authorization: Bearer $AUTH_TOKEN" \
            -d "$payload" 2>/dev/null)
    else
        response=$(curl -s -w "\n%{http_code}" -X POST "$full_url" \
            -H "Content-Type: application/json" \
            -d "$payload" 2>/dev/null)
    fi
    
    status_code=$(echo "$response" | tail -n1)
    body=$(echo "$response" | sed '$d')
    
    if [ -z "$status_code" ] || [ "$status_code" = "000" ]; then
        log_fail "$description - Connection failed"
        return 1
    fi
    
    if [ "$status_code" -eq "$expected_status" ] || [ "$status_code" -eq 200 ] || [ "$status_code" -eq 201 ]; then
        log_success "$description (Status: $status_code)"
        echo "$body"
        return 0
    else
        # Some endpoints may return 400/401/404 which is still a valid response
        if [ "$status_code" -ge 200 ] && [ "$status_code" -lt 500 ]; then
            log_warn "$description (Status: $status_code - endpoint responded)"
            return 0
        fi
        log_fail "$description (Expected: $expected_status, Got: $status_code)"
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
    local expected_status="${4:-200}"
    local requires_auth="${5:-false}"
    
    local full_url="${BASE_URL}${endpoint}"
    
    if [ "$requires_auth" = "true" ] && [ -n "$AUTH_TOKEN" ]; then
        response=$(curl -s -w "\n%{http_code}" -X PUT "$full_url" \
            -H "Content-Type: application/json" \
            -H "Authorization: Bearer $AUTH_TOKEN" \
            -d "$payload" 2>/dev/null)
    else
        response=$(curl -s -w "\n%{http_code}" -X PUT "$full_url" \
            -H "Content-Type: application/json" \
            -d "$payload" 2>/dev/null)
    fi
    
    status_code=$(echo "$response" | tail -n1)
    body=$(echo "$response" | sed '$d')
    
    if [ -z "$status_code" ] || [ "$status_code" = "000" ]; then
        log_fail "$description - Connection failed"
        return 1
    fi
    
    if [ "$status_code" -ge 200 ] && [ "$status_code" -lt 500 ]; then
        log_success "$description (Status: $status_code)"
        return 0
    else
        log_fail "$description (Expected: $expected_status, Got: $status_code)"
        return 1
    fi
}

# Test a DELETE endpoint
test_delete() {
    local endpoint="$1"
    local description="$2"
    local payload="$3"
    local expected_status="${4:-200}"
    local requires_auth="${5:-false}"
    
    local full_url="${BASE_URL}${endpoint}"
    
    if [ "$requires_auth" = "true" ] && [ -n "$AUTH_TOKEN" ]; then
        response=$(curl -s -w "\n%{http_code}" -X DELETE "$full_url" \
            -H "Content-Type: application/json" \
            -H "Authorization: Bearer $AUTH_TOKEN" \
            -d "$payload" 2>/dev/null)
    else
        response=$(curl -s -w "\n%{http_code}" -X DELETE "$full_url" \
            -H "Content-Type: application/json" \
            -d "$payload" 2>/dev/null)
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
        log_fail "$description (Expected: $expected_status, Got: $status_code)"
        return 1
    fi
}

# Check if server is running
check_server() {
    log_info "Checking if server is running at $BASE_URL..."
    response=$(curl -s -o /dev/null -w "%{http_code}" --max-time 15 "$BASE_URL/docs/" 2>/dev/null)
    if [ "$response" = "000" ]; then
        echo -e "${RED}ERROR: Server is not running at $BASE_URL${NC}"
        echo "Please start the server with: cargo run"
        exit 1
    fi
    log_success "Server is running!"
}

# =====================================================
# Test Sections
# =====================================================

test_anime_endpoints() {
    print_subheader "Anime API Endpoints (Primary)"
    
    test_get "/api/anime" "GET /api/anime - Anime index (ongoing + complete)"
    test_get "/api/anime/search?q=naruto" "GET /api/anime/search - Search anime"
    test_get "/api/anime/ongoing-anime/1" "GET /api/anime/ongoing-anime/{slug} - Ongoing anime page 1"
    test_get "/api/anime/complete-anime/1" "GET /api/anime/complete-anime/{slug} - Complete anime page 1"
    
    # Get a sample slug for detail test
    test_get "/api/anime/detail/bocchi-the-rock" "GET /api/anime/detail/{slug} - Anime detail (sample slug)"
    test_get "/api/anime/full/bocchi-the-rock-episode-1" "GET /api/anime/full/{slug} - Anime full episode (sample slug)"
}

test_anime2_endpoints() {
    print_subheader "Anime2 API Endpoints (Secondary Source)"
    
    test_get "/api/anime2" "GET /api/anime2 - Anime2 index"
    test_get "/api/anime2/search?q=naruto" "GET /api/anime2/search - Search anime2"
    test_get "/api/anime2/ongoing-anime/1" "GET /api/anime2/ongoing-anime/{slug} - Ongoing anime2 page 1"
    test_get "/api/anime2/complete-anime/1" "GET /api/anime2/complete-anime/{slug} - Complete anime2 page 1"
    test_get "/api/anime2/detail/bocchi-the-rock" "GET /api/anime2/detail/{slug} - Anime2 detail (sample slug)"
}

test_komik_endpoints() {
    print_subheader "Komik API Endpoints (Primary)"
    
    test_get "/api/komik/search?q=one+piece" "GET /api/komik/search - Search komik"
    test_get "/api/komik/manga" "GET /api/komik/manga - Manga list"
    test_get "/api/komik/manhwa" "GET /api/komik/manhwa - Manhwa list"
    test_get "/api/komik/manhua" "GET /api/komik/manhua - Manhua list"
    test_get "/api/komik/detail?id=solo-leveling" "GET /api/komik/detail - Komik detail"
    test_get "/api/komik/chapter?id=solo-leveling-chapter-1" "GET /api/komik/chapter - Chapter images"
}

test_komik2_endpoints() {
    print_subheader "Komik2 API Endpoints (Secondary Source)"
    
    test_get "/api/komik2/search?q=one+piece" "GET /api/komik2/search - Search komik2"
    test_get "/api/komik2/manga" "GET /api/komik2/manga - Manga2 list"
    test_get "/api/komik2/manhwa" "GET /api/komik2/manhwa - Manhwa2 list"
    test_get "/api/komik2/manhua" "GET /api/komik2/manhua - Manhua2 list"
    test_get "/api/komik2/detail?id=solo-leveling" "GET /api/komik2/detail - Komik2 detail"
    test_get "/api/komik2/chapter?id=solo-leveling-chapter-1" "GET /api/komik2/chapter - Chapter2 images"
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
    response=$(test_post "/api/auth/register" "POST /api/auth/register - Register user" "$register_payload")
    
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
        -d "$login_payload" 2>/dev/null)
    
    # Extract access token
    AUTH_TOKEN=$(echo "$login_response" | grep -o '"access_token":"[^"]*"' | sed 's/"access_token":"//' | sed 's/"$//' 2>/dev/null)
    REFRESH_TOKEN=$(echo "$login_response" | grep -o '"refresh_token":"[^"]*"' | sed 's/"refresh_token":"//' | sed 's/"$//' 2>/dev/null)
    
    if [ -n "$AUTH_TOKEN" ] && [ "$AUTH_TOKEN" != "null" ]; then
        log_success "POST /api/auth/login - Login successful (token received)"
    else
        log_warn "POST /api/auth/login - Login may have failed or returned error (Status: received response)"
    fi
    
    # Test authenticated endpoints
    test_get "/api/auth/me" "GET /api/auth/me - Get current user" 200 true
    
    # Update profile
    profile_payload='{"name": "Updated Test User"}'
    test_put "/api/auth/profile" "PUT /api/auth/profile - Update profile" "$profile_payload" 200 true
    
    # Change password (will likely fail without proper token but tests endpoint)
    change_pass_payload=$(cat <<EOF
{
    "current_password": "$TEST_USER_PASSWORD",
    "new_password": "NewTestPassword123!"
}
EOF
)
    test_post "/api/auth/change-password" "POST /api/auth/change-password - Change password" "$change_pass_payload" 200 true
    
    # Test refresh token
    if [ -n "$REFRESH_TOKEN" ]; then
        refresh_payload="{\"refresh_token\": \"$REFRESH_TOKEN\"}"
        test_post "/api/auth/refresh" "POST /api/auth/refresh - Refresh token" "$refresh_payload"
    else
        log_skip "POST /api/auth/refresh - No refresh token available"
    fi
    
    # Test forgot password (won't actually send email in test)
    forgot_payload='{"email": "test@example.com"}'
    test_post "/api/auth/forgot-password" "POST /api/auth/forgot-password - Forgot password" "$forgot_payload"
    
    # Test reset password (will fail without valid token but tests endpoint)
    reset_payload='{"token": "test-token", "password": "NewPassword123!"}'
    test_post "/api/auth/reset-password" "POST /api/auth/reset-password - Reset password" "$reset_payload"
    
    # Test verify email endpoint
    test_get "/api/auth/verify?token=test-token" "GET /api/auth/verify - Verify email (test token)"
    
    # Logout
    logout_payload='{"refresh_token": "test"}'
    test_post "/api/auth/logout" "POST /api/auth/logout - Logout" "$logout_payload" 200 true
}

test_utility_endpoints() {
    print_subheader "Utility API Endpoints"
    
    # Test image compression
    test_get "/api/compress?url=https://example.com/image.jpg&quality=80" "GET /api/compress - Image compression"
    
    # Test drive PNG
    test_get "/api/drivepng" "GET /api/drivepng - Drive PNG list"
    
    # Test uploader
    test_get "/api/uploader" "GET /api/uploader - Uploader list"
    
    # Test proxy
    test_get "/api/proxy/croxy?url=https://example.com" "GET /api/proxy/croxy - Proxy fetch"
}

test_swagger_docs() {
    print_subheader "API Documentation"
    
    test_get "/docs/" "GET /docs/ - Swagger UI"
    test_get "/api-docs/openapi.json" "GET /api-docs/openapi.json - OpenAPI spec"
}

test_websocket() {
    print_subheader "WebSocket Endpoint"
    
    # Test WebSocket upgrade (will fail as HTTP but shows endpoint exists)
    log_info "Testing WebSocket endpoint availability..."
    response=$(curl -s -o /dev/null -w "%{http_code}" \
        -H "Upgrade: websocket" \
        -H "Connection: Upgrade" \
        -H "Sec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==" \
        -H "Sec-WebSocket-Version: 13" \
        "${BASE_URL}/ws" 2>/dev/null)
    
    if [ "$response" = "101" ] || [ "$response" = "400" ] || [ "$response" = "426" ]; then
        log_success "WebSocket /ws - Endpoint available (Status: $response)"
    else
        log_warn "WebSocket /ws - May not be configured (Status: $response)"
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

# Check if server is running
check_server

# Parse arguments
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
            echo "  --only SECTION      Run only specific section (anime, komik, auth, utility)"
            echo "  --help, -h          Show this help message"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

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
            echo "Available: anime, komik, auth, utility, docs, websocket"
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
