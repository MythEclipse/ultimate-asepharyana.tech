//! API versioning utilities.
//!
//! Provides helpers for versioned API routes.

use axum::Router;

/// API version prefix.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApiVersion {
    V1,
    V2,
}

impl ApiVersion {
    /// Get the URL prefix for this version.
    pub fn prefix(&self) -> &'static str {
        match self {
            ApiVersion::V1 => "/api/v1",
            ApiVersion::V2 => "/api/v2",
        }
    }
}

impl std::fmt::Display for ApiVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiVersion::V1 => write!(f, "v1"),
            ApiVersion::V2 => write!(f, "v2"),
        }
    }
}

/// Builder for versioned API routes.
///
/// # Example
///
/// ```ignore
/// use rust::versioning::VersionedApi;
///
/// let app = VersionedApi::new()
///     .v1(users_v1_routes())
///     .v1(products_v1_routes())
///     .v2(users_v2_routes())
///     .build();
/// ```
pub struct VersionedApi<S = ()>
where
    S: Clone + Send + Sync + 'static,
{
    v1_routes: Vec<Router<S>>,
    v2_routes: Vec<Router<S>>,
}

impl<S> VersionedApi<S>
where
    S: Clone + Send + Sync + 'static,
{
    /// Create a new versioned API builder.
    pub fn new() -> Self {
        Self {
            v1_routes: Vec::new(),
            v2_routes: Vec::new(),
        }
    }

    /// Add routes to API v1.
    pub fn v1(mut self, routes: Router<S>) -> Self {
        self.v1_routes.push(routes);
        self
    }

    /// Add routes to API v2.
    pub fn v2(mut self, routes: Router<S>) -> Self {
        self.v2_routes.push(routes);
        self
    }

    /// Build the combined router with versioned prefixes.
    pub fn build(self) -> Router<S> {
        let mut app = Router::new();

        // Merge v1 routes under /api/v1
        if !self.v1_routes.is_empty() {
            let mut v1_router = Router::new();
            for routes in self.v1_routes {
                v1_router = v1_router.merge(routes);
            }
            app = app.nest("/api/v1", v1_router);
        }

        // Merge v2 routes under /api/v2
        if !self.v2_routes.is_empty() {
            let mut v2_router = Router::new();
            for routes in self.v2_routes {
                v2_router = v2_router.merge(routes);
            }
            app = app.nest("/api/v2", v2_router);
        }

        app
    }
}

impl<S> Default for VersionedApi<S>
where
    S: Clone + Send + Sync + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}

/// Create versioned routes with automatic fallback.
///
/// Routes in v2 will be served under /api/v2.
/// Routes in v1 will be served under /api/v1 AND as fallback under /api/v2 if not overridden.
pub fn versioned_routes<S>(v1: Router<S>, v2: Router<S>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .nest("/api/v1", v1.clone())
        .nest("/api/v2", v1.merge(v2)) // v2 includes v1 as fallback
}

/// Extract API version from request path.
pub fn extract_version(path: &str) -> Option<ApiVersion> {
    if path.starts_with("/api/v2") {
        Some(ApiVersion::V2)
    } else if path.starts_with("/api/v1") {
        Some(ApiVersion::V1)
    } else {
        None
    }
}
