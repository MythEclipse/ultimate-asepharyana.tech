use std::collections::HashSet;
use std::fs;
use std::path::{ Path };

use anyhow::{ anyhow, Context, Result };
use once_cell::sync::Lazy;
use regex::Regex;

static HANDLER_FN_REGEX: Lazy<Regex> = Lazy::new(||
  Regex::new(r"pub async fn\s+([a-zA-Z0-9_]+)\s*\(").unwrap()
);

static STRUCT_REGEX: Lazy<Regex> = Lazy::new(||
  Regex::new(r"(?m)^pub struct (\w+?)(Data|Response)\s*\{").unwrap()
);

#[derive(Debug, Clone)]
pub struct HandlerRouteInfo {
  pub func_name: String,
  pub http_method: String,
  pub route_path: String,
  pub handler_module_path: String,
  pub utoipa_macro_content: String,
  pub utoipa_path_name: String,
  pub mod_path_for_utoipa: String,
  pub route_tag: String,
  pub response_body: String,
  pub route_description: String,
}

fn to_pascal_case(s: &str) -> String {
  s.split('_')
    .map(|word| {
      let mut c = word.chars();
      match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
      }
    })
    .collect()
}

// Function to generate OpenAPI (Utoipa) macro content
fn generate_utoipa_macro(
  http_method: &str,
  route_path: &str,
  route_tag: &str,
  response_body: &str,
  route_description: &str,
  params_parts: Vec<String>
) -> Result<String> {
  let params_block = if params_parts.is_empty() {
    String::new()
  } else {
    format!(", params({})", params_parts.join(", "))
  };

  let mut macro_content = format!(
    r#"{}, path = "{}", tag = "{}", responses((status = 200, description = "{}", body = {}), (status = 500, description = "Internal Server Error"))"#,
    http_method,
    route_path.replace("{", "{{").replace("}", "}}"),
    route_tag,
    route_description,
    response_body
  );
  macro_content.push_str(&params_block);
  Ok(macro_content)
}

fn inject_to_schema_derive(
  content: &String,
  module_path: &str,
  schemas: &mut HashSet<String>
) -> Result<()> {
  for cap in STRUCT_REGEX.captures_iter(content) {
    let name_prefix = cap[1].to_string();
    let suffix = cap[2].to_string();
    schemas.insert(format!("{}::{}{}", module_path, name_prefix, suffix));
  }
  Ok(())
}

fn update_handler_file(
  path: &Path,
  schemas: &mut HashSet<String>,
  module_path_prefix: &str,
  root_api_path: &Path
) -> Result<Option<HandlerRouteInfo>> {
  let content = fs
    ::read_to_string(path)
    .with_context(|| format!("Failed to read handler file: {:?}", path))?;

  if !HANDLER_FN_REGEX.is_match(&content) {
    return Ok(None);
  }

  let handler_caps = HANDLER_FN_REGEX.captures(&content).ok_or_else(||
    anyhow!("Handler function not found in {:?}", path)
  )?;
  let func_name = handler_caps[1].to_string();

  let file_stem = path
    .file_stem()
    .and_then(|s| s.to_str())
    .ok_or_else(|| anyhow!("Could not get file stem from {:?}", path))?;

  // Infer method from function name (simple heuristic)
  let http_method = (
    if func_name.starts_with("get_") {
      "get"
    } else if func_name.starts_with("post_") {
      "post"
    } else if func_name.starts_with("put_") {
      "put"
    } else if func_name.starts_with("delete_") {
      "delete"
    } else if func_name.starts_with("patch_") {
      "patch"
    } else if func_name.starts_with("head_") {
      "head"
    } else if func_name.starts_with("options_") {
      "options"
    } else {
      "get" // Default to GET if not specified
    }
  ).to_string();

  // Infer route path from module path and file name
  let relative_path = path.strip_prefix(root_api_path).unwrap();
  let route_path = format!(
    "/{}",
    relative_path.to_str().unwrap().trim_end_matches(".rs").replace(std::path::MAIN_SEPARATOR, "/")
  );

  // Infer route tag from the parent directory name
  let route_tag = path
    .parent()
    .and_then(|p| p.file_name())
    .and_then(|s| s.to_str())
    .unwrap_or("api")
    .to_string();

  // Infer response body (simplified for now, ideally parsed from handler function signature)
  let response_body = if content.contains("impl IntoResponse") {
    // A very basic heuristic: if it implements IntoResponse, it might be Json
    "Json(Value)".to_string()
  } else {
    "String".to_string() // Default to String
  };

  let route_description = format!(
    "{} handler",
    func_name.replace("_handler", "").replace("_", " ")
  );

  let utoipa_macro_content = generate_utoipa_macro(
    &http_method,
    &route_path,
    &route_tag,
    &response_body,
    &route_description,
    Vec::new() // No params inferred automatically yet
  )?;

  inject_to_schema_derive(&content, module_path_prefix, schemas)?;

  Ok(
    Some(HandlerRouteInfo {
      func_name: func_name.clone(),
      http_method,
      route_path,
      handler_module_path: format!("{}::{}", module_path_prefix, file_stem),
      utoipa_macro_content,
      utoipa_path_name: func_name.clone(),
      mod_path_for_utoipa: module_path_prefix.to_string(),
      route_tag,
      response_body,
      route_description,
    })
  )
}

