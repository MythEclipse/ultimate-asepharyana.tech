// Error module for crate-wide error handling

use thiserror::Error;
use axum::{
  http::StatusCode,
  response::{IntoResponse, Response},
  Json,
};
use serde_json::json;
use serde::{Deserialize, Serialize};
use crate::utils::error::AppError;

#[derive(Error, Debug)]
pub enum LibError {
  #[error("An unknown error occurred")]
  Unknown,
  #[error("Fantoccini error: {0}")] FantocciniError(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

impl From<AppError> for ErrorResponse {
    fn from(app_error: AppError) -> Self {
        ErrorResponse {
            error: app_error.to_string(),
        }
    }
}
impl IntoResponse for ErrorResponse {
  fn into_response(self) -> Response {
    let (status, error_message) = match self.error.as_str() {
      _ if self.error.contains("Environment variable not found") =>
        (StatusCode::INTERNAL_SERVER_ERROR, self.error),
      _ if self.error.contains("Redis error") => (StatusCode::INTERNAL_SERVER_ERROR, self.error),
      _ if self.error.contains("Reqwest error") => (StatusCode::INTERNAL_SERVER_ERROR, self.error),
      _ if self.error.contains("JSON serialization/deserialization error") =>
        (StatusCode::INTERNAL_SERVER_ERROR, self.error),
      _ if self.error.contains("URL parsing error") => (StatusCode::BAD_REQUEST, self.error),
      _ if self.error.contains("JWT error") => (StatusCode::UNAUTHORIZED, self.error),
      _ if self.error.contains("Scraper error") => (StatusCode::BAD_GATEWAY, self.error),
      _ if self.error.contains("Fantoccini error") => (StatusCode::BAD_GATEWAY, self.error),
      _ if self.error.contains("IO error") => (StatusCode::INTERNAL_SERVER_ERROR, self.error),
      _ if self.error.contains("Timeout error") => (StatusCode::REQUEST_TIMEOUT, self.error),
      _ => (StatusCode::INTERNAL_SERVER_ERROR, self.error),
    };

    let body = Json(json!({
            "error": error_message,
        }));

    (status, body).into_response()
  }
}

pub use ErrorResponse;
