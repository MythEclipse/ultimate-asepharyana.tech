#!/bin/bash

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Parse command line arguments
START_SERVER=false
while getopts "s" opt; do
    case $opt in
        s)
            START_SERVER=true
            ;;
        \?)
            echo "Invalid option: -$OPTARG" >&2
            echo "Usage: $0 [-s]"
            echo "  -s: Start server automatically before testing"
            exit 1
            ;;
    esac
done

# API Base URL
BASE_URL="${API_URL:-http://localhost:4092}"

# Variable to track server PID
SERVER_PID=""

# Test counters
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0

# Global variables for tokens and IDs
ACCESS_TOKEN=""
REFRESH_TOKEN=""
USER_ID=""
POST_ID=""
COMMENT_ID=""
ROOM_ID=""
MESSAGE_ID=""
TEST_EMAIL="test_$(date +%s)@example.com"
TEST_PASSWORD="TestPassword123!"
TEST_USERNAME="testuser_$(date +%s)"

# Function to print colored output
print_status() {
    local status=$1
    local message=$2
    if [ "$status" = "PASS" ]; then
        echo -e "${GREEN}✓ PASS${NC}: $message"
        ((PASSED_TESTS++))
    elif [ "$status" = "FAIL" ]; then
        echo -e "${RED}✗ FAIL${NC}: $message"
        ((FAILED_TESTS++))
    elif [ "$status" = "INFO" ]; then
        echo -e "${BLUE}ℹ INFO${NC}: $message"
    elif [ "$status" = "WARN" ]; then
        echo -e "${YELLOW}⚠ WARN${NC}: $message"
    fi
    ((TOTAL_TESTS++))
}

# Function to make API request and check status
test_api() {
    local method=$1
    local endpoint=$2
    local data=$3
    local expected_status=$4
    local description=$5
    local auth_header=$6

    local url="${BASE_URL}${endpoint}"
    local response
    local http_code

    if [ -n "$auth_header" ]; then
        if [ "$method" = "GET" ]; then
            response=$(curl -s -w "\n%{http_code}" -X "$method" \
                -H "Authorization: Bearer $auth_header" \
                -H "Content-Type: application/json" \
                "$url")
        else
            response=$(curl -s -w "\n%{http_code}" -X "$method" \
                -H "Authorization: Bearer $auth_header" \
                -H "Content-Type: application/json" \
                -d "$data" \
                "$url")
        fi
    else
        if [ "$method" = "GET" ]; then
            response=$(curl -s -w "\n%{http_code}" -X "$method" \
                -H "Content-Type: application/json" \
                "$url")
        else
            response=$(curl -s -w "\n%{http_code}" -X "$method" \
                -H "Content-Type: application/json" \
                -d "$data" \
                "$url")
        fi
    fi

    http_code=$(echo "$response" | tail -n1)
    body=$(echo "$response" | sed '$d')

    if [ "$http_code" = "$expected_status" ]; then
        print_status "PASS" "$description (HTTP $http_code)"
        echo "$body"
        return 0
    else
        print_status "FAIL" "$description (Expected: $expected_status, Got: $http_code)"
        echo "Response: $body"
        return 1
    fi
}

# Function to start server
start_server() {
    echo -e "\n${CYAN}>>> Starting ElysiaJS server...${NC}"

    # Start server in background
    bun run dev > server-output.log 2> server-error.log &
    SERVER_PID=$!

    echo -e "${GREEN}Server started with PID: $SERVER_PID${NC}"
    echo -e "${YELLOW}Waiting for server to be ready...${NC}"

    # Wait for server to be ready (max 30 seconds)
    local max_retries=30
    local retry_count=0
    local server_ready=false

    while [ $retry_count -lt $max_retries ] && [ "$server_ready" = false ]; do
        sleep 1
        if curl -s "$BASE_URL/health" > /dev/null 2>&1; then
            server_ready=true
            echo -e "${GREEN}Server is ready!${NC}"
        else
            ((retry_count++))
            echo -n "."
        fi
    done

    echo ""

    if [ "$server_ready" = false ]; then
        echo -e "${RED}Server failed to start within 30 seconds${NC}"
        stop_server
        exit 1
    fi
}

