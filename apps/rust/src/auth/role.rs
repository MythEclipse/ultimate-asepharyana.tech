//! Role definition for RBAC.

use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// A role with associated permissions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    /// Role name (e.g., "admin", "editor", "user").
    pub name: String,
    /// Human-readable description.
    pub description: Option<String>,
    /// Permissions granted to this role.
    pub permissions: HashSet<String>,
}

impl Role {
    /// Create a new role.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            description: None,
            permissions: HashSet::new(),
        }
    }

    /// Create a role with description.
    pub fn with_description(name: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            description: Some(description.to_string()),
            permissions: HashSet::new(),
        }
    }

    /// Add a permission to this role.
    pub fn grant(mut self, permission: &str) -> Self {
        self.permissions.insert(permission.to_string());
        self
    }

    /// Add multiple permissions.
    pub fn grant_many(mut self, permissions: &[&str]) -> Self {
        for p in permissions {
            self.permissions.insert(p.to_string());
        }
        self
    }

    /// Check if role has a permission.
    pub fn has_permission(&self, permission: &str) -> bool {
        self.permissions.contains(permission)
    }

    /// Check if role has any of the given permissions.
    pub fn has_any_permission(&self, permissions: &[&str]) -> bool {
        permissions.iter().any(|p| self.has_permission(p))
    }

    /// Check if role has all of the given permissions.
    pub fn has_all_permissions(&self, permissions: &[&str]) -> bool {
        permissions.iter().all(|p| self.has_permission(p))
    }
}

/// Common predefined roles.
pub mod roles {
    use super::Role;

    /// Super admin with all permissions.
    pub fn super_admin() -> Role {
        Role::with_description("super_admin", "Super administrator with full access").grant("*")
    }

    /// Regular admin.
    pub fn admin() -> Role {
        Role::with_description("admin", "Administrator").grant_many(&[
            "users.view",
            "users.create",
            "users.update",
            "users.delete",
            "posts.view",
            "posts.create",
            "posts.update",
            "posts.delete",
            "settings.view",
            "settings.update",
        ])
    }

    /// Editor role.
    pub fn editor() -> Role {
        Role::with_description("editor", "Content editor").grant_many(&[
            "posts.view",
            "posts.create",
            "posts.update",
        ])
    }

    /// Regular user.
    pub fn user() -> Role {
        Role::with_description("user", "Regular user").grant_many(&[
            "posts.view",
            "profile.view",
            "profile.update",
        ])
    }

    /// Guest (unauthenticated).
    pub fn guest() -> Role {
        Role::with_description("guest", "Unauthenticated user").grant("posts.view")
    }
}
