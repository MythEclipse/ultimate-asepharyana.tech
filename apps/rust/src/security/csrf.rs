//! CSRF (Cross-Site Request Forgery) protection.
//!
//! Implements the double-submit cookie pattern for CSRF protection.
//!
//! # Example
//!
//! ```ignore
//! use rust::security::{CsrfConfig, CsrfLayer, CsrfToken};
//! use axum::{Router, routing::post};
//!
//! // Add CSRF protection to routes
//! let app = Router::new()
//!     .route("/api/submit", post(handler))
//!     .layer(CsrfLayer::new(CsrfConfig::default()));
//!
//! // In your handler, get the CSRF token for forms
//! async fn form_handler(csrf: CsrfToken) -> impl IntoResponse {
//!     // Include csrf.token() in your form as a hidden field
//! }
//! ```

use axum::{
    body::Body,
    extract::{FromRequestParts, Request},
    http::{header, request::Parts, HeaderValue, Method, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use rand::RngCore;
use std::collections::HashSet;
use std::sync::Arc;

/// CSRF protection configuration.
#[derive(Debug, Clone)]
pub struct CsrfConfig {
    /// Cookie name for CSRF token
    pub cookie_name: String,
    /// Header name to check for token
    pub header_name: String,
    /// Form field name to check for token
    pub field_name: String,
    /// Token length in bytes (before base64 encoding)
    pub token_length: usize,
    /// Cookie path
    pub path: String,
    /// Secure cookie (HTTPS only)
    pub secure: bool,
    /// SameSite policy
    pub same_site: SameSite,
    /// Paths to exclude from CSRF protection
    pub exclude_paths: HashSet<String>,
    /// HTTP methods that require CSRF validation
    pub protected_methods: HashSet<Method>,
}

/// SameSite cookie attribute
#[derive(Debug, Clone, Copy)]
pub enum SameSite {
    Strict,
    Lax,
    None,
}

impl Default for CsrfConfig {
    fn default() -> Self {
        let mut protected = HashSet::new();
        protected.insert(Method::POST);
        protected.insert(Method::PUT);
        protected.insert(Method::PATCH);
        protected.insert(Method::DELETE);

        Self {
            cookie_name: "csrf_token".to_string(),
            header_name: "X-CSRF-TOKEN".to_string(),
            field_name: "_csrf".to_string(),
            token_length: 32,
            path: "/".to_string(),
            secure: true,
            same_site: SameSite::Lax,
            exclude_paths: HashSet::new(),
            protected_methods: protected,
        }
    }
}

impl CsrfConfig {
    /// Create a new CSRF config with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a path to exclude from CSRF protection.
    pub fn exclude_path(mut self, path: &str) -> Self {
        self.exclude_paths.insert(path.to_string());
        self
    }

    /// Set the cookie to be secure (HTTPS only).
    pub fn secure(mut self, secure: bool) -> Self {
        self.secure = secure;
        self
    }
}

/// CSRF token that can be extracted in handlers.
#[derive(Clone)]
pub struct CsrfToken {
    token: String,
}

impl CsrfToken {
    /// Get the CSRF token value.
    pub fn token(&self) -> &str {
        &self.token
    }

    /// Generate a hidden input field for forms.
    pub fn hidden_input(&self, field_name: &str) -> String {
        format!(
            r#"<input type="hidden" name="{}" value="{}" />"#,
            field_name, self.token
        )
    }
}

/// Generate a cryptographically secure random token.
fn generate_token(length: usize) -> String {
    let mut bytes = vec![0u8; length];
    rand::rng().fill_bytes(&mut bytes);
    URL_SAFE_NO_PAD.encode(&bytes)
}

/// CSRF middleware layer.
#[derive(Clone)]
#[allow(dead_code)]
pub struct CsrfLayer {
    config: Arc<CsrfConfig>,
}

impl CsrfLayer {
    /// Create a new CSRF layer with the given configuration.
    pub fn new(config: CsrfConfig) -> Self {
        Self {
            config: Arc::new(config),
        }
    }

    /// Create with default configuration.
    pub fn default_layer() -> Self {
        Self::new(CsrfConfig::default())
    }
}

/// CSRF middleware function.
pub async fn csrf_middleware(config: Arc<CsrfConfig>, req: Request, next: Next) -> Response {
    let method = req.method().clone();
    let path = req.uri().path().to_string();

    // Check if path is excluded
    if config.exclude_paths.contains(&path) {
        return next.run(req).await;
    }

    // Extract existing CSRF token from cookie
    let cookie_token = req
        .headers()
        .get(header::COOKIE)
        .and_then(|h| h.to_str().ok())
        .and_then(|cookies| {
            cookies.split(';').find_map(|cookie| {
                let cookie = cookie.trim();
                if cookie.starts_with(&config.cookie_name) {
                    cookie
                        .strip_prefix(&config.cookie_name)?
                        .strip_prefix('=')
                        .map(|s| s.to_string())
                } else {
                    None
                }
            })
        });

    // For protected methods, validate CSRF token
    if config.protected_methods.contains(&method) {
        let cookie_token = match cookie_token.as_ref() {
            Some(t) => t,
            None => {
                return (StatusCode::FORBIDDEN, "CSRF token missing from cookie").into_response();
            }
        };

        // Check for token in header first
        let request_token = req
            .headers()
            .get(&config.header_name)
            .and_then(|h| h.to_str().ok())
            .map(|s| s.to_string());

        // If not in header, we'd need to check the body (form field)
        // For simplicity, we only check header here
        // Full implementation would need to peek at body for form submissions
        let request_token = match request_token {
            Some(t) => t,
            None => {
                return (
                    StatusCode::FORBIDDEN,
                    "CSRF token missing from request header",
                )
                    .into_response();
            }
        };

        // Validate tokens match (using constant-time comparison)
        if !constant_time_compare(cookie_token, &request_token) {
            return (StatusCode::FORBIDDEN, "CSRF token mismatch").into_response();
        }
    }

    // Generate or use existing token
    let is_new_token = cookie_token.is_none();
    let token = cookie_token.unwrap_or_else(|| generate_token(config.token_length));

    // Create CSRF token for extraction in handlers
    let csrf_token = CsrfToken {
        token: token.clone(),
    };

    // Add CSRF token to request extensions
    let mut req = req;
    req.extensions_mut().insert(csrf_token);

    // Run the request
    let response = next.run(req).await;

    // Set CSRF cookie if not already set
    if is_new_token {
        let cookie_value = build_csrf_cookie(&config, &token);
        let mut response = response;
        if let Ok(header_val) = HeaderValue::from_str(&cookie_value) {
            response
                .headers_mut()
                .insert(header::SET_COOKIE, header_val);
        }
        response
    } else {
        response
    }
}

/// Constant-time string comparison to prevent timing attacks.
fn constant_time_compare(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();
    let mut result = 0u8;

    for (x, y) in a_bytes.iter().zip(b_bytes.iter()) {
        result |= x ^ y;
    }

    result == 0
}

/// Build CSRF cookie value.
fn build_csrf_cookie(config: &CsrfConfig, token: &str) -> String {
    let mut cookie = format!("{}={}; Path={}", config.cookie_name, token, config.path);

    if config.secure {
        cookie.push_str("; Secure");
    }

    let same_site_str = match config.same_site {
        SameSite::Strict => "Strict",
        SameSite::Lax => "Lax",
        SameSite::None => "None",
    };
    cookie.push_str(&format!("; SameSite={}", same_site_str));

    cookie
}

/// Extractor for CsrfToken in route handlers.
impl<S> FromRequestParts<S> for CsrfToken
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts.extensions.get::<CsrfToken>().cloned().ok_or((
            StatusCode::INTERNAL_SERVER_ERROR,
            "CSRF token not found. Did you add CsrfLayer?",
        ))
    }
}

/// Create CSRF middleware from config.
pub fn with_csrf(
    config: CsrfConfig,
) -> impl Fn(
    Request<Body>,
    Next,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = Response> + Send>>
       + Clone
       + Send {
    let config = Arc::new(config);
    move |req: Request<Body>, next: Next| {
        let config = config.clone();
        Box::pin(async move { csrf_middleware(config, req, next).await })
    }
}
