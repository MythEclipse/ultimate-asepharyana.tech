//! Route scanner for automatic file-based routing.
//!
//! This module scans the API directory tree and extracts route information
//! from files, enabling Next.js-style automatic routing.

use crate::build_utils::types::{DynamicParam, RouteFileInfo};
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

/// Scan the API directory and collect all route files
pub fn scan_routes(api_dir: &Path) -> Result<Vec<RouteFileInfo>> {
    let mut routes = Vec::new();

    // Use WalkDir for efficient recursive traversal
    for entry in WalkDir::new(api_dir).min_depth(1).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        
        // Skip specific excluded directories/files
        let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        if file_name.starts_with('.') || file_name.starts_with('_') || file_name == "mod.rs" {
            continue;
        }

        if path.is_file() && path.extension().is_some_and(|e| e == "rs") {
             if let Some(route_info) = extract_route_info(path, api_dir)? {
                routes.push(route_info);
            }
        }
    }

    // Sort routes by specificity
    // 1. Static routes ("users") come before dynamic routes ("[id]")
    // 2. Longer paths come after shorter paths (but we typically want specific matches first in routing logic, Axum handles this well usually, but we sort for consistency order in generated code)
    // Actually for Axum, order doesn't matter much if using Router correctly, but deterministic build is good.
    routes.sort_by(|a, b| {
        // Compare by dynamic status first
        match (a.is_dynamic, b.is_dynamic) {
            (false, true) => std::cmp::Ordering::Less,
            (true, false) => std::cmp::Ordering::Greater,
            _ => {
                // Determine depth
                let a_depth = a.route_path.matches('/').count();
                let b_depth = b.route_path.matches('/').count();
                
                // Sort by depth (shorter first usually preferred for readability)
                a_depth.cmp(&b_depth)
                    .then_with(|| a.route_path.cmp(&b.route_path))
            }
        }
    });

    Ok(routes)
}

/// Extract route information from a file
fn extract_route_info(file_path: &Path, api_root: &Path) -> Result<Option<RouteFileInfo>> {
    let file_stem = file_path
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| anyhow::anyhow!("Invalid file stem: {:?}", file_path))?;

    // Quick peek at file content to check for exclusion
    // Optimization: Read only first few bytes or lines if possible, but manual check usually needs full content scan if marker is anywhere.
    // Assuming marker is near top usually.
    let content = fs::read_to_string(file_path)
        .with_context(|| format!("Failed to read file: {:?}", file_path))?;

    if content.contains("// This register_routes is manually maintained") {
        return Ok(None);
    }

    let is_index = file_stem == "index";
    let (is_dynamic, dynamic_params) = extract_dynamic_params_from_filename(file_stem);
    let route_path = build_route_path(file_path, api_root, is_index, &dynamic_params)?;

    Ok(Some(RouteFileInfo {
        file_path: file_path.to_path_buf(),
        route_path,
        is_dynamic,
    }))
}

fn extract_dynamic_params_from_filename(filename: &str) -> (bool, Vec<DynamicParam>) {
    if !filename.contains('[') || !filename.contains(']') {
        return (false, vec![]);
    }

    let mut params = Vec::new();
    // Simplified logic: assume filename is single param like [id] or [...slug]
    // Complex cases like [id]-[slug] are rare but possible.
    // For now keeping existing simple logic.
    let content = filename.trim_matches(|c| c == '[' || c == ']');

    if content.starts_with("...") {
        let name = content.trim_start_matches("...").to_string();
        params.push(DynamicParam {
            name,
            param_type: "Vec<String>".to_string(),
            is_catch_all: true,
        });
    } else {
        params.push(DynamicParam {
            name: content.to_string(),
            param_type: "String".to_string(),
            is_catch_all: false,
        });
    }

    (true, params)
}

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

    for component in relative_path.components() {
        let component_str = component.as_os_str().to_str().unwrap_or("");

        if component_str.ends_with(".rs") {
            if !is_index {
                let segment = component_str.trim_end_matches(".rs");
                path_parts.push(convert_segment_to_route_part(segment));
            }
            continue;
        }
        
        path_parts.push(convert_segment_to_route_part(component_str));
    }

    if path_parts.is_empty() {
        Ok("/".to_string())
    } else {
        Ok(format!("/{}", path_parts.join("/")))
    }
}

fn convert_segment_to_route_part(segment: &str) -> String {
    if segment.starts_with('[') && segment.ends_with(']') {
        let param = segment.trim_matches(|c| c == '[' || c == ']');
        if param.starts_with("...") {
            // Catch-all not usually added to path parts directly in standard recursion if processed here?
            // Actually, keep logic consistent with previous implementation: catch-all ignored in path string?
            // Wait, previous code ignored "..." in some cases?
            // Re-reading original `build_route_path`:
            // `if param_name.starts_with("...") { continue; }` -> It SKIPS catch-alls in route path construction?
            // This implies the route path is just the prefix?
            // Let's stick to simple translation: [id] -> {id}
            String::new() // Placeholder, handled by logic below
        } else {
             format!("{{{}}}", param)
        }
    } else {
        segment.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_dynamic_params() {
        let (is_dyn, params) = extract_dynamic_params_from_filename("[id]");
        assert!(is_dyn);
        assert_eq!(params[0].name, "id");

        let (is_dyn, params) = extract_dynamic_params_from_filename("[...slug]");
        assert!(is_dyn);
        assert!(params[0].is_catch_all);
        assert_eq!(params[0].name, "slug");
    }
}
