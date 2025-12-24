//! Response helpers for consistent API responses.

use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

/// Result type for API handlers.
pub type ApiResult<T> = Result<JsonResponse<T>, ErrorResponse>;

/// Success JSON response wrapper.
#[derive(Debug, Clone, Serialize)]
pub struct JsonResponse<T> {
    pub success: bool,
    pub data: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl<T: Serialize> JsonResponse<T> {
    /// Create a success response.
    pub fn ok(data: T) -> Self {
        Self {
            success: true,
            data,
            message: None,
        }
    }

    /// Create a success response with message.
    pub fn ok_with_message(data: T, message: impl Into<String>) -> Self {
        Self {
            success: true,
            data,
            message: Some(message.into()),
        }
    }
}

impl<T: Serialize> IntoResponse for JsonResponse<T> {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

/// Error response wrapper.
#[derive(Debug, Clone, Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip)]
    pub status: StatusCode,
}

impl ErrorResponse {
    /// Create an error response.
    pub fn new(status: StatusCode, error: impl Into<String>) -> Self {
        Self {
            success: false,
            error: error.into(),
            code: None,
            status,
        }
    }

    /// Add an error code.
    pub fn with_code(mut self, code: impl Into<String>) -> Self {
        self.code = Some(code.into());
        self
    }

    // Common error constructors
    
    /// 400 Bad Request
    pub fn bad_request(error: impl Into<String>) -> Self {
        Self::new(StatusCode::BAD_REQUEST, error)
    }

    /// 401 Unauthorized
    pub fn unauthorized(error: impl Into<String>) -> Self {
        Self::new(StatusCode::UNAUTHORIZED, error)
    }

    /// 403 Forbidden
    pub fn forbidden(error: impl Into<String>) -> Self {
        Self::new(StatusCode::FORBIDDEN, error)
    }

    /// 404 Not Found
    pub fn not_found(error: impl Into<String>) -> Self {
        Self::new(StatusCode::NOT_FOUND, error)
    }

    /// 409 Conflict
    pub fn conflict(error: impl Into<String>) -> Self {
        Self::new(StatusCode::CONFLICT, error)
    }

    /// 422 Unprocessable Entity
    pub fn unprocessable(error: impl Into<String>) -> Self {
        Self::new(StatusCode::UNPROCESSABLE_ENTITY, error)
    }

    /// 500 Internal Server Error
    pub fn internal(error: impl Into<String>) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, error)
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> axum::response::Response {
        (self.status, Json(&self)).into_response()
    }
}

impl From<anyhow::Error> for ErrorResponse {
    fn from(err: anyhow::Error) -> Self {
        Self::internal(err.to_string())
    }
}

impl From<sea_orm::DbErr> for ErrorResponse {
    fn from(err: sea_orm::DbErr) -> Self {
        Self::internal(format!("Database error: {}", err))
    }
}

// Convenience functions

/// Create a success JSON response.
pub fn json_ok<T: Serialize>(data: T) -> JsonResponse<T> {
    JsonResponse::ok(data)
}

/// Create an empty success response.
pub fn ok() -> impl IntoResponse {
    (StatusCode::OK, Json(serde_json::json!({"success": true})))
}

/// Create a created response (201).
pub fn created<T: Serialize>(data: T) -> impl IntoResponse {
    (StatusCode::CREATED, Json(JsonResponse::ok(data)))
}

/// Create a no content response (204).
pub fn no_content() -> impl IntoResponse {
    StatusCode::NO_CONTENT
}
