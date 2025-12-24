//! Handler for reset password endpoint.
#![allow(dead_code)]

use axum::{extract::State, response::IntoResponse, routing::post, Json, Router};
use bcrypt::{hash, DEFAULT_COST};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use utoipa::ToSchema;

// SeaORM imports
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use crate::entities::{user, password_reset_token};

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

    // Find password reset token using SeaORM (like Elysia)
    let token_model = password_reset_token::Entity::find()
        .filter(password_reset_token::Column::Token.eq(&payload.token))
        .one(state.sea_orm())
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .ok_or_else(|| AppError::Other("Invalid reset token".to_string()))?;

    // Check if token is already used (like Elysia)
    if token_model.used != 0 {
        return Err(AppError::Other("Reset token has already been used".to_string()));
    }

    // Check if token is expired (like Elysia)
    if token_model.expires_at < Utc::now() {
        return Err(AppError::Other("Reset token has expired".to_string()));
    }

    // Hash new password
    let password_hash = hash(&payload.new_password, DEFAULT_COST)?;

    // Find user
    let user_model = user::Entity::find_by_id(&token_model.user_id)
        .one(state.sea_orm())
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .ok_or(AppError::UserNotFound)?;

    // Get user info for notification email before updating
    let user_email = user_model.email.clone().unwrap_or_default();
    let user_name = user_model.name.clone().unwrap_or_else(|| "User".to_string());

    // Update user's password
    let mut user_active: user::ActiveModel = user_model.into();
    user_active.password = Set(Some(password_hash));
    user_active.update(state.sea_orm())
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    // Mark token as used
    let token_user_id = token_model.user_id.clone();
    let mut token_active: password_reset_token::ActiveModel = token_model.into();
    token_active.used = Set(1);
    token_active.update(state.sea_orm())
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    // Revoke all refresh tokens for security (clear refreshToken field)
    // Since refreshToken is a field in User table, we just clear it
    let mut user_update: user::ActiveModel = user::Entity::find_by_id(&token_user_id)
        .one(state.sea_orm())
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .ok_or(AppError::UserNotFound)?
        .into();
    user_update.refresh_token = Set(None);
    user_update.update(state.sea_orm())
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    // Send password changed notification email
    if !user_email.is_empty() {
        let email_service = EmailService::new();
        if let Err(e) = email_service.send_password_changed_email(&user_email, &user_name).await {
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