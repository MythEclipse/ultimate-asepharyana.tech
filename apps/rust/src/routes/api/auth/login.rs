//! Handler for the login endpoint.

use axum::{extract::State, response::IntoResponse, routing::post, Json, Router};
use bcrypt::verify;
use chrono::Utc;
use serde::Deserialize;
use std::sync::Arc;
use utoipa::ToSchema;
use uuid::Uuid;

// SeaORM imports
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use crate::entities::{user};

use crate::models::user::{LoginResponse, UserResponse};
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
    /// User email address
    pub email: String,
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
    // Find user by email using SeaORM
    let user_model: Option<user::Model> = user::Entity::find()
        .filter(user::Column::Email.eq(&payload.email))
        .one(state.sea_orm())
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    let user_model = user_model.ok_or(AppError::InvalidCredentials)?;

    // Verify password
    let password_valid = verify(
        &payload.password,
        user_model.password.as_ref().ok_or(AppError::InvalidCredentials)?
    )?;

    if !password_valid {
        // Log failed login attempt (still using SQLx temporarily)
        log_login_attempt(&state, &user_model.id, false, Some("Invalid password")).await?;
        return Err(AppError::InvalidCredentials);
    }

    // Check if account is active
    // Note: is_active field doesn't exist in current schema, skip for now
    // if !user_model.is_active {
    //     return Err(AppError::AccountInactive);
    // }

    // Check if email is verified (optional - you can skip this check)
    if user_model.email_verified.is_some() {
        // email_verified is a timestamp, if Some then it's verified
        // If you want to enforce verification, uncomment:
        // if user_model.email_verified.is_none() {
        //     return Err(AppError::EmailNotVerified);
        // }
    }

    // Generate JWT tokens
    let token_expiry = if payload.remember_me { 30 * 24 * 3600 } else { 24 * 3600 }; // 30 days or 24 hours
    let exp = (Utc::now().timestamp() + token_expiry) as usize;

    let claims = Claims {
        user_id: user_model.id.clone(),
        email: user_model.email.clone().unwrap_or_else(|| "".to_string()),
        name: user_model.name.clone().unwrap_or_else(|| "".to_string()),
        exp,
    };

    let access_token = encode_jwt(claims)?;

    // Generate refresh token
    let refresh_token = Uuid::new_v4().to_string();

    // Store refresh token in User table (not separate refresh_tokens table)
    let mut user_active: user::ActiveModel = user_model.clone().into();
    user_active.refresh_token = Set(Some(refresh_token.clone()));
    user_active.update(state.sea_orm())
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    // Log successful login
    log_login_attempt(&state, &user_model.id, true, None).await?;

    // Convert SeaORM model to response format using From trait
    let user_response: UserResponse = user_model.into();

    Ok(Json(LoginResponse {
        user: user_response,
        access_token,
        refresh_token,
        token_type: "Bearer".to_string(),
        expires_in: token_expiry,
    }))
}

/// Log login attempt for security tracking (optional, no-op if table doesn't exist)
async fn log_login_attempt(
    _: &AppState,
    user_id: &str,
    success: bool,
    failure_reason: Option<&str>,
) -> Result<(), AppError> {
    // TODO: Implement login history tracking if needed
    // For now, just log to console
    tracing::info!(
        "Login attempt - user_id: {}, success: {}, reason: {:?}",
        user_id,
        success,
        failure_reason
    );
    Ok(())
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, post(login))
}