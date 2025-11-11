//! Handler for forgot password endpoint.
#![allow(dead_code)]

use axum::{extract::State, response::IntoResponse, routing::post, Json, Router};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::Row;
use std::sync::Arc;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::routes::AppState;
use crate::utils::email::EmailService;
use crate::utils::error::AppError;

pub const ENDPOINT_METHOD: &str = "post";
pub const ENDPOINT_PATH: &str = "/api/auth/forgot-password";
pub const ENDPOINT_DESCRIPTION: &str = "Request password reset";
pub const ENDPOINT_TAG: &str = "auth";
pub const OPERATION_ID: &str = "auth_forgot_password";
pub const SUCCESS_RESPONSE_BODY: &str = "Json<ForgotPasswordResponse>";

/// Forgot password request
#[derive(Debug, Deserialize, ToSchema)]
pub struct ForgotPasswordRequest {
    pub email: String,
}

/// Forgot password response
#[derive(Debug, Serialize, ToSchema)]
pub struct ForgotPasswordResponse {
    pub success: bool,
    pub message: String,
    pub reset_token: Option<String>, // Only for development/testing
}

#[utoipa::path(
    post,
    path = "/api/auth/forgot-password",
    tag = "auth",
    operation_id = "auth_forgot_password",
    responses(
        (status = 200, description = "Request password reset", body = ForgotPasswordResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn forgot_password(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ForgotPasswordRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Find user by email
    let user_id: Option<String> = sqlx::query_scalar(
        "SELECT id FROM users WHERE email = ? AND is_active = TRUE"
    )
    .bind(&payload.email)
    .fetch_optional(&state.sqlx_pool)
    .await?;

    // Always return success to prevent user enumeration
    // But only actually send email if user exists
    if let Some(user_id) = user_id {
        // Delete old password reset tokens for this user
        sqlx::query("DELETE FROM password_reset_tokens WHERE user_id = ?")
            .bind(&user_id)
            .execute(&state.sqlx_pool)
            .await?;

        // Generate password reset token
        let reset_token = Uuid::new_v4().to_string();
        let expires_at = Utc::now() + chrono::Duration::hours(1); // 1 hour expiry

        sqlx::query(
            r#"
            INSERT INTO password_reset_tokens (id, user_id, token, expires_at, created_at)
            VALUES (?, ?, ?, ?, ?)
            "#,
        )
        .bind(Uuid::new_v4().to_string())
        .bind(&user_id)
        .bind(&reset_token)
        .bind(expires_at)
        .bind(Utc::now())
        .execute(&state.sqlx_pool)
        .await?;

        // Get user name for email
        let user_name: Option<String> = sqlx::query_scalar(
            "SELECT COALESCE(full_name, username) FROM users WHERE id = ?"
        )
        .bind(&user_id)
        .fetch_optional(&state.sqlx_pool)
        .await?;

        // Send password reset email
        let email_service = EmailService::new();
        let name = user_name.as_deref().unwrap_or("User");
        if let Err(e) = email_service
            .send_password_reset_email(&payload.email, name, &reset_token)
            .await
        {
            tracing::warn!("Failed to send password reset email: {}", e);
        }

        Ok(Json(ForgotPasswordResponse {
            success: true,
            message: "If the email exists, a password reset link has been sent".to_string(),
            reset_token: Some(reset_token), // Remove in production
        }))
    } else {
        // Return same response to prevent user enumeration
        Ok(Json(ForgotPasswordResponse {
            success: true,
            message: "If the email exists, a password reset link has been sent".to_string(),
            reset_token: None,
        }))
    }
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, post(forgot_password))
}