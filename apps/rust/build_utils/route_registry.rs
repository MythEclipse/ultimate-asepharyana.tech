//! Route registry for optimal route ordering and management.
//!
//! This module provides utilities for organizing and ordering routes
//! to ensure correct matching priority (static before dynamic).

use crate::build_utils::types::RouteFileInfo;
use std::collections::HashMap;

/// Route registry that organizes routes by directory
pub struct RouteRegistry {
    /// Map of directory path to routes in that directory
    routes_by_dir: HashMap<String, Vec<RouteFileInfo>>,
}

impl RouteRegistry {
    /// Create a new route registry
    pub fn new() -> Self {
        Self {
            routes_by_dir: HashMap::new(),
        }
    }

    /// Add a route to the registry
    pub fn add_route(&mut self, route: RouteFileInfo) {
        let dir_path = route
            .file_path
            .parent()
            .and_then(|p| p.to_str())
            .unwrap_or("")
            .to_string();

        self.routes_by_dir
            .entry(dir_path)
            .or_insert_with(Vec::new)
            .push(route);
    }

    /// Get routes for a specific directory, sorted by specificity
    pub fn get_dir_routes(&self, dir_path: &str) -> Vec<&RouteFileInfo> {
        self.routes_by_dir
            .get(dir_path)
            .map(|routes| {
                let mut sorted: Vec<&RouteFileInfo> = routes.iter().collect();
                // Sort by specificity score (lower is more specific)
                sorted.sort_by_key(|r| r.specificity_score());
                sorted
            })
            .unwrap_or_default()
    }

    /// Get all directories that have routes
    pub fn get_directories(&self) -> Vec<String> {
        let mut dirs: Vec<String> = self.routes_by_dir.keys().cloned().collect();
        dirs.sort();
        dirs
    }

    /// Generate module names for routes in a directory
    pub fn get_module_names(&self, dir_path: &str) -> Vec<String> {
        self.get_dir_routes(dir_path)
            .iter()
            .filter_map(|route| route.module_name())
            .collect()
    }

    /// Generate route registration code for a directory
    pub fn generate_registration_code(&self, dir_path: &str) -> String {
        let routes = self.get_dir_routes(dir_path);
        
        if routes.is_empty() {
            return "router".to_string();
        }

        routes
            .iter()
            .rev() // Reverse for proper chaining
            .fold("router".to_string(), |acc, route| {
                if let Some(module_name) = route.module_name() {
                    format!("{}::register_routes({})", module_name, acc)
                } else {
                    acc
                }
            })
    }
}

impl Default for RouteRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use crate::build_utils::types::DynamicParam;

    #[test]
    fn test_route_ordering() {
        let mut registry = RouteRegistry::new();

        // Add routes in random order
        registry.add_route(RouteFileInfo {
            file_path: PathBuf::from("/api/users/[id].rs"),
            route_path: "/users/{id}".to_string(),
            module_path: "crate::routes::api::users::id".to_string(),
            is_index: false,
            is_dynamic: true,
            is_catch_all: false,
            dynamic_params: vec![DynamicParam {
                name: "id".to_string(),
                param_type: "String".to_string(),
                is_catch_all: false,
            }],
            http_method: "get".to_string(),
        });

        registry.add_route(RouteFileInfo {
            file_path: PathBuf::from("/api/users/profile.rs"),
            route_path: "/users/profile".to_string(),
            module_path: "crate::routes::api::users::profile".to_string(),
            is_index: false,
            is_dynamic: false,
            is_catch_all: false,
            dynamic_params: vec![],
            http_method: "get".to_string(),
        });

        let routes = registry.get_dir_routes("/api/users");
        
        // Static route should come before dynamic
        assert_eq!(routes.len(), 2);
        assert_eq!(routes[0].route_path, "/users/profile");
        assert_eq!(routes[1].route_path, "/users/{id}");
    }
}
