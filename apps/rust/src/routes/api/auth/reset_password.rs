//! Handler for reset password endpoint.
#![allow(dead_code)]

use axum::{extract::State, response::IntoResponse, routing::post, Json, Router};
use bcrypt::{hash, DEFAULT_COST};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::Row;
use std::sync::Arc;
use utoipa::ToSchema;

use crate::routes::AppState;
use crate::utils::email::EmailService;
use crate::utils::error::AppError;

pub const ENDPOINT_METHOD: &str = "post";
pub const ENDPOINT_PATH: &str = "/api/auth/reset-password";
pub const ENDPOINT_DESCRIPTION: &str = "Reset password with token";
pub const ENDPOINT_TAG: &str = "auth";
pub const OPERATION_ID: &str = "auth_reset_password";
pub const SUCCESS_RESPONSE_BODY: &str = "Json<ResetPasswordResponse>";

/// Reset password request
#[derive(Debug, Deserialize, ToSchema)]
pub struct ResetPasswordRequest {
    pub token: String,
    pub new_password: String,
}

/// Reset password response
#[derive(Debug, Serialize, ToSchema)]
pub struct ResetPasswordResponse {
    pub success: bool,
    pub message: String,
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
    path = "/api/auth/reset-password",
    tag = "auth",
    operation_id = "auth_reset_password",
    responses(
        (status = 200, description = "Reset password with token", body = ResetPasswordResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn reset_password(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ResetPasswordRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Validate password strength
    validate_password_strength(&payload.new_password)?;

    // Find password reset token
    let token_data: Option<(String, String, chrono::DateTime<Utc>, bool)> = sqlx::query_as(
        r#"
        SELECT id, user_id, expires_at, used
        FROM password_reset_tokens
        WHERE token = ?
        "#,
    )
    .bind(&payload.token)
    .fetch_optional(&state.db)
    .await?;

    let (token_id, user_id, expires_at, used) = token_data.ok_or(AppError::InvalidToken)?;

    // Check if token is already used
    if used {
        return Err(AppError::InvalidToken);
    }

    // Check if token is expired
    if expires_at < Utc::now() {
        return Err(AppError::TokenExpired);
    }

    // Hash new password
    let password_hash = hash(&payload.new_password, DEFAULT_COST)?;

    // Update user's password
    sqlx::query(
        "UPDATE users SET password_hash = ?, updated_at = ? WHERE id = ?"
    )
    .bind(&password_hash)
    .bind(Utc::now())
    .bind(&user_id)
    .execute(&state.db)
    .await?;

    // Mark token as used
    sqlx::query("UPDATE password_reset_tokens SET used = TRUE WHERE id = ?")
        .bind(&token_id)
        .execute(&state.db)
        .await?;

    // Revoke all refresh tokens for security
    sqlx::query("UPDATE refresh_tokens SET revoked = TRUE WHERE user_id = ?")
        .bind(&user_id)
        .execute(&state.db)
        .await?;

    // Get user info for notification email
    let user_info: Option<(String, String)> = sqlx::query_as(
        "SELECT email, COALESCE(full_name, username) FROM users WHERE id = ?"
    )
    .bind(&user_id)
    .fetch_optional(&state.db)
    .await?;

    // Send password changed notification email
    if let Some((email, name)) = user_info {
        let email_service = EmailService::new();
        if let Err(e) = email_service.send_password_changed_email(&email, &name).await {
            tracing::warn!("Failed to send password changed notification: {}", e);
        }
    }

    Ok(Json(ResetPasswordResponse {
        success: true,
        message: "Password reset successfully. Please login with your new password.".to_string(),
    }))
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, post(reset_password))
}