use anyhow::{anyhow, Context, Result};
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

mod param_parsing;
mod response_enhancement;
mod schema_injection;
mod utoipa_generation;

use crate::build_utils::constants::{ENDPOINT_METADATA_REGEX, HANDLER_FN_REGEX};
use crate::build_utils::handler_template::generate_handler_template;
use crate::build_utils::handler_updater::param_parsing::parse_query_params;
use crate::build_utils::handler_updater::response_enhancement::enhance_response_struct;
use crate::build_utils::handler_updater::schema_injection::inject_schemas;
use crate::build_utils::handler_updater::utoipa_generation::generate_utoipa_macro;
use crate::build_utils::path_utils::{
    extract_path_params, generate_default_description, parse_path_params_from_signature,
    sanitize_operation_id, sanitize_tag,
};

pub struct HandlerRouteInfo {
    pub func_name: String,
    pub handler_module_path: String,
    pub http_method: String,
    pub route_path: String,
    pub is_protected: bool,
}

pub fn update_handler_file(
    path: &Path,
    schemas: &mut HashSet<String>,
    module_path_prefix: &str,
    root_api_path: &Path,
) -> Result<Vec<HandlerRouteInfo>> {
    let initial_content = read_and_check_file(path)?;

    let content = match initial_content {
        Some(c) => c,
        None => {
            handle_empty_file(path, root_api_path)?;
            return Ok(Vec::new());
        }
    };

    let file_stem = get_file_stem(path)?;
    let doc_comment = get_doc_comment(&content);

    // Try to find multiple handlers with utoipa::path
    let mut handlers = Vec::new();
    let mut has_utoipa = false;

    for cap in crate::build_utils::constants::HANDLER_WITH_PATH_REGEX.captures_iter(&content) {
        has_utoipa = true;
        let macro_content = &cap[1];
        let func_name = &cap[2];

        // Parse individual handler metadata
        let mut metadata = HashMap::new();
        for method in ["get", "post", "put", "delete", "patch"] {
            if regex::Regex::new(&format!(r"\b{}\b", method)).unwrap().is_match(macro_content) {
                metadata.insert("ENDPOINT_METHOD".to_string(), method.to_string());
                break;
            }
        }
        let kv_regex = regex::Regex::new(r#"(path|tag|operation_id)\s*=\s*"([^"]*)""#).unwrap();
        for kv_cap in kv_regex.captures_iter(macro_content) {
             metadata.insert(match &kv_cap[1] {
                "path" => "ENDPOINT_PATH",
                "tag" => "ENDPOINT_TAG",
                "operation_id" => "OPERATION_ID",
                _ => continue,
             }.to_string(), kv_cap[2].to_string());
        }

        let http_method = metadata.get("ENDPOINT_METHOD").cloned().unwrap_or_else(|| "get".to_string());
        let route_path = metadata.get("ENDPOINT_PATH").cloned().unwrap_or_else(|| {
             let rel = path.strip_prefix(root_api_path).unwrap().with_extension("");
             let p = rel.to_string_lossy().to_string().replace("\\", "/");
             if file_stem == "index" && p.ends_with("/index") {
                 p.strip_suffix("/index").unwrap_or("/").to_string()
             } else {
                 p
             }
        });

        handlers.push(HandlerRouteInfo {
            func_name: func_name.to_string(),
            handler_module_path: format!("{}::{}", module_path_prefix, file_stem),
            http_method,
            route_path: normalize_route_path(&route_path),
            is_protected: is_handler_protected(&content),
        });
    }

    if has_utoipa {
        inject_schemas(&content, &format!("{}::{}", module_path_prefix, file_stem), schemas)?;
        let final_content = generate_and_update_register_routes(&content, &content, "get", &file_stem, &HashMap::new())?;
        write_updated_content(path, &content, &final_content, true)?;
        return Ok(handlers);
    }

    // Fallback to old single-handler logic (for legacy constants/api_route)
    let metadata_map = extract_and_normalize_metadata(&content, path, root_api_path, &file_stem, doc_comment)?;
    let metadata = parse_and_normalize_metadata_from_map(&metadata_map)?;

    let (updated_content, utoipa_replaced) = generate_and_update_utoipa_macro(
        &content,
        &metadata.http_method,
        &metadata.route_path,
        &metadata.route_tag,
        metadata.response_body.as_deref(),
        &metadata.route_description,
        &metadata.operation_id,
    )?;

    let (final_content_with_sigs, _path_params) =
        update_function_signature_with_path_params(updated_content, &content, &metadata.axum_path)?;

    let final_content_with_routes =
        generate_and_update_register_routes(&final_content_with_sigs, &content, &metadata.http_method, &file_stem, &metadata_map)?;

    let final_content = enhance_response_struct(&final_content_with_routes, &metadata.axum_path)?;

    write_updated_content(path, &content, &final_content, utoipa_replaced)?;
    inject_schemas(&final_content, &format!("{}::{}", module_path_prefix, file_stem), schemas)?;

    let res = HandlerRouteInfo {
        func_name: HANDLER_FN_REGEX
            .captures(&final_content)
            .map(|c| c[1].to_string())
            .unwrap_or_else(|| file_stem.to_string()),
        handler_module_path: format!("{}::{}", module_path_prefix, file_stem),
        http_method: metadata.http_method,
        route_path: metadata.route_path,
        is_protected: is_handler_protected(&content),
    };

    Ok(vec![res])
}

fn is_scaffolded_file(content: &str) -> bool {
    content.contains("//! Handler for the")
        && content.contains("#![allow(dead_code)]")
        && content.contains("ENDPOINT_METHOD")
        && content.contains("#[utoipa::path(")
        && content.contains("pub async fn")
        && content.contains("register_routes")
}

fn read_and_check_file(path: &Path) -> Result<Option<String>> {
    let content =
        fs::read_to_string(path).with_context(|| format!("Failed to read file: {:?}", path))?;

    if content.contains("// This register_routes is manually maintained") {
        println!(
            "cargo:warning=Skipping {:?} as it has manual register_routes",
            path
        );
        Ok(None)
    } else {
        Ok(Some(content))
    }
}

fn handle_empty_file(path: &Path, root_api_path: &Path) -> Result<()> {
    generate_handler_template(path, root_api_path)?;
    println!(
        "cargo:warning=Generated new handler template for {:?}",
        path
    );
    Ok(())
}

fn get_file_stem(path: &Path) -> Result<String> {
    Ok(path
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| anyhow!("Could not get file stem from {:?}", path))?
        .replace(['[', ']'], "")
        .replace('-', "_")
        .to_string())
}

