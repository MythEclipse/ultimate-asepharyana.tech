#!/bin/bash

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# API Base URL
BASE_URL="http://localhost:4091"

# Variables
SERVER_PID=""
PASSED_TESTS=0
FAILED_TESTS=0
TOTAL_TESTS=0

# Config
START_SERVER=false

# Parse args
while getopts "s" opt; do
    case $opt in
        s)
            START_SERVER=true
            ;;
        \?)
            echo "Invalid option: -$OPTARG" >&2
            exit 1
            ;;
    esac
done

print_status() {
    local status=$1
    local message=$2
    if [ "$status" = "PASS" ]; then
        echo -e "${GREEN}✓ PASS${NC}: $message"
        ((PASSED_TESTS++))
    elif [ "$status" = "FAIL" ]; then
        echo -e "${RED}✗ FAIL${NC}: $message"
        ((FAILED_TESTS++))
    fi
    ((TOTAL_TESTS++))
}

test_api() {
    local method=$1
    local endpoint=$2
    local expected_status=$3
    local description=$4

    local url="${BASE_URL}${endpoint}"
    local response
    local http_code

    response=$(curl -s -w "\n%{http_code}" -X "$method" "$url")
    http_code=$(echo "$response" | tail -n1)
    body=$(echo "$response" | sed '$d')

    if [ "$http_code" = "$expected_status" ]; then
        print_status "PASS" "$description (HTTP $http_code)"
        # echo "Response: $body" | head -c 200 # Preview
        # echo "..."
    else
        print_status "FAIL" "$description (Expected: $expected_status, Got: $http_code)"
        echo "Response: $body"
    fi
}

start_server() {
    echo -e "\n${CYAN}>>> Starting Rust server...${NC}"
    # Start server in background
    # Assuming running from root of repo or adjust path
    # We will run this script from apps/rust, so valid path is ./Cargo.toml
    # But user might run from root. Let's find Cargo.toml
    
    if [ -f "Cargo.toml" ]; then
        MANIFEST_PATH="Cargo.toml"
    elif [ -f "apps/rust/Cargo.toml" ]; then
        MANIFEST_PATH="apps/rust/Cargo.toml"
    else 
        echo -e "${RED}Could not find apps/rust/Cargo.toml${NC}"
        exit 1
    fi

    echo "Using manifest: $MANIFEST_PATH"
    cargo run --manifest-path "$MANIFEST_PATH" > rust-server.log 2>&1 &
    SERVER_PID=$!
    
    echo -e "${GREEN}Server started with PID: $SERVER_PID${NC}"
    echo -e "${YELLOW}Waiting for server to be ready...${NC}"

    local max_retries=60
    local retry_count=0
    local server_ready=false

    while [ $retry_count -lt $max_retries ] && [ "$server_ready" = false ]; do
        sleep 2
        # Check if port 4091 is accepting connections or if /api/health works (if exists) 
        # Checking root endpoint of anime2
        if curl -s "$BASE_URL/api/anime2" > /dev/null 2>&1; then
             server_ready=true
             echo -e "${GREEN}Server is ready!${NC}"
        else
             ((retry_count++))
             echo -n "."
        fi
    done

    echo ""
    if [ "$server_ready" = false ]; then
        echo -e "${RED}Server failed to start within timeout${NC}"
        cat rust-server.log | tail -n 20
        stop_server
        exit 1
    fi
}

stop_server() {
    if [ -n "$SERVER_PID" ]; then
        echo -e "\n${CYAN}>>> Stopping server...${NC}"
        kill $SERVER_PID 2>/dev/null
        echo -e "${GREEN}Server stopped${NC}"
    fi
}

trap stop_server EXIT

if [ "$START_SERVER" = true ]; then
    start_server
fi

echo -e "\n${YELLOW}=== Testing Anime2 Endpoints ===${NC}"

# 1. Index
test_api "GET" "/api/anime2" "200" "Get Anime2 Index"

# 2. Latest
test_api "GET" "/api/anime2/latest?page=1" "200" "Get Latest Anime"

# Extract a slug from latest for further tests
LATEST_RES=$(curl -s "${BASE_URL}/api/anime2/latest?page=1")
SLUG=$(echo "$LATEST_RES" | grep -o '"slug":"[^"]*"' | head -1 | cut -d'"' -f4)

if [ -z "$SLUG" ]; then
    echo -e "${YELLOW}Could not extract slug from latest, using default 'one-piece'${NC}"
    SLUG="one-piece"
else
    echo -e "${BLUE}Using slug: $SLUG${NC}"
fi

# 3. Detail
test_api "GET" "/api/anime2/detail/$SLUG" "200" "Get Anime Detail ($SLUG)"

# 4. Search
test_api "GET" "/api/anime2/search?q=naruto&page=1" "200" "Search Anime (naruto)"

# 5. Genres
test_api "GET" "/api/anime2/genres" "200" "Get Genres List"

# 6. Filter (might need params)
test_api "GET" "/api/anime2/filter?page=1" "200" "Filter Endpoint"

# 7. Ongoing Anime (might need slug from a list of ongoing)
# Let's try to get an ongoing slug if available, or just test endpoint existence
test_api "GET" "/api/anime2/ongoing-anime/$SLUG" "200" "Ongoing Anime Info ($SLUG)"

# 8. Complete Anime 
test_api "GET" "/api/anime2/complete-anime/$SLUG" "200" "Complete Anime Info ($SLUG)"

# 9. Genre Slug (valid genre needed, e.g., 'action')
test_api "GET" "/api/anime2/genre/action?page=1" "200" "Get Genre 'action'"

echo -e "\n${BLUE}================================${NC}"
echo -e "Total Tests: ${TOTAL_TESTS}"
echo -e "${GREEN}Passed: ${PASSED_TESTS}${NC}"
echo -e "${RED}Failed: ${FAILED_TESTS}${NC}"

if [ $FAILED_TESTS -eq 0 ]; then
    exit 0
else
    exit 1
fi
