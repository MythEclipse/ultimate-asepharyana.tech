use std::collections::{ HashMap, HashSet };
use std::fs;
use std::path::{ Path, PathBuf };

use anyhow::{ anyhow, Context, Result };
use once_cell::sync::Lazy;
use regex::Regex;

static HANDLER_FN_REGEX: Lazy<Regex> = Lazy::new(|| {
  Regex::new(r"pub async fn\s+([a-zA-Z0-9_]+)\s*\(").unwrap()
});

static ENDPOINT_METADATA_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
      r#"const\s+(ENDPOINT_METHOD|ENDPOINT_PATH|ENDPOINT_DESCRIPTION|ENDPOINT_TAG|OPERATION_ID|SUCCESS_RESPONSE_BODY):\s*&\s*str\s*=\s*"([^"]*)";"#
    ).unwrap()
  });

static DYNAMIC_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\[([^\]]+)\]").unwrap());

fn extract_path_params(axum_path: &str) -> Vec<(String, String)> {
    let re = Regex::new(r"\{([^}]+)\}").unwrap();
    re.captures_iter(axum_path)
        .map(|cap| {
            let param = &cap[1];
            if param.starts_with("...") {
                (param.strip_prefix("...").unwrap().to_string(), "Vec<String>".to_string())
            } else {
                (param.to_string(), "String".to_string())
            }
        })
        .collect()
}

fn parse_path_params_from_signature(content: &str) -> Vec<(String, String)> {
    let fn_regex = Regex::new(r"pub async fn \w+\s*\(([^)]*)\)\s*->").unwrap();
    if let Some(cap) = fn_regex.captures(content) {
        let params = &cap[1];
        let param_regex = Regex::new(r"Path\((\w+)\):\s*Path<([^>]+)>").unwrap();
        param_regex.captures_iter(params).map(|cap| {
            let name = cap[1].to_string();
            let typ = cap[2].to_string();
            (name, typ)
        }).collect()
    } else {
        vec![]
    }
}

fn generate_default_description(path_str: &str, method: &str) -> String {
    let path_segments: Vec<&str> = path_str.trim_matches('/').split('/').collect();
    let last_segment = path_segments.last().unwrap_or(&"");
    let second_last_segment = if path_segments.len() > 1 {
        path_segments[path_segments.len() - 2]
    } else {
        ""
    };

    match *last_segment {
        "search" => format!("Searches for {} based on query parameters.", second_last_segment),
        "detail" => format!("Retrieves details for a specific {} by ID.", second_last_segment),
        "list" => format!("Retrieves a list of {}.", second_last_segment),
        "[slug]" => format!("Retrieves details for a specific {} by slug.", second_last_segment),
        "[[...file]]" => format!("Handles file operations (upload/download) for {}.", second_last_segment),
        _ => format!("Handles {} requests for the {} endpoint.", method.to_uppercase(), path_str),
    }
}

fn sanitize_operation_id(path_str: &str) -> String {
    let s = path_str.replace(std::path::MAIN_SEPARATOR, "_").replace('-', "_");
    let s = DYNAMIC_REGEX.replace_all(&s, |caps: &regex::Captures| {
        let inner = &caps[1];
        if inner.starts_with("...") {
            "_catch_all".to_string()
        } else {
            format!("_{}", inner)
        }
    }).to_string();
    s.trim_matches('_').replace("__", "_")
}

fn enhance_response_struct(content: &str, axum_path: &str) -> String {
    let struct_regex = Regex::new(r"(?ms)#\[derive\([^)]*\)\]\s*pub struct (\w+Response)\s*\{\s*pub message: String,\s*\}").unwrap();

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

        let old_struct_regex = Regex::new(r"(?ms)#\[derive\([^)]*\)\]\s*pub struct \w+Response\s*\{\s*pub message: String,\s*\}").unwrap();
        old_struct_regex.replace(content, &enhanced_struct).to_string()
    } else {
        content.to_string()
    }
}

fn sanitize_tag(path_str: &str) -> String {
    let s = path_str.replace(std::path::MAIN_SEPARATOR, ".").replace('-', "_");
    let s = DYNAMIC_REGEX.replace_all(&s, |caps: &regex::Captures| {
        let inner = &caps[1];
        if inner.starts_with("...") {
            ".catch_all".to_string()
        } else {
            format!(".{}", inner)
        }
    }).to_string();
    s.trim_matches('.').to_string()
}

static STRUCT_REGEX: Lazy<Regex> = Lazy::new(|| {
  Regex::new(r"(?m)^pub struct (\w+?)(Data|Response)\s*\{").unwrap()
});