fn get_doc_comment(content: &str) -> Option<String> {
    let register_pos = content
        .find("pub fn register_routes")
        .unwrap_or(content.len());
    let before = &content[..register_pos];
    let lines: Vec<&str> = before.lines().rev().collect();
    let mut doc_lines = Vec::new();
    for line in lines {
        if line.trim_start().starts_with("//!") {
            doc_lines.push(line.trim_start().strip_prefix("//!").unwrap_or(line).trim());
        } else if !line.trim().is_empty() {
            break;
        }
    }
    doc_lines.reverse();
    if doc_lines.is_empty() {
        None
    } else {
        Some(doc_lines.join(" "))
    }
}

struct Metadata {
    http_method: String,
    route_path: String,
    route_tag: String,
    operation_id: String,
    route_description: String,
    response_body: Option<String>,
    axum_path: String,
}

fn parse_and_normalize_metadata_from_map(
    metadata_map: &HashMap<String, String>,
) -> Result<Metadata> {
    let http_method = metadata_map.get("ENDPOINT_METHOD").cloned().unwrap();
    let route_path = metadata_map.get("ENDPOINT_PATH").cloned().unwrap();
    let route_tag = metadata_map.get("ENDPOINT_TAG").cloned().unwrap();
    let operation_id = metadata_map.get("OPERATION_ID").cloned().unwrap();
    let route_description = metadata_map.get("ENDPOINT_DESCRIPTION").cloned().unwrap();
    let response_body = metadata_map.get("SUCCESS_RESPONSE_BODY").cloned();

    let axum_path = if route_path.contains('[') && route_path.contains(']') {
        Regex::new(r"\[(.*?)\]")
            .unwrap()
            .replace_all(&route_path, "{$1}")
            .to_string()
    } else {
        let mut axum_path = route_path.clone();
        let dynamic_patterns = ["_id", "id", "slug", "uuid", "key"];
        for pattern in &dynamic_patterns {
            if route_path.ends_with(pattern) {
                let param_name = pattern.trim_start_matches('_');
                axum_path = route_path.replace(pattern, &format!("{{{}}}", param_name));
                break;
            }
        }
        axum_path
    };

    Ok(Metadata {
        http_method,
        route_path,
        route_tag,
        operation_id,
        route_description,
        response_body,
        axum_path,
    })
}

fn is_handler_protected(content: &str) -> bool {
    // Check if register_routes contains AuthMiddleware::layer()
    content.contains("AuthMiddleware::layer()") ||
  // Check if function signature contains Extension(claims): Extension<Claims>
  content.contains("Extension(claims): Extension<Claims>")
}

