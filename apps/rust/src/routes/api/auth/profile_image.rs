//! Profile image upload endpoint.
//!
//! Provides multipart file upload for user profile images.
//! Images are stored in MinIO and the URL is saved to the user's profile.

use axum::{
    extract::{Multipart, State},
    http::HeaderMap,
    response::IntoResponse,
    Json, Router,
};
use serde::Serialize;
use std::sync::Arc;
use utoipa::ToSchema;

use crate::entities::user;
use sea_orm::{ActiveModelTrait, EntityTrait, Set};

use crate::models::user::UserResponse;
use crate::routes::AppState;
use crate::utils::auth::decode_jwt;
use crate::utils::error::AppError;

use crate::helpers::profile_storage::{
    self, delete_profile_image, upload_profile_image, ProfileStorageError, MAX_FILE_SIZE,
};

pub const ENDPOINT_METHOD: &str = "post";
pub const ENDPOINT_PATH: &str = "/api/auth/profile/image";
pub const ENDPOINT_DESCRIPTION: &str = "Upload profile image";
pub const ENDPOINT_TAG: &str = "auth";
pub const OPERATION_ID: &str = "auth_upload_profile_image";

/// Profile image upload response
#[derive(Debug, Serialize, ToSchema)]
pub struct UploadProfileImageResponse {
    pub success: bool,
    pub message: String,
    pub image_url: String,
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

/// Convert ProfileStorageError to AppError
impl From<ProfileStorageError> for AppError {
    fn from(err: ProfileStorageError) -> Self {
        match err {
            ProfileStorageError::StorageNotConfigured => {
                AppError::Other("Image storage is not configured".to_string())
            }
            ProfileStorageError::FileTooLarge(size, max) => AppError::Other(format!(
                "File too large: {} bytes (max: {} bytes)",
                size, max
            )),
            ProfileStorageError::InvalidFileType(mime) => AppError::Other(format!(
                "Invalid file type: {}. Allowed: JPEG, PNG, GIF, WebP",
                mime
            )),
            ProfileStorageError::UnknownFileType => AppError::Other(
                "Unknown file type. Please upload JPEG, PNG, GIF, or WebP".to_string(),
            ),
            ProfileStorageError::Storage(e) => AppError::Other(format!("Storage error: {}", e)),
        }
    }
}

#[utoipa::path(
    post,
    path = "/api/auth/profile/image",
    tag = "auth",
    operation_id = "auth_upload_profile_image",
    responses(
        (status = 200, description = "Upload profile image"),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn upload_image(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, AppError> {
    // Extract and decode JWT token
    let token = extract_token(&headers)?;
    let claims = decode_jwt(&token)?;
    let user_id = &claims.user_id;

    // Check if storage is configured
    if profile_storage::get_storage().is_none() {
        return Err(AppError::Other(
            "Image storage is not configured".to_string(),
        ));
    }

    // Process multipart form
    let mut image_data: Option<Vec<u8>> = None;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| AppError::Other(format!("Failed to read multipart: {}", e)))?
    {
        let name = field.name().unwrap_or("").to_string();

        if name == "image" || name == "file" {
            // Read the file data
            let data = field
                .bytes()
                .await
                .map_err(|e| AppError::Other(format!("Failed to read file: {}", e)))?;

            if data.len() > MAX_FILE_SIZE {
                return Err(AppError::Other(format!(
                    "File too large: {} bytes (max: {} bytes)",
                    data.len(),
                    MAX_FILE_SIZE
                )));
            }

            image_data = Some(data.to_vec());
            break;
        }
    }

    let image_bytes = image_data.ok_or_else(|| {
        AppError::Other("No image file provided. Use field name 'image' or 'file'".to_string())
    })?;

    // Get current user to check for existing image
    let user_model = user::Entity::find_by_id(user_id)
        .one(state.sea_orm())
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .ok_or(AppError::UserNotFound)?;

    // Delete old image if exists
    if let Some(ref old_image_url) = user_model.image {
        if !old_image_url.is_empty() {
            if let Err(e) = delete_profile_image(old_image_url).await {
                tracing::warn!(
                    user_id = %user_id,
                    old_url = %old_image_url,
                    error = %e,
                    "Failed to delete old profile image"
                );
            }
        }
    }

    // Upload new image
    let image_url = upload_profile_image(user_id, &image_bytes).await?;

    // Update user profile with new image URL
    let mut user_active: user::ActiveModel = user_model.into();
    user_active.image = Set(Some(image_url.clone()));

    let updated_user = user_active
        .update(state.sea_orm())
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    tracing::info!(
        user_id = %user_id,
        image_url = %image_url,
        "Profile image updated successfully"
    );

    Ok(Json(UploadProfileImageResponse {
        success: true,
        message: "Profile image uploaded successfully".to_string(),
        image_url,
        user: updated_user.into(),
    }))
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router
}