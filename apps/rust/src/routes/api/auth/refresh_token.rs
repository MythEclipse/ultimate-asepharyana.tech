//! Handler for refresh token endpoint.
#![allow(dead_code)]

use axum::{extract::State, response::IntoResponse, routing::post, Json, Router};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::Row;
use std::sync::Arc;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::routes::AppState;
use crate::utils::auth::{encode_jwt, Claims};
use crate::utils::error::AppError;

pub const ENDPOINT_METHOD: &str = "post";
pub const ENDPOINT_PATH: &str = "/api/auth/refresh";
pub const ENDPOINT_DESCRIPTION: &str = "Refresh JWT access token";
pub const ENDPOINT_TAG: &str = "auth";
pub const OPERATION_ID: &str = "auth_refresh";
pub const SUCCESS_RESPONSE_BODY: &str = "Json<RefreshResponse>";

/// Refresh token request
#[derive(Debug, Deserialize, ToSchema)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

/// Refresh token response
#[derive(Debug, Serialize, ToSchema)]
pub struct RefreshResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: i64,
}

#[utoipa::path(
    post,
    path = "/api/auth/refresh",
    tag = "auth",
    operation_id = "auth_refresh",
    responses(
        (status = 200, description = "Refresh JWT access token", body = RefreshResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn refresh(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RefreshRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Verify refresh token exists and is valid
    let token_data: Option<(String, chrono::DateTime<Utc>, bool)> = sqlx::query_as(
        r#"
        SELECT user_id, expires_at, revoked
        FROM refresh_tokens
        WHERE token = ?
        "#,
    )
    .bind(&payload.refresh_token)
    .fetch_optional(&state.db)
    .await?;

    let (user_id, expires_at, revoked) = token_data.ok_or(AppError::InvalidToken)?;

    // Check if token is revoked
    if revoked {
        return Err(AppError::InvalidToken);
    }

    // Check if token is expired
    if expires_at < Utc::now() {
        return Err(AppError::TokenExpired);
    }

    // Get user details
    let user_data: Option<(String, String)> = sqlx::query_as(
        "SELECT email, username FROM users WHERE id = ? AND is_active = TRUE"
    )
    .bind(&user_id)
    .fetch_optional(&state.db)
    .await?;

    let (email, username) = user_data.ok_or(AppError::UserNotFound)?;

    // Generate new access token
    let token_expiry = 24 * 3600; // 24 hours
    let exp = (Utc::now().timestamp() + token_expiry) as usize;

    let claims = Claims {
        user_id: user_id.clone(),
        email,
        name: username,
        exp,
    };

    let access_token = encode_jwt(claims)?;

    // Generate new refresh token
    let new_refresh_token = Uuid::new_v4().to_string();
    let refresh_expires_at = Utc::now() + chrono::Duration::days(30);

    // Revoke old refresh token
    sqlx::query(
        "UPDATE refresh_tokens SET revoked = TRUE WHERE token = ?"
    )
    .bind(&payload.refresh_token)
    .execute(&state.db)
    .await?;

    // Store new refresh token
    sqlx::query(
        r#"
        INSERT INTO refresh_tokens (id, user_id, token, expires_at, created_at)
        VALUES (?, ?, ?, ?, ?)
        "#,
    )
    .bind(Uuid::new_v4().to_string())
    .bind(&user_id)
    .bind(&new_refresh_token)
    .bind(refresh_expires_at)
    .bind(Utc::now())
    .execute(&state.db)
    .await?;

    Ok(Json(RefreshResponse {
        access_token,
        refresh_token: new_refresh_token,
        token_type: "Bearer".to_string(),
        expires_in: token_expiry,
    }))
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, post(refresh))
}