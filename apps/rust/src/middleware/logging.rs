//! Request/Response logging middleware.
//!
//! Provides structured logging for HTTP requests with timing and correlation.
//!
//! # Example
//!
//! ```ignore
//! use rust::middleware::logging::{LoggingConfig, with_logging};
//! use axum::Router;
//!
//! let app = Router::new()
//!     .route("/api/test", get(handler))
//!     .layer(axum::middleware::from_fn(with_logging(LoggingConfig::default())));
//! ```

use axum::{body::Body, extract::Request, http::StatusCode, middleware::Next, response::Response};
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Instant;
use uuid::Uuid;

/// Logging configuration.
#[derive(Debug, Clone)]
pub struct LoggingConfig {
    /// Log request headers
    pub log_headers: bool,
    /// Log request body (be careful with sensitive data)
    pub log_body: bool,
    /// Log response body
    pub log_response_body: bool,
    /// Maximum body size to log (bytes)
    pub max_body_size: usize,
    /// Paths to exclude from logging
    pub exclude_paths: HashSet<String>,
    /// Log level for successful requests
    pub success_level: LogLevel,
    /// Log level for client errors (4xx)
    pub client_error_level: LogLevel,
    /// Log level for server errors (5xx)
    pub server_error_level: LogLevel,
}

/// Log level enum.
#[derive(Debug, Clone, Copy)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        let mut exclude = HashSet::new();
        exclude.insert("/health".to_string());
        exclude.insert("/metrics".to_string());
        exclude.insert("/favicon.ico".to_string());

        Self {
            log_headers: false,
            log_body: false,
            log_response_body: false,
            max_body_size: 1024,
            exclude_paths: exclude,
            success_level: LogLevel::Info,
            client_error_level: LogLevel::Warn,
            server_error_level: LogLevel::Error,
        }
    }
}

impl LoggingConfig {
    /// Create a new logging config.
    pub fn new() -> Self {
        Self::default()
    }

    /// Enable header logging.
    pub fn with_headers(mut self) -> Self {
        self.log_headers = true;
        self
    }

    /// Add a path to exclude from logging.
    pub fn exclude_path(mut self, path: &str) -> Self {
        self.exclude_paths.insert(path.to_string());
        self
    }
}

/// Request ID extension for correlation.
#[derive(Clone, Debug)]
pub struct RequestId(pub String);

impl RequestId {
    /// Generate a new request ID.
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    /// Get the request ID as a string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for RequestId {
    fn default() -> Self {
        Self::new()
    }
}