fn generate_and_update_utoipa_macro(
    content: &str,
    http_method: &str,
    route_path: &str,
    route_tag: &str,
    response_body: Option<&str>,
    route_description: &str,
    operation_id: &str,
) -> Result<(String, bool)> {
    let sanitized_response = response_body.map(|body| {
        if body.starts_with("Json<") {
            body.trim_start_matches("Json<")
                .trim_end_matches('>')
                .split("::")
                .last()
                .unwrap_or(body)
                .to_string()
        } else {
            body.split("::").last().unwrap_or(body).to_string()
        }
    });

    let parsed_path_params = parse_path_params_from_signature(content)?;
    let path_params = if !parsed_path_params.is_empty() {
        parsed_path_params
    } else {
        extract_path_params(route_path)
    };

    let query_params = parse_query_params(content)?;

    let is_protected = is_handler_protected(content);

    let new_utoipa_macro = generate_utoipa_macro(
        http_method,
        route_path,
        route_tag,
        sanitized_response.as_deref(),
        route_description,
        operation_id,
        &path_params,
        &query_params,
        is_protected,
    );

    let mut updated_content = content.to_string();
    let mut utoipa_replaced = false;

    if let Some(start_pos) = content.find("#[utoipa::path(") {
        if let Some(end_marker_pos) = content[start_pos..].find(")]") {
            let end_pos = start_pos + end_marker_pos + 2;
            let before = &content[..start_pos];
            let after = &content[end_pos..];
            updated_content = format!("{}{}{}", before, new_utoipa_macro, after);
            utoipa_replaced = content != updated_content;
        }
    } else {
        let fn_regex = Regex::new(r"(pub async fn \w+)").unwrap();
        if let Some(cap) = fn_regex.find(content) {
            let before_fn = &content[..cap.start()];
            let after_fn = &content[cap.start()..];
            updated_content = format!("{}{}\n{}", before_fn, new_utoipa_macro, after_fn);
            utoipa_replaced = true;
        }
    }
    Ok((updated_content, utoipa_replaced))
}

fn update_function_signature_with_path_params(
    mut content: String,
    original_content: &str,
    axum_path: &str,
) -> Result<(String, Vec<(String, String)>)> {
    let parsed_path_params = parse_path_params_from_signature(original_content)?;
    let path_params = if !parsed_path_params.is_empty() {
        parsed_path_params
    } else {
        extract_path_params(axum_path)
    };

    if !path_params.is_empty() {
        let fn_regex =
            Regex::new(r"(pub async fn \w+)\s*\([^)]*\)\s*->\s*impl IntoResponse").unwrap();
        if let Some(cap) = fn_regex.captures(&content) {
            let fn_start = &cap[1];
            let params_str = path_params
                .iter()
                .map(|(name, typ)| format!("Path({}): Path<{}>", name, typ))
                .collect::<Vec<_>>()
                .join(", ");
            let new_fn = format!("{}({}) -> impl IntoResponse", fn_start, params_str);
            content = fn_regex.replace(&content, &new_fn).to_string();
        }

        if !content.contains("extract::Path") {
            let import_regex = Regex::new(r"use axum::\{([^}]*)\};").unwrap();
            if let Some(cap) = import_regex.captures(&content) {
                let existing = &cap[1];
                let new_import = format!("extract::Path, {}", existing);
                content = import_regex
                    .replace(&content, &format!("use axum::{{{}}};", new_import))
                    .to_string();
            }
        }
    }
    Ok((content, path_params))
}

fn generate_and_update_register_routes(
    content: &str,
    original_content: &str,
    _http_method: &str,
    _file_stem: &str,
    _metadata: &HashMap<String, String>,
) -> Result<String> {

    let new_register_fn = "pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {\n    router\n}".to_string();

    let mut final_content = content.to_string();

    let register_regex =
        Regex::new(r"(?s)pub fn register_routes\(.*?\)\s*->\s*Router<Arc<AppState>>\s*\{.*\}\s*")
            .unwrap();

    if let Some(existing_register) = register_regex.find(&final_content) {
        let route_count = existing_register.as_str().matches(".route(").count();
        if route_count > 1 {
            final_content = original_content.to_string();
        } else {
            final_content = register_regex.replace_all(&final_content, "").to_string();
            final_content = final_content.trim_end().to_string();
            final_content.push_str("\n\n");
            final_content.push_str(&new_register_fn);
        }
    } else {
        final_content = final_content.trim_end().to_string();
        final_content.push_str("\n\n");
        final_content.push_str(&new_register_fn);
    }
    Ok(final_content)
}

