use std::collections::{ HashMap, HashSet };
use std::fs;
use std::path::Path;
use anyhow::{ anyhow, Context, Result };
use regex::Regex;
use crate::build_utils::constants::{ ENDPOINT_METADATA_REGEX, HANDLER_FN_REGEX, STRUCT_REGEX };
use crate::build_utils::path_utils::{
  extract_path_params,
  generate_default_description,
  parse_path_params_from_signature,
  sanitize_operation_id,
  sanitize_tag,
  is_dynamic_segment,
};
use crate::build_utils::handler_template::generate_handler_template;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct HandlerRouteInfo {
  pub func_name: String,
  pub http_method: String,
  pub route_path: String,
  pub handler_module_path: String,
  pub route_tag: String,
}

fn enhance_response_struct(content: &str, axum_path: &str) -> String {
  let struct_regex = Regex::new(
    r"(?ms)#\[derive\([^)]*\)\]\s*pub struct (\w+Response)\s*\{\s*pub message: String,\s*\}"
  ).unwrap();

  if let Some(cap) = struct_regex.captures(content) {
    let struct_name = &cap[1];
    let enhanced_struct = if axum_path.contains("/search") {
      format!(r#"/// Response structure for search endpoints.
/// Replace `serde_json::Value` with your actual data types and implement `utoipa::ToSchema` for complex types.
#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct {} {{
    /// Success message
    pub message: String,
    /// Search results - replace with actual Vec<T> where T implements ToSchema
    pub data: Vec<serde_json::Value>,
    /// Total number of results
    pub total: Option<u64>,
    /// Current page
    pub page: Option<u32>,
    /// Results per page
    pub per_page: Option<u32>,
}}"#, struct_name)
    } else if axum_path.contains('{') || axum_path.contains("/detail") {
      format!(r#"/// Response structure for detail endpoints.
/// Replace `serde_json::Value` with your actual data type and implement `utoipa::ToSchema` for complex types.
#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct {} {{
    /// Success message
    pub message: String,
    /// Detailed data - replace with actual T where T implements ToSchema
    pub data: serde_json::Value,
}}"#, struct_name)
    } else {
      format!(r#"/// Response structure for list endpoints.
/// Replace `serde_json::Value` with your actual data types and implement `utoipa::ToSchema` for complex types.
#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct {} {{
    /// Success message
    pub message: String,
    /// List of items - replace with actual Vec<T> where T implements ToSchema
    pub data: Vec<serde_json::Value>,
    /// Total number of items
    pub total: Option<u64>,
}}"#, struct_name)
    };

    let old_struct_regex = Regex::new(
      r"(?ms)#\[derive\([^)]*\)\]\s*pub struct \w+Response\s*\{\s*pub message: String,\s*\}"
    ).unwrap();
    old_struct_regex.replace(content, &enhanced_struct).to_string()
  } else {
    content.to_string()
  }
}

