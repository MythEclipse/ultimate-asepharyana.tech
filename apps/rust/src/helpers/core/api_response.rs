//! Standardized API response helpers.
//!
//! Provides consistent JSON response structures across all API endpoints.
//!
//! # Example
//!
//! ```ignore
//! use rustexpress::helpers::api_response::{ApiResponse, ApiResult};
//!
//! async fn get_user(id: i32) -> ApiResult<User> {
//!     let user = find_user(id)?;
//!     Ok(ApiResponse::success(user))
//! }
//!
//! async fn list_users(page: u32) -> ApiResult<Vec<User>> {
//!     let (users, total) = find_users_paginated(page)?;
//!     Ok(ApiResponse::paginated(users, page, 20, total))
//! }
//!
//! async fn delete_user(id: i32) -> ApiResult<()> {
//!     delete_user(id)?;
//!     Ok(ApiResponse::no_content())
//! }
//! ```

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Standard API response wrapper.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ApiResponse<T> {
    /// Whether the request was successful
    pub success: bool,
    /// Response data (if successful)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    /// Error message (if failed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetails>,
    /// Pagination metadata (if applicable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<PaginationMeta>,
    /// Additional metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Option<Object>)]
    pub meta: Option<serde_json::Value>,
}

/// Error details for failed responses.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ErrorDetails {
    /// Error code (machine-readable)
    pub code: String,
    /// Human-readable error message
    pub message: String,
    /// Field-level validation errors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<FieldError>>,
}

/// Field-level validation error.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct FieldError {
    /// Field name
    pub field: String,
    /// Error message
    pub message: String,
}

/// Pagination metadata.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PaginationMeta {
    /// Current page number
    pub page: u32,
    /// Items per page
    pub per_page: u32,
    /// Total number of items
    pub total: u64,
    /// Total number of pages
    pub total_pages: u32,
    /// Whether there's a next page
    pub has_next: bool,
    /// Whether there's a previous page
    pub has_prev: bool,
}

impl<T: Serialize> ApiResponse<T> {
    /// Create a successful response with data.
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            pagination: None,
            meta: None,
        }
    }

    /// Create a successful response with data and metadata.
    pub fn success_with_meta(data: T, meta: serde_json::Value) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            pagination: None,
            meta: Some(meta),
        }
    }

    /// Create a paginated response.
    pub fn paginated(data: T, page: u32, per_page: u32, total: u64) -> Self {
        let total_pages = ((total as f64) / (per_page as f64)).ceil() as u32;
        Self {
            success: true,
            data: Some(data),
            error: None,
            pagination: Some(PaginationMeta {
                page,
                per_page,
                total,
                total_pages,
                has_next: page < total_pages,
                has_prev: page > 1,
            }),
            meta: None,
        }
    }
}

impl ApiResponse<()> {
    /// Create a successful response with no data.
    pub fn no_content() -> Self {
        Self {
            success: true,
            data: None,
            error: None,
            pagination: None,
            meta: None,
        }
    }

    /// Create a successful message response.
    pub fn message(msg: &str) -> ApiResponse<MessageOnly> {
        ApiResponse {
            success: true,
            data: Some(MessageOnly {
                message: msg.to_string(),
            }),
            error: None,
            pagination: None,
            meta: None,
        }
    }
}

/// Simple message-only response data.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MessageOnly {
    pub message: String,
}

/// API error response builder.
#[derive(Debug)]
pub struct ApiError {
    pub status: StatusCode,
    pub code: String,
    pub message: String,
    pub fields: Option<Vec<FieldError>>,
}

impl ApiError {
    /// Create a new API error.
    pub fn new(status: StatusCode, code: &str, message: &str) -> Self {
        Self {
            status,
            code: code.to_string(),
            message: message.to_string(),
            fields: None,
        }
    }

    /// Create a 400 Bad Request error.
    pub fn bad_request(message: &str) -> Self {
        Self::new(StatusCode::BAD_REQUEST, "BAD_REQUEST", message)
    }

    /// Create a 401 Unauthorized error.
    pub fn unauthorized(message: &str) -> Self {
        Self::new(StatusCode::UNAUTHORIZED, "UNAUTHORIZED", message)
    }

    /// Create a 403 Forbidden error.
    pub fn forbidden(message: &str) -> Self {
        Self::new(StatusCode::FORBIDDEN, "FORBIDDEN", message)
    }

    /// Create a 404 Not Found error.
    pub fn not_found(message: &str) -> Self {
        Self::new(StatusCode::NOT_FOUND, "NOT_FOUND", message)
    }

    /// Create a 409 Conflict error.
    pub fn conflict(message: &str) -> Self {
        Self::new(StatusCode::CONFLICT, "CONFLICT", message)
    }

    /// Create a 422 Unprocessable Entity error with field errors.
    pub fn validation(fields: Vec<FieldError>) -> Self {
        Self {
            status: StatusCode::UNPROCESSABLE_ENTITY,
            code: "VALIDATION_ERROR".to_string(),
            message: "Validation failed".to_string(),
            fields: Some(fields),
        }
    }

    /// Create a 429 Too Many Requests error.
    pub fn too_many_requests(message: &str) -> Self {
        Self::new(StatusCode::TOO_MANY_REQUESTS, "RATE_LIMITED", message)
    }

    /// Create a 500 Internal Server Error.
    pub fn internal(message: &str) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", message)
    }

    /// Create a 503 Service Unavailable error.
    pub fn service_unavailable(message: &str) -> Self {
        Self::new(
            StatusCode::SERVICE_UNAVAILABLE,
            "SERVICE_UNAVAILABLE",
            message,
        )
    }

    /// Add field errors.
    pub fn with_fields(mut self, fields: Vec<FieldError>) -> Self {
        self.fields = Some(fields);
        self
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let body = ApiResponse::<()> {
            success: false,
            data: None,
            error: Some(ErrorDetails {
                code: self.code,
                message: self.message,
                fields: self.fields,
            }),
            pagination: None,
            meta: None,
        };

        (self.status, Json(body)).into_response()
    }
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        let status = if self.success {
            StatusCode::OK
        } else {
            StatusCode::BAD_REQUEST
        };
        (status, Json(self)).into_response()
    }
}

/// Type alias for API handler results.
pub type ApiResult<T> = Result<ApiResponse<T>, ApiError>;

/// Helper to convert any serializable to success response.
pub fn success<T: Serialize>(data: T) -> ApiResponse<T> {
    ApiResponse::success(data)
}

/// Helper to convert any serializable to paginated response.
pub fn paginated<T: Serialize>(data: T, page: u32, per_page: u32, total: u64) -> ApiResponse<T> {
    ApiResponse::paginated(data, page, per_page, total)
}

/// Helper for quick internal error.
pub fn internal_err(msg: &str) -> ApiError {
    ApiError::internal(msg)
}

/// Helper for quick not found error.
pub fn not_found(msg: &str) -> ApiError {
    ApiError::not_found(msg)
}

/// Helper for quick bad request error.
pub fn bad_request(msg: &str) -> ApiError {
    ApiError::bad_request(msg)
}

