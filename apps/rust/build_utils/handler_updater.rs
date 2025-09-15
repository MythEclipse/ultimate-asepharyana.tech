use std::collections::{ HashMap, HashSet };
use std::fs;
use std::path::Path;
use anyhow::{ anyhow, Context, Result };
use regex::Regex;

use crate::build_utils::constants::{ ENDPOINT_METADATA_REGEX, HANDLER_FN_REGEX };
use crate::build_utils::path_utils::{
  extract_path_params,
  generate_default_description,
  parse_path_params_from_signature,
  sanitize_operation_id,
  sanitize_tag,
};
use crate::build_utils::handler_template::generate_handler_template;
use crate::build_utils::handler_updater::param_parsing::parse_query_params;
use crate::build_utils::handler_updater::utoipa_generation::generate_utoipa_macro;
use crate::build_utils::handler_updater::response_enhancement::enhance_response_struct;
use crate::build_utils::handler_utils::schema_injection::inject_schemas;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct HandlerRouteInfo {
  pub func_name: String,
  pub http_method: String,
  pub route_path: String,
  pub handler_module_path: String,
  pub route_tag: String,
}

pub fn update_handler_file(
  path: &Path,
  schemas: &mut HashSet<String>,
  module_path_prefix: &str,
  root_api_path: &Path
) -> Result<Option<HandlerRouteInfo>> {
  let mut content = fs
    ::read_to_string(path)
    .with_context(|| format!("Failed to read file: {:?}", path))?;

  // Check if the file has a comment indicating manual maintenance
  if content.contains("// This register_routes is manually maintained") {
    println!("cargo:warning=Skipping {:?} as it has manual register_routes", path);
    return Ok(None);
  }

  if content.trim().is_empty() {
    generate_handler_template(path, root_api_path)?;
    println!("cargo:warning=Generated new handler template for {:?}", path);
    return Ok(None);
  }

  let file_stem = path
    .file_stem()
    .and_then(|s| s.to_str())
    .ok_or_else(|| anyhow!("Could not get file stem from {:?}", path))?
    .replace(['[', ']'], "")
    .replace('-', "_");

  let doc_comment = {
    let register_pos = content.find("pub fn register_routes").unwrap_or(content.len());
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
  };

  let metadata = extract_and_normalize_metadata(&content, path, root_api_path, &file_stem, doc_comment)?;

  let http_method = metadata.get("ENDPOINT_METHOD").cloned().unwrap();
  let route_path = metadata.get("ENDPOINT_PATH").cloned().unwrap();
  let route_tag = metadata.get("ENDPOINT_TAG").cloned().unwrap();
  let operation_id = metadata.get("OPERATION_ID").cloned().unwrap();
  let route_description = metadata.get("ENDPOINT_DESCRIPTION").cloned().unwrap();
  let response_body = metadata.get("SUCCESS_RESPONSE_BODY").cloned().unwrap();

  let axum_path = if route_path.contains('[') && route_path.contains(']') {
      Regex::new(r"\[(.*?)\]").unwrap().replace_all(&route_path, "{$1}").to_string()
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

  // Sanitize response body for utoipa: strip Json<> wrapper and module path, keep only the type name.
  let sanitized_response = if response_body.starts_with("Json<") {
    response_body
      .trim_start_matches("Json<")
      .trim_end_matches('>')
      .split("::")
      .last()
      .unwrap_or(&response_body)
      .to_string()
  } else {
    response_body.split("::").last().unwrap_or(&response_body).to_string()
  };

  let parsed_path_params = parse_path_params_from_signature(&content)?;
  let path_params = if !parsed_path_params.is_empty() {
    parsed_path_params
  } else {
    extract_path_params(&axum_path)
  };

  // Parse query parameters from the handler content
  let query_params = parse_query_params(&content)?;

  let new_utoipa_macro = generate_utoipa_macro(
    &http_method,
    &route_path, // Use route_path here as it's the normalized one
    &route_tag,
    &sanitized_response,
    &route_description,
    &operation_id,
    &path_params,
    &query_params
  );

  let mut updated_content = content.clone();
  let mut utoipa_replaced = false;

  if let Some(start_pos) = content.find("#[utoipa::path(") {
    // Find the end by looking for the closing )] pattern
    if let Some(end_marker_pos) = content[start_pos..].find(")]") {
      let end_pos = start_pos + end_marker_pos + 2; // +2 to include the )]
      let before = &content[..start_pos];
      let after = &content[end_pos..];
      updated_content = format!("{}{}{}", before, new_utoipa_macro, after);
      utoipa_replaced = content != updated_content;
    }
  } else {
    // Add new utoipa macro before the function
    let fn_regex = Regex::new(r"(pub async fn \w+)").unwrap();
    if let Some(cap) = fn_regex.find(&content) {
      let before_fn = &content[..cap.start()];
      let after_fn = &content[cap.start()..];
      updated_content = format!("{}{}\n{}", before_fn, new_utoipa_macro, after_fn);
      utoipa_replaced = true;
    }
  }

  // Update function signature if there are path params
  if !path_params.is_empty() {
    let fn_regex = Regex::new(r"(pub async fn \w+)\s*\([^)]*\)\s*->\s*impl IntoResponse").unwrap();
    if let Some(cap) = fn_regex.captures(&updated_content) {
      let fn_start = &cap[1];
      let params_str = path_params
        .iter()
        .map(|(name, typ)| format!("Path({}): Path<{}>", name, typ))
        .collect::<Vec<_>>()
        .join(", ");
      let new_fn = format!("{}({}) -> impl IntoResponse", fn_start, params_str);
      updated_content = fn_regex.replace(&updated_content, &new_fn).to_string();
    }

    // Ensure Path import
    if !updated_content.contains("extract::Path") {
      let import_regex = Regex::new(r"use axum::\{([^}]*)\};").unwrap();
      if let Some(cap) = import_regex.captures(&updated_content) {
        let existing = &cap[1];
        let new_import = format!("extract::Path, {}", existing);
        updated_content = import_regex
          .replace(&updated_content, &format!("use axum::{{{}}};", new_import))
          .to_string();
      }
    }
  }

  let actual_func_name = HANDLER_FN_REGEX.captures(&updated_content)
    .map(|c| c[1].to_string())
    .unwrap_or_else(|| file_stem.to_string());

  let new_register_fn = format!(
    "pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {{\n    router.route(ENDPOINT_PATH, {}({}))\n}}",
    http_method.to_lowercase(),
    actual_func_name
  );

  let mut final_content = updated_content.clone();

  let register_regex = Regex::new(
    r"(?s)pub fn register_routes\(.*?\)\s*->\s*Router<Arc<AppState>>\s*\{.*?\}\s*"
  ).unwrap();

  // Check if existing register_routes contains multiple routes
  if let Some(existing_register) = register_regex.find(&final_content) {
    let route_count = existing_register.as_str().matches(".route(").count();
    if route_count > 1 {
      // Skip updating if it already has multiple routes
      final_content = content.clone();
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

  // Enhance response struct if it's basic
  final_content = enhance_response_struct(&final_content, &axum_path)?;

  if content != final_content || utoipa_replaced {
    fs::write(path, &final_content)?;
  }

  inject_schemas(&final_content, &format!("{}::{}", module_path_prefix, file_stem), schemas)?;

  Ok(
    Some(HandlerRouteInfo {
      func_name: actual_func_name,
      http_method,
      route_path,
      handler_module_path: format!("{}::{}", module_path_prefix, file_stem),
      route_tag,
    })
  )
}

#[allow(dead_code)]
fn update_uploader_file(
  path: &Path,
  schemas: &mut HashSet<String>,
  module_path_prefix: &str,
  _root_api_path: &Path
) -> Result<Option<HandlerRouteInfo>> {
  let content = fs
    ::read_to_string(path)
    .with_context(|| format!("Failed to read file: {:?}", path))?;

  // Check if register_routes already exists and has multiple routes
  let register_regex = Regex::new(
    r"(?s)pub fn register_routes\(.*?\)\s*->\s*Router<Arc<AppState>>\s*\{.*?\}\s*"
  ).unwrap();

  if let Some(existing_register) = register_regex.find(&content) {
    let route_count = existing_register.as_str().matches(".route(").count();
    if route_count > 1 {
      // Already has multiple routes, don't modify
      inject_schemas(&content, &format!("{}::uploader", module_path_prefix), schemas)?;
      return Ok(
        Some(HandlerRouteInfo {
          func_name: "upload_file".to_string(),
          http_method: "post".to_string(),
          route_path: "/uploader".to_string(),
          handler_module_path: format!("{}::uploader", module_path_prefix),
          route_tag: "uploader".to_string(),
        })
      );
    }
  }

  // Generate register_routes for uploader with both routes
  let new_register_fn =
    r#"pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
  router
    .route("/uploader", post(upload_file))
    .route("/{file_name}", get(download_file))
}"#;

  let mut new_content = content.clone();
  new_content = register_regex.replace_all(&new_content, "").to_string();
  new_content = new_content.trim_end().to_string();
  new_content.push_str("\n\n");
  new_content.push_str(new_register_fn);

  if content != new_content {
    fs::write(path, &new_content)?;
  }

  inject_schemas(&new_content, &format!("{}::uploader", module_path_prefix), schemas)?;

  Ok(
    Some(HandlerRouteInfo {
      func_name: "upload_file".to_string(),
      http_method: "post".to_string(),
      route_path: "/uploader".to_string(),
      handler_module_path: format!("{}::uploader", module_path_prefix),
      route_tag: "uploader".to_string(),
    })
  )
}

/// Extracts and normalizes metadata from handler content.
fn extract_and_normalize_metadata(
    content: &str,
    path: &Path,
    root_api_path: &Path,
    file_stem: &str,
    doc_comment: Option<String>
) -> Result<HashMap<String, String>> {
    let mut metadata = HashMap::new();

    // Extract existing metadata
    for cap in ENDPOINT_METADATA_REGEX.captures_iter(content) {
        metadata.insert(cap[1].to_string(), cap[2].to_string());
    }

    let relative_path_no_ext = path.strip_prefix(root_api_path).unwrap().with_extension("");
    let relative_path_str = relative_path_no_ext.to_str().unwrap();

    // Default ENDPOINT_TAG
    let default_tag = {
        let tag_str = sanitize_tag(relative_path_str);
        if tag_str.is_empty() { "api".to_string() } else { tag_str }
    };
    metadata.entry("ENDPOINT_TAG".to_string()).or_insert(default_tag);

    // Default OPERATION_ID
    let operation_id = sanitize_operation_id(relative_path_str);
    metadata.entry("OPERATION_ID".to_string()).or_insert(operation_id);

    // Default ENDPOINT_METHOD
    metadata.entry("ENDPOINT_METHOD".to_string()).or_insert("get".to_string());

    // Default ENDPOINT_PATH
    let default_route_path = relative_path_no_ext.to_str().unwrap().replace("\\", "/");
    let mut route_path = metadata.entry("ENDPOINT_PATH".to_string()).or_insert(default_route_path).clone();

    // Normalize route_path
    route_path = normalize_route_path(&route_path);
    metadata.insert("ENDPOINT_PATH".to_string(), route_path.clone());

    // Update ENDPOINT_PATH if there are path params and route_path doesn't have {param}
    let parsed_path_params = parse_path_params_from_signature(content)?;
    if !parsed_path_params.is_empty() {
        for (param_name, _) in &parsed_path_params {
            if route_path.ends_with(&format!("/{}", file_stem)) && !route_path.contains(&format!("{{{}}}", param_name)) {
                let new_route_path = route_path.replace(&format!("/{}", file_stem), &format!("/{{{}}}", param_name));
                metadata.insert("ENDPOINT_PATH".to_string(), new_route_path.clone());
                route_path = new_route_path;
                break;
            }
        }
    }

    // Default ENDPOINT_DESCRIPTION
    let axum_path = if route_path.contains('[') && route_path.contains(']') {
        Regex::new(r"\[(.*?)\]").unwrap().replace_all(&route_path, "{$1}").to_string()
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
    let default_description = doc_comment.unwrap_or_else(|| generate_default_description(&axum_path, &http_method));
    metadata.entry("ENDPOINT_DESCRIPTION".to_string()).or_insert(default_description);

    // Default SUCCESS_RESPONSE_BODY
    metadata.entry("SUCCESS_RESPONSE_BODY".to_string()).or_insert("String".to_string());

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
    if !normalized_path.starts_with("/api/") {
        if normalized_path.starts_with("api/") {
            normalized_path = format!("/{}", normalized_path);
        } else {
            normalized_path = format!("/api/{}", normalized_path);
        }
    }
    normalized_path
}
