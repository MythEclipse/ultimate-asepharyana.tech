//! Authorization gate for policy-based access control.

use super::role::Role;
use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Trait for entities that can be authorized.
pub trait Authorizable: Send + Sync {
    /// Get the user's ID.
    fn user_id(&self) -> String;

    /// Get the user's roles.
    fn roles(&self) -> Vec<String>;

    /// Get the user's direct permissions.
    fn permissions(&self) -> Vec<String>;

    /// Check if user has a specific role.
    fn has_role(&self, role: &str) -> bool {
        self.roles().contains(&role.to_string())
    }

    /// Check if user has any of the roles.
    fn has_any_role(&self, roles: &[&str]) -> bool {
        roles.iter().any(|r| self.has_role(r))
    }

    /// Check if user has a direct permission.
    fn has_permission(&self, permission: &str) -> bool {
        self.permissions().contains(&permission.to_string())
    }

    /// Check if user is super admin.
    fn is_super_admin(&self) -> bool {
        self.has_role("super_admin") || self.has_permission("*")
    }
}

/// Policy function type.
pub type PolicyFn<T> = Box<dyn Fn(&T) -> bool + Send + Sync>;

/// Authorization gate for managing policies.
pub struct Gate<T: Authorizable> {
    policies: Arc<RwLock<HashMap<String, PolicyFn<T>>>>,
    roles: Arc<RwLock<HashMap<String, Role>>>,
}

impl<T: Authorizable> Default for Gate<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Authorizable> Gate<T> {
    /// Create a new gate.
    pub fn new() -> Self {
        Self {
            policies: Arc::new(RwLock::new(HashMap::new())),
            roles: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Define a policy.
    pub async fn define<F>(&self, ability: &str, policy: F)
    where
        F: Fn(&T) -> bool + Send + Sync + 'static,
    {
        self.policies
            .write()
            .await
            .insert(ability.to_string(), Box::new(policy));
    }

    /// Register a role.
    pub async fn register_role(&self, role: Role) {
        self.roles.write().await.insert(role.name.clone(), role);
    }

    /// Check if user is allowed to perform an ability.
    pub async fn allows(&self, user: &T, ability: &str) -> bool {
        // Super admins can do anything
        if user.is_super_admin() {
            return true;
        }

        // Check direct permissions
        if user.has_permission(ability) {
            return true;
        }

        // Check wildcard permissions
        if let Some(resource) = ability.split('.').next() {
            if user.has_permission(&format!("{}.*", resource)) {
                return true;
            }
        }

        // Check role-based permissions
        let roles = self.roles.read().await;
        for role_name in user.roles() {
            if let Some(role) = roles.get(&role_name) {
                if role.has_permission(ability) || role.has_permission("*") {
                    return true;
                }
                // Check wildcard in role
                if let Some(resource) = ability.split('.').next() {
                    if role.has_permission(&format!("{}.*", resource)) {
                        return true;
                    }
                }
            }
        }

        // Check custom policy
        if let Some(policy) = self.policies.read().await.get(ability) {
            return policy(user);
        }

        false
    }

    /// Check if user is denied.
    pub async fn denies(&self, user: &T, ability: &str) -> bool {
        !self.allows(user, ability).await
    }

    /// Authorize or return error.
    pub async fn authorize(&self, user: &T, ability: &str) -> Result<(), AuthorizationError> {
        if self.allows(user, ability).await {
            Ok(())
        } else {
            Err(AuthorizationError::Forbidden(ability.to_string()))
        }
    }
}

/// Authorization error.
#[derive(Debug, thiserror::Error)]
pub enum AuthorizationError {
    #[error("Forbidden: {0}")]
    Forbidden(String),
    #[error("Unauthenticated")]
    Unauthenticated,
}

impl IntoResponse for AuthorizationError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AuthorizationError::Forbidden(ability) => (
                StatusCode::FORBIDDEN,
                format!("You are not authorized to: {}", ability),
            ),
            AuthorizationError::Unauthenticated => (
                StatusCode::UNAUTHORIZED,
                "Authentication required".to_string(),
            ),
        };

        (status, message).into_response()
    }
}

/// Policy trait for resource-based authorization.
#[async_trait::async_trait]
pub trait Policy<U: Authorizable, R>: Send + Sync {
    /// Check if user can view the resource.
    async fn view(&self, _user: &U, _resource: &R) -> bool {
        false
    }

    /// Check if user can create resources.
    async fn create(&self, _user: &U) -> bool {
        false
    }

    /// Check if user can update the resource.
    async fn update(&self, _user: &U, _resource: &R) -> bool {
        false
    }

    /// Check if user can delete the resource.
    async fn delete(&self, _user: &U, _resource: &R) -> bool {
        false
    }
}

/// Middleware to require a specific permission.
pub fn require_permission(
    permission: &'static str,
) -> impl Fn(Request, Next) -> std::pin::Pin<Box<dyn std::future::Future<Output = Response> + Send>>
       + Clone
       + Send {
    move |req: Request, next: Next| {
        Box::pin(async move {
            // Check if user has permission in request extensions
            // This assumes auth middleware has set the user
            if let Some(user) = req.extensions().get::<SimpleUser>() {
                if user.has_permission(permission) || user.is_super_admin() {
                    return next.run(req).await;
                }
                if let Some(resource) = permission.split('.').next() {
                    if user.has_permission(&format!("{}.*", resource)) {
                        return next.run(req).await;
                    }
                }
            }

            AuthorizationError::Forbidden(permission.to_string()).into_response()
        })
    }
}

/// Simple user struct for demonstration.
#[derive(Debug, Clone, Serialize)]
pub struct SimpleUser {
    pub id: String,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
}

impl Authorizable for SimpleUser {
    fn user_id(&self) -> String {
        self.id.clone()
    }

    fn roles(&self) -> Vec<String> {
        self.roles.clone()
    }

    fn permissions(&self) -> Vec<String> {
        self.permissions.clone()
    }
}
