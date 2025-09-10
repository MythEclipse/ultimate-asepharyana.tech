use std::collections::{ HashMap, HashSet };
use std::fs;
use std::path::Path;
use anyhow::{ anyhow, Context, Result };
use regex::Regex;

// Re-export modules for public API
pub mod param_parsing;
pub mod utoipa_generation;
pub mod response_enhancement;
pub mod schema_injection;

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
use crate::build_utils::handler_updater::utoipa_generation::generate_utoipa_macro as imported_generate_utoipa_macro;
use crate::build_utils::handler_updater::response_enhancement::enhance_response_struct;
use crate::build_utils::handler_updater::schema_injection::inject_schemas;

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

  let relative_path = path.strip_prefix(root_api_path).unwrap();
  let relative_path_no_ext = relative_path.with_extension("");
  let relative_path_str = relative_path_no_ext.to_str().unwrap();
  let tag_str = sanitize_tag(relative_path_str);
  let default_tag = if tag_str.is_empty() { "api".to_string() } else { tag_str };

  let operation_id = sanitize_operation_id(relative_path_str);

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

  let mut metadata = HashMap::new();
  for cap in ENDPOINT_METADATA_REGEX.captures_iter(&content) {
    metadata.insert(cap[1].to_string(), cap[2].to_string());
  }

  // Only set ENDPOINT_TAG if it doesn't exist
  let tag_regex = Regex::new(r#"const\s+ENDPOINT_TAG:\s*&\s*str\s*=\s*"[^"]*";"#).unwrap();
  if !tag_regex.is_match(&content) {
    content = tag_regex
      .replace(&content, &format!(r#"const ENDPOINT_TAG: &str = "{}";"#, default_tag))
      .to_string();
  }

  // Only set OPERATION_ID if it doesn't exist
  let operation_id_regex = Regex::new(r#"const\s+OPERATION_ID:\s*&\s*str\s*=\s*"[^"]*";"#).unwrap();
  if !operation_id_regex.is_match(&content) {
    content = operation_id_regex
      .replace(&content, &format!(r#"const OPERATION_ID: &str = "{}";"#, operation_id))
      .to_string();
  }

  let http_method = metadata
    .get("ENDPOINT_METHOD")
    .cloned()
    .unwrap_or_else(|| "get".to_string());
  let mut route_path = metadata
    .get("ENDPOINT_PATH")
    .cloned()
    .unwrap_or_else(|| {
      let relative_path_no_ext = path.strip_prefix(root_api_path).unwrap().with_extension("");
      relative_path_no_ext.to_str().unwrap().replace("\\", "/")
    })
    .trim_start_matches('/') // Strip all leading slashes from current path
    .to_string();

  // Prepend "/api/" to ensure consistency, but avoid duplication
  if route_path.contains("/api/api/") {
    // Replace all duplicate /api/ patterns
    route_path = route_path.replace("/api/api/", "/api/");
  }
  if !route_path.starts_with("/api/") && !route_path.starts_with("api/") {
    route_path = format!("/api/{}", route_path);
  }
  // Remove any leading "api/" without slash and replace with "/api/"
  if route_path.starts_with("api/") {
    route_path = format!("/{}", route_path);
  }

  // Parse path params from existing function signature
  let parsed_path_params = parse_path_params_from_signature(&content)?;

  // Update ENDPOINT_PATH if there are path params and route_path doesn't have {param}
  if !parsed_path_params.is_empty() {
    for (param_name, _) in &parsed_path_params {
      if
        route_path.ends_with(&format!("/{}", file_stem)) &&
        !route_path.contains(&format!("{{{}}}", param_name))
      {
        let new_route_path = route_path.replace(
          &format!("/{}", file_stem),
          &format!("/{{{}}}", param_name)
        );
        // Only update ENDPOINT_PATH for path params if it doesn't exist
        let endpoint_path_regex = Regex::new(
          r#"const\s+ENDPOINT_PATH:\s*&\s*str\s*=\s*"[^"]*";"#
        ).unwrap();
        if !endpoint_path_regex.is_match(&content) {
          content = endpoint_path_regex
            .replace(&content, &format!(r#"const ENDPOINT_PATH: &str = "{}";"#, new_route_path))
            .to_string();
        }
        route_path = new_route_path;
        // Continue to apply the correct ENDPOINT_PATH below
      }
    }
  }

  // Only generate ENDPOINT_PATH if it doesn't exist
  let endpoint_path_regex = Regex::new(
    r#"const\s+ENDPOINT_PATH:\s*&\s*str\s*=\s*"[^"]*";"#
  ).unwrap();
  if !endpoint_path_regex.is_match(&content) {
    content = endpoint_path_regex
      .replace(&content, &format!(r#"const ENDPOINT_PATH: &str = "{}";"#, route_path))
      .to_string();
  }

  let route_tag = default_tag.clone();
  let response_body = metadata
    .get("SUCCESS_RESPONSE_BODY")
    .cloned()
    .unwrap_or_else(|| "String".to_string());
  let axum_path = if route_path.contains('[') && route_path.contains(']') {
    // Legacy bracket notation
    Regex::new(r"\[(.*?)\]").unwrap().replace_all(&route_path, "{$1}").to_string()
  } else {
    // New pattern-based dynamic detection
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
  let existing_description = metadata.get("ENDPOINT_DESCRIPTION").cloned();
  let route_description = if let Some(desc) = &existing_description {
    // Prefer doc comment over generic descriptions
    if desc.starts_with("Description for the") || desc == "Description for the X endpoint" {
      doc_comment.unwrap_or_else(|| generate_default_description(&axum_path, &http_method))
    } else {
      desc.clone()
    }
  } else {
    doc_comment.unwrap_or_else(|| generate_default_description(&axum_path, &http_method))
  };

  // Only set ENDPOINT_DESCRIPTION if it doesn't exist
  let description_regex = Regex::new(
    r#"const\s+ENDPOINT_DESCRIPTION:\s*&\s*str\s*=\s*"[^"]*";"#
  ).unwrap();
  if !description_regex.is_match(&content) {
    content = description_regex
      .replace(
        &content,
        &format!(
          r#"const ENDPOINT_DESCRIPTION: &str = "{}";"#,
          route_description.replace("\"", "\\\"")
        )
      )
      .to_string();
  }

  let openapi_route_path = route_path.clone();

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

  let path_params = if !parsed_path_params.is_empty() {
    parsed_path_params
  } else {
    extract_path_params(&axum_path)
  };

  // Parse query parameters from the handler content
  let query_params = parse_query_params(&content)?;

  println!("cargo:warning=File: {:?}", path);
  println!("cargo:warning=axum_path: {}", axum_path);
  println!("cargo:warning=route_path: {}", route_path);
  println!("cargo:warning=path_params: {:?}", path_params);
  println!("cargo:warning=query_params: {:?}", query_params);

  let new_utoipa_macro = imported_generate_utoipa_macro(
    &http_method,
    &openapi_route_path,
    &route_tag,
    &sanitized_response,
    &route_description,
    &operation_id,
    &path_params,
    &query_params
  );

  println!("cargo:warning=Generated utoipa macro:");
  println!("cargo:warning={}", new_utoipa_macro);

  // Debug: Check if content will actually change
  println!("cargo:warning=Content length before: {}", content.len());

  // Find and replace utoipa macro manually
  let mut updated_content = content.clone();
  let mut utoipa_replaced = false;
  if let Some(start_pos) = content.find("#[utoipa::path(") {
    println!("cargo:warning=Found utoipa macro at position {}", start_pos);

    // Find the end by looking for the closing )] pattern
    if let Some(end_marker_pos) = content[start_pos..].find(")]") {
      let end_pos = start_pos + end_marker_pos + 2; // +2 to include the )]
      println!(
        "cargo:warning=Found closing )] at position {}, end_pos = {}",
        start_pos + end_marker_pos,
        end_pos
      );
      println!("cargo:warning=Replacing utoipa macro from {} to {}", start_pos, end_pos);
      let before = &content[..start_pos];
      let after = &content[end_pos..];
      updated_content = format!("{}{}{}", before, new_utoipa_macro, after);
      utoipa_replaced = content != updated_content;
      println!("cargo:warning=Content length after replacement: {}", updated_content.len());
      println!("cargo:warning=Utoipa macro replaced: {}", utoipa_replaced);
    } else {
      println!("cargo:warning=Could not find closing )] pattern");
    }
  } else {
    println!("cargo:warning=No utoipa macro found, adding new one");
    // Add new utoipa macro before the function
    let fn_regex = Regex::new(r"(pub async fn \w+)").unwrap();
    if let Some(cap) = fn_regex.find(&content) {
      let before_fn = &content[..cap.start()];
      let after_fn = &content[cap.start()..];
      updated_content = format!("{}{}\n{}", before_fn, new_utoipa_macro, after_fn);
      utoipa_replaced = true;
      println!("cargo:warning=Added new utoipa macro");
    }
  }

  // Update function signature if there are path params
  if !path_params.is_empty() {
    let fn_regex = Regex::new(r"(pub async fn \w+)\s*\([^)]*\)\s*->\s*impl IntoResponse").unwrap();
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

    // Ensure Path import
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
  // Only set SUCCESS_RESPONSE_BODY if it doesn't exist
  let response_body_regex = Regex::new(
    r#"const\s+SUCCESS_RESPONSE_BODY:\s*&\s*str\s*=\s*"[^"]*";"#
  ).unwrap();
  if !response_body_regex.is_match(&content) {
    content = response_body_regex
      .replace(&content, &format!(r#"const SUCCESS_RESPONSE_BODY: &str = "{}";"#, response_body))
      .to_string();
  }
  let actual_func_name = HANDLER_FN_REGEX.captures(&content)
    .map(|c| c[1].to_string())
    .unwrap_or_else(|| file_stem.to_string());
  let _fn_signature = format!("pub async fn {}(", actual_func_name);

  // Simple approach: just generate for the first handler
  let new_register_fn = format!(
    "pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {{\n    router.route(ENDPOINT_PATH, {}({}))\n}}",
    http_method.to_lowercase(),
    actual_func_name
  );

  let mut new_content = updated_content.clone();

  // Don't remove existing utoipa::path macros if we're parsing them
  // Only update if there are no utoipa macros or if we need to regenerate them

  // Remove existing register_routes and add new one
  let register_regex = Regex::new(
    r"(?s)pub fn register_routes\(.*?\)\s*->\s*Router<Arc<AppState>>\s*\{.*?\}\s*"
  ).unwrap();

  // Check if existing register_routes contains multiple routes
  if let Some(existing_register) = register_regex.find(&content) {
    let route_count = existing_register.as_str().matches(".route(").count();
    if route_count > 1 {
      // Skip updating if it already has multiple routes
      new_content = content.clone();
    } else {
      new_content = register_regex.replace_all(&new_content, "").to_string();
      new_content = new_content.trim_end().to_string();
      new_content.push_str("\n\n");
      new_content.push_str(&new_register_fn);
    }
  } else {
    new_content = new_content.trim_end().to_string();
    new_content.push_str("\n\n");
    new_content.push_str(&new_register_fn);
  }

  // Enhance response struct if it's basic
  new_content = enhance_response_struct(&new_content, &axum_path)?;

  if updated_content != new_content || utoipa_replaced {
    println!("cargo:warning=Writing updated content to file: {:?}", path);
    fs::write(path, &new_content)?;
    println!("cargo:warning=File write completed successfully");
  } else {
    println!("cargo:warning=No changes detected, file not written");
  }

  inject_schemas(&new_content, &format!("{}::{}", module_path_prefix, file_stem), schemas)?;

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
