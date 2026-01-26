//! Handler for the logout endpoint.

use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use chrono::Utc;
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use utoipa::ToSchema;

// SeaORM imports
use crate::entities::user;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};

use crate::routes::AppState;
use crate::utils::auth::decode_jwt;
use crate::utils::error::AppError;

pub const ENDPOINT_METHOD: &str = "post";
pub const ENDPOINT_PATH: &str = "/api/auth/logout";
pub const ENDPOINT_DESCRIPTION: &str = "Logout user and invalidate tokens";
pub const ENDPOINT_TAG: &str = "auth";
pub const OPERATION_ID: &str = "auth_logout";
pub const SUCCESS_RESPONSE_BODY: &str = "Json<LogoutResponse>";

/// Logout request payload
#[derive(Debug, Deserialize, ToSchema)]
pub struct LogoutRequest {
    /// Refresh token to revoke
    pub refresh_token: Option<String>,
    /// Logout from all devices
    #[serde(default)]
    pub logout_all: bool,
}

/// Logout response
#[derive(Debug, Serialize, ToSchema)]
pub struct LogoutResponse {
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
    post,
    path = "/api/auth/logout",
    tag = "auth",
    operation_id = "auth_logout",
    responses(
        (status = 200, description = "Logout user and invalidate tokens", body = LogoutResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn logout(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(payload): Json<LogoutRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Extract and decode JWT token
    let token = extract_token(&headers)?;
    let claims = decode_jwt(&token)?;

    // Calculate token TTL for Redis blacklist
    let now = Utc::now().timestamp() as usize;
    let ttl = if claims.exp > now {
        (claims.exp - now) as u64
    } else {
        0
    };

    // Blacklist the access token in Redis
    if ttl > 0 {
        let mut redis_conn = state.redis_pool.get().await?;
        let blacklist_key = format!("blacklist:token:{}", token);
        redis_conn
            .set_ex::<_, _, ()>(&blacklist_key, "1", ttl)
            .await
            .map_err(|e| AppError::RedisError(e))?;
    }

    // Revoke refresh token if provided (clear refreshToken in User table)
    if let Some(ref refresh_token) = payload.refresh_token {
        let user_model = user::Entity::find()
            .filter(user::Column::RefreshToken.eq(refresh_token))
            .filter(user::Column::Id.eq(&claims.user_id))
            .one(state.sea_orm())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if let Some(user) = user_model {
            let mut user_active: user::ActiveModel = user.into();
            user_active.refresh_token = Set(None);
            user_active
                .update(state.sea_orm())
                .await
                .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        }
    }

    // If logout_all is true, clear refresh token for this user
    if payload.logout_all {
        let user_model = user::Entity::find_by_id(&claims.user_id)
            .one(state.sea_orm())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if let Some(user) = user_model {
            let mut user_active: user::ActiveModel = user.into();
            user_active.refresh_token = Set(None);
            user_active
                .update(state.sea_orm())
                .await
                .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        }
    }

    Ok((
        StatusCode::OK,
        Json(LogoutResponse {
            success: true,
            message: if payload.logout_all {
                "Logged out from all devices successfully".to_string()
            } else {
                "Logged out successfully".to_string()
            },
        }),
    ))
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, post(logout))
}