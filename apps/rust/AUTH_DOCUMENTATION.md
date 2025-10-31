# Authentication System Documentation

## Overview

Sistem authentication yang lengkap dan production-ready dengan fitur:
- User registration dengan validasi email dan password strength
- Login dengan JWT tokens (access + refresh token)
- Logout dengan token blacklisting menggunakan Redis
- Email verification
- Password reset flow (forgot password → reset password)
- Change password untuk authenticated users
- Protected routes dengan middleware
- Role-based access control (RBAC) ready

## Database Schema

### Tables Created:
1. **users** - User accounts dengan fields lengkap
2. **email_verification_tokens** - Token untuk verifikasi email
3. **password_reset_tokens** - Token untuk reset password
4. **refresh_tokens** - Refresh tokens dengan expiry tracking
5. **login_history** - Audit log untuk tracking login attempts

### Run Migration:
```bash
sqlx migrate run
```

## API Endpoints

### 1. Register User
**POST** `/api/auth/register`

Request:
```json
{
  "email": "user@example.com",
  "username": "johndoe",
  "password": "SecurePass123!",
  "full_name": "John Doe"
}
```

Response (201):
```json
{
  "success": true,
  "message": "User registered successfully. Please check your email to verify your account.",
  "user": {
    "id": "uuid",
    "email": "user@example.com",
    "username": "johndoe",
    "full_name": "John Doe",
    "email_verified": false,
    "is_active": true,
    "role": "user",
    "created_at": "2025-11-01T00:00:00Z",
    "updated_at": "2025-11-01T00:00:00Z"
  },
  "verification_token": "uuid"
}
```

### 2. Login
**POST** `/api/auth/login`

Request:
```json
{
  "login": "user@example.com",  // email or username
  "password": "SecurePass123!",
  "remember_me": false
}
```

Response (200):
```json
{
  "user": { /* user object */ },
  "access_token": "eyJhbGciOiJIUzI1NiIs...",
  "refresh_token": "uuid",
  "token_type": "Bearer",
  "expires_in": 86400
}
```

### 3. Logout
**POST** `/api/auth/logout`
**Headers:** `Authorization: Bearer {access_token}`

Request:
```json
{
  "refresh_token": "uuid",
  "logout_all": false  // true to logout from all devices
}
```

Response (200):
```json
{
  "success": true,
  "message": "Logged out successfully"
}
```

### 4. Verify Email
**GET** `/api/auth/verify?token={verification_token}`

Response (200):
```json
{
  "success": true,
  "message": "Email verified successfully"
}
```

### 5. Resend Verification Email
**POST** `/api/auth/verify/resend`

Request:
```json
{
  "email": "user@example.com"
}
```

### 6. Refresh Token
**POST** `/api/auth/refresh`

Request:
```json
{
  "refresh_token": "uuid"
}
```

Response (200):
```json
{
  "access_token": "new_jwt_token",
  "refresh_token": "new_refresh_token",
  "token_type": "Bearer",
  "expires_in": 86400
}
```

### 7. Forgot Password
**POST** `/api/auth/forgot-password`

Request:
```json
{
  "email": "user@example.com"
}
```

Response (200):
```json
{
  "success": true,
  "message": "If the email exists, a password reset link has been sent",
  "reset_token": "uuid"  // Only in development
}
```

### 8. Reset Password
**POST** `/api/auth/reset-password`

Request:
```json
{
  "token": "reset_token_uuid",
  "new_password": "NewSecurePass123!"
}
```

Response (200):
```json
{
  "success": true,
  "message": "Password reset successfully. Please login with your new password."
}
```

### 9. Change Password (Protected)
**POST** `/api/auth/change-password`
**Headers:** `Authorization: Bearer {access_token}`

Request:
```json
{
  "current_password": "SecurePass123!",
  "new_password": "NewSecurePass456!"
}
```

Response (200):
```json
{
  "success": true,
  "message": "Password changed successfully. Please login again."
}
```

### 10. Get Current User (Protected)
**GET** `/api/auth/me`
**Headers:** `Authorization: Bearer {access_token}`

Response (200):
```json
{
  "id": "uuid",
  "email": "user@example.com",
  "username": "johndoe",
  "full_name": "John Doe",
  "avatar_url": null,
  "email_verified": true,
  "is_active": true,
  "role": "user",
  "last_login_at": "2025-11-01T00:00:00Z",
  "created_at": "2025-11-01T00:00:00Z",
  "updated_at": "2025-11-01T00:00:00Z"
}
```

