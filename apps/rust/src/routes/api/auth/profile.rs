//! Handler for update profile endpoint.

use axum::{
    extract::State,
    http::HeaderMap,
    response::IntoResponse,
    routing::put,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use utoipa::ToSchema;
use validator::Validate;

// SeaORM imports
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use crate::entities::{user};

use crate::models::user::UserResponse;
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
    pub name: Option<String>, // Changed from full_name to name
    pub image: Option<String>, // Changed from avatar_url to image

    #[validate(email)]
    pub email: Option<String>,
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

    // Check if email is being changed and if it's already taken
    if let Some(ref new_email) = payload.email {
        let email_exists = user::Entity::find()
            .filter(user::Column::Email.eq(new_email))
            .filter(user::Column::Id.ne(&claims.user_id))
            .one(state.sea_orm())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if email_exists.is_some() {
            return Err(AppError::EmailAlreadyExists);
        }
    }

    // Get current user
    let user_model = user::Entity::find_by_id(&claims.user_id)
        .one(state.sea_orm())
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .ok_or(AppError::UserNotFound)?;

    // Update user fields
    let mut user_active: user::ActiveModel = user_model.into();
    let mut has_updates = false;

    if let Some(name) = payload.name {
        user_active.name = Set(Some(name));
        has_updates = true;
    }

    if let Some(image) = payload.image {
        user_active.image = Set(Some(image));
        has_updates = true;
    }

    if let Some(email) = payload.email {
        user_active.email = Set(Some(email));
        user_active.email_verified = Set(None); // Reset verification when email changes
        has_updates = true;
    }

    if !has_updates {
        return Err(AppError::Other("No fields to update".to_string()));
    }

    // Save changes
    let updated_user = user_active.update(state.sea_orm())
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(Json(UpdateProfileResponse {
        success: true,
        message: "Profile updated successfully".to_string(),
        user: updated_user.into(),
    }))
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, put(update_profile))
}