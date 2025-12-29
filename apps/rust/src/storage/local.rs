//! Local filesystem storage driver.

use super::driver::{StorageDriver, StorageError};
use super::FileMetadata;
use async_trait::async_trait;
use std::path::{Path, PathBuf};
use tokio::fs;

/// Local filesystem storage driver.
#[derive(Clone)]
pub struct LocalDriver {
    base_path: PathBuf,
    base_url: Option<String>,
}

impl LocalDriver {
    /// Create a new local driver with the given base path.
    pub fn new(base_path: &str) -> Self {
        Self {
            base_path: PathBuf::from(base_path),
            base_url: None,
        }
    }

    /// Set the base URL for public file access.
    pub fn with_base_url(mut self, url: &str) -> Self {
        self.base_url = Some(url.trim_end_matches('/').to_string());
        self
    }

    /// Get the full path for a relative path.
    fn full_path(&self, path: &str) -> Result<PathBuf, StorageError> {
        let path = path.trim_start_matches('/');

        // Security: prevent path traversal
        if path.contains("..") {
            return Err(StorageError::InvalidPath(
                "Path traversal not allowed".to_string(),
            ));
        }

        Ok(self.base_path.join(path))
    }

    /// Ensure parent directory exists.
    async fn ensure_parent(&self, path: &Path) -> Result<(), StorageError> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await?;
        }
        Ok(())
    }
}

#[async_trait]
impl StorageDriver for LocalDriver {
    async fn put(&self, path: &str, content: &[u8]) -> Result<(), StorageError> {
        let full_path = self.full_path(path)?;
        self.ensure_parent(&full_path).await?;
        fs::write(&full_path, content).await?;
        Ok(())
    }

    async fn get(&self, path: &str) -> Result<Vec<u8>, StorageError> {
        let full_path = self.full_path(path)?;
        let content = fs::read(&full_path).await?;
        Ok(content)
    }

    async fn exists(&self, path: &str) -> Result<bool, StorageError> {
        let full_path = self.full_path(path)?;
        Ok(full_path.exists())
    }

    async fn delete(&self, path: &str) -> Result<(), StorageError> {
        let full_path = self.full_path(path)?;
        if full_path.exists() {
            fs::remove_file(&full_path).await?;
        }
        Ok(())
    }

    async fn url(&self, path: &str) -> Result<String, StorageError> {
        match &self.base_url {
            Some(base) => {
                let path = path.trim_start_matches('/');
                Ok(format!("{}/{}", base, path))
            }
            None => {
                let full_path = self.full_path(path)?;
                Ok(full_path.to_string_lossy().to_string())
            }
        }
    }

    async fn metadata(&self, path: &str) -> Result<FileMetadata, StorageError> {
        let full_path = self.full_path(path)?;
        let meta = fs::metadata(&full_path).await?;

        let mime_type = mime_guess::from_path(&full_path)
            .first()
            .map(|m| m.to_string());

        let modified = meta
            .modified()
            .ok()
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|d| d.as_secs() as i64);

        let created = meta
            .created()
            .ok()
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|d| d.as_secs() as i64);

        Ok(FileMetadata {
            size: meta.len(),
            mime_type,
            modified,
            created,
        })
    }

    async fn list(&self, directory: &str) -> Result<Vec<String>, StorageError> {
        let full_path = self.full_path(directory)?;

        if !full_path.exists() {
            return Ok(Vec::new());
        }

        let mut entries = Vec::new();
        let mut dir = fs::read_dir(&full_path).await?;

        while let Some(entry) = dir.next_entry().await? {
            let path = entry.path();
            if path.is_file() {
                if let Ok(relative) = path.strip_prefix(&self.base_path) {
                    entries.push(relative.to_string_lossy().to_string());
                }
            }
        }

        Ok(entries)
    }

    async fn copy(&self, from: &str, to: &str) -> Result<(), StorageError> {
        let from_path = self.full_path(from)?;
        let to_path = self.full_path(to)?;
        self.ensure_parent(&to_path).await?;
        fs::copy(&from_path, &to_path).await?;
        Ok(())
    }

    async fn rename(&self, from: &str, to: &str) -> Result<(), StorageError> {
        let from_path = self.full_path(from)?;
        let to_path = self.full_path(to)?;
        self.ensure_parent(&to_path).await?;
        fs::rename(&from_path, &to_path).await?;
        Ok(())
    }
}
