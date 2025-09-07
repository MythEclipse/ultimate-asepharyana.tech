//! Error types for the headless Chrome library

use thiserror::Error;

/// Main error type for browser operations
#[derive(Error, Debug)]
pub enum BrowserError {
    #[error("Browser startup failed: {0}")]
    BrowserStartupError(String),

    #[error("Tab creation failed: {0}")]
    TabCreationError(String),

    #[error("Navigation failed: {0}")]
    NavigationError(String),

    #[error("Element not found: {0}")]
    ElementNotFound(String),

    #[error("Timeout error: {0}")]
    TimeoutError(String),

    #[error("Retry limit exceeded: {0}")]
    RetryLimitExceeded(String),

    #[error("Cloudflare challenge detected: {0}")]
    CloudflareChallenge(String),

    #[error("Proxy error: {0}")]
    ProxyError(String),

    #[error("Semaphore error: {0}")]
    SemaphoreError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Chromiumoxide error: {0}")]
    ChromiumoxideError(String),

    #[error("Serialization error: {0}")]
    SerdeError(#[from] serde_json::Error),

    #[error("URL parse error: {0}")]
    UrlParseError(#[from] url::ParseError),

    #[error("Generic error: {0}")]
    GenericError(String),
}

impl From<chromiumoxide::error::CdpError> for BrowserError {
    fn from(err: chromiumoxide::error::CdpError) -> Self {
        BrowserError::ChromiumoxideError(err.to_string())
    }
}

impl From<Box<dyn std::error::Error + Send + Sync>> for BrowserError {
    fn from(err: Box<dyn std::error::Error + Send + Sync>) -> Self {
        BrowserError::GenericError(err.to_string())
    }
}

/// Result type alias for browser operations
pub type BrowserResult<T> = Result<T, BrowserError>;
