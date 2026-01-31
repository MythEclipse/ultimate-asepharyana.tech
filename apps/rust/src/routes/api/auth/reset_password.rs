//! Handler for reset password endpoint - Enhanced with form_request validation.

use axum::{extract::State, response::IntoResponse, Json, Router};
use bcrypt::{hash, DEFAULT_COST};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use utoipa::ToSchema;

// SeaORM imports
use crate::entities::{password_reset_token, user};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};

use crate::routes::AppState;
use crate::utils::email::EmailService;
use crate::utils::error::AppError;

// New helpers
use crate::helpers::form_request::{validate, ValidationRules};

pub const ENDPOINT_METHOD: &str = "post";
pub const ENDPOINT_PATH: &str = "/api/auth/reset-password";
pub const ENDPOINT_DESCRIPTION: &str = "Reset password with token";
pub const ENDPOINT_TAG: &str = "auth";
pub const OPERATION_ID: &str = "auth_reset_password";
pub const SUCCESS_RESPONSE_BODY: &str = "Json<ResetPasswordResponse>";

/// Reset password request
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ResetPasswordRequest {
    pub token: String,
    pub new_password: String,
    pub password_confirmation: Option<String>,
}

/// Reset password response
#[derive(Debug, Serialize, ToSchema)]
pub struct ResetPasswordResponse {
    pub success: bool,
    pub message: String,
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
    // Validate with form_request helper
    let data = serde_json::to_value(&payload).unwrap_or_default();
    let mut rules = ValidationRules::new();
    rules
        .required("token")
        .required("new_password")
        .min_length("new_password", 8);

    if payload.password_confirmation.is_some() {
        rules.confirmed("new_password", "password_confirmation");
    }

    let validation = validate(&data, &rules);
    if !validation.is_valid() {
        return Err(AppError::Other(
            validation
                .errors
                .first()
                .map(|e| e.message.clone())
                .unwrap_or_default(),
        ));
    }

    // Validate password strength
    validate_password_strength(&payload.new_password)?;

    // Find token
    let token_model = password_reset_token::Entity::find()
        .filter(password_reset_token::Column::Token.eq(&payload.token))
        .one(state.sea_orm())
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .ok_or_else(|| AppError::Other("Invalid reset token".to_string()))?;

    // Check if used
    if token_model.used != 0 {
        return Err(AppError::Other(
            "Reset token has already been used".to_string(),
        ));
    }

    // Check expiry
    if token_model.expires_at < Utc::now() {
        return Err(AppError::Other("Reset token has expired".to_string()));
    }

    // Hash password
    let password_hash = hash(&payload.new_password, DEFAULT_COST)?;

    // Find user
    let user_model = user::Entity::find_by_id(&token_model.user_id)
        .one(state.sea_orm())
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .ok_or(AppError::UserNotFound)?;

    let user_email = user_model.email.clone().unwrap_or_default();
    let user_name = user_model
        .name
        .clone()
        .unwrap_or_else(|| "User".to_string());
    let user_id = user_model.id.clone();

    // Update password
    let mut user_active: user::ActiveModel = user_model.into();
    user_active.password = Set(Some(password_hash));
    user_active.refresh_token = Set(None); // Clear refresh token
    user_active
        .update(state.sea_orm())
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    // Mark token as used
    let mut token_active: password_reset_token::ActiveModel = token_model.into();
    token_active.used = Set(1);
    token_active
        .update(state.sea_orm())
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    tracing::info!("Password reset completed for user {}", user_id);

    // Send notification email
    if !user_email.is_empty() {
        let email_service = EmailService::new();
        if let Err(e) = email_service
            .send_password_changed_email(&user_email, &user_name)
            .await
        {
            tracing::warn!("Failed to send password changed notification: {}", e);
        }
    }

    Ok(Json(ResetPasswordResponse {
        success: true,
        message: "Password reset successfully. Please login with your new password.".to_string(),
    }))
}

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

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router
}