//! Profile storage utilities for MinIO/S3 image upload.
//!
//! This module provides helper functions for uploading, deleting, and
//! managing profile images in MinIO storage.

use crate::core::config::MINIO_CONFIG;
use crate::storage::{S3Config, S3Driver, Storage, StorageError};
use once_cell::sync::Lazy;
use std::sync::Arc;

/// Global profile storage instance
static PROFILE_STORAGE: Lazy<Option<Arc<Storage>>> = Lazy::new(|| {
    let config = MINIO_CONFIG.as_ref()?;

    let s3_config = S3Config {
        bucket: config.bucket_name.clone(),
        region: config.region.clone(),
        endpoint: Some(config.endpoint.clone()),
        access_key: config.access_key.clone(),
        secret_key: config.secret_key.clone(),
        path_style: true, // MinIO uses path-style URLs
        public_url: config.public_url.clone(),
    };

    let driver = S3Driver::new(s3_config);
    Some(Arc::new(Storage::new(driver)))
});

/// Get the profile storage instance.
/// Returns None if MinIO is not configured.
pub fn get_storage() -> Option<Arc<Storage>> {
    PROFILE_STORAGE.clone()
}

/// Allowed image MIME types for profile pictures
const ALLOWED_MIME_TYPES: &[&str] = &["image/jpeg", "image/png", "image/gif", "image/webp"];

/// Maximum file size for profile images (5MB)
pub const MAX_FILE_SIZE: usize = 5 * 1024 * 1024;

/// Validate that the content is an allowed image type.
/// Returns the detected MIME type if valid.
pub fn validate_image(content: &[u8]) -> Result<String, ProfileStorageError> {
    if content.len() > MAX_FILE_SIZE {
        return Err(ProfileStorageError::FileTooLarge(
            content.len(),
            MAX_FILE_SIZE,
        ));
    }

    let kind = infer::get(content).ok_or(ProfileStorageError::UnknownFileType)?;

    let mime_type = kind.mime_type();

    if !ALLOWED_MIME_TYPES.contains(&mime_type) {
        return Err(ProfileStorageError::InvalidFileType(mime_type.to_string()));
    }

    Ok(mime_type.to_string())
}

/// Get the file extension for a MIME type
pub fn mime_to_extension(mime_type: &str) -> &'static str {
    match mime_type {
        "image/jpeg" => "jpg",
        "image/png" => "png",
        "image/gif" => "gif",
        "image/webp" => "webp",
        _ => "bin",
    }
}

/// Generate a unique path for a profile image.
/// Format: {avatar_prefix}/{user_id}/{uuid}.{ext}
pub fn generate_image_path(user_id: &str, extension: &str) -> String {
    let config = MINIO_CONFIG.as_ref();
    let prefix = config
        .map(|c| c.avatar_prefix.as_str())
        .unwrap_or("avatars");

    let uuid = uuid::Uuid::new_v4();
    format!("{}/{}/{}.{}", prefix, user_id, uuid, extension)
}

/// Upload a profile image to MinIO storage.
/// Returns the public URL of the uploaded image.
pub async fn upload_profile_image(
    user_id: &str,
    content: &[u8],
) -> Result<String, ProfileStorageError> {
    // Validate image
    let mime_type = validate_image(content)?;
    let extension = mime_to_extension(&mime_type);

    // Get storage
    let storage = get_storage().ok_or(ProfileStorageError::StorageNotConfigured)?;

    // Generate path
    let path = generate_image_path(user_id, extension);

    // Upload
    storage
        .put(&path, content)
        .await
        .map_err(ProfileStorageError::Storage)?;

    // Get public URL
    let url = storage
        .url(&path)
        .await
        .map_err(ProfileStorageError::Storage)?;

    tracing::info!(
        user_id = %user_id,
        path = %path,
        size = content.len(),
        mime = %mime_type,
        "Profile image uploaded successfully"
    );

    Ok(url)
}

/// Delete a profile image from storage.
/// Extracts the path from a full URL and deletes it.
pub async fn delete_profile_image(image_url: &str) -> Result<(), ProfileStorageError> {
    let storage = get_storage().ok_or(ProfileStorageError::StorageNotConfigured)?;

    // Extract path from URL
    let config = MINIO_CONFIG
        .as_ref()
        .ok_or(ProfileStorageError::StorageNotConfigured)?;

    // Try to extract the path after the avatar prefix
    let path = if let Some(pos) = image_url.find(&config.avatar_prefix) {
        &image_url[pos..]
    } else {
        tracing::warn!(
            url = %image_url,
            "Could not extract path from image URL, skipping deletion"
        );
        return Ok(());
    };

    // Delete the file
    match storage.delete(path).await {
        Ok(()) => {
            tracing::info!(path = %path, "Profile image deleted successfully");
            Ok(())
        }
        Err(StorageError::NotFound(_)) => {
            tracing::warn!(path = %path, "Profile image not found, skipping deletion");
            Ok(())
        }
        Err(e) => Err(ProfileStorageError::Storage(e)),
    }
}

/// Errors that can occur during profile storage operations.
#[derive(Debug, thiserror::Error)]
pub enum ProfileStorageError {
    #[error("MinIO storage is not configured")]
    StorageNotConfigured,

    #[error("File too large: {0} bytes (max: {1} bytes)")]
    FileTooLarge(usize, usize),

    #[error("Invalid file type: {0}. Allowed: JPEG, PNG, GIF, WebP")]
    InvalidFileType(String),

    #[error("Unknown file type")]
    UnknownFileType,

    #[error("Storage error: {0}")]
    Storage(#[from] StorageError),
}
