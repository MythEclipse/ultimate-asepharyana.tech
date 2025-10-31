//! Handler for the login endpoint.
#![allow(dead_code)]

use axum::{extract::State, response::IntoResponse, routing::post, Json, Router};
use bcrypt::verify;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::models::user::{LoginResponse, User, UserResponse};
use crate::routes::AppState;
use crate::utils::auth::{encode_jwt, Claims};
use crate::utils::error::AppError;

pub const ENDPOINT_METHOD: &str = "post";
pub const ENDPOINT_PATH: &str = "/api/auth/login";
pub const ENDPOINT_DESCRIPTION: &str = "Authenticate user and return JWT tokens";
pub const ENDPOINT_TAG: &str = "auth";
pub const OPERATION_ID: &str = "auth_login";
pub const SUCCESS_RESPONSE_BODY: &str = "Json<LoginResponse>";

/// Login request payload
#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginRequest {
    /// Email or username
    pub login: String,
    /// User password
    pub password: String,
    /// Remember me option (extends token expiry)
    #[serde(default)]
    pub remember_me: bool,
}

/// Login metadata for tracking
#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginMetadata {
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}

#[utoipa::path(
    post,
    path = "/api/auth/login",
    tag = "auth",
    operation_id = "auth_login",
    responses(
        (status = 200, description = "Authenticate user and return JWT tokens", body = LoginResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Find user by email or username
    let user: Option<User> = sqlx::query_as(
        r#"
        SELECT id, email, username, password_hash, full_name, avatar_url,
               email_verified, is_active, role, last_login_at, created_at, updated_at
        FROM users
        WHERE email = ? OR username = ?
        "#,
    )
    .bind(&payload.login)
    .bind(&payload.login)
    .fetch_optional(&state.db)
    .await?;

    let user = user.ok_or(AppError::InvalidCredentials)?;

    // Verify password
    let password_valid = verify(&payload.password, &user.password_hash)?;
    if !password_valid {
        // Log failed login attempt
        log_login_attempt(&state, &user.id, false, Some("Invalid password")).await?;
        return Err(AppError::InvalidCredentials);
    }

    // Check if account is active
    if !user.is_active {
        return Err(AppError::AccountInactive);
    }

    // Check if email is verified (optional - you can skip this check)
    // if !user.email_verified {
    //     return Err(AppError::EmailNotVerified);
    // }

    // Generate JWT tokens
    let token_expiry = if payload.remember_me { 30 * 24 * 3600 } else { 24 * 3600 }; // 30 days or 24 hours
    let exp = (Utc::now().timestamp() + token_expiry) as usize;

    let claims = Claims {
        user_id: user.id.clone(),
        email: user.email.clone(),
        name: user.username.clone(),
        exp,
    };

    let access_token = encode_jwt(claims)?;

    // Generate refresh token
    let refresh_token = Uuid::new_v4().to_string();
    let refresh_expires_at = Utc::now() + chrono::Duration::days(30);

    // Store refresh token in database
    sqlx::query(
        r#"
        INSERT INTO refresh_tokens (id, user_id, token, expires_at, created_at)
        VALUES (?, ?, ?, ?, ?)
        "#,
    )
    .bind(Uuid::new_v4().to_string())
    .bind(&user.id)
    .bind(&refresh_token)
    .bind(refresh_expires_at)
    .bind(Utc::now())
    .execute(&state.db)
    .await?;

    // Update last login timestamp
    sqlx::query("UPDATE users SET last_login_at = ? WHERE id = ?")
        .bind(Utc::now())
        .bind(&user.id)
        .execute(&state.db)
        .await?;

    // Log successful login
    log_login_attempt(&state, &user.id, true, None).await?;

    Ok(Json(LoginResponse {
        user: user.into(),
        access_token,
        refresh_token,
        token_type: "Bearer".to_string(),
        expires_in: token_expiry,
    }))
}

/// Log login attempt for security tracking
async fn log_login_attempt(
    state: &AppState,
    user_id: &str,
    success: bool,
    failure_reason: Option<&str>,
) -> Result<(), AppError> {
    sqlx::query(
        r#"
        INSERT INTO login_history (id, user_id, success, failure_reason, created_at)
        VALUES (?, ?, ?, ?, ?)
        "#,
    )
    .bind(Uuid::new_v4().to_string())
    .bind(user_id)
    .bind(success)
    .bind(failure_reason)
    .bind(Utc::now())
    .execute(&state.db)
    .await?;

    Ok(())
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, post(login))
}