## Using Authentication Middleware

### Protect Routes with Auth Middleware

```rust
use axum::{routing::get, Router, middleware};
use rust::middleware::auth::auth_layer;
use std::sync::Arc;

// Protected route example
async fn protected_handler() -> &'static str {
    "This is a protected route"
}

// Apply middleware to specific routes
let protected_routes = Router::new()
    .route("/protected", get(protected_handler))
    .layer(middleware::from_fn_with_state(state.clone(), auth_layer));

// Or use the extractor in handlers
use rust::middleware::auth::AuthMiddleware;

async fn handler(AuthMiddleware(claims): AuthMiddleware) -> String {
    format!("Hello, user {}!", claims.user_id)
}
```

### Optional Authentication

```rust
use rust::middleware::auth::optional_auth_layer;

// Routes that work with or without authentication
let optional_routes = Router::new()
    .route("/optional", get(optional_handler))
    .layer(middleware::from_fn_with_state(state.clone(), optional_auth_layer));
```

### Access User Claims in Handlers

```rust
use axum::{extract::Request, Json};
use rust::middleware::auth::get_claims_from_request;

async fn my_handler(req: Request) -> Json<String> {
    if let Some(claims) = get_claims_from_request(&req) {
        Json(format!("Authenticated as: {}", claims.email))
    } else {
        Json("Not authenticated".to_string())
    }
}
```

## Security Features

### Password Requirements:
- Minimum 8 characters
- At least one uppercase letter
- At least one lowercase letter
- At least one number
- At least one special character (recommended)

### Token Security:
- Access tokens: 24 hours expiry (30 days with remember_me)
- Refresh tokens: 30 days expiry
- Tokens blacklisted in Redis on logout
- Automatic cleanup of expired tokens

### Rate Limiting (Recommended):
Add rate limiting middleware to prevent brute force attacks:
```rust
// Add to login endpoint
.layer(middleware::from_fn(rate_limit_middleware))
```

## Environment Variables

Required in `.env`:
```env
JWT_SECRET=your-super-secret-jwt-key-change-this
DATABASE_URL=mysql://user:pass@localhost/dbname
REDIS_URL=redis://localhost:6379
```

## Testing the Endpoints

### Using curl:

#### Register:
```bash
curl -X POST http://localhost:3000/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@example.com",
    "username": "testuser",
    "password": "Test123!@#",
    "full_name": "Test User"
  }'
```

#### Login:
```bash
curl -X POST http://localhost:3000/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "login": "test@example.com",
    "password": "Test123!@#"
  }'
```

#### Access Protected Route:
```bash
curl http://localhost:3000/api/auth/me \
  -H "Authorization: Bearer YOUR_ACCESS_TOKEN"
```

#### Logout:
```bash
curl -X POST http://localhost:3000/api/auth/logout \
  -H "Authorization: Bearer YOUR_ACCESS_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "refresh_token": "YOUR_REFRESH_TOKEN"
  }'
```

## Next Steps

1. **Email Service Integration**: Implement actual email sending for verification and password reset
2. **Rate Limiting**: Add rate limiting to prevent brute force attacks
3. **2FA Support**: Add two-factor authentication
4. **OAuth Integration**: Add social login (Google, GitHub, etc.)
5. **Session Management**: Track active sessions and allow users to view/revoke them
6. **Password History**: Prevent password reuse
7. **Account Lockout**: Lock accounts after multiple failed login attempts

## Error Codes

- `400` - Bad Request (validation errors, weak password)
- `401` - Unauthorized (invalid credentials, invalid/expired token)
- `403` - Forbidden (account inactive, email not verified)
- `404` - Not Found (user not found)
- `409` - Conflict (email/username already exists)
- `500` - Internal Server Error

## Production Checklist

- [ ] Change JWT_SECRET to a strong random key
- [ ] Set up proper CORS policies
- [ ] Enable HTTPS in production
- [ ] Set up email service (SendGrid, AWS SES, etc.)
- [ ] Configure Redis for production
- [ ] Add rate limiting
- [ ] Set up monitoring and logging
- [ ] Implement backup strategy for database
- [ ] Set up automated token cleanup jobs
- [ ] Configure proper RBAC policies
- [ ] Add security headers (helmet middleware)
- [ ] Set up CI/CD for automated testing
