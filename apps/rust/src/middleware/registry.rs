//! Named middleware registry for flexible middleware management.
//!
//! Allows registering middleware by name and applying them to routes or groups.

use std::collections::HashMap;
use std::sync::Arc;

use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
};

/// A registry for named middleware functions.
///
/// This allows you to define middleware once and apply them to routes by name,
/// similar to Laravel's middleware groups.
///
/// # Example
///
/// ```ignore
/// use rustexpress::middleware::MiddlewareRegistry;
///
/// let mut registry = MiddlewareRegistry::new();
/// registry.register("auth", auth_layer);
/// registry.register("rate_limit", rate_limit_layer);
///
/// // Create a group
/// registry.create_group("api", vec!["rate_limit", "auth"]);
///
/// // Apply to router
/// let router = Router::new()
///     .route("/protected", get(handler))
///     .layer(registry.get_group_layer("api"));
/// ```
pub struct MiddlewareRegistry<S: Clone + Send + Sync + 'static> {
    /// Named middleware functions
    middleware: HashMap<&'static str, MiddlewareFn<S>>,
    /// Middleware groups (ordered list of middleware names)
    groups: HashMap<&'static str, Vec<&'static str>>,
}

/// Type alias for middleware functions.
pub type MiddlewareFn<S> = Arc<
    dyn Fn(axum::extract::State<S>, Request, Next) -> std::pin::Pin<Box<dyn std::future::Future<Output = Response> + Send>>
        + Send
        + Sync,
>;

impl<S: Clone + Send + Sync + 'static> MiddlewareRegistry<S> {
    /// Create a new empty middleware registry.
    pub fn new() -> Self {
        Self {
            middleware: HashMap::new(),
            groups: HashMap::new(),
        }
    }

    /// Register a middleware by name.
    ///
    /// The middleware function signature should be:
    /// `async fn(State<S>, Request, Next) -> Response`
    pub fn register(&mut self, name: &'static str, middleware_fn: MiddlewareFn<S>) {
        self.middleware.insert(name, middleware_fn);
    }

    /// Create a middleware group.
    ///
    /// Groups allow you to apply multiple middleware at once.
    /// Middleware are applied in the order specified.
    pub fn create_group(&mut self, name: &'static str, middleware_names: Vec<&'static str>) {
        self.groups.insert(name, middleware_names);
    }

    /// Get a middleware by name.
    pub fn get(&self, name: &'static str) -> Option<&MiddlewareFn<S>> {
        self.middleware.get(name)
    }

    /// Get all middleware names in a group.
    pub fn get_group(&self, name: &'static str) -> Option<&Vec<&'static str>> {
        self.groups.get(name)
    }

    /// Check if a middleware is registered.
    pub fn has(&self, name: &'static str) -> bool {
        self.middleware.contains_key(name)
    }

    /// Check if a group exists.
    pub fn has_group(&self, name: &'static str) -> bool {
        self.groups.contains_key(name)
    }

    /// List all registered middleware names.
    pub fn list_middleware(&self) -> Vec<&'static str> {
        self.middleware.keys().copied().collect()
    }

    /// List all middleware groups.
    pub fn list_groups(&self) -> Vec<&'static str> {
        self.groups.keys().copied().collect()
    }
}

impl<S: Clone + Send + Sync + 'static> Default for MiddlewareRegistry<S> {
    fn default() -> Self {
        Self::new()
    }
}

/// Predefined middleware group names.
pub mod groups {
    /// Middleware for public API endpoints (rate limiting, CORS)
    pub const API: &str = "api";
    /// Middleware for authenticated endpoints (auth, session)
    pub const WEB: &str = "web";
    /// Middleware for admin endpoints (auth, admin check)
    pub const ADMIN: &str = "admin";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone)]
    struct TestState;

    #[test]
    fn test_create_group() {
        let mut registry = MiddlewareRegistry::<TestState>::new();
        registry.create_group("api", vec!["rate_limit", "cors"]);

        assert!(registry.has_group("api"));
        let group = registry.get_group("api").unwrap();
        assert_eq!(group, &vec!["rate_limit", "cors"]);
    }
}
