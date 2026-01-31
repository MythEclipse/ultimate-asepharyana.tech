//! Handler for refresh token endpoint.

use axum::{extract::State, response::IntoResponse, Json, Router};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use utoipa::ToSchema;
use uuid::Uuid;

// SeaORM imports
use crate::entities::user;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};

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
    // Verify refresh token from user.refreshToken field
    let user_model = user::Entity::find()
        .filter(user::Column::RefreshToken.eq(&payload.refresh_token))
        .one(state.sea_orm())
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .ok_or(AppError::InvalidToken)?;

    // Generate new access token
    let token_expiry = 24 * 3600; // 24 hours
    let exp = (Utc::now().timestamp() + token_expiry) as usize;

    let claims = Claims {
        user_id: user_model.id.clone(),
        email: user_model.email.clone().unwrap_or_default(),
        name: user_model.name.clone().unwrap_or_default(),
        exp,
    };

    let access_token = encode_jwt(claims)?;

    // Generate new refresh token
    let new_refresh_token = Uuid::new_v4().to_string();

    // Update user's refresh token
    let mut user_active: user::ActiveModel = user_model.into();
    user_active.refresh_token = Set(Some(new_refresh_token.clone()));
    user_active
        .update(state.sea_orm())
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(Json(RefreshResponse {
        access_token,
        refresh_token: new_refresh_token,
        token_type: "Bearer".to_string(),
        expires_in: token_expiry,
    }))
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router
}