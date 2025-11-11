//! Handler for the login endpoint.
#![allow(dead_code)]

use axum::{extract::State, response::IntoResponse, routing::post, Json, Router};
use bcrypt::verify;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use utoipa::ToSchema;
use uuid::Uuid;

// SeaORM imports
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use crate::entities::{user, prelude::*};

use crate::models::user::{LoginResponse, User as LegacyUser, UserResponse};
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
    // Find user by email using SeaORM (name field is used as username in this schema)
    let user_model: Option<user::Model> = user::Entity::find()
        .filter(
            user::Column::Email
                .eq(&payload.login)
                .or(user::Column::Name.eq(&payload.login))
        )
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
    if let Some(email_verified) = user_model.email_verified {
        // email_verified is a timestamp, if Some then it's verified
        // If you want to enforce verification, uncomment:
        // if email_verified.is_none() {
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
    let refresh_expires_at = Utc::now() + chrono::Duration::days(30);

    // Store refresh token in database (using SQLx temporarily)
    sqlx::query(
        r#"
        INSERT INTO refresh_tokens (id, user_id, token, expires_at, created_at)
        VALUES (?, ?, ?, ?, ?)
        "#,
    )
    .bind(Uuid::new_v4().to_string())
    .bind(&user_model.id)
    .bind(&refresh_token)
    .bind(refresh_expires_at)
    .bind(Utc::now())
    .execute(&state.sqlx_pool)
    .await?;

    // Update last login timestamp (TODO: migrate to SeaORM when last_login_at added to schema)
    // For now, skip since last_login_at doesn't exist in current schema
    // sqlx::query("UPDATE users SET last_login_at = ? WHERE id = ?")
    //     .bind(Utc::now())
    //     .bind(&user_model.id)
    //     .execute(&state.sqlx_pool)
    //     .await?;

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
    .execute(&state.sqlx_pool)
    .await?;

    Ok(())
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, post(login))
}