# Function to stop server
stop_server() {
    if [ -n "$SERVER_PID" ]; then
        echo -e "\n${CYAN}>>> Stopping server...${NC}"
        kill $SERVER_PID 2>/dev/null
        echo -e "${GREEN}Server stopped (PID: $SERVER_PID)${NC}"
    fi
}

# Trap to ensure server is stopped on exit
trap stop_server EXIT

echo -e "${BLUE}================================${NC}"
echo -e "${BLUE}  ElysiaJS API Testing Script${NC}"
echo -e "${BLUE}================================${NC}"
echo -e "${BLUE}Base URL: $BASE_URL${NC}"
echo -e "${BLUE}Test Email: $TEST_EMAIL${NC}"
echo -e "${BLUE}Auto-start server: $(if [ "$START_SERVER" = true ]; then echo "Yes"; else echo "No"; fi)${NC}"
echo ""

# Start server if -s flag is provided
if [ "$START_SERVER" = true ]; then
    start_server
fi

# ========================
# 1. Health & Basic Tests
# ========================
echo -e "\n${YELLOW}=== Health & Basic Endpoints ===${NC}"

test_api "GET" "/" "" "200" "Root endpoint"
test_api "GET" "/health" "" "200" "Health check endpoint"
test_api "GET" "/api/hello/World" "" "200" "Hello endpoint with parameter"
test_api "POST" "/api/echo" '{"test":"data"}' "200" "Echo endpoint"

# ========================
# 2. Authentication Tests
# ========================
echo -e "\n${YELLOW}=== Authentication Endpoints ===${NC}"

# Register new user
echo -e "\n${BLUE}>>> Registering new user...${NC}"
REGISTER_RESPONSE=$(curl -s -X POST "${BASE_URL}/api/auth/register" \
    -H "Content-Type: application/json" \
    -d "{
        \"email\": \"$TEST_EMAIL\",
        \"password\": \"$TEST_PASSWORD\",
        \"name\": \"Test User\",
        \"username\": \"$TEST_USERNAME\"
    }")

if echo "$REGISTER_RESPONSE" | grep -q "user"; then
    print_status "PASS" "User registration"
    USER_ID=$(echo "$REGISTER_RESPONSE" | grep -o '"id":"[^"]*"' | head -1 | cut -d'"' -f4)
    echo "User ID: $USER_ID"
else
    print_status "FAIL" "User registration"
    echo "Response: $REGISTER_RESPONSE"
fi

# Login
echo -e "\n${BLUE}>>> Logging in...${NC}"
LOGIN_RESPONSE=$(curl -s -X POST "${BASE_URL}/api/auth/login" \
    -H "Content-Type: application/json" \
    -d "{
        \"email\": \"$TEST_EMAIL\",
        \"password\": \"$TEST_PASSWORD\"
    }")

if echo "$LOGIN_RESPONSE" | grep -q "accessToken"; then
    print_status "PASS" "User login"
    ACCESS_TOKEN=$(echo "$LOGIN_RESPONSE" | grep -o '"accessToken":"[^"]*"' | cut -d'"' -f4)
    REFRESH_TOKEN=$(echo "$LOGIN_RESPONSE" | grep -o '"refreshToken":"[^"]*"' | cut -d'"' -f4)
    echo "Access Token: ${ACCESS_TOKEN:0:50}..."
    echo "Refresh Token: ${REFRESH_TOKEN:0:50}..."
else
    print_status "FAIL" "User login"
    echo "Response: $LOGIN_RESPONSE"
    exit 1
fi

# Get current user info
echo -e "\n${BLUE}>>> Getting current user info...${NC}"
test_api "GET" "/api/auth/me" "" "200" "Get current user info" "$ACCESS_TOKEN"

# Refresh token
echo -e "\n${BLUE}>>> Refreshing token...${NC}"
REFRESH_RESPONSE=$(curl -s -X POST "${BASE_URL}/api/auth/refresh-token" \
    -H "Content-Type: application/json" \
    -d "{\"refreshToken\": \"$REFRESH_TOKEN\"}")

