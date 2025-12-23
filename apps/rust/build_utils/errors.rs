//! Custom error types for build operations with helpful error messages.

use std::path::PathBuf;
use thiserror::Error;

/// Errors that can occur during build operations
#[derive(Error, Debug)]
pub enum BuildError {
    #[error("Failed to read file: {path}\n  Caused by: {source}")]
    FileRead {
        path: PathBuf,
        source: std::io::Error,
    },

    #[error("Failed to write file: {path}\n  Caused by: {source}")]
    FileWrite {
        path: PathBuf,
        source: std::io::Error,
    },

    #[error("Invalid file path: {path}\n  Suggestion: Ensure the path is valid and the file has a proper extension")]
    InvalidPath { path: PathBuf },

    #[error("File stem could not be extracted from: {path}\n  Suggestion: Ensure the file has a valid name")]
    NoFileStem { path: PathBuf },

    #[error("Invalid HTTP method: '{method}'\n  Supported methods: GET, POST, PUT, DELETE, PATCH")]
    InvalidHttpMethod { method: String },

    #[error("Regex compilation failed: {pattern}\n  Caused by: {source}")]
    RegexError {
        pattern: String,
        source: regex::Error,
    },

    #[error("Template generation failed for: {path}\n  Caused by: {message}")]
    TemplateError { path: PathBuf, message: String },

    #[error("OpenAPI validation failed\n  {details}\n  Suggestion: Check your response schemas and endpoint definitions")]
    OpenApiValidation { details: String },

    #[error("Module generation failed for directory: {path}\n  Caused by: {source}")]
    ModuleGeneration {
        path: PathBuf,
        source: Box<dyn std::error::Error + Send + Sync>,
    },
}

impl BuildError {
    /// Create a file read error with context
    pub fn file_read(path: impl Into<PathBuf>, source: std::io::Error) -> Self {
        Self::FileRead {
            path: path.into(),
            source,
        }
    }

    /// Create a file write error with context
    pub fn file_write(path: impl Into<PathBuf>, source: std::io::Error) -> Self {
        Self::FileWrite {
            path: path.into(),
            source,
        }
    }

    /// Create an invalid path error
    pub fn invalid_path(path: impl Into<PathBuf>) -> Self {
        Self::InvalidPath { path: path.into() }
    }

    /// Create a no file stem error
    pub fn no_file_stem(path: impl Into<PathBuf>) -> Self {
        Self::NoFileStem { path: path.into() }
    }
}

/// Result type for build operations
pub type BuildResult<T> = Result<T, anyhow::Error>;
