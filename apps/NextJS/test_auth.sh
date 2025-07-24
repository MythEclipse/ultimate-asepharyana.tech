#!/bin/bash

# Define the base URL for the Next.js app
NEXTJS_APP_URL="http://localhost:4090"

# Function to run the Next.js app in the background
run_nextjs_app() {
    echo "Starting Next.js app in background..."
    (bun run dev > /dev/null 2>&1) & # Removed 'cd apps/NextJS' as the script is executed from apps/NextJS
    NEXTJS_PID=$!
    echo "Next.js app started with PID: $NEXTJS_PID"
    
    # Wait for the app to start (adjust sleep time if needed)
    echo "Waiting for Next.js app to be ready..."
    sleep 10 # Give the app some time to fully start
    echo "Next.js app should be ready."
}

# Function to stop the Next.js app
stop_nextjs_app() {
    if [ -n "$NEXTJS_PID" ]; then
        echo "Stopping Next.js app (PID: $NEXTJS_PID)..."
        kill "$NEXTJS_PID"
        wait "$NEXTJS_PID" 2>/dev/null
        echo "Next.js app stopped."
    fi
}

# Trap to ensure the Next.js app is stopped on script exit
trap stop_nextjs_app EXIT

# Start the Next.js app
run_nextjs_app

# --- Test JWT Authentication Endpoints with curl ---

echo "--- Testing Register Endpoint ---"
REGISTER_RESPONSE=$(curl -s -X POST "${NEXTJS_APP_URL}/api/jwt-auth/register" \
-H "Content-Type: application/json" \
-d '{"email": "test@example.com", "password": "password123"}' \
-c "cookie-jar.txt") # Store cookies for subsequent requests
echo "Register Response: $REGISTER_RESPONSE"

echo "--- Testing Login Endpoint ---"
LOGIN_RESPONSE=$(curl -s -X POST "${NEXTJS_APP_URL}/api/jwt-auth/login" \
-H "Content-Type: application/json" \
-d '{"email": "test@example.com", "password": "password123"}' \
-c "cookie-jar.txt" -b "cookie-jar.txt") # Send and receive cookies
echo "Login Response: $LOGIN_RESPONSE"

# Extract tokens from login response (if response is JSON and tokens are in body)
AUTH_TOKEN=$(echo "$LOGIN_RESPONSE" | grep -o '"token":"[^"]*"' | cut -d'"' -f4)
REFRESH_TOKEN=$(echo "$LOGIN_RESPONSE" | grep -o '"refreshToken":"[^"]*"' | cut -d'"' -f4)

if [ -n "$AUTH_TOKEN" ]; then
    echo "Auth Token obtained: $AUTH_TOKEN"
    echo "Refresh Token obtained: $REFRESH_TOKEN"
else
    echo "Failed to obtain tokens. Check login response."
fi

# --- Test a protected endpoint (e.g., /api/sosmed/posts) ---
echo "--- Testing Protected Endpoint (/api/sosmed/posts) with Auth Token ---"
# This assumes your /api/sosmed/posts GET endpoint requires authentication
PROTECTED_RESPONSE=$(curl -s -X GET "${NEXTJS_APP_URL}/api/sosmed/posts" \
-b "cookie-jar.txt") # Send stored cookies
echo "Protected Endpoint Response: $PROTECTED_RESPONSE"

echo "--- Testing Refresh Token Endpoint ---"
REFRESH_RESPONSE=$(curl -s -X POST "${NEXTJS_APP_URL}/api/jwt-auth/refresh" \
-b "cookie-jar.txt" -c "cookie-jar.txt") # Send refresh token and get new ones
echo "Refresh Token Response: $REFRESH_RESPONSE"

# Clean up
rm -f cookie-jar.txt

echo "Tests finished."