fn write_updated_content(
    path: &Path,
    original_content: &str,
    final_content: &str,
    utoipa_replaced: bool,
) -> Result<()> {
    if original_content != final_content || utoipa_replaced {
        fs::write(path, final_content)?;
    }
    Ok(())
}

fn inject_schemas_and_return_info(
    content: &str,
    module_path_prefix: &str,
    file_stem: &str,
    schemas: &mut HashSet<String>,
    http_method: &str,
    route_path: &str,
    is_protected: bool,
) -> Result<Option<HandlerRouteInfo>> {
    inject_schemas(
        content,
        &format!("{}::{}", module_path_prefix, file_stem),
        schemas,
    )?;

    Ok(Some(HandlerRouteInfo {
        func_name: HANDLER_FN_REGEX
            .captures(content)
            .map(|c| c[1].to_string())
            .unwrap_or_else(|| file_stem.to_string()),
        handler_module_path: format!("{}::{}", module_path_prefix, file_stem),
        http_method: http_method.to_string(),
        route_path: route_path.to_string(),
        is_protected,
    }))
}

/// Extracts and normalizes metadata from handler content.
fn extract_and_normalize_metadata(
    content: &str,
    path: &Path,
    root_api_path: &Path,
    file_stem: &str,
    doc_comment: Option<String>,
) -> Result<HashMap<String, String>> {
    let mut metadata = HashMap::new();

    // Extract valid metadata
    
    // Check for #[utoipa::path(...)] macro first (Source of Truth)
    if let Some(cap) = crate::build_utils::constants::UTOIPA_PATH_REGEX.captures(content) {
        let macro_content = &cap[1];
        
        // Basic parsing for utoipa attributes
        // 1. Method (bare word)
        // 2. path = "..."
        // 3. tag = "..."
        // 4. operation_id = "..."
        // 5. responses(...) -> body = Type

        // Method finding
        for method in ["get", "post", "put", "delete", "patch"] {
            // Check if method exists as a standalone word (roughly)
            if regex::Regex::new(&format!(r"\b{}\b", method)).unwrap().is_match(macro_content) {
                metadata.insert("ENDPOINT_METHOD".to_string(), method.to_string());
                break;
            }
        }

        // Key-Value parsing
        let kv_regex = regex::Regex::new(r#"(path|tag|operation_id)\s*=\s*"([^"]*)""#).unwrap();
        for cap in kv_regex.captures_iter(macro_content) {
             let key = &cap[1];
             let val = &cap[2];
             match key {
                "path" => { metadata.insert("ENDPOINT_PATH".to_string(), val.to_string()); },
                "tag" => { metadata.insert("ENDPOINT_TAG".to_string(), val.to_string()); },
                "operation_id" => { metadata.insert("OPERATION_ID".to_string(), val.to_string()); },
                _ => {}
             }
        }

        // Responses parsing to find body
        // looking for: (status = 200, ..., body = ResponseType)
        // This is a bit complex with regex, simplified approach:
        // Find `body = Type` inside `responses(...)`
        if let Some(responses_match) = regex::Regex::new(r"responses\s*\(([\s\S]*?)\)").unwrap().find(macro_content) {
            let responses_inner = responses_match.as_str();
            // Look for body = Something associated with status = 200
            // Simplified: just find `body = ([^,)]+)`
            if let Some(body_cap) = regex::Regex::new(r"body\s*=\s*([a-zA-Z0-9_<>]+)").unwrap().captures(responses_inner) {
                metadata.insert("SUCCESS_RESPONSE_BODY".to_string(), body_cap[1].to_string());
            }
        }
    } else if let Some(cap) = crate::build_utils::constants::API_ROUTE_REGEX.captures(content) {
        let macro_content = &cap[1];
        
        // Parse comma-separated key-value pairs or bare words (like 'get')
        // Simple parser for extracting fields
        let parts: Vec<&str> = macro_content.split(',').map(|s| s.trim()).collect();
        for part in parts {
            if part.is_empty() { continue; }
            
            // Check for HTTP methods (bare words)
            let lower_part = part.to_lowercase();
            if ["get", "post", "put", "delete", "patch"].contains(&lower_part.as_str()) {
                metadata.insert("ENDPOINT_METHOD".to_string(), lower_part);
                continue;
            }

            // Parse key=value or key="value"
            if let Some((key, val)) = part.split_once('=') {
                let key = key.trim();
                let val = val.trim().trim_matches('"');
                
                match key {
                    "path" => { metadata.insert("ENDPOINT_PATH".to_string(), val.to_string()); },
                    "tag" => { metadata.insert("ENDPOINT_TAG".to_string(), val.to_string()); },
                    "operation_id" => { metadata.insert("OPERATION_ID".to_string(), val.to_string()); },
                    "description" => { metadata.insert("ENDPOINT_DESCRIPTION".to_string(), val.to_string()); },
                    "response_body" => { metadata.insert("SUCCESS_RESPONSE_BODY".to_string(), val.to_string()); },
                    _ => {} // Ignore unknown keys
                }
            }
        }
    }

    // Fallback to constants if not set (Backward Compatibility)
    for cap in ENDPOINT_METADATA_REGEX.captures_iter(content) {
        if !metadata.contains_key(&cap[1]) {
             metadata.insert(cap[1].to_string(), cap[2].to_string());
        }
    }

    let relative_path_no_ext = path.strip_prefix(root_api_path).unwrap().with_extension("");
    let relative_path_str = relative_path_no_ext.to_str().unwrap();

    // Default ENDPOINT_TAG
    let default_tag = {
        let tag_str = sanitize_tag(relative_path_str);
        if tag_str.is_empty() {
            "api".to_string()
        } else {
            tag_str
        }
    };
    metadata
        .entry("ENDPOINT_TAG".to_string())
        .or_insert(default_tag);

    // Default OPERATION_ID
    let operation_id = sanitize_operation_id(relative_path_str);
    metadata
        .entry("OPERATION_ID".to_string())
        .or_insert(operation_id);

    // Default ENDPOINT_METHOD
    metadata
        .entry("ENDPOINT_METHOD".to_string())
        .or_insert("get".to_string());

    // Default ENDPOINT_PATH
    let default_route_path = relative_path_no_ext.to_str().unwrap().replace("\\", "/");
    let mut route_path = metadata
        .entry("ENDPOINT_PATH".to_string())
        .or_insert(default_route_path)
        .clone();

    // Special handling for index.rs files
    if file_stem == "index" && route_path.ends_with("/index") {
        route_path = route_path.strip_suffix("/index").unwrap_or("/").to_string();
    }

    // Normalize route_path
    route_path = normalize_route_path(&route_path);
    metadata.insert("ENDPOINT_PATH".to_string(), route_path.clone());

    // Update ENDPOINT_PATH if there are path params and route_path doesn't have {param}
    let parsed_path_params = parse_path_params_from_signature(content)?;
    if !parsed_path_params.is_empty() {
        for (param_name, _) in &parsed_path_params {
            if route_path.ends_with(&format!("/{}", file_stem))
                && !route_path.contains(&format!("{{{}}}", param_name))
            {
                let new_route_path =
                    route_path.replace(&format!("/{}", file_stem), &format!("/{{{}}}", param_name));
                metadata.insert("ENDPOINT_PATH".to_string(), new_route_path.clone());
                route_path = new_route_path;
                break;
            }
        }
    }

    // Default ENDPOINT_DESCRIPTION
    let axum_path = if route_path.contains('[') && route_path.contains(']') {
        Regex::new(r"\[(.*?)\]")
            .unwrap()
            .replace_all(&route_path, "{$1}")
            .to_string()
    } else {
        let mut axum_path = route_path.clone();
        let dynamic_patterns = ["_id", "id", "slug", "uuid", "key"];
        for pattern in &dynamic_patterns {
            if route_path.ends_with(pattern) {
                let param_name = pattern.trim_start_matches('_');
                axum_path = route_path.replace(pattern, &format!("{{{}}}", param_name));
                break;
            }
        }
        axum_path
    };
    let http_method = metadata.get("ENDPOINT_METHOD").unwrap().clone();
    let default_description =
        doc_comment.unwrap_or_else(|| generate_default_description(&axum_path, &http_method));
    metadata
        .entry("ENDPOINT_DESCRIPTION".to_string())
        .or_insert(default_description);

    // SUCCESS_RESPONSE_BODY is now optional - no default

    Ok(metadata)
}

/// Normalizes a route path by ensuring it starts with "/api/" and removing duplicates.
fn normalize_route_path(route_path: &str) -> String {
    let mut normalized_path = route_path.to_string();

    // Replace all duplicate /api/ patterns
    while normalized_path.contains("/api/api/") {
        normalized_path = normalized_path.replace("/api/api/", "/api/");
    }

    // Ensure it starts with "/api/"
    if !(normalized_path.starts_with("/api/") || (normalized_path == "/" && route_path.is_empty()))
    {
        if normalized_path.starts_with("api/") {
            normalized_path = format!("/{}", normalized_path);
        } else {
            normalized_path = format!("/api/{}", normalized_path);
        }
    }
    normalized_path
}
