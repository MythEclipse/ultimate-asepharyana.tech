use std::collections::HashSet;
use std::fs;
use std::path::{ Path };

use anyhow::{ anyhow, Context, Result };
use once_cell::sync::Lazy;
use regex::Regex;

static META_REGEX: Lazy<Regex> = Lazy::new(|| {
  Regex::new(
    r#"(?s)const\s+ENDPOINT_METHOD\s*:\s*&str\s*=\s*"([^"]*)";\s*const\s+ENDPOINT_PATH\s*:\s*&str\s*=\s*"([^"]*)";.*?// --- AKHIR METADATA ---"#
  ).unwrap()
});

static HANDLER_FN_REGEX: Lazy<Regex> = Lazy::new(||
  Regex::new(r"pub async fn\s+([a-zA-Z0-9_]+)\s*\(").unwrap()
);

static STRUCT_REGEX: Lazy<Regex> = Lazy::new(||
  Regex::new(r"(?m)^pub struct (\w+?)(Data|Response)\s*\{").unwrap()
);

static ADDITIONAL_PARAMS_REGEX: Lazy<Regex> = Lazy::new(||
  Regex::new(r#"const\s+([A-Z_]+)\s*:\s*&str\s*=\s*"([^"]*)";"#).unwrap()
);

static ALL_META_FIELDS_REGEX: Lazy<Regex> = Lazy::new(|| {
  Regex::new(r#"const\s+([A-Z_]+)\s*:\s*&str\s*=\s*"([^"]*)";"#).unwrap()
});

#[derive(Debug, Clone)]
pub struct HandlerRouteInfo {
  pub func_name: String,
  pub method: String,
  pub api_path: String,
  pub handler_module_path: String, // Renamed from full_handler_path
  pub utoipa_macro_content: String,
  pub utoipa_path_name: String,
  pub mod_path_for_utoipa: String,
  pub description: String,
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

fn extract_metadata_from_str(content: &str) -> Result<std::collections::HashMap<String, String>> {
  let mut metadata = std::collections::HashMap::new();
  for cap in ALL_META_FIELDS_REGEX.captures_iter(content) {
    metadata.insert(cap[1].to_string(), cap[2].to_string());
  }
  Ok(metadata)
}

fn inject_to_schema_derive(
  content: &String, // Removed mut, content is not modified
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

fn generate_utoipa_macro(
  api_path: &str,
  metadata: &std::collections::HashMap<String, String>
) -> Result<String> {
  let method = metadata
    .get("ENDPOINT_METHOD")
    .ok_or_else(|| anyhow!("ENDPOINT_METHOD not found"))?
    .to_lowercase();
  let tag = metadata.get("ENDPOINT_TAG").ok_or_else(|| anyhow!("ENDPOINT_TAG not found"))?;
  let success_body = metadata
    .get("SUCCESS_RESPONSE_BODY")
    .ok_or_else(|| anyhow!("SUCCESS_RESPONSE_BODY not found"))?;

  let mut params_parts = Vec::new();
  for (key, value) in metadata {
    if key.ends_with("_DESCRIPTION") {
      let param_name = key.trim_end_matches("_DESCRIPTION").to_lowercase();
      if api_path.contains(&format!("{{{}}}", param_name)) {
        params_parts.push(
          format!(r#"("{} = String, Path, description = "{}")"#, param_name, value)
        );
      }
    }
  }

  let params_block = if params_parts.is_empty() {
    String::new()
  } else {
    format!(", params({})", params_parts.join(", "))
  };

  Ok(
    format!(
      r#"{}, path = "{}", tag = "{}", responses((status = 200, description = "Success", body = {}), (status = 500, description = "Internal Server Error")){}"#,
      method,
      api_path,
      tag,
      success_body,
      params_block
    )
  )
}

fn update_handler_file(
  path: &Path,
  schemas: &mut HashSet<String>,
  module_path_prefix: &str
) -> Result<Option<HandlerRouteInfo>> {
  let content = fs
    ::read_to_string(path)
    .with_context(|| format!("Failed to read handler file: {:?}", path))?;

  if !META_REGEX.is_match(&content) || !HANDLER_FN_REGEX.is_match(&content) {
    return Ok(None);
  }

  // No longer modifying handler files, only collecting schemas
  inject_to_schema_derive(&content, module_path_prefix, schemas)?;

  let metadata = extract_metadata_from_str(&content)?;
  let api_path = metadata
    .get("ENDPOINT_PATH")
    .cloned()
    .ok_or_else(|| anyhow!("ENDPOINT_PATH not found"))?;

  let handler_caps = HANDLER_FN_REGEX.captures(&content).ok_or_else(||
    anyhow!("Handler function not found in {:?}", path)
  )?;
  let func_name = handler_caps[1].to_string();

  let file_stem = path
    .file_stem()
    .and_then(|s| s.to_str())
    .ok_or_else(|| anyhow!("Could not get file stem from {:?}", path))?;

  let full_handler_path = module_path_prefix.to_string();
  let description = metadata
    .get("ENDPOINT_DESCRIPTION")
    .cloned()
    .unwrap_or_else(|| "No description.".to_string());
  let utoipa_macro_content = generate_utoipa_macro(&api_path, &metadata)?;

  // No writing back to handler files

  Ok(
    Some(HandlerRouteInfo {
      func_name: func_name.clone(),
      method: metadata.get("ENDPOINT_METHOD").unwrap().to_lowercase(),
      api_path,
      handler_module_path: format!("{}::{}", module_path_prefix, file_stem),
      utoipa_macro_content,
      utoipa_path_name: func_name.clone(),
      mod_path_for_utoipa: module_path_prefix.to_string(),
      description,
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
          if let Some(handler_info) = update_handler_file(&path, all_schemas, &module_path_prefix)? {
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
  let pascal_case_module = to_pascal_case(current_module_name);

  let mut utoipa_path_declarations = Vec::new();
  for handler in &current_level_handlers {
    let func_name_uppercase = handler.func_name.to_uppercase();
    let const_name = format!(
      "{}_UTOIPA",
      func_name_uppercase
    );
    utoipa_path_declarations.push(format!(
      "#[utoipa::path({})]\npub fn {}() {{}}",
      handler.utoipa_macro_content, const_name
    ));
  }

  mod_content.push_str(&utoipa_path_declarations.join("\n\n"));
  mod_content.push_str("\n\n");

  let mut handler_uses = Vec::new();
  for handler in &current_level_handlers {
    handler_uses.push(format!("use {}::{};", handler.handler_module_path, handler.func_name));
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

  current_level_handlers.sort_by(|a, b| a.api_path.cmp(&b.api_path));

  for handler in &current_level_handlers {
    let current_router_full_path = module_path_prefix
      .replace("crate::routes::api", "/api")
      .replace("::", "/");
    let route_segment = handler.api_path
      .strip_prefix(&current_router_full_path)
      .unwrap_or(&handler.api_path);
    let final_route = if !route_segment.starts_with('/') {
      format!("/{}", route_segment)
    } else {
      route_segment.to_string()
    };

    mod_content.push_str(
      &format!("\n    let router = router.route(\"{}\", {}({}));", final_route, handler.method, handler.func_name)
    );
  }
  mod_content.push_str("\n    router\n}\n");

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

  let api_docs: Vec<(String, String)> = modules
    .iter()
    .map(|m| (m.clone(), to_pascal_case(m)))
    .collect();

  generate_root_api_mod(api_routes_path, &modules, &all_handlers, &all_schemas)?;

  Ok(())
}
