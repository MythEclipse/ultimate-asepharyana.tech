//! Handler for the register endpoint.
#![allow(dead_code)]

use axum::{extract::State, response::IntoResponse, routing::post, Json, Router};
use bcrypt::{hash, DEFAULT_COST};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::Row;
use std::sync::Arc;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::models::user::{User, UserResponse};
use crate::routes::AppState;
use crate::utils::email::EmailService;
use crate::utils::error::AppError;

pub const ENDPOINT_METHOD: &str = "post";
pub const ENDPOINT_PATH: &str = "/api/auth/register";
pub const ENDPOINT_DESCRIPTION: &str = "Register a new user account";
pub const ENDPOINT_TAG: &str = "auth";
pub const OPERATION_ID: &str = "auth_register";
pub const SUCCESS_RESPONSE_BODY: &str = "Json<RegisterResponse>";

/// Register request payload
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct RegisterRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 3, max = 50, message = "Username must be between 3 and 50 characters"))]
    pub username: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,

    pub full_name: Option<String>,
}

/// Register response
#[derive(Debug, Serialize, ToSchema)]
pub struct RegisterResponse {
    pub success: bool,
    pub message: String,
    pub user: UserResponse,
    pub verification_token: Option<String>,
}

/// Validate password strength
fn validate_password_strength(password: &str) -> Result<(), AppError> {
    if password.len() < 8 {
        return Err(AppError::WeakPassword(
            "Password must be at least 8 characters".to_string(),
        ));
    }

    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    let has_lowercase = password.chars().any(|c| c.is_lowercase());
    let has_digit = password.chars().any(|c| c.is_numeric());
    let has_special = password.chars().any(|c| !c.is_alphanumeric());

    if !has_uppercase || !has_lowercase || !has_digit {
        return Err(AppError::WeakPassword(
            "Password must contain uppercase, lowercase, and numbers".to_string(),
        ));
    }

    if !has_special {
        return Err(AppError::WeakPassword(
            "Password should contain at least one special character".to_string(),
        ));
    }

    Ok(())
}

#[utoipa::path(
    post,
    path = "/api/auth/register",
    tag = "auth",
    operation_id = "auth_register",
    responses(
        (status = 200, description = "Register a new user account", body = RegisterResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Validate input
    payload
        .validate()
        .map_err(|e| AppError::Other(format!("Validation error: {}", e)))?;

    // Validate password strength
    validate_password_strength(&payload.password)?;

    // Check if email already exists
    let email_exists: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM users WHERE email = ?)"
    )
    .bind(&payload.email)
    .fetch_one(&state.db)
    .await?;

    if email_exists {
        return Err(AppError::EmailAlreadyExists);
    }

    // Check if username already exists
    let username_exists: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM users WHERE username = ?)"
    )
    .bind(&payload.username)
    .fetch_one(&state.db)
    .await?;

    if username_exists {
        return Err(AppError::UsernameAlreadyExists);
    }

    // Hash password
    let password_hash = hash(&payload.password, DEFAULT_COST)?;

    // Generate user ID
    let user_id = Uuid::new_v4().to_string();
    let now = Utc::now();

    // Insert user into database
    sqlx::query(
        r#"
        INSERT INTO users (
            id, email, username, password_hash, full_name,
            email_verified, is_active, role, created_at, updated_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&user_id)
    .bind(&payload.email)
    .bind(&payload.username)
    .bind(&password_hash)
    .bind(&payload.full_name)
    .bind(false)
    .bind(true)
    .bind("user")
    .bind(now)
    .bind(now)
    .execute(&state.db)
    .await?;

    // Generate email verification token
    let verification_token = Uuid::new_v4().to_string();
    let expires_at = Utc::now() + chrono::Duration::hours(24);

    sqlx::query(
        r#"
        INSERT INTO email_verification_tokens (id, user_id, token, expires_at, created_at)
        VALUES (?, ?, ?, ?, ?)
        "#,
    )
    .bind(Uuid::new_v4().to_string())
    .bind(&user_id)
    .bind(&verification_token)
    .bind(expires_at)
    .bind(now)
    .execute(&state.db)
    .await?;

    // Fetch the created user
    let user: User = sqlx::query_as(
        r#"
        SELECT id, email, username, password_hash, full_name, avatar_url,
               email_verified, is_active, role, last_login_at, created_at, updated_at
        FROM users WHERE id = ?
        "#,
    )
    .bind(&user_id)
    .fetch_one(&state.db)
    .await?;

    // Send verification email
    let email_service = EmailService::new();
    let user_name = payload.full_name.as_deref().unwrap_or(&payload.username);
    if let Err(e) = email_service
        .send_verification_email(&user.email, user_name, &verification_token)
        .await
    {
        tracing::warn!("Failed to send verification email: {}", e);
        // Don't fail registration if email fails
    }

    Ok((
        axum::http::StatusCode::CREATED,
        Json(RegisterResponse {
            success: true,
            message: "User registered successfully. Please check your email to verify your account.".to_string(),
            user: user.into(),
            verification_token: Some(verification_token),
        }),
    ))
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, post(register))
}