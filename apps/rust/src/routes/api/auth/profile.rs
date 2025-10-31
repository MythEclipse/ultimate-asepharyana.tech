//! Handler for update profile endpoint.
#![allow(dead_code)]

use axum::{
    extract::State,
    http::HeaderMap,
    response::IntoResponse,
    routing::put,
    Json, Router,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use utoipa::ToSchema;
use validator::Validate;

use crate::models::user::{User, UserResponse};
use crate::routes::AppState;
use crate::utils::auth::decode_jwt;
use crate::utils::error::AppError;

pub const ENDPOINT_METHOD: &str = "put";
pub const ENDPOINT_PATH: &str = "/api/auth/profile";
pub const ENDPOINT_DESCRIPTION: &str = "Update user profile";
pub const ENDPOINT_TAG: &str = "auth";
pub const OPERATION_ID: &str = "auth_update_profile";
pub const SUCCESS_RESPONSE_BODY: &str = "Json<UpdateProfileResponse>";

/// Update profile request
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateProfileRequest {
    pub full_name: Option<String>,
    pub avatar_url: Option<String>,

    #[validate(length(min = 3, max = 50, message = "Username must be between 3 and 50 characters"))]
    pub username: Option<String>,
}

/// Update profile response
#[derive(Debug, Serialize, ToSchema)]
pub struct UpdateProfileResponse {
    pub success: bool,
    pub message: String,
    pub user: UserResponse,
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
    put,
    path = "/api/auth/profile",
    tag = "auth",
    operation_id = "auth_update_profile",
    responses(
        (status = 200, description = "Update user profile", body = UpdateProfileResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn update_profile(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(payload): Json<UpdateProfileRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Extract and decode JWT token
    let token = extract_token(&headers)?;
    let claims = decode_jwt(&token)?;

    // Validate input
    payload
        .validate()
        .map_err(|e| AppError::Other(format!("Validation error: {}", e)))?;

    // Check if username is being changed and if it's already taken
    if let Some(ref new_username) = payload.username {
        let username_exists: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM users WHERE username = ? AND id != ?)"
        )
        .bind(new_username)
        .bind(&claims.user_id)
        .fetch_one(&state.db)
        .await?;

        if username_exists {
            return Err(AppError::UsernameAlreadyExists);
        }
    }

    // Build update query dynamically
    let mut updates = Vec::new();
    let mut values: Vec<String> = Vec::new();

    if let Some(ref full_name) = payload.full_name {
        updates.push("full_name = ?");
        values.push(full_name.clone());
    }

    if let Some(ref avatar_url) = payload.avatar_url {
        updates.push("avatar_url = ?");
        values.push(avatar_url.clone());
    }

    if let Some(ref username) = payload.username {
        updates.push("username = ?");
        values.push(username.clone());
    }

    if updates.is_empty() {
        return Err(AppError::Other("No fields to update".to_string()));
    }

    updates.push("updated_at = ?");
    let now = Utc::now();

    // Execute update
    let query = format!(
        "UPDATE users SET {} WHERE id = ?",
        updates.join(", ")
    );

    let mut query_builder = sqlx::query(&query);
    for value in values {
        query_builder = query_builder.bind(value);
    }
    query_builder = query_builder.bind(now).bind(&claims.user_id);

    query_builder.execute(&state.db).await?;

    // Fetch updated user
    let user: User = sqlx::query_as(
        r#"
        SELECT id, email, username, password_hash, full_name, avatar_url,
               email_verified, is_active, role, last_login_at, created_at, updated_at
        FROM users WHERE id = ?
        "#,
    )
    .bind(&claims.user_id)
    .fetch_one(&state.db)
    .await?;

    Ok(Json(UpdateProfileResponse {
        success: true,
        message: "Profile updated successfully".to_string(),
        user: user.into(),
    }))
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, put(update_profile))
}