if echo "$REFRESH_RESPONSE" | grep -q "accessToken"; then
    print_status "PASS" "Token refresh"
    ACCESS_TOKEN=$(echo "$REFRESH_RESPONSE" | grep -o '"accessToken":"[^"]*"' | cut -d'"' -f4)
else
    print_status "FAIL" "Token refresh"
fi

# ========================
# 3. Social Media Tests
# ========================
echo -e "\n${YELLOW}=== Social Media Endpoints ===${NC}"

# Create post
echo -e "\n${BLUE}>>> Creating a post...${NC}"
POST_RESPONSE=$(curl -s -X POST "${BASE_URL}/api/sosmed/posts" \
    -H "Authorization: Bearer $ACCESS_TOKEN" \
    -H "Content-Type: application/json" \
    -d '{
        "content": "This is a test post from API testing script!",
        "imageUrl": "https://example.com/test-image.jpg"
    }')

if echo "$POST_RESPONSE" | grep -q "id"; then
    print_status "PASS" "Create post"
    POST_ID=$(echo "$POST_RESPONSE" | grep -o '"id":"[^"]*"' | head -1 | cut -d'"' -f4)
    echo "Post ID: $POST_ID"
else
    print_status "FAIL" "Create post"
    echo "Response: $POST_RESPONSE"
fi

# Get all posts
echo -e "\n${BLUE}>>> Getting all posts...${NC}"
test_api "GET" "/api/sosmed/posts" "" "200" "Get all posts" "$ACCESS_TOKEN"

# Update post
if [ -n "$POST_ID" ]; then
    echo -e "\n${BLUE}>>> Updating post...${NC}"
    test_api "PUT" "/api/sosmed/posts/$POST_ID" \
        '{"content": "Updated test post content"}' \
        "200" "Update post" "$ACCESS_TOKEN"
fi

# Like post
if [ -n "$POST_ID" ]; then
    echo -e "\n${BLUE}>>> Liking post...${NC}"
    test_api "POST" "/api/sosmed/posts/$POST_ID/like" "" "200" "Like post" "$ACCESS_TOKEN"
fi

# Create comment
if [ -n "$POST_ID" ]; then
    echo -e "\n${BLUE}>>> Creating comment...${NC}"
    COMMENT_RESPONSE=$(curl -s -X POST "${BASE_URL}/api/sosmed/comments" \
        -H "Authorization: Bearer $ACCESS_TOKEN" \
        -H "Content-Type: application/json" \
        -d "{
            \"postId\": \"$POST_ID\",
            \"content\": \"This is a test comment!\"
        }")

    if echo "$COMMENT_RESPONSE" | grep -q "id"; then
        print_status "PASS" "Create comment"
        COMMENT_ID=$(echo "$COMMENT_RESPONSE" | grep -o '"id":"[^"]*"' | head -1 | cut -d'"' -f4)
        echo "Comment ID: $COMMENT_ID"
    else
        print_status "FAIL" "Create comment"
        echo "Response: $COMMENT_RESPONSE"
    fi
fi

# Update comment
if [ -n "$COMMENT_ID" ]; then
    echo -e "\n${BLUE}>>> Updating comment...${NC}"
    test_api "PUT" "/api/sosmed/comments/$COMMENT_ID" \
        '{"content": "Updated comment content"}' \
        "200" "Update comment" "$ACCESS_TOKEN"
fi

# Delete comment
if [ -n "$COMMENT_ID" ]; then
    echo -e "\n${BLUE}>>> Deleting comment...${NC}"
    test_api "DELETE" "/api/sosmed/comments/$COMMENT_ID" "" "200" "Delete comment" "$ACCESS_TOKEN"
fi

# Unlike post
if [ -n "$POST_ID" ]; then
    echo -e "\n${BLUE}>>> Unliking post...${NC}"
    test_api "DELETE" "/api/sosmed/posts/$POST_ID/like" "" "200" "Unlike post" "$ACCESS_TOKEN"
fi