#[derive(Debug, Clone)]
pub struct HandlerRouteInfo {
  pub func_name: String,
  pub http_method: String,
  pub route_path: String,
  pub handler_module_path: String,
  pub route_tag: String,
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
    let params: Vec<String> = path_params.iter()
      .map(|(name, typ)| format!(r#"("{}" = {}, Path, description = "The {} identifier")"#, name, typ, name))
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

fn update_handler_file(
  path: &Path,
  schemas: &mut HashSet<String>,
  module_path_prefix: &str,
  root_api_path: &Path
) -> Result<Option<HandlerRouteInfo>> {
  let mut content = fs
    ::read_to_string(path)
    .with_context(|| format!("Failed to read file: {:?}", path))?;

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

  if content.trim().is_empty() {
    let relative_path = path.strip_prefix(root_api_path).unwrap();
    let mut route_path_str = relative_path
      .with_extension("")
      .to_str()
      .unwrap()
      .replace(std::path::MAIN_SEPARATOR, "/");

    if route_path_str.ends_with("/index") {
      route_path_str = route_path_str.strip_suffix("/index").unwrap_or("").to_string();
      if route_path_str.is_empty() {
        route_path_str = "/".to_string();
      }
    }

    let axum_path = Regex::new(r"\[(.*?)\]")
      .unwrap()
      .replace_all(&route_path_str, "{$1}")
      .to_string();

    let pascal_case_name = file_stem
      .split('_')
      .map(|s| s.chars().next().unwrap().to_uppercase().to_string() + &s[1..])
      .collect::<String>();

    let default_description = generate_default_description(&axum_path, "get");

    let (response_struct_name, response_fields, success_response_body) = if axum_path.contains("/search") {
        ("SearchResponse", r#"
    /// Success message
    pub message: String,
    /// Search results - replace with actual Vec<T> where T implements ToSchema
    pub data: Vec<serde_json::Value>,
    /// Total number of results
    pub total: Option<u64>,
    /// Current page
    pub page: Option<u32>,
    /// Results per page
    pub per_page: Option<u32>,"#, "Json<SearchResponse>")
    } else if axum_path.contains('{') || axum_path.contains("/detail") {
        ("DetailResponse", r#"
    /// Success message
    pub message: String,
    /// Detailed data - replace with actual T where T implements ToSchema
    pub data: serde_json::Value,"#, "Json<DetailResponse>")
    } else {
        ("ListResponse", r#"
    /// Success message
    pub message: String,
    /// List of items - replace with actual Vec<T> where T implements ToSchema
    pub data: Vec<serde_json::Value>,
    /// Total number of items
    pub total: Option<u64>,"#, "Json<ListResponse>")
    };

    // Extract dynamic parameters from route_path_str
    let mut path_params = Vec::new();
    for cap in DYNAMIC_REGEX.captures_iter(&route_path_str) {
        let param = &cap[1];
        if param.starts_with("...") {
            // Catch-all parameter
            let param_name = param.strip_prefix("...").unwrap_or(param);
            path_params.push((param_name.to_string(), "Vec<String>"));
        } else {
            path_params.push((param.to_string(), "String"));
        }
    }

    // Build function signature with Path extractors
    let func_signature = if path_params.is_empty() {
        format!("pub async fn {}() -> impl IntoResponse", func_name)
    } else {
        let params_str = path_params.iter()
            .map(|(name, typ)| format!("Path({}): Path<{}>", name, typ))
            .collect::<Vec<_>>()
            .join(", ");
        format!("pub async fn {}({}) -> impl IntoResponse", func_name, params_str)
    };

    // Build imports with Path if needed
    let imports = if path_params.is_empty() {
        "use axum::{response::IntoResponse, routing::get, Json, Router};".to_string()
    } else {
        "use axum::{extract::Path, response::IntoResponse, routing::get, Json, Router};".to_string()
    };

    let template = format!(
      r#"//! Handler for the {} endpoint.
    #![allow(dead_code)]

    {}
    use std::sync::Arc;
    use crate::routes::AppState;
    use serde::{{Deserialize, Serialize}};
    use serde_json;
    use utoipa::ToSchema;

    pub const ENDPOINT_METHOD: &str = "get";
    pub const ENDPOINT_PATH: &str = "/{}";
    pub const ENDPOINT_DESCRIPTION: &str = "{}";
    pub const ENDPOINT_TAG: &str = "{}";
    pub const OPERATION_ID: &str = "{}";
    pub const SUCCESS_RESPONSE_BODY: &str = "{}";

    /// Response structure for the {} endpoint.
    /// Replace `serde_json::Value` with your actual data types and implement `utoipa::ToSchema` for complex types.
    #[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
    pub struct {} {{{}
    }}

    {} {{
        Json({} {{
            message: "Hello from {}!".to_string(),{}
        }})
    }}

    /// {}
    pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {{
        router.route(ENDPOINT_PATH, get({}))
    }}
    "#,
        func_name,
        imports,
        axum_path,
        default_description,
        default_tag,
        operation_id,
        success_response_body,
        pascal_case_name,
        response_struct_name,
        response_fields,
        func_signature,
        response_struct_name,
        func_name,
        if response_struct_name == "SearchResponse" {
            r#"
            data: vec![],
            total: None,
            page: None,
            per_page: None,"#
        } else if response_struct_name == "ListResponse" {
            r#"
            data: vec![],
            total: None,"#
        } else {
            r#"
            data: serde_json::json!(null),"#
        },
        default_description,
        func_name
      );

    fs::write(path, template)?;
    println!("cargo:warning=Generated new handler template for {:?}", path);
    return Ok(None);
  }

  let mut metadata = HashMap::new();
  for cap in ENDPOINT_METADATA_REGEX.captures_iter(&content) {
    metadata.insert(cap[1].to_string(), cap[2].to_string());
  }

  // Overwrite ENDPOINT_TAG with hierarchical tag
  let tag_regex = Regex::new(r#"const\s+ENDPOINT_TAG:\s*&\s*str\s*=\s*"[^"]*";"#).unwrap();
  let mut content = tag_regex.replace(&content, &format!(r#"const ENDPOINT_TAG: &str = "{}";"#, default_tag)).to_string();

  // Overwrite OPERATION_ID with generated operation_id
  let operation_id_regex = Regex::new(r#"const\s+OPERATION_ID:\s*&\s*str\s*=\s*"[^"]*";"#).unwrap();
  let mut content = operation_id_regex.replace(&content, &format!(r#"const OPERATION_ID: &str = "{}";"#, operation_id)).to_string();

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
      if route_path.ends_with(&format!("/{}", param_name)) && !route_path.contains(&format!("{{{}}}", param_name)) {
        let new_route_path = route_path.replace(&format!("/{}", param_name), &format!("/{{{}}}", param_name));
        let endpoint_path_regex = Regex::new(r#"const\s+ENDPOINT_PATH:\s*&\s*str\s*=\s*"[^"]*";"#).unwrap();
        content = endpoint_path_regex.replace(&content, &format!(r#"const ENDPOINT_PATH: &str = "{}";"#, new_route_path)).to_string();
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
  let axum_path = Regex::new(r"\[(.*?)\]")
    .unwrap()
    .replace_all(&route_path, "{$1}")
    .to_string();
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
  println!("cargo:warning=File: {:?}", path);
  println!("cargo:warning=axum_path: {}", axum_path);
  println!("cargo:warning=route_path: {}", route_path);
  println!("cargo:warning=path_params: {:?}", path_params);
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
      let params_str = path_params.iter()
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
        content = import_regex.replace(&content, &format!("use axum::{{{}}};", new_import)).to_string();
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
  let macro_regex = Regex::new(r"(?m)^\s*#\[utoipa::path\([^\]]*\)\]\n").unwrap();
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

  let handler_full_module_path = format!("{}::{}", module_path_prefix, file_stem);
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

// FIX: This function correctly constructs the module path.
fn generate_mod_for_directory(
  current_dir: &Path,
  root_api_path: &Path,
  all_handlers: &mut Vec<HandlerRouteInfo>,
  all_schemas: &mut HashSet<String>
) -> Result<()> {
  let mut pub_mods = Vec::new();
  let mut route_registrations = Vec::new();

  let relative_path = current_dir.strip_prefix(root_api_path).unwrap_or(Path::new(""));
  let relative_path_str = relative_path.to_str().unwrap().replace(std::path::MAIN_SEPARATOR, "::").replace('-', "_");

  let module_path_prefix = if relative_path_str.is_empty() {
    "crate::routes::api".to_string()
  } else {
    format!("crate::routes::api::{}", relative_path_str)
  };

  let mut entries: Vec<PathBuf> = fs
    ::read_dir(current_dir)?
    .filter_map(Result::ok)
    .map(|e| e.path())
    .collect();
  entries.sort();

  for path in entries {
    let file_name = path
      .file_name()
      .and_then(|s| s.to_str())
      .unwrap_or("");
    if file_name.starts_with('.') || file_name == "mod.rs" {
      continue;
    }

    if path.is_dir() {
      let mod_name = file_name.replace('-', "_");
      pub_mods.push(format!("pub mod {};", mod_name));
      route_registrations.push(format!("{}", mod_name));
      generate_mod_for_directory(&path, root_api_path, all_handlers, all_schemas)?;
    } else if path.is_file() && path.extension().map_or(false, |e| e == "rs") {
      let file_stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap();

      let mod_name = file_stem.replace(['[', ']'], "").replace('-', "_");
      pub_mods.push(format!("pub mod {};", mod_name));
      route_registrations.push(format!("{}", mod_name));

      if
        let Some(handler_info) = update_handler_file(
          &path,
          all_schemas,
          &module_path_prefix,
          root_api_path
        )?
      {
        all_handlers.push(handler_info);
      }
    }
  }

  let mod_content = format!(
    r#"/// THIS FILE IS AUTOMATICALLY GENERATED BY build.rs
/// DO NOT EDIT THIS FILE MANUALLY

{}

/// Register routes for this directory
use axum::Router;
use std::sync::Arc;
use crate::routes::AppState;

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {{
  {}
}}
"#,
    pub_mods.join("\n"),
    if route_registrations.is_empty() {
      "router".to_string()
    } else {
      route_registrations
        .iter()
        .rev()
        .fold("router".to_string(), |acc, reg| format!("{}::register_routes({})", reg, acc))
    }
  );

  fs::write(current_dir.join("mod.rs"), mod_content)?;
  Ok(())
}

fn generate_root_api_mod(
  api_routes_path: &Path,
  modules: &[String],
  all_handlers: &[HandlerRouteInfo],
  schemas: &HashSet<String>
) -> Result<()> {
  let mut content = String::from(
    "//! THIS FILE IS AUTOMATICALLY GENERATED BY build.rs\n//! DO NOT EDIT THIS FILE MANUALLY\n\n"
  );
  content.push_str(
    "use axum::Router;\nuse std::sync::Arc;\nuse utoipa::OpenApi;\nuse crate::routes::AppState;\n\n"
  );

  for module in modules {
    content.push_str(&format!("pub mod {};\n", module));
  }
  content.push_str("\n");

  let all_paths: Vec<String> = all_handlers
    .iter()
    .map(|h| format!("        {}::{}", h.handler_module_path, h.func_name))
    .collect();

  let mut sorted_schemas: Vec<String> = schemas.iter().cloned().collect();
  sorted_schemas.sort();

  // Generate `use` imports for each fully-qualified schema path and collect simple type names for components.
  // Handle duplicate type names by using aliases
  let mut type_name_count: HashMap<String, usize> = HashMap::new();
  let mut use_lines: Vec<String> = Vec::new();
  let mut simple_names: Vec<String> = Vec::new();

  for full in &sorted_schemas {
    let simple_name = full.split("::").last().unwrap().to_string();
    let count = type_name_count.entry(simple_name.clone()).or_insert(0);
    *count += 1;

    if *count > 1 {
      // Use alias for duplicates
      let alias = format!("{} as {}_{}", full, simple_name.to_lowercase(), *count - 1);
      use_lines.push(format!("use {};", alias));
      simple_names.push(format!("{}_{}", simple_name.to_lowercase(), *count - 1));
    } else {
      use_lines.push(format!("use {};", full));
      simple_names.push(simple_name);
    }
  }

  content.push_str(
    &format!(
      "{}\n#[derive(OpenApi)]\n#[openapi(\n    paths(\n{}\n    ),\n    components(schemas({})),\n    tags((\n        name = \"api\", description = \"Main API\"\n    ))\n)]\npub struct ApiDoc;\n\n",
      use_lines.join("\n"),
      all_paths.join(",\n"),
      simple_names.join(", ")
    )
  );

  let router_registrations = modules
    .iter()
    .map(|m| format!("    router = {}::register_routes(router);", m))
    .collect::<Vec<_>>()
    .join("\n");

  content.push_str(
    &format!("pub fn create_api_routes() -> Router<Arc<AppState>> {{\n    let mut router = Router::new();\n{}\n    router\n}}\n", router_registrations)
  );

  fs::write(api_routes_path.join("mod.rs"), content)?;
  Ok(())
}

fn main() -> Result<()> {
  println!("cargo:rerun-if-changed=build.rs");
  let api_routes_path = Path::new("src/routes/api");
  fs::create_dir_all(api_routes_path)?;
  println!("cargo:rerun-if-changed=src/routes/api/");

  // No cleanup needed; mod.rs files are overwritten by fs::write

  let mut all_handlers = Vec::new();
  let mut all_schemas = HashSet::new();

  generate_mod_for_directory(
    api_routes_path,
    api_routes_path,
    &mut all_handlers,
    &mut all_schemas
  )?;

  let mut modules = Vec::new();
  for entry in fs::read_dir(api_routes_path)?.flatten() {
    if let Some(name) = entry.file_name().to_str() {
      if name != "mod.rs" && !name.starts_with('.') {
        let file_stem = name.strip_suffix(".rs").unwrap_or(name);
        modules.push(file_stem.replace(['[', ']'], "").replace('-', "_"));
      }
    }
  }
  modules.sort();
  modules.dedup();

  generate_root_api_mod(api_routes_path, &modules, &all_handlers, &all_schemas)?;

  Ok(())
}
