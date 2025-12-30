//! Validated extractors for automatic request validation.
//!
//! These extractors integrate with the `validator` crate to automatically
//! validate incoming request data and return proper 422 responses on failure.

use axum::{
    extract::{FromRequest, FromRequestParts, Query, Request},
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde::de::DeserializeOwned;
use serde_json::json;
use std::fmt;
use validator::{Validate, ValidationErrors};

// ============================================================================
// Validation Error Response
// ============================================================================

/// Error type for validation failures.
/// Returns a 422 Unprocessable Entity with detailed error messages.
#[derive(Debug)]
pub struct ValidationError(pub ValidationErrors);

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Validation failed: {:?}", self.0)
    }
}

impl IntoResponse for ValidationError {
    fn into_response(self) -> Response {
        let errors = format_validation_errors(&self.0);
        let body = json!({
            "error": "Validation failed",
            "code": "VALIDATION_ERROR",
            "details": errors
        });
        (StatusCode::UNPROCESSABLE_ENTITY, Json(body)).into_response()
    }
}

/// Format validation errors into a user-friendly structure.
/// Output format: { "field_name": ["error message 1", "error message 2"] }
fn format_validation_errors(errors: &ValidationErrors) -> serde_json::Value {
    let mut result = serde_json::Map::new();

    for (field, field_errors) in errors.field_errors() {
        let messages: Vec<String> = field_errors
            .iter()
            .map(|e| {
                e.message
                    .as_ref()
                    .map(|m| m.to_string())
                    .unwrap_or_else(|| format!("Invalid value for {}", field))
            })
            .collect();
        result.insert(field.to_string(), json!(messages));
    }

    serde_json::Value::Object(result)
}

// ============================================================================
// ValidatedJson Extractor
// ============================================================================

/// A validated JSON extractor.
///
/// This extractor works like `axum::Json`, but also validates the deserialized
/// data using the `validator` crate. If validation fails, it returns a 422
/// response with detailed error messages.
///
/// # Example
///
/// ```ignore
/// use validator::Validate;
/// use serde::Deserialize;
/// use rustexpress::extractors::ValidatedJson;
///
/// #[derive(Deserialize, Validate)]
/// struct CreateUser {
///     #[validate(email(message = "Invalid email address"))]
///     email: String,
///     
///     #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
///     password: String,
/// }
///
/// async fn create_user(ValidatedJson(payload): ValidatedJson<CreateUser>) {
///     // payload is guaranteed to be valid here
/// }
/// ```
#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedJson<T>(pub T);

impl<S, T> FromRequest<S> for ValidatedJson<T>
where
    S: Send + Sync,
    T: DeserializeOwned + Validate + Send,
{
    type Rejection = Response;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        // First, extract JSON
        let Json(data) = Json::<T>::from_request(req, state)
            .await
            .map_err(|e| {
                let body = json!({
                    "error": "Invalid JSON body",
                    "code": "JSON_PARSE_ERROR",
                    "details": e.to_string()
                });
                (StatusCode::BAD_REQUEST, Json(body)).into_response()
            })?;

        // Then, validate
        data.validate().map_err(|e| ValidationError(e).into_response())?;

        Ok(ValidatedJson(data))
    }
}

// ============================================================================
// ValidatedQuery Extractor
// ============================================================================

/// A validated query string extractor.
///
/// Similar to `ValidatedJson`, but for query parameters.
///
/// # Example
///
/// ```ignore
/// use validator::Validate;
/// use serde::Deserialize;
/// use rustexpress::extractors::ValidatedQuery;
///
/// #[derive(Deserialize, Validate)]
/// struct Pagination {
///     #[validate(range(min = 1, message = "Page must be at least 1"))]
///     page: u32,
///     
///     #[validate(range(min = 1, max = 100, message = "Limit must be between 1 and 100"))]
///     limit: u32,
/// }
///
/// async fn list_items(ValidatedQuery(pagination): ValidatedQuery<Pagination>) {
///     // pagination is guaranteed to be valid here
/// }
/// ```
#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedQuery<T>(pub T);

impl<S, T> FromRequestParts<S> for ValidatedQuery<T>
where
    S: Send + Sync,
    T: DeserializeOwned + Validate + Send,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // First, extract query params
        let Query(data) = Query::<T>::from_request_parts(parts, state)
            .await
            .map_err(|e| {
                let body = json!({
                    "error": "Invalid query parameters",
                    "code": "QUERY_PARSE_ERROR",
                    "details": e.to_string()
                });
                (StatusCode::BAD_REQUEST, Json(body)).into_response()
            })?;

        // Then, validate
        data.validate().map_err(|e| ValidationError(e).into_response())?;

        Ok(ValidatedQuery(data))
    }
}

// ============================================================================
// Convenience type aliases
// ============================================================================

/// Result type for handlers that may return a validation error
pub type ValidatedResult<T> = Result<T, ValidationError>;
