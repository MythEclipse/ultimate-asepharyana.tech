//! Permission definition.

use serde::{Deserialize, Serialize};

/// A permission that can be granted to roles.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Permission {
    /// Permission name (e.g., "posts.create", "users.delete").
    pub name: String,
    /// Human-readable description.
    pub description: Option<String>,
    /// Resource this permission applies to.
    pub resource: Option<String>,
    /// Action this permission allows.
    pub action: Option<String>,
}

impl Permission {
    /// Create a new permission.
    pub fn new(name: &str) -> Self {
        let parts: Vec<&str> = name.split('.').collect();
        let (resource, action) = if parts.len() >= 2 {
            (Some(parts[0].to_string()), Some(parts[1].to_string()))
        } else {
            (None, None)
        };

        Self {
            name: name.to_string(),
            description: None,
            resource,
            action,
        }
    }

    /// Create with description.
    pub fn with_description(name: &str, description: &str) -> Self {
        let mut p = Self::new(name);
        p.description = Some(description.to_string());
        p
    }

    /// Check if this permission matches a pattern.
    /// Supports wildcards: "posts.*" matches "posts.create", "posts.delete", etc.
    pub fn matches(&self, pattern: &str) -> bool {
        if pattern == "*" {
            return true;
        }

        if pattern.ends_with(".*") {
            let prefix = &pattern[..pattern.len() - 2];
            return self.name.starts_with(prefix);
        }

        self.name == pattern
    }
}

/// Common permission patterns.
pub mod permissions {
    /// CRUD permissions for a resource.
    pub fn crud(resource: &str) -> Vec<String> {
        vec![
            format!("{}.view", resource),
            format!("{}.create", resource),
            format!("{}.update", resource),
            format!("{}.delete", resource),
        ]
    }

    /// Read-only permissions.
    pub fn readonly(resource: &str) -> Vec<String> {
        vec![format!("{}.view", resource)]
    }

    /// Full access to a resource.
    pub fn full_access(resource: &str) -> String {
        format!("{}.*", resource)
    }
}
