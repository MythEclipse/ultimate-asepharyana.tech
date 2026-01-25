//! Request ID middleware for request tracing.

use axum::{extract::Request, http::HeaderValue, middleware::Next, response::Response};
use uuid::Uuid;

/// Request ID header name.
pub const REQUEST_ID_HEADER: &str = "x-request-id";

/// Extension to access request ID in handlers.
#[derive(Clone, Debug)]
pub struct RequestId(pub String);

impl RequestId {
    /// Generate a new request ID.
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    /// Get the request ID string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for RequestId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for RequestId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Middleware that adds a unique request ID to each request.
///
/// The request ID is:
/// - Taken from the `x-request-id` header if present
/// - Generated as a new UUID if not present
/// - Added to the response headers
/// - Available via the `RequestId` extension in handlers
///
/// # Example
///
/// ```ignore
/// use axum::Extension;
/// use rustexpress::observability::RequestId;
///
/// async fn handler(Extension(req_id): Extension<RequestId>) {
///     println!("Request ID: {}", req_id);
/// }
/// ```
pub async fn request_id_middleware(mut req: Request, next: Next) -> Response {
    // Get or generate request ID
    let request_id = req
        .headers()
        .get(REQUEST_ID_HEADER)
        .and_then(|v| v.to_str().ok())
        .map(|s| RequestId(s.to_string()))
        .unwrap_or_else(RequestId::new);

    // Add to tracing span
    let span = tracing::info_span!(
        "request",
        request_id = %request_id,
        method = %req.method(),
        uri = %req.uri(),
    );
    let _guard = span.enter();

    // Insert as extension for handlers
    req.extensions_mut().insert(request_id.clone());

    // Process request
    let mut response = next.run(req).await;

    // Add request ID to response headers
    if let Ok(value) = HeaderValue::from_str(&request_id.0) {
        response.headers_mut().insert(REQUEST_ID_HEADER, value);
    }

    response
}
