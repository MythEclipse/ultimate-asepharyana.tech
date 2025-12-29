//! File storage abstraction layer.
//!
//! Provides a unified interface for file storage operations
//! with support for local filesystem and S3-compatible storage.
//!
//! # Example
//!
//! ```ignore
//! use rust::storage::{Storage, LocalDriver};
//!
//! let storage = Storage::local("./uploads");
//!
//! // Store a file
//! storage.put("images/photo.jpg", &bytes).await?;
//!
//! // Get a file
//! let content = storage.get("images/photo.jpg").await?;
//!
//! // Check if file exists
//! let exists = storage.exists("images/photo.jpg").await?;
//!
//! // Delete a file
//! storage.delete("images/photo.jpg").await?;
//! ```

pub mod driver;
pub mod local;
pub mod s3;

pub use driver::{StorageDriver, StorageError};
pub use local::LocalDriver;
pub use s3::{S3Config, S3Driver};

use std::sync::Arc;

/// High-level storage interface.
#[derive(Clone)]
pub struct Storage {
    driver: Arc<dyn StorageDriver>,
}

impl Storage {
    /// Create a new storage instance with the given driver.
    pub fn new<D: StorageDriver + 'static>(driver: D) -> Self {
        Self {
            driver: Arc::new(driver),
        }
    }

    /// Create a local filesystem storage.
    pub fn local(base_path: &str) -> Self {
        Self::new(LocalDriver::new(base_path))
    }

    /// Store a file.
    pub async fn put(&self, path: &str, content: &[u8]) -> Result<(), StorageError> {
        self.driver.put(path, content).await
    }

    /// Store a file with MIME type.
    pub async fn put_with_mime(
        &self,
        path: &str,
        content: &[u8],
        mime_type: &str,
    ) -> Result<(), StorageError> {
        self.driver.put_with_mime(path, content, mime_type).await
    }

    /// Get a file's content.
    pub async fn get(&self, path: &str) -> Result<Vec<u8>, StorageError> {
        self.driver.get(path).await
    }

    /// Check if a file exists.
    pub async fn exists(&self, path: &str) -> Result<bool, StorageError> {
        self.driver.exists(path).await
    }

    /// Delete a file.
    pub async fn delete(&self, path: &str) -> Result<(), StorageError> {
        self.driver.delete(path).await
    }

    /// Get the URL to access a file (if supported).
    pub async fn url(&self, path: &str) -> Result<String, StorageError> {
        self.driver.url(path).await
    }

    /// Get file metadata.
    pub async fn metadata(&self, path: &str) -> Result<FileMetadata, StorageError> {
        self.driver.metadata(path).await
    }

    /// List files in a directory.
    pub async fn list(&self, directory: &str) -> Result<Vec<String>, StorageError> {
        self.driver.list(directory).await
    }

    /// Copy a file.
    pub async fn copy(&self, from: &str, to: &str) -> Result<(), StorageError> {
        self.driver.copy(from, to).await
    }

    /// Move a file.
    pub async fn rename(&self, from: &str, to: &str) -> Result<(), StorageError> {
        self.driver.rename(from, to).await
    }
}

/// File metadata.
#[derive(Debug, Clone)]
pub struct FileMetadata {
    /// File size in bytes
    pub size: u64,
    /// MIME type (if known)
    pub mime_type: Option<String>,
    /// Last modified timestamp
    pub modified: Option<i64>,
    /// Created timestamp
    pub created: Option<i64>,
}
