#!/bin/bash

# Define the base URL for the Next.js app
NEXTJS_APP_URL="http://localhost:4090"

# Global PIDs for background processes
NEXTJS_PID=""
EXPRESS_PID=""

# Function to run the applications in the background
run_apps() {
    # echo "Building Express app..."
    # (cd apps/Express && npm run build) # Build Express app first
    # if [ $? -ne 0 ]; then
    #     echo "Express app build failed. Exiting."
    #     exit 1
    # fi

    # echo "Starting Express app in background..."
    # (cd apps/Express && npm start) & # Start Express app using npm start
    # EXPRESS_PID=$!
    # echo "Express app started with PID: $EXPRESS_PID"

    echo "Starting Next.js app in background..."
    (bun run dev) &
    NEXTJS_PID=$!
    echo "Next.js app started with PID: $NEXTJS_PID"
    
    # Wait for the apps to start (adjust sleep time if needed)
    echo "Waiting for apps to be ready..."
    sleep 15 # Give the apps some time to fully start, increased from 10
    echo "Apps should be ready."
}

# Function to stop the applications
stop_apps() {
    if [ -n "$NEXTJS_PID" ]; then
        echo "Stopping Next.js app (PID: $NEXTJS_PID)..."
        kill "$NEXTJS_PID"
        wait "$NEXTJS_PID" 2>/dev/null
        echo "Next.js app stopped."
    fi
    if [ -n "$EXPRESS_PID" ]; then
        echo "Stopping Express app (PID: $EXPRESS_PID)..."
        kill "$EXPRESS_PID"
        wait "$EXPRESS_PID" 2>/dev/null
        echo "Express app stopped."
    fi
}

# Trap to ensure the applications are stopped on script exit
trap stop_apps EXIT

# Start the applications
run_apps

# Check if jq is installed
if ! command -v jq &> /dev/null
then
    echo "jq could not be found, please install it to parse JSON response."
    exit 1
fi

# --- Test JWT Authentication Endpoints with curl ---

echo "--- Testing Register Endpoint ---"
REGISTER_RESPONSE=$(curl -s -X POST "${NEXTJS_APP_URL}/api/jwt-auth/register" \
-H "Content-Type: application/json" \
-d '{"name": "Test User", "email": "test@example.com", "password": "password123"}' \
-c "cookie-jar.txt") # Store cookies for subsequent requests
echo "Register Response: $REGISTER_RESPONSE"

echo "--- Testing Login Endpoint ---"
LOGIN_RESPONSE=$(curl -s -X POST "${NEXTJS_APP_URL}/api/jwt-auth/login" \
-H "Content-Type: application/json" \
-d '{"email": "test@example.com", "password": "password123"}' \
-c "cookie-jar.txt" -b "cookie-jar.txt") # Send and receive cookies
echo "Login Response: $LOGIN_RESPONSE"

ACCESS_TOKEN=$(echo "$LOGIN_RESPONSE" | jq -r '.token')
REFRESH_TOKEN=$(echo "$LOGIN_RESPONSE" | jq -r '.refreshToken')

if [ "$ACCESS_TOKEN" != "null" ] && [ -n "$ACCESS_TOKEN" ] && \
   [ "$REFRESH_TOKEN" != "null" ] && [ -n "$REFRESH_TOKEN" ]; then
    echo "Login successful. Access Token received: $ACCESS_TOKEN"
    echo "Refresh Token received: $REFRESH_TOKEN"
else
    echo "Login failed or tokens not found in response."
    exit 1
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

NEW_ACCESS_TOKEN=$(echo "$REFRESH_RESPONSE" | jq -r '.accessToken')

if [ "$NEW_ACCESS_TOKEN" != "null" ] && [ -n "$NEW_ACCESS_TOKEN" ]; then
    echo "Refresh successful. New Access Token received: $NEW_ACCESS_TOKEN"
else
    echo "Refresh failed or new access token not found in response."
    exit 1
fi

# Clean up
rm -f cookie-jar.txt

echo "Tests finished."