/// Logging middleware.
pub async fn logging_middleware(config: Arc<LoggingConfig>, req: Request, next: Next) -> Response {
    let path = req.uri().path().to_string();

    // Skip excluded paths
    if config.exclude_paths.contains(&path) {
        return next.run(req).await;
    }

    // Generate or extract request ID
    let request_id = req
        .headers()
        .get("X-Request-ID")
        .and_then(|h| h.to_str().ok())
        .map(|s| RequestId(s.to_string()))
        .unwrap_or_else(RequestId::new);

    let method = req.method().clone();
    let uri = req.uri().clone();
    let _version = format!("{:?}", req.version());

    // Log headers if configured
    let headers_log = if config.log_headers {
        let headers: Vec<String> = req
            .headers()
            .iter()
            .filter(|(name, _)| {
                // Skip sensitive headers
                let name_str = name.as_str().to_lowercase();
                !name_str.contains("authorization")
                    && !name_str.contains("cookie")
                    && !name_str.contains("x-api-key")
            })
            .map(|(name, value)| format!("{}: {}", name, value.to_str().unwrap_or("<binary>")))
            .collect();
        Some(headers)
    } else {
        None
    };

    // Add request ID to extensions
    let mut req = req;
    req.extensions_mut().insert(request_id.clone());

    // Start timing
    let start = Instant::now();

    // Log incoming request
    tracing::info!(
        request_id = %request_id.0,
        method = %method,
        path = %uri.path(),
        query = ?uri.query(),
        "→ Request started"
    );

    if let Some(ref headers) = headers_log {
        tracing::debug!(request_id = %request_id.0, headers = ?headers, "Request headers");
    }

    // Execute request
    let response = next.run(req).await;

    // Calculate duration
    let duration = start.elapsed();
    let status = response.status();

    // Determine log level based on status
    let log_entry = format!(
        "← Response: {} {} -> {} in {:?}",
        method,
        uri.path(),
        status,
        duration
    );

    match status.as_u16() {
        100..=399 => match config.success_level {
            LogLevel::Trace => {
                tracing::trace!(request_id = %request_id.0, status = %status, duration_ms = %duration.as_millis(), "{}", log_entry)
            }
            LogLevel::Debug => {
                tracing::debug!(request_id = %request_id.0, status = %status, duration_ms = %duration.as_millis(), "{}", log_entry)
            }
            LogLevel::Info => {
                tracing::info!(request_id = %request_id.0, status = %status, duration_ms = %duration.as_millis(), "{}", log_entry)
            }
            LogLevel::Warn => {
                tracing::warn!(request_id = %request_id.0, status = %status, duration_ms = %duration.as_millis(), "{}", log_entry)
            }
            LogLevel::Error => {
                tracing::error!(request_id = %request_id.0, status = %status, duration_ms = %duration.as_millis(), "{}", log_entry)
            }
        },
        400..=499 => match config.client_error_level {
            LogLevel::Trace => {
                tracing::trace!(request_id = %request_id.0, status = %status, duration_ms = %duration.as_millis(), "{}", log_entry)
            }
            LogLevel::Debug => {
                tracing::debug!(request_id = %request_id.0, status = %status, duration_ms = %duration.as_millis(), "{}", log_entry)
            }
            LogLevel::Info => {
                tracing::info!(request_id = %request_id.0, status = %status, duration_ms = %duration.as_millis(), "{}", log_entry)
            }
            LogLevel::Warn => {
                tracing::warn!(request_id = %request_id.0, status = %status, duration_ms = %duration.as_millis(), "{}", log_entry)
            }
            LogLevel::Error => {
                tracing::error!(request_id = %request_id.0, status = %status, duration_ms = %duration.as_millis(), "{}", log_entry)
            }
        },
        _ => match config.server_error_level {
            LogLevel::Trace => {
                tracing::trace!(request_id = %request_id.0, status = %status, duration_ms = %duration.as_millis(), "{}", log_entry)
            }
            LogLevel::Debug => {
                tracing::debug!(request_id = %request_id.0, status = %status, duration_ms = %duration.as_millis(), "{}", log_entry)
            }
            LogLevel::Info => {
                tracing::info!(request_id = %request_id.0, status = %status, duration_ms = %duration.as_millis(), "{}", log_entry)
            }
            LogLevel::Warn => {
                tracing::warn!(request_id = %request_id.0, status = %status, duration_ms = %duration.as_millis(), "{}", log_entry)
            }
            LogLevel::Error => {
                tracing::error!(request_id = %request_id.0, status = %status, duration_ms = %duration.as_millis(), "{}", log_entry)
            }
        },
    }

    response
}

/// Create logging middleware.
pub fn with_logging(
    config: LoggingConfig,
) -> impl Fn(
    Request<Body>,
    Next,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = Response> + Send>>
       + Clone
       + Send {
    let config = Arc::new(config);
    move |req: Request<Body>, next: Next| {
        let config = config.clone();
        Box::pin(async move { logging_middleware(config, req, next).await })
    }
}

/// Extractor for RequestId in route handlers.
impl<S> axum::extract::FromRequestParts<S> for RequestId
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        parts.extensions.get::<RequestId>().cloned().ok_or((
            StatusCode::INTERNAL_SERVER_ERROR,
            "RequestId not found. Did you add logging middleware?",
        ))
    }
}