// FIX: This function uses a 'match' statement for macro robustness.
fn generate_utoipa_macro(
  http_method: &str,
  route_path: &str,
  route_tag: &str,
  response_body: &str,
  route_description: &str,
  operation_id: &str,
  path_params: &[(String, String)]
) -> String {
  let method_ident = match http_method.to_uppercase().as_str() {
    "POST" => "post",
    "PUT" => "put",
    "DELETE" => "delete",
    "PATCH" => "patch",
    "HEAD" => "head",
    "OPTIONS" => "options",
    "TRACE" => "trace",
    _ => "get", // Default to GET
  };

  let params_str = if path_params.is_empty() {
    String::new()
  } else {
    let params: Vec<String> = path_params
      .iter()
      .map(|(name, typ)|
        format!(r#"("{}" = {}, Path, description = "The {} identifier")"#, name, typ, name)
      )
      .collect();
    format!(",\n    params(\n        {}\n    )", params.join(",\n        "))
  };

  format!(
    r#"#[utoipa::path(
    {}{},
    path = "{}",
    tag = "{}",
    operation_id = "{}",
    responses(
        (status = 200, description = "{}", body = {}),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]"#,
    method_ident,
    params_str,
    route_path,
    route_tag,
    operation_id,
    route_description,
    response_body
  )
}

fn inject_schemas(
  content: &str,
  handler_module_path: &str,
  schemas: &mut HashSet<String>
) -> Result<()> {
  for cap in STRUCT_REGEX.captures_iter(content) {
    let name_prefix = &cap[1];
    let suffix = &cap[2];
    // Register schema by its fully-qualified type path so we can import it in the generated mod.
    schemas.insert(format!("{}::{}{}", handler_module_path, name_prefix, suffix));
  }
  Ok(())
}

pub fn update_handler_file(
  path: &Path,
  schemas: &mut HashSet<String>,
  module_path_prefix: &str,
  root_api_path: &Path
) -> Result<Option<HandlerRouteInfo>> {
  let content = fs
    ::read_to_string(path)
    .with_context(|| format!("Failed to read file: {:?}", path))?;

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

  let func_name = &file_stem;

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

  // Overwrite ENDPOINT_TAG with hierarchical tag
  let tag_regex = Regex::new(r#"const\s+ENDPOINT_TAG:\s*&\s*str\s*=\s*"[^"]*";"#).unwrap();
  let mut content = tag_regex
    .replace(&content, &format!(r#"const ENDPOINT_TAG: &str = "{}";"#, default_tag))
    .to_string();

  // Overwrite OPERATION_ID with generated operation_id
  let operation_id_regex = Regex::new(r#"const\s+OPERATION_ID:\s*&\s*str\s*=\s*"[^"]*";"#).unwrap();
  content = operation_id_regex
    .replace(&content, &format!(r#"const OPERATION_ID: &str = "{}";"#, operation_id))
    .to_string();

  let http_method = metadata
    .get("ENDPOINT_METHOD")
    .cloned()
    .unwrap_or_else(|| "get".to_string());
  let mut route_path = metadata
    .get("ENDPOINT_PATH")
    .cloned()
    .unwrap_or_else(|| format!("/{}", file_stem));

  // Parse path params from existing function signature
  let parsed_path_params = parse_path_params_from_signature(&content);

  // Update ENDPOINT_PATH if there are path params and route_path doesn't have {param}
  if !parsed_path_params.is_empty() {
    for (param_name, _) in &parsed_path_params {
      if
        route_path.ends_with(&format!("/{}", param_name)) &&
        !route_path.contains(&format!("{{{}}}", param_name))
      {
        let new_route_path = route_path.replace(
          &format!("/{}", param_name),
          &format!("/{{{}}}", param_name)
        );
        let endpoint_path_regex = Regex::new(
          r#"const\s+ENDPOINT_PATH:\s*&\s*str\s*=\s*"[^"]*";"#
        ).unwrap();
        content = endpoint_path_regex
          .replace(&content, &format!(r#"const ENDPOINT_PATH: &str = "{}";"#, new_route_path))
          .to_string();
        route_path = new_route_path;
        break; // Assume only one param for simplicity
      }
    }
  }

  let route_tag = default_tag.clone();
  let response_body = metadata
    .get("SUCCESS_RESPONSE_BODY")
    .cloned()
    .unwrap_or_else(|| "String".to_string());
  let axum_path = Regex::new(r"\[(.*?)\]").unwrap().replace_all(&route_path, "{$1}").to_string();
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

  let actual_func_name = HANDLER_FN_REGEX.captures(&content)
    .map(|c| c[1].to_string())
    .unwrap_or_else(|| func_name.to_string());

  let openapi_route_path = if route_path == "/" {
    "/api/".to_string()
  } else {
    format!("/api{}", route_path)
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

  let path_params = if !parsed_path_params.is_empty() {
    parsed_path_params
  } else {
    extract_path_params(&axum_path)
  };
  println!("cargo:info=File: {:?}", path);
  println!("cargo:info=axum_path: {}", axum_path);
  println!("cargo:info=route_path: {}", route_path);
  println!("cargo:info=path_params: {:?}", path_params);
  let new_utoipa_macro = generate_utoipa_macro(
    &http_method,
    &openapi_route_path,
    &route_tag,
    &sanitized_response,
    &route_description,
    &operation_id,
    &path_params
  );

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
  let fn_signature = format!("pub async fn {}(", actual_func_name);
  let new_register_fn = format!(
    "pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {{\n    router.route(ENDPOINT_PATH, {}({}))\n}}",
    http_method.to_lowercase(),
    actual_func_name
  );

  let mut new_content = content.clone();

  // Remove all existing utoipa::path macros
  let macro_regex = Regex::new(r"(?s)#\[utoipa::path\(.*?\)\]\s*").unwrap();
  new_content = macro_regex.replace_all(&new_content, "").to_string();

  // Add the new macro before the function if function exists
  if let Some(pos) = new_content.find(&fn_signature) {
    let before = &new_content[..pos];
    let after = &new_content[pos..];
    new_content = format!("{}{}\n{}", before, new_utoipa_macro, after);
  }

  // Remove existing register_routes and add new one
  let register_regex = Regex::new(
    r"(?s)pub fn register_routes\(.*?\)\s*->\s*Router<Arc<AppState>>\s*\{.*?\}\s*"
  ).unwrap();
  new_content = register_regex.replace_all(&new_content, "").to_string();
  new_content = new_content.trim_end().to_string();
  new_content.push_str("\n\n");
  new_content.push_str(&new_register_fn);

  // Enhance response struct if it's basic
  new_content = enhance_response_struct(&new_content, &axum_path);

  if content != new_content {
    fs::write(path, &new_content)?;
  }

  inject_schemas(&new_content, &format!("{}::{}", module_path_prefix, file_stem), schemas)?;

  let handler_full_module_path = if
    file_stem == "index" &&
    path.parent().map_or(false, |p| is_dynamic_segment(p.file_name().unwrap().to_str().unwrap()))
  {
    let parent_dir_name = path.parent().unwrap().file_name().unwrap().to_str().unwrap();
    let sanitized_parent_mod_name = parent_dir_name
      .trim_matches(|c| (c == '[' || c == ']'))
      .replace('-', "_");
    format!("{}::{}::index", module_path_prefix, sanitized_parent_mod_name)
  } else {
    format!("{}::{}", module_path_prefix, file_stem)
  };
  Ok(
    Some(HandlerRouteInfo {
      func_name: actual_func_name,
      http_method,
      route_path,
      handler_module_path: handler_full_module_path,
      route_tag,
    })
  )
}
