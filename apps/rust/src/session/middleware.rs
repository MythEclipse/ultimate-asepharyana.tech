//! Session middleware for Axum.

use super::store::{SameSite, SessionData, SessionStore};
use axum::{
    body::Body,
    extract::{FromRequestParts, Request},
    http::{header, request::Parts, HeaderValue, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Session handle that can be used in route handlers.
///
/// # Example
///
/// ```ignore
/// async fn handler(session: Session) -> impl IntoResponse {
///     session.set("user_id", "123").await;
///     let user_id: Option<String> = session.get("user_id").await;
///     "OK"
/// }
/// ```
#[derive(Clone)]
pub struct Session {
    inner: Arc<RwLock<SessionData>>,
    store: Arc<SessionStore>,
    modified: Arc<RwLock<bool>>,
}

impl Session {
    /// Create a new session handle.
    pub fn new(data: SessionData, store: Arc<SessionStore>) -> Self {
        Self {
            inner: Arc::new(RwLock::new(data)),
            store,
            modified: Arc::new(RwLock::new(false)),
        }
    }

    /// Get a value from the session.
    pub async fn get<T: serde::de::DeserializeOwned>(&self, key: &str) -> Option<T> {
        self.inner.read().await.get(key)
    }

    /// Set a value in the session.
    pub async fn set<T: serde::Serialize>(&self, key: &str, value: T) {
        self.inner.write().await.set(key, value);
        *self.modified.write().await = true;
    }

    /// Remove a value from the session.
    pub async fn remove(&self, key: &str) {
        self.inner.write().await.remove(key);
        *self.modified.write().await = true;
    }

    /// Check if a key exists.
    pub async fn has(&self, key: &str) -> bool {
        self.inner.read().await.has(key)
    }

    /// Get the session ID.
    pub async fn id(&self) -> String {
        self.inner.read().await.id.clone()
    }

    /// Set a flash message (available on next request only).
    pub async fn flash(&self, key: &str, message: &str) {
        self.inner.write().await.set_flash(key, message);
        *self.modified.write().await = true;
    }

    /// Get a flash message (consumed after reading).
    pub async fn get_flash(&self, key: &str) -> Option<String> {
        let msg = self.inner.write().await.get_flash(key);
        if msg.is_some() {
            *self.modified.write().await = true;
        }
        msg
    }

    /// Regenerate session ID (call after login for security).
    pub async fn regenerate(&self) -> Result<String, super::store::SessionError> {
        let mut data = self.inner.write().await;
        let new_id = self.store.regenerate(&mut data).await?;
        *self.modified.write().await = false; // Already saved
        Ok(new_id)
    }

    /// Destroy the session.
    pub async fn destroy(&self) -> Result<(), super::store::SessionError> {
        let id = self.inner.read().await.id.clone();
        self.store.destroy(&id).await
    }

    /// Save session if modified.
    pub async fn save_if_modified(&self) -> Result<bool, super::store::SessionError> {
        if *self.modified.read().await {
            self.store.save(&*self.inner.read().await).await?;
            *self.modified.write().await = false;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

/// Session middleware layer.
#[derive(Clone)]
#[allow(dead_code)]
pub struct SessionLayer {
    store: Arc<SessionStore>,
}

impl SessionLayer {
    /// Create a new session layer with the given store.
    pub fn new(store: SessionStore) -> Self {
        Self {
            store: Arc::new(store),
        }
    }
}

/// Middleware function for session handling.
pub async fn session_middleware(store: Arc<SessionStore>, req: Request, next: Next) -> Response {
    let config = store.config();
    let cookie_name = &config.cookie_name;

    // Extract session ID from cookie
    let session_id = req
        .headers()
        .get(header::COOKIE)
        .and_then(|h| h.to_str().ok())
        .and_then(|cookies| {
            cookies.split(';').find_map(|cookie| {
                let cookie = cookie.trim();
                if cookie.starts_with(cookie_name) {
                    cookie
                        .strip_prefix(cookie_name)?
                        .strip_prefix('=')
                        .map(|s| s.to_string())
                } else {
                    None
                }
            })
        });

    // Load or create session
    let (session_data, is_new) = match session_id {
        Some(ref id) => match store.load(id).await {
            Ok(Some(data)) => (data, false),
            Ok(None) => {
                // Session expired or invalid, create new
                match store.create().await {
                    Ok(data) => (data, true),
                    Err(e) => {
                        tracing::error!("Failed to create session: {}", e);
                        return (StatusCode::INTERNAL_SERVER_ERROR, "Session error")
                            .into_response();
                    }
                }
            }
            Err(e) => {
                tracing::error!("Failed to load session: {}", e);
                return (StatusCode::INTERNAL_SERVER_ERROR, "Session error").into_response();
            }
        },
        None => {
            // No session cookie, create new
            match store.create().await {
                Ok(data) => (data, true),
                Err(e) => {
                    tracing::error!("Failed to create session: {}", e);
                    return (StatusCode::INTERNAL_SERVER_ERROR, "Session error").into_response();
                }
            }
        }
    };

    let session = Session::new(session_data.clone(), store.clone());
    let session_id_for_cookie = session_data.id.clone();

    // Add session to request extensions
    let mut req = req;
    req.extensions_mut().insert(session.clone());

    // Run the request
    let response = next.run(req).await;

    // Save session if modified
    if let Err(e) = session.save_if_modified().await {
        tracing::error!("Failed to save session: {}", e);
    }

    // Set session cookie if new
    if is_new {
        let config = store.config();
        let cookie_value = build_cookie(
            &config.cookie_name,
            &session_id_for_cookie,
            config.ttl.as_secs() as i64,
            &config.path,
            config.domain.as_deref(),
            config.secure,
            config.http_only,
            config.same_site,
        );

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

/// Build a Set-Cookie header value.
fn build_cookie(
    name: &str,
    value: &str,
    max_age: i64,
    path: &str,
    domain: Option<&str>,
    secure: bool,
    http_only: bool,
    same_site: SameSite,
) -> String {
    let mut cookie = format!("{}={}; Max-Age={}; Path={}", name, value, max_age, path);

    if let Some(d) = domain {
        cookie.push_str(&format!("; Domain={}", d));
    }

    if secure {
        cookie.push_str("; Secure");
    }

    if http_only {
        cookie.push_str("; HttpOnly");
    }

    let same_site_str = match same_site {
        SameSite::Strict => "Strict",
        SameSite::Lax => "Lax",
        SameSite::None => "None",
    };
    cookie.push_str(&format!("; SameSite={}", same_site_str));

    cookie
}

/// Extractor for Session in route handlers.
impl<S> FromRequestParts<S> for Session
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts.extensions.get::<Session>().cloned().ok_or((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Session not found in request. Did you add SessionLayer?",
        ))
    }
}

/// Create session middleware from store.
pub fn with_session(
    store: SessionStore,
) -> impl Fn(
    Request<Body>,
    Next,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = Response> + Send>>
       + Clone
       + Send {
    let store = Arc::new(store);
    move |req: Request<Body>, next: Next| {
        let store = store.clone();
        Box::pin(async move { session_middleware(store, req, next).await })
    }
}
