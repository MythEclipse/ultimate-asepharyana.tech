//! Handler for delete account endpoint.
#![allow(dead_code)]

use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::delete,
    Json, Router,
};
use bcrypt::verify;
use chrono::Utc;
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use utoipa::ToSchema;

use crate::routes::AppState;
use crate::utils::auth::decode_jwt;
use crate::utils::error::AppError;

pub const ENDPOINT_METHOD: &str = "delete";
pub const ENDPOINT_PATH: &str = "/api/auth/account";
pub const ENDPOINT_DESCRIPTION: &str = "Delete user account";
pub const ENDPOINT_TAG: &str = "auth";
pub const OPERATION_ID: &str = "auth_delete_account";
pub const SUCCESS_RESPONSE_BODY: &str = "Json<DeleteAccountResponse>";

/// Delete account request
#[derive(Debug, Deserialize, ToSchema)]
pub struct DeleteAccountRequest {
    pub password: String,
    pub confirmation: String, // Must be "DELETE" or "CONFIRM"
}

/// Delete account response
#[derive(Debug, Serialize, ToSchema)]
pub struct DeleteAccountResponse {
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

#[utoipa::path(
    delete,
    path = "/api/auth/account",
    tag = "auth",
    operation_id = "auth_delete_account",
    responses(
        (status = 200, description = "Delete user account", body = DeleteAccountResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn delete_account(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(payload): Json<DeleteAccountRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Extract and decode JWT token
    let token = extract_token(&headers)?;
    let claims = decode_jwt(&token)?;

    // Verify confirmation text
    if payload.confirmation != "DELETE" && payload.confirmation != "CONFIRM" {
        return Err(AppError::Other(
            "Invalid confirmation. Please type 'DELETE' or 'CONFIRM'".to_string(),
        ));
    }

    // Get current password hash
    let current_password_hash: String = sqlx::query_scalar(
        "SELECT password_hash FROM users WHERE id = ?"
    )
    .bind(&claims.user_id)
    .fetch_one(&state.db)
    .await?;

    // Verify password
    let password_valid = verify(&payload.password, &current_password_hash)?;
    if !password_valid {
        return Err(AppError::InvalidCredentials);
    }

    // Delete user and all related data (cascade delete)
    // The foreign keys should handle cascading, but we can be explicit

    // Delete email verification tokens
    sqlx::query("DELETE FROM email_verification_tokens WHERE user_id = ?")
        .bind(&claims.user_id)
        .execute(&state.db)
        .await?;

    // Delete password reset tokens
    sqlx::query("DELETE FROM password_reset_tokens WHERE user_id = ?")
        .bind(&claims.user_id)
        .execute(&state.db)
        .await?;

    // Delete refresh tokens
    sqlx::query("DELETE FROM refresh_tokens WHERE user_id = ?")
        .bind(&claims.user_id)
        .execute(&state.db)
        .await?;

    // Delete login history
    sqlx::query("DELETE FROM login_history WHERE user_id = ?")
        .bind(&claims.user_id)
        .execute(&state.db)
        .await?;

    // Finally, delete the user
    sqlx::query("DELETE FROM users WHERE id = ?")
        .bind(&claims.user_id)
        .execute(&state.db)
        .await?;

    // Blacklist the current token
    let now = Utc::now().timestamp() as usize;
    let ttl = if claims.exp > now {
        (claims.exp - now) as u64
    } else {
        0
    };

    if ttl > 0 {
        let mut redis_conn = state.redis_pool.get().await?;
        let blacklist_key = format!("blacklist:token:{}", token);
        redis_conn
            .set_ex::<_, _, ()>(&blacklist_key, "1", ttl)
            .await
            .map_err(|e| AppError::RedisError(e))?;
    }

    Ok((
        StatusCode::OK,
        Json(DeleteAccountResponse {
            success: true,
            message: "Account deleted successfully".to_string(),
        }),
    ))
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, delete(delete_account))
}