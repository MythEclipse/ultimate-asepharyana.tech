//! Handler for change password endpoint.
#![allow(dead_code)]

use axum::{
    extract::State,
    http::HeaderMap,
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use utoipa::ToSchema;

use crate::routes::AppState;
use crate::utils::auth::decode_jwt;
use crate::utils::email::EmailService;
use crate::utils::error::AppError;

pub const ENDPOINT_METHOD: &str = "post";
pub const ENDPOINT_PATH: &str = "/api/auth/change-password";
pub const ENDPOINT_DESCRIPTION: &str = "Change user password (authenticated)";
pub const ENDPOINT_TAG: &str = "auth";
pub const OPERATION_ID: &str = "auth_change_password";
pub const SUCCESS_RESPONSE_BODY: &str = "Json<ChangePasswordResponse>";

/// Change password request
#[derive(Debug, Deserialize, ToSchema)]
pub struct ChangePasswordRequest {
    pub current_password: String,
    pub new_password: String,
}

/// Change password response
#[derive(Debug, Serialize, ToSchema)]
pub struct ChangePasswordResponse {
    pub success: bool,
    pub message: String,
}

/// Extract Bearer token from Authorization header
fn extract_token(headers: &HeaderMap) -> Result<String, AppError> {
    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(AppError::Unauthorized)?;

    if !auth_header.starts_with("Bearer ") {
        return Err(AppError::Unauthorized);
    }

    Ok(auth_header[7..].to_string())
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

    if !has_uppercase || !has_lowercase || !has_digit {
        return Err(AppError::WeakPassword(
            "Password must contain uppercase, lowercase, and numbers".to_string(),
        ));
    }

    Ok(())
}

#[utoipa::path(
    post,
    path = "/api/auth/change-password",
    tag = "auth",
    operation_id = "auth_change_password",
    responses(
        (status = 200, description = "Change user password (authenticated)", body = ChangePasswordResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn change_password(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(payload): Json<ChangePasswordRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Extract and decode JWT token
    let token = extract_token(&headers)?;
    let claims = decode_jwt(&token)?;

    // Validate new password strength
    validate_password_strength(&payload.new_password)?;

    // Get current password hash
    let current_password_hash: String = sqlx::query_scalar(
        "SELECT password_hash FROM users WHERE id = ?"
    )
    .bind(&claims.user_id)
    .fetch_one(&state.db)
    .await?;

    // Verify current password
    let password_valid = verify(&payload.current_password, &current_password_hash)?;
    if !password_valid {
        return Err(AppError::InvalidCredentials);
    }

    // Hash new password
    let new_password_hash = hash(&payload.new_password, DEFAULT_COST)?;

    // Update password
    sqlx::query(
        "UPDATE users SET password_hash = ?, updated_at = ? WHERE id = ?"
    )
    .bind(&new_password_hash)
    .bind(Utc::now())
    .bind(&claims.user_id)
    .execute(&state.db)
    .await?;

    // Optionally revoke all refresh tokens for security
    sqlx::query("UPDATE refresh_tokens SET revoked = TRUE WHERE user_id = ?")
        .bind(&claims.user_id)
        .execute(&state.db)
        .await?;

    // Send password changed notification email
    let email_service = EmailService::new();
    if let Err(e) = email_service
        .send_password_changed_email(&claims.email, &claims.name)
        .await
    {
        tracing::warn!("Failed to send password changed notification: {}", e);
    }

    Ok(Json(ChangePasswordResponse {
        success: true,
        message: "Password changed successfully. Please login again.".to_string(),
    }))
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, post(change_password))
}