fn generate_mod_for_directory(
  current_dir: &Path,
  root_api_path: &Path,
  all_handlers: &mut Vec<HandlerRouteInfo>,
  all_schemas: &mut HashSet<String>
) -> Result<()> {
  let mut pub_mods = Vec::new();
  let mut current_level_handlers = Vec::new();

  let relative_path = current_dir.strip_prefix(root_api_path).unwrap();
  let module_path_prefix = format!(
    "crate::routes::api::{}",
    relative_path.to_str().unwrap().replace(std::path::MAIN_SEPARATOR, "::")
  )
    .trim_end_matches("::")
    .to_string();

  for entry in fs::read_dir(current_dir)? {
    let path = entry?.path();
    let file_name = path
      .file_name()
      .and_then(|s| s.to_str())
      .unwrap_or("");

    if file_name.starts_with('.') {
      continue;
    }

    if path.is_dir() {
      pub_mods.push(format!("pub mod {};", file_name));
      generate_mod_for_directory(&path, root_api_path, all_handlers, all_schemas)?;
    } else if path.is_file() && path.extension().map_or(false, |e| e == "rs") {
      if let Some(file_stem) = path.file_stem().and_then(|s| s.to_str()) {
        if file_stem != "mod" {
          pub_mods.push(format!("pub mod {};", file_stem)); // Re-added
          if
            let Some(handler_info) = update_handler_file(
              &path,
              all_schemas,
              &module_path_prefix,
              root_api_path
            )?
          {
            current_level_handlers.push(handler_info.clone());
            all_handlers.push(handler_info);
          }
        }
      }
    }
  }

  pub_mods.sort();
  let mut mod_content = String::from(
    "//! THIS FILE IS AUTOMATICALLY GENERATED BY build.rs\n//! DO NOT EDIT THIS FILE MANUALLY\n\n"
  );
  mod_content.push_str(&pub_mods.join("\n"));
  mod_content.push_str("\n\n");

  let current_module_name = current_dir
    .file_name()
    .and_then(|s| s.to_str())
    .unwrap_or("api");
  let _pascal_case_module = to_pascal_case(current_module_name);

  let mut utoipa_path_declarations = Vec::new();
  for handler in &current_level_handlers {
    let func_name_uppercase = handler.func_name.to_uppercase();
    let const_name = format!("{}_UTOIPA", func_name_uppercase);
    utoipa_path_declarations.push(
      format!("#[utoipa::path({})]\npub fn {}() {{}}", handler.utoipa_macro_content, const_name)
    );
  }

  mod_content.push_str(&utoipa_path_declarations.join("\n\n"));
  mod_content.push_str("\n\n");

  let mut handler_uses = Vec::new();
  for handler in &current_level_handlers {
    handler_uses.push(format!("use {};", handler.handler_module_path));
  }
  mod_content.push_str(&handler_uses.join("\n"));
  mod_content.push_str("\n\n");

  mod_content.push_str(
    "use axum::{routing::{get, post, put, delete, patch, head, options}, Router};\n"
  );
  mod_content.push_str("use crate::routes::ChatState;\n");
  mod_content.push_str("use std::sync::Arc;\n\n");
  mod_content.push_str(
    &format!("pub fn create_routes() -> Router<Arc<ChatState>> {{\n    let router = Router::new();")
  );

  current_level_handlers.sort_by(|a, b| a.route_path.cmp(&b.route_path));

  for handler in &current_level_handlers {
    let current_router_full_path = module_path_prefix
      .replace("crate::routes::api", "/api")
      .replace("::", "/");
    let route_segment = handler.route_path
      .strip_prefix(&current_router_full_path)
      .unwrap_or(&handler.route_path);
    let final_route = if !route_segment.starts_with('/') {
      format!("/{}", route_segment)
    } else {
      route_segment.to_string()
    };

    println!(
      "DEBUG: final_route: \"{}\", method: \"{}\", handler_module_path: \"{}\", func_name: \"{}\"",
      final_route,
      handler.http_method,
      handler.handler_module_path,
      handler.func_name
    );
    let mut route_line = String::new();
    route_line.push_str("\n    let router = router.route(\"");
    route_line.push_str(&final_route); // final_route is "/{slug}"
    route_line.push_str("\", ");
    route_line.push_str(&handler.http_method);
    route_line.push_str("(");
    route_line.push_str(handler.handler_module_path.split("::").last().unwrap());
    route_line.push_str("::");
    route_line.push_str(&handler.func_name);
    route_line.push_str("));");
    mod_content.push_str(&route_line);
  }
  mod_content.push_str("\n    router\n}\n");

  fs::write(current_dir.join("mod.rs"), &mod_content)?;
  println!("DEBUG: Content written to mod.rs:\n{}", mod_content); // Use println! for build script output
  Ok(()) // Added Ok(()) here
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
    "use axum::Router;\nuse std::sync::Arc;\nuse utoipa::OpenApi;\nuse crate::routes::ChatState;\n\n"
  );

  for module in modules {
    content.push_str(&format!("pub mod {};\n", module));
  }
  content.push_str("\n");

  let all_paths: Vec<String> = all_handlers
    .iter()
    .map(|h| format!("{}::{}_UTOIPA", h.mod_path_for_utoipa, h.utoipa_path_name.to_uppercase()))
    .collect();

  let mut sorted_schemas: Vec<String> = schemas.iter().cloned().collect();
  sorted_schemas.sort();

  content.push_str(
    &format!(
      "#[derive(OpenApi)]\n#[openapi(\n    paths(\n{}\n    ),\n    components(schemas({})),\n    tags((\n        name = \"api\", description = \"Main API\"\n    ))\n)]\npub struct ApiDoc;\n\n",
      all_paths.join(",\n"),
      sorted_schemas.join(", ")
    )
  );

  let router_nesting = modules
    .iter()
    .map(|m| format!("        .nest(\"/{}\", {}::create_routes())", m, m))
    .collect::<Vec<_>>()
    .join("\n");

  content.push_str(
    &format!("pub fn create_api_routes() -> Router<Arc<ChatState>> {{\n    Router::new()\n{}\n}}\n", router_nesting)
  );

  fs::write(api_routes_path.join("mod.rs"), content)?;
  Ok(())
}

fn main() -> Result<()> {
  println!("cargo:rerun-if-changed=build.rs");
  let api_routes_path = Path::new("src/routes/api");
  fs::create_dir_all(api_routes_path)?;

  let mut modules = Vec::new();
  for entry in fs::read_dir(api_routes_path)?.flatten() {
    let path = entry.path();
    if path.is_dir() {
      if let Some(dir_name) = path.file_name().and_then(|s| s.to_str()) {
        modules.push(dir_name.to_string());
      }
    }
  }
  modules.sort();

  let mut all_handlers = Vec::new();
  let mut all_schemas = HashSet::new();

  for module in &modules {
    println!("cargo:rerun-if-changed=src/routes/api/{}/", module);
    let module_path = api_routes_path.join(module);
    generate_mod_for_directory(&module_path, api_routes_path, &mut all_handlers, &mut all_schemas)?;
  }

  let _api_docs: Vec<(String, String)> = modules // Renamed to _api_docs
    .iter()
    .map(|m| (m.clone(), to_pascal_case(m)))
    .collect();

  generate_root_api_mod(api_routes_path, &modules, &all_handlers, &all_schemas)?;

  Ok(())
}
