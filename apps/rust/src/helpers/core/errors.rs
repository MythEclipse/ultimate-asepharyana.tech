//! Axum error response helpers.

use axum::http::StatusCode;

/// Shorthand for creating error tuples for Axum handlers.
pub type HandlerError = (StatusCode, String);

/// Create internal server error.
pub fn internal_error(msg: impl Into<String>) -> HandlerError {
    (StatusCode::INTERNAL_SERVER_ERROR, msg.into())
}

/// Create internal server error from any error type.
pub fn internal_err<E: std::fmt::Display>(e: E) -> HandlerError {
    (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
}

/// Create bad request error.
pub fn bad_request(msg: impl Into<String>) -> HandlerError {
    (StatusCode::BAD_REQUEST, msg.into())
}

/// Create not found error.
pub fn not_found(msg: impl Into<String>) -> HandlerError {
    (StatusCode::NOT_FOUND, msg.into())
}

/// Create unauthorized error.
pub fn unauthorized(msg: impl Into<String>) -> HandlerError {
    (StatusCode::UNAUTHORIZED, msg.into())
}

/// Create forbidden error.
pub fn forbidden(msg: impl Into<String>) -> HandlerError {
    (StatusCode::FORBIDDEN, msg.into())
}

/// Create conflict error.
pub fn conflict(msg: impl Into<String>) -> HandlerError {
    (StatusCode::CONFLICT, msg.into())
}

/// Create too many requests error.
pub fn too_many_requests(msg: impl Into<String>) -> HandlerError {
    (StatusCode::TOO_MANY_REQUESTS, msg.into())
}

/// Map any error to internal server error.
pub fn map_internal<E: std::fmt::Display>(e: E) -> HandlerError {
    internal_err(e)
}

/// Create Redis error response.
pub fn redis_error<E: std::fmt::Display>(e: E) -> HandlerError {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Redis error: {}", e),
    )
}

/// Create database error response.
pub fn db_error<E: std::fmt::Display>(e: E) -> HandlerError {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Database error: {}", e),
    )
}

/// Create serialization error response.
pub fn serialization_error<E: std::fmt::Display>(e: E) -> HandlerError {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Serialization error: {}", e),
    )
}

/// Trait extension for Result to easily convert errors.
pub trait ResultExt<T, E> {
    /// Map error to internal server error.
    fn map_internal(self) -> Result<T, HandlerError>;

    /// Map error to bad request.
    fn map_bad_request(self) -> Result<T, HandlerError>;

    /// Map error to not found.
    fn map_not_found(self) -> Result<T, HandlerError>;
}

impl<T, E: std::fmt::Display> ResultExt<T, E> for Result<T, E> {
    fn map_internal(self) -> Result<T, HandlerError> {
        self.map_err(internal_err)
    }

    fn map_bad_request(self) -> Result<T, HandlerError> {
        self.map_err(|e| bad_request(e.to_string()))
    }

    fn map_not_found(self) -> Result<T, HandlerError> {
        self.map_err(|e| not_found(e.to_string()))
    }
}
