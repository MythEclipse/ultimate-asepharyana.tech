//! Storage driver trait definition.

use super::FileMetadata;
use async_trait::async_trait;

/// Errors that can occur during storage operations.
#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("File not found: {0}")]
    NotFound(String),
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    #[error("IO error: {0}")]
    IoError(String),
    #[error("Invalid path: {0}")]
    InvalidPath(String),
    #[error("Storage error: {0}")]
    Other(String),
}

impl From<std::io::Error> for StorageError {
    fn from(e: std::io::Error) -> Self {
        match e.kind() {
            std::io::ErrorKind::NotFound => StorageError::NotFound(e.to_string()),
            std::io::ErrorKind::PermissionDenied => StorageError::PermissionDenied(e.to_string()),
            _ => StorageError::IoError(e.to_string()),
        }
    }
}

/// Storage driver trait for different storage backends.
#[async_trait]
pub trait StorageDriver: Send + Sync {
    /// Store a file.
    async fn put(&self, path: &str, content: &[u8]) -> Result<(), StorageError>;

    /// Store a file with MIME type hint.
    async fn put_with_mime(
        &self,
        path: &str,
        content: &[u8],
        _mime_type: &str,
    ) -> Result<(), StorageError> {
        // Default implementation ignores MIME type
        self.put(path, content).await
    }

    /// Get a file's content.
    async fn get(&self, path: &str) -> Result<Vec<u8>, StorageError>;

    /// Check if a file exists.
    async fn exists(&self, path: &str) -> Result<bool, StorageError>;

    /// Delete a file.
    async fn delete(&self, path: &str) -> Result<(), StorageError>;

    /// Get the public URL for a file (if supported).
    async fn url(&self, path: &str) -> Result<String, StorageError>;

    /// Get file metadata.
    async fn metadata(&self, path: &str) -> Result<FileMetadata, StorageError>;

    /// List files in a directory.
    async fn list(&self, directory: &str) -> Result<Vec<String>, StorageError>;

    /// Copy a file.
    async fn copy(&self, from: &str, to: &str) -> Result<(), StorageError> {
        let content = self.get(from).await?;
        self.put(to, &content).await
    }

    /// Move/rename a file.
    async fn rename(&self, from: &str, to: &str) -> Result<(), StorageError> {
        self.copy(from, to).await?;
        self.delete(from).await
    }
}
