#!/bin/bash
# Authentication Testing Script for Bash
# Run this script to test all authentication endpoints

BASE_URL="http://localhost:3000"
EMAIL="test@example.com"
USERNAME="testuser"
PASSWORD="TestPass123!@#"

echo "=== Authentication Testing Script ==="

# 1. Register User
echo -e "\n1. Testing User Registration..."
REGISTER_RESPONSE=$(curl -s -X POST "$BASE_URL/api/auth/register" \
  -H "Content-Type: application/json" \
  -d "{
    \"email\": \"$EMAIL\",
    \"username\": \"$USERNAME\",
    \"password\": \"$PASSWORD\",
    \"full_name\": \"Test User\"
  }")

echo "Registration Response: $REGISTER_RESPONSE"
VERIFICATION_TOKEN=$(echo $REGISTER_RESPONSE | jq -r '.verification_token')
echo "Verification Token: $VERIFICATION_TOKEN"

# 2. Verify Email
echo -e "\n2. Testing Email Verification..."
VERIFY_RESPONSE=$(curl -s -X GET "$BASE_URL/api/auth/verify?token=$VERIFICATION_TOKEN")
echo "Verification Response: $VERIFY_RESPONSE"

# 3. Login
echo -e "\n3. Testing Login..."
LOGIN_RESPONSE=$(curl -s -X POST "$BASE_URL/api/auth/login" \
  -H "Content-Type: application/json" \
  -d "{
    \"login\": \"$EMAIL\",
    \"password\": \"$PASSWORD\",
    \"remember_me\": false
  }")

echo "Login Response: $LOGIN_RESPONSE"
ACCESS_TOKEN=$(echo $LOGIN_RESPONSE | jq -r '.access_token')
REFRESH_TOKEN=$(echo $LOGIN_RESPONSE | jq -r '.refresh_token')
echo "Access Token: $ACCESS_TOKEN"

# 4. Get Current User
echo -e "\n4. Testing Get Current User..."
ME_RESPONSE=$(curl -s -X GET "$BASE_URL/api/auth/me" \
  -H "Authorization: Bearer $ACCESS_TOKEN")
echo "Current User: $ME_RESPONSE"

# 5. Change Password
echo -e "\n5. Testing Change Password..."
NEW_PASSWORD="NewTestPass456!@#"
CHANGE_PASSWORD_RESPONSE=$(curl -s -X POST "$BASE_URL/api/auth/change-password" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $ACCESS_TOKEN" \
  -d "{
    \"current_password\": \"$PASSWORD\",
    \"new_password\": \"$NEW_PASSWORD\"
  }")
echo "Change Password Response: $CHANGE_PASSWORD_RESPONSE"

# 6. Login with New Password
echo -e "\n6. Testing Login with New Password..."
LOGIN_RESPONSE2=$(curl -s -X POST "$BASE_URL/api/auth/login" \
  -H "Content-Type: application/json" \
  -d "{
    \"login\": \"$EMAIL\",
    \"password\": \"$NEW_PASSWORD\",
    \"remember_me\": false
  }")

ACCESS_TOKEN=$(echo $LOGIN_RESPONSE2 | jq -r '.access_token')
REFRESH_TOKEN=$(echo $LOGIN_RESPONSE2 | jq -r '.refresh_token')
echo "New Access Token: $ACCESS_TOKEN"

# 7. Refresh Token
echo -e "\n7. Testing Token Refresh..."
REFRESH_RESPONSE=$(curl -s -X POST "$BASE_URL/api/auth/refresh" \
  -H "Content-Type: application/json" \
  -d "{
    \"refresh_token\": \"$REFRESH_TOKEN\"
  }")
echo "Refresh Response: $REFRESH_RESPONSE"
ACCESS_TOKEN=$(echo $REFRESH_RESPONSE | jq -r '.access_token')
REFRESH_TOKEN=$(echo $REFRESH_RESPONSE | jq -r '.refresh_token')

# 8. Forgot Password
echo -e "\n8. Testing Forgot Password..."
FORGOT_RESPONSE=$(curl -s -X POST "$BASE_URL/api/auth/forgot-password" \
  -H "Content-Type: application/json" \
  -d "{
    \"email\": \"$EMAIL\"
  }")
echo "Forgot Password Response: $FORGOT_RESPONSE"
RESET_TOKEN=$(echo $FORGOT_RESPONSE | jq -r '.reset_token')
echo "Reset Token: $RESET_TOKEN"

# 9. Reset Password
if [ "$RESET_TOKEN" != "null" ]; then
  echo -e "\n9. Testing Password Reset..."
  FINAL_PASSWORD="FinalPass789!@#"
  RESET_RESPONSE=$(curl -s -X POST "$BASE_URL/api/auth/reset-password" \
    -H "Content-Type: application/json" \
    -d "{
      \"token\": \"$RESET_TOKEN\",
      \"new_password\": \"$FINAL_PASSWORD\"
    }")
  echo "Reset Password Response: $RESET_RESPONSE"

  # Login with final password
  echo -e "\n10. Testing Login with Reset Password..."
  LOGIN_RESPONSE3=$(curl -s -X POST "$BASE_URL/api/auth/login" \
    -H "Content-Type: application/json" \
    -d "{
      \"login\": \"$EMAIL\",
      \"password\": \"$FINAL_PASSWORD\",
      \"remember_me\": false
    }")
  ACCESS_TOKEN=$(echo $LOGIN_RESPONSE3 | jq -r '.access_token')
  echo "Login with Reset Password: Success"
fi

# 11. Logout
echo -e "\n11. Testing Logout..."
LOGOUT_RESPONSE=$(curl -s -X POST "$BASE_URL/api/auth/logout" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $ACCESS_TOKEN" \
  -d "{
    \"refresh_token\": \"$REFRESH_TOKEN\",
    \"logout_all\": false
  }")
echo "Logout Response: $LOGOUT_RESPONSE"

# 12. Try to access protected route (should fail)
echo -e "\n12. Testing Access After Logout (should fail)..."
ME_RESPONSE2=$(curl -s -X GET "$BASE_URL/api/auth/me" \
  -H "Authorization: Bearer $ACCESS_TOKEN")
echo "Access After Logout: $ME_RESPONSE2"

echo -e "\n=== All Tests Completed ==="