# Delete post
if [ -n "$POST_ID" ]; then
    echo -e "\n${BLUE}>>> Deleting post...${NC}"
    test_api "DELETE" "/api/sosmed/posts/$POST_ID" "" "200" "Delete post" "$ACCESS_TOKEN"
fi

# ========================
# 4. Chat Tests
# ========================
echo -e "\n${YELLOW}=== Chat Endpoints ===${NC}"

# Create chat room
echo -e "\n${BLUE}>>> Creating chat room...${NC}"
ROOM_RESPONSE=$(curl -s -X POST "${BASE_URL}/api/chat/rooms" \
    -H "Authorization: Bearer $ACCESS_TOKEN" \
    -H "Content-Type: application/json" \
    -d '{
        "name": "Test Chat Room",
        "description": "A test chat room created by API testing script"
    }')

if echo "$ROOM_RESPONSE" | grep -q "id"; then
    print_status "PASS" "Create chat room"
    ROOM_ID=$(echo "$ROOM_RESPONSE" | grep -o '"id":"[^"]*"' | head -1 | cut -d'"' -f4)
    echo "Room ID: $ROOM_ID"
else
    print_status "FAIL" "Create chat room"
    echo "Response: $ROOM_RESPONSE"
fi

# Get all rooms
echo -e "\n${BLUE}>>> Getting all chat rooms...${NC}"
test_api "GET" "/api/chat/rooms" "" "200" "Get all chat rooms" "$ACCESS_TOKEN"

# Join chat room
if [ -n "$ROOM_ID" ]; then
    echo -e "\n${BLUE}>>> Joining chat room...${NC}"
    test_api "POST" "/api/chat/rooms/$ROOM_ID/join" "" "200" "Join chat room" "$ACCESS_TOKEN"
fi

# Send message
if [ -n "$ROOM_ID" ]; then
    echo -e "\n${BLUE}>>> Sending message...${NC}"
    MESSAGE_RESPONSE=$(curl -s -X POST "${BASE_URL}/api/chat/rooms/$ROOM_ID/messages" \
        -H "Authorization: Bearer $ACCESS_TOKEN" \
        -H "Content-Type: application/json" \
        -d '{
            "content": "Hello! This is a test message from API testing script."
        }')

    if echo "$MESSAGE_RESPONSE" | grep -q "id"; then
        print_status "PASS" "Send message"
        MESSAGE_ID=$(echo "$MESSAGE_RESPONSE" | grep -o '"id":"[^"]*"' | head -1 | cut -d'"' -f4)
        echo "Message ID: $MESSAGE_ID"
    else
        print_status "FAIL" "Send message"
        echo "Response: $MESSAGE_RESPONSE"
    fi
fi

# Get messages
if [ -n "$ROOM_ID" ]; then
    echo -e "\n${BLUE}>>> Getting messages...${NC}"
    test_api "GET" "/api/chat/rooms/$ROOM_ID/messages" "" "200" "Get messages" "$ACCESS_TOKEN"
fi

# Leave chat room
if [ -n "$ROOM_ID" ]; then
    echo -e "\n${BLUE}>>> Leaving chat room...${NC}"
    test_api "POST" "/api/chat/rooms/$ROOM_ID/leave" "" "200" "Leave chat room" "$ACCESS_TOKEN"
fi

# ========================
# 5. Logout
# ========================
echo -e "\n${YELLOW}=== Logout ===${NC}"
test_api "POST" "/api/auth/logout" "" "200" "User logout" "$ACCESS_TOKEN"

# ========================
# Test Summary
# ========================
echo -e "\n${BLUE}================================${NC}"
echo -e "${BLUE}      Test Summary${NC}"
echo -e "${BLUE}================================${NC}"
echo -e "Total Tests: ${TOTAL_TESTS}"
echo -e "${GREEN}Passed: ${PASSED_TESTS}${NC}"
echo -e "${RED}Failed: ${FAILED_TESTS}${NC}"

if [ $FAILED_TESTS -eq 0 ]; then
    echo -e "\n${GREEN}✓ All tests passed!${NC}"
    exit 0
else
    echo -e "\n${RED}✗ Some tests failed!${NC}"
    exit 1
fi
