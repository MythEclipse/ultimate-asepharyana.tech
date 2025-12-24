//! Route scanner for automatic file-based routing.
//!
//! This module scans the API directory tree and extracts route information
//! from files, enabling Next.js-style automatic routing.

use crate::build_utils::types::{DynamicParam, RouteFileInfo};
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

/// Scan the API directory and collect all route files
pub fn scan_routes(api_dir: &Path) -> Result<Vec<RouteFileInfo>> {
    let mut routes = Vec::new();
    scan_directory_recursive(api_dir, api_dir, &mut routes)?;

    // Sort routes by specificity (static before dynamic, shorter before longer)
    routes.sort_by(|a, b| {
        // Static routes before dynamic
        match (a.is_dynamic, b.is_dynamic) {
            (false, true) => std::cmp::Ordering::Less,
            (true, false) => std::cmp::Ordering::Greater,
            _ => {
                // For same type, sort by path length (shorter first)
                let a_len = a.route_path.matches('/').count();
                let b_len = b.route_path.matches('/').count();
                a_len
                    .cmp(&b_len)
                    .then_with(|| a.route_path.cmp(&b.route_path))
            }
        }
    });

    Ok(routes)
}

/// Recursively scan a directory for route files
fn scan_directory_recursive(
    current_dir: &Path,
    api_root: &Path,
    routes: &mut Vec<RouteFileInfo>,
) -> Result<()> {
    let entries = fs::read_dir(current_dir)
        .with_context(|| format!("Failed to read directory: {:?}", current_dir))?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

        // Skip hidden files, mod.rs, and private modules
        if file_name.starts_with('.') || file_name.starts_with('_') || file_name == "mod.rs" {
            continue;
        }

        if path.is_dir() {
            // Recursively scan subdirectories
            scan_directory_recursive(&path, api_root, routes)?;
        } else if path.is_file() && path.extension().is_some_and(|e| e == "rs") {
            // Process route file
            if let Some(route_info) = extract_route_info(&path, api_root)? {
                routes.push(route_info);
            }
        }
    }

    Ok(())
}

/// Extract route information from a file
fn extract_route_info(file_path: &Path, api_root: &Path) -> Result<Option<RouteFileInfo>> {
    let file_stem = file_path
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| anyhow::anyhow!("Invalid file stem: {:?}", file_path))?;

    // Read file to get metadata
    let content = fs::read_to_string(file_path)
        .with_context(|| format!("Failed to read file: {:?}", file_path))?;

    // Check if file should be skipped (manual register_routes)
    if content.contains("// This register_routes is manually maintained") {
        return Ok(None);
    }


    // Determine if this is an index file
    let is_index = file_stem == "index";

    // Check if filename indicates dynamic route
    let (is_dynamic, dynamic_params) = extract_dynamic_params_from_filename(file_stem);

    // Build route path from file location
    let route_path = build_route_path(file_path, api_root, is_index, &dynamic_params)?;


    Ok(Some(RouteFileInfo {
        file_path: file_path.to_path_buf(),
        route_path,
        is_dynamic,
    }))
}

/// Extract HTTP method from file content

/// Extract dynamic parameters from filename
/// Examples:
/// - "[id].rs" → (true, [DynamicParam { name: "id", is_catch_all: false }])
/// - "[...slug].rs" → (true, [DynamicParam { name: "slug", is_catch_all: true }])
/// - "users.rs" → (false, [])
fn extract_dynamic_params_from_filename(filename: &str) -> (bool, Vec<DynamicParam>) {
    if !filename.contains('[') || !filename.contains(']') {
        return (false, vec![]);
    }

    let mut params = Vec::new();
    let content = filename.trim_matches(|c| c == '[' || c == ']');

    if content.starts_with("...") {
        // Catch-all parameter
        let name = content.trim_start_matches("...").to_string();
        params.push(DynamicParam {
            name,
            param_type: "Vec<String>".to_string(),
            is_catch_all: true,
        });
    } else {
        // Regular dynamic parameter
        params.push(DynamicParam {
            name: content.to_string(),
            param_type: "String".to_string(),
            is_catch_all: false,
        });
    }

    (true, params)
}

/// Build route path from file location
/// Examples:
/// - "users/index.rs" → "/users"
/// - "users/[id].rs" → "/users/{id}"
/// - "users/[id]/posts.rs" → "/users/{id}/posts"
/// - "index.rs" → "/"
fn build_route_path(
    file_path: &Path,
    api_root: &Path,
    is_index: bool,
    _dynamic_params: &[DynamicParam],
) -> Result<String> {
    let relative_path = file_path
        .strip_prefix(api_root)
        .context("File path is not under api_root")?;

    let mut path_parts: Vec<String> = Vec::new();

    // Process each component of the path
    for component in relative_path.components() {
        let component_str = component.as_os_str().to_str().unwrap_or("");

        // Skip the filename itself
        if component_str.ends_with(".rs") {
            if !is_index {
                // Add filename as path segment (unless it's index.rs)
                let segment = component_str.trim_end_matches(".rs");

                // Check if this segment is dynamic
                if segment.starts_with('[') && segment.ends_with(']') {
                    let param_name = segment.trim_matches(|c| c == '[' || c == ']');
                    if param_name.starts_with("...") {
                        // Catch-all - don't add to path, it's handled specially
                        continue;
                    } else {
                        path_parts.push(format!("{{{}}}", param_name.trim_start_matches("...")));
                    }
                } else {
                    path_parts.push(segment.to_string());
                }
            }
            continue;
        }

        // Check if directory name contains dynamic segments
        if component_str.starts_with('[') && component_str.ends_with(']') {
            let param_name = component_str.trim_matches(|c| c == '[' || c == ']');
            path_parts.push(format!("{{{}}}", param_name));
        } else {
            path_parts.push(component_str.to_string());
        }
    }

    // Build final path
    let route_path = if path_parts.is_empty() {
        "/".to_string()
    } else {
        format!("/{}", path_parts.join("/"))
    };

    Ok(route_path)
}

/// Build module path from file location
/// Examples:
/// - "users/index.rs" → "crate::routes::api::users::index"
/// - "users/[id].rs" → "crate::routes::api::users::id"


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_dynamic_params() {
        let (is_dyn, params) = extract_dynamic_params_from_filename("[id]");
        assert!(is_dyn);
        assert_eq!(params.len(), 1);
        assert_eq!(params[0].name, "id");
        assert!(!params[0].is_catch_all);

        let (is_dyn, params) = extract_dynamic_params_from_filename("[...slug]");
        assert!(is_dyn);
        assert_eq!(params.len(), 1);
        assert_eq!(params[0].name, "slug");
        assert!(params[0].is_catch_all);

        let (is_dyn, _params) = extract_dynamic_params_from_filename("users");
        assert!(!is_dyn);
    }
}
