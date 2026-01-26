//! Handler for forgot password endpoint - Enhanced with form_request validation.

use axum::{extract::State, response::IntoResponse, routing::post, Json, Router};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use utoipa::ToSchema;
use uuid::Uuid;

// SeaORM imports
use crate::entities::{password_reset_token, user};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};

use crate::routes::AppState;
use crate::utils::email::EmailService;
use crate::utils::error::AppError;

// New helpers
use crate::helpers::email_template::password_reset_email;
use crate::helpers::form_request::{validate, ValidationRules};

pub const ENDPOINT_METHOD: &str = "post";
pub const ENDPOINT_PATH: &str = "/api/auth/forgot-password";
pub const ENDPOINT_DESCRIPTION: &str = "Request password reset";
pub const ENDPOINT_TAG: &str = "auth";
pub const OPERATION_ID: &str = "auth_forgot_password";
pub const SUCCESS_RESPONSE_BODY: &str = "Json<ForgotPasswordResponse>";

/// Forgot password request
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ForgotPasswordRequest {
    pub email: String,
}

/// Forgot password response
#[derive(Debug, Serialize, ToSchema)]
pub struct ForgotPasswordResponse {
    pub success: bool,
    pub message: String,
    pub reset_token: Option<String>,
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
    // Validate with form_request helper
    let data = serde_json::to_value(&payload).unwrap_or_default();
    let mut rules = ValidationRules::new();
    rules.required("email").email("email");

    let validation = validate(&data, &rules);
    if !validation.is_valid() {
        return Err(AppError::Other("Invalid email format".to_string()));
    }

    // Find user by email
    let user_model = user::Entity::find()
        .filter(user::Column::Email.eq(&payload.email))
        .one(state.sea_orm())
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    // Always return success to prevent user enumeration
    if let Some(user_model) = user_model {
        // Delete old tokens
        password_reset_token::Entity::delete_many()
            .filter(password_reset_token::Column::UserId.eq(&user_model.id))
            .exec(state.sea_orm())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Generate token
        let reset_token = Uuid::new_v4().to_string();
        let expires_at = Utc::now() + chrono::Duration::hours(1);

        let new_reset_token = password_reset_token::ActiveModel {
            id: Set(Uuid::new_v4().to_string()),
            user_id: Set(user_model.id.clone()),
            token: Set(reset_token.clone()),
            expires_at: Set(expires_at),
            created_at: Set(Utc::now()),
            used: Set(0),
        };

        new_reset_token
            .insert(state.sea_orm())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        tracing::info!("Password reset requested for user {}", user_model.id);

        // Generate email using email_template helper
        let name = user_model.name.as_deref().unwrap_or("User");
        let reset_url = format!(
            "https://asepharyana.cloud/reset-password?token={}",
            reset_token
        );
        let _email = password_reset_email(name, &reset_url);

        // Send email
        let email_service = EmailService::new();
        let email = user_model.email.as_deref().unwrap_or(&payload.email);

        if let Err(e) = email_service
            .send_password_reset_email(email, name, &reset_token)
            .await
        {
            tracing::warn!("Failed to send password reset email: {}", e);
        }

        Ok(Json(ForgotPasswordResponse {
            success: true,
            message: "If the email exists, a password reset link has been sent".to_string(),
            reset_token: Some(reset_token),
        }))
    } else {
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