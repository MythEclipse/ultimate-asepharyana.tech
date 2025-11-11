//! Handler for the verify email endpoint.
#![allow(dead_code)]

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use utoipa::ToSchema;
use uuid::Uuid;

// SeaORM imports
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use crate::entities::{user, email_verification_token, prelude::*};

use crate::routes::AppState;
use crate::utils::email::EmailService;
use crate::utils::error::AppError;

pub const ENDPOINT_METHOD: &str = "get";
pub const ENDPOINT_PATH: &str = "/api/auth/verify";
pub const ENDPOINT_DESCRIPTION: &str = "Verify user email address";
pub const ENDPOINT_TAG: &str = "auth";
pub const OPERATION_ID: &str = "auth_verify";
pub const SUCCESS_RESPONSE_BODY: &str = "Json<VerifyResponse>";

/// Verify email query parameters
#[derive(Debug, Deserialize, ToSchema)]
pub struct VerifyQuery {
    /// Email verification token
    pub token: String,
}

/// Verify email response
#[derive(Debug, Serialize, ToSchema)]
pub struct VerifyResponse {
    pub success: bool,
    pub message: String,
}

/// Resend verification email request
#[derive(Debug, Deserialize, ToSchema)]
pub struct ResendVerificationRequest {
    pub email: String,
}

#[utoipa::path(
    get,
    path = "/api/auth/verify",
    tag = "auth",
    operation_id = "auth_verify",
    params(
        ("token" = String, Query, description = "Email verification token")
    ),
    responses(
        (status = 200, description = "Email verified successfully", body = VerifyResponse),
        (status = 400, description = "Bad Request - Invalid or expired token", body = String),
        (status = 404, description = "Token not found", body = String),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn verify(
    State(state): State<Arc<AppState>>,
    Query(query): Query<VerifyQuery>,
) -> Result<impl IntoResponse, AppError> {
    // Find verification token using SeaORM
    let token_model = email_verification_token::Entity::find()
        .filter(email_verification_token::Column::Token.eq(&query.token))
        .one(state.sea_orm())
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .ok_or(AppError::InvalidToken)?;

    // Check if token is expired
    if token_model.expires_at < Utc::now() {
        return Err(AppError::TokenExpired);
    }

    // Find user
    let user_model = user::Entity::find_by_id(&token_model.user_id)
        .one(state.sea_orm())
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .ok_or(AppError::UserNotFound)?;

    // Check if user is already verified
    if user_model.email_verified.is_some() {
        return Ok((
            StatusCode::OK,
            Json(VerifyResponse {
                success: true,
                message: "Email already verified".to_string(),
            }),
        ));
    }

    // Get user info for welcome email before updating
    let user_email = user_model.email.clone().unwrap_or_default();
    let user_name = user_model.name.clone().unwrap_or_else(|| "User".to_string());

    // Update user's email_verified status
    let mut user_active: user::ActiveModel = user_model.into();
    user_active.email_verified = Set(Some(Utc::now()));
    user_active.update(state.sea_orm())
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    // Delete used verification token
    email_verification_token::Entity::delete_by_id(&token_model.id)
        .exec(state.sea_orm())
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    // Send welcome email
    let email_service = EmailService::new();
    if !user_email.is_empty() {
        if let Err(e) = email_service.send_welcome_email(&user_email, &user_name).await {
            tracing::warn!("Failed to send welcome email: {}", e);
        }
    }

    Ok((
        StatusCode::OK,
        Json(VerifyResponse {
            success: true,
            message: "Email verified successfully".to_string(),
        }),
    ))
}

/// Resend verification email
#[utoipa::path(
    post,
    path = "/api/auth/verify/resend",
    tag = "auth",
    operation_id = "auth_verify_resend",
    request_body = ResendVerificationRequest,
    responses(
        (status = 200, description = "Verification email sent", body = VerifyResponse),
        (status = 400, description = "Email already verified", body = String),
        (status = 404, description = "User not found", body = String),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn resend_verification(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ResendVerificationRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Find user by email using SeaORM
    let user_model = user::Entity::find()
        .filter(user::Column::Email.eq(&payload.email))
        .one(state.sea_orm())
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .ok_or(AppError::UserNotFound)?;

    // Check if already verified
    if user_model.email_verified.is_some() {
        return Err(AppError::Other("Email already verified".to_string()));
    }

    // Delete old verification tokens for this user using SeaORM
    email_verification_token::Entity::delete_many()
        .filter(email_verification_token::Column::UserId.eq(&user_model.id))
        .exec(state.sea_orm())
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    // Generate new verification token
    let verification_token = Uuid::new_v4().to_string();
    let expires_at = Utc::now() + chrono::Duration::hours(24);

    // Insert new verification token using SeaORM
    let new_token = email_verification_token::ActiveModel {
        id: Set(Uuid::new_v4().to_string()),
        user_id: Set(user_model.id.clone()),
        token: Set(verification_token.clone()),
        expires_at: Set(expires_at),
        created_at: Set(Utc::now()),
    };
    new_token.insert(state.sea_orm())
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    // Get user name (use name field, fallback to "User")
    let user_name = user_model.name.as_deref().unwrap_or("User");

    // Send verification email
    let email_service = EmailService::new();
    if let Err(e) = email_service
        .send_verification_email(&payload.email, user_name, &verification_token)
        .await
    {
        tracing::warn!("Failed to send verification email: {}", e);
    }

    Ok((
        StatusCode::OK,
        Json(VerifyResponse {
            success: true,
            message: format!("Verification email sent to {}", payload.email),
        }),
    ))
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router
        .route(ENDPOINT_PATH, get(verify))
        .route("/api/auth/verify/resend", post(resend_verification))
}

