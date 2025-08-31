use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Environment variable not found: {0}")]
    EnvVarNotFound(String),
    #[error("Redis error: {0}")]
    RedisError(#[from] redis::RedisError),
    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("JSON serialization/deserialization error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("URL parsing error: {0}")]
    UrlParseError(#[from] url::ParseError),
    #[error("JWT error: {0}")]
    JwtError(#[from] jsonwebtoken::errors::Error),
    #[error("Scraper error: {0}")]
    ScraperError(String), // Custom error for scraper issues
    #[error("Headless Chrome error: {0}")]
    // Use the correct error type for headless_chrome (TabError or BrowserError)
    HeadlessChromeError(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Timeout error: {0}")]
    TimeoutError(String),
    #[error("Other error: {0}")]
    Other(String),
}

impl From<&str> for AppError {
    fn from(s: &str) -> Self {
        AppError::Other(s.to_string())
    }
}
