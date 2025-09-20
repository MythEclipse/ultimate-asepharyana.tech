use thiserror::Error;
use axum::response::IntoResponse;

#[derive(Error, Debug)]
pub enum AppError {
  #[error("Environment variable not found: {0}")] EnvVarNotFound(String),
  #[error("Redis error: {0}")] RedisError(#[from] redis::RedisError),
  #[error("Reqwest error: {0}")] ReqwestError(#[from] reqwest::Error),
  #[error("JSON serialization/deserialization error: {0}")] SerdeJsonError(
    #[from] serde_json::Error,
  ),
  #[error("URL parsing error: {0}")] UrlParseError(#[from] url::ParseError),
  #[error("JWT error: {0}")] JwtError(#[from] jsonwebtoken::errors::Error),
  #[error("Scraper error: {0}")] ScraperError(String), // Custom error for scraper issues
  #[error("Fantoccini error: {0}")] FantocciniError(String),
  #[error("Chromiumoxide error: {0}")] ChromiumoxideError(String),
  #[error("IO error: {0}")] IoError(#[from] std::io::Error),
  #[error("Timeout error: {0}")] TimeoutError(String),
  #[error("Other error: {0}")] Other(String),
  #[error("HTTP error: {0}")] HttpError(#[from] http::Error),
}

impl From<failure::Error> for AppError {
    fn from(err: failure::Error) -> Self {
        AppError::Other(err.to_string())
    }
}

impl From<&str> for AppError {
  fn from(s: &str) -> Self {
    AppError::Other(s.to_string())
  }
}

impl From<Box<dyn std::error::Error + Send + Sync>> for AppError {
    fn from(err: Box<dyn std::error::Error + Send + Sync>) -> Self {
        AppError::Other(err.to_string())
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::Other(err.to_string())
    }
}

impl From<deadpool_redis::PoolError> for AppError {
    fn from(err: deadpool_redis::PoolError) -> Self {
        AppError::Other(err.to_string())
    }
}

impl From<tokio::task::JoinError> for AppError {
    fn from(err: tokio::task::JoinError) -> Self {
        AppError::Other(err.to_string())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (http::StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
    }
}
