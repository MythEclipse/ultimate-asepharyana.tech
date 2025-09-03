use std::collections::HashSet;
use std::fs;
use std::path::{ Path, PathBuf };

use anyhow::{ anyhow, Context, Result };
use once_cell::sync::Lazy;
use regex::Regex;

static HANDLER_FN_REGEX: Lazy<Regex> = Lazy::new(||
  Regex::new(r"pub async fn\s+([a-zA-Z0-9_]+)\s*\(").unwrap()
);

static ENDPOINT_METADATA_REGEX: Lazy<Regex> = Lazy::new(|| {
  Regex::new(
    r#"const\s+(ENDPOINT_METHOD|ENDPOINT_PATH|ENDPOINT_DESCRIPTION|ENDPOINT_TAG|SUCCESS_RESPONSE_BODY):\s*&\s*str\s*=\s*"([^"]*)";"#
  ).unwrap()
});

static STRUCT_REGEX: Lazy<Regex> = Lazy::new(||
  Regex::new(r"(?m)^pub struct (\w+?)(Data|Response)\s*\{").unwrap()
);

#[derive(Debug, Clone)]
pub struct HandlerRouteInfo {
  pub func_name: String,
  pub http_method: String,
  pub route_path: String,
  pub handler_module_path: String,
  pub route_tag: String,
}

fn generate_utoipa_macro(
  http_method: &str,
  route_path: &str,
  route_tag: &str,
  response_body: &str,
  route_description: &str
) -> String {
  format!(
    r#"#[utoipa::path(
    {},
    path = "{}",
    tag = "{}",
    responses(
        (status = 200, description = "{}", body = {}),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]"#,
    http_method,
    route_path,
    route_tag,
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
  let content = fs
    ::read_to_string(path)
    .with_context(|| format!("Failed to read file: {:?}", path))?;

  let file_stem = path
    .file_stem()
    .and_then(|s| s.to_str())
    .ok_or_else(|| anyhow!("Could not get file stem from {:?}", path))?
    .replace(['[', ']'], ""); // Bersihkan karakter kurung siku

  let func_name = &file_stem;

  // Jika file kosong, generate template lengkap
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

    // Konversi [param] menjadi {param} untuk Axum
    let axum_path = Regex::new(r"\[(.*?)\]")
      .unwrap()
      .replace_all(&route_path_str, "{$1}")
      .to_string();

    let default_tag = path
      .parent()
      .and_then(|p| p.file_name())
      .and_then(|s| s.to_str())
      .unwrap_or("api")
      .to_string();

    let pascal_case_name = file_stem
      .split('_')
      .map(|s| s.chars().next().unwrap().to_uppercase().to_string() + &s[1..])
      .collect::<String>();

    let template = format!(
      r#"//! Handler for the {} endpoint.
use axum::{{response::IntoResponse, routing::get, Json, Router}};
use std::sync::Arc;
use crate::routes::ChatState;
use serde::{{Deserialize, Serialize}};
use utoipa::ToSchema;

// --- API METADATA ---
pub const ENDPOINT_METHOD: &str = "get";
pub const ENDPOINT_PATH: &str = "/{}";
pub const ENDPOINT_DESCRIPTION: &str = "Description for the {} endpoint";
pub const ENDPOINT_TAG: &str = "{}";
pub const SUCCESS_RESPONSE_BODY: &str = "Json<{}Response>";

// --- RESPONSE STRUCTURE ---
#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct {}Response {{
    pub message: String,
}}

// --- HANDLER FUNCTION ---
// The Utoipa macro is automatically generated/updated by build.rs
pub async fn {}() -> impl IntoResponse {{
    Json({}Response {{
        message: "Hello from {}!".to_string(),
    }})
}}

// --- ROUTE REGISTRATION ---
// This function is automatically generated/updated by build.rs
pub fn register_routes(router: Router<Arc<ChatState>>) -> Router<Arc<ChatState>> {{
    router.route(ENDPOINT_PATH, get({}))
}}
"#,
      func_name,
      axum_path,
      func_name,
      default_tag,
      pascal_case_name,
      pascal_case_name,
      func_name,
      pascal_case_name,
      func_name,
      func_name
    );

    fs::write(path, template)?;
    println!("cargo:warning=Generated new handler template for {:?}", path);
    // Kembali lebih awal, biarkan proses build selanjutnya yang memprosesnya
    return Ok(None);
  }

  // Ekstrak metadata dari konstanta
  let mut metadata = std::collections::HashMap::new();
  for cap in ENDPOINT_METADATA_REGEX.captures_iter(&content) {
    metadata.insert(cap[1].to_string(), cap[2].to_string());
  }

  let http_method = metadata
    .get("ENDPOINT_METHOD")
    .cloned()
    .unwrap_or_else(|| "get".to_string());
  let route_path = metadata
    .get("ENDPOINT_PATH")
    .cloned()
    .unwrap_or_else(|| format!("/{}", file_stem));
  let route_tag = metadata
    .get("ENDPOINT_TAG")
    .cloned()
    .unwrap_or_else(|| "api".to_string());
  let response_body = metadata
    .get("SUCCESS_RESPONSE_BODY")
    .cloned()
    .unwrap_or_else(|| "String".to_string());
  let route_description = metadata
    .get("ENDPOINT_DESCRIPTION")
    .cloned()
    .unwrap_or_else(|| format!("Handler for {}", func_name));

  // Temukan nama fungsi handler aktual
  let actual_func_name = HANDLER_FN_REGEX.captures(&content)
    .map(|c| c[1].to_string())
    .unwrap_or_else(|| func_name.to_string());

  // Generate makro Utoipa dan fungsi register_routes yang baru
  let new_utoipa_macro = generate_utoipa_macro(
    &http_method,
    &route_path,
    &route_tag,
    &response_body,
    &route_description
  );
  let new_register_fn = format!(
    "pub fn register_routes(router: Router<Arc<ChatState>>) -> Router<Arc<ChatState>> {{\n    router.route(ENDPOINT_PATH, axum::routing::{}({}))\n}}",
    http_method,
    actual_func_name
  );

  // Ganti atau sisipkan makro Utoipa
  let utoipa_regex = Regex::new(
    &format!(r"(?s)#\[utoipa::path\(.*?\)\s*\n*\]\s*\npub async fn\s+{}\s*\(", actual_func_name)
  ).unwrap();
  let fn_signature = format!("pub async fn {} (", actual_func_name);
  let mut new_content = if utoipa_regex.is_match(&content) {
    utoipa_regex
      .replace(&content, format!("{}\npub async fn {} (", new_utoipa_macro, actual_func_name))
      .to_string()
  } else {
    content.replace(&fn_signature, &format!("{}\n{}", new_utoipa_macro, fn_signature))
  };

  // Ganti atau sisipkan fungsi register_routes
  let register_regex = Regex::new(
    r"(?s)pub fn register_routes\(.*?\)\s*->\s*Router<Arc<ChatState>>\s*\{.*?\n\}\n*"
  ).unwrap();
  if register_regex.is_match(&new_content) {
    new_content = register_regex.replace_all(&new_content, &new_register_fn).to_string();
  } else {
    new_content.push_str("\n\n");
    new_content.push_str(&new_register_fn);
  }

  // Tulis kembali ke file jika ada perubahan
  if content != new_content {
    fs::write(path, &new_content)?;
  }

  let handler_full_module_path = format!("{}::{}", module_path_prefix, file_stem);
  inject_schemas(&new_content, &handler_full_module_path, schemas)?;

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

fn generate_mod_for_directory(
  current_dir: &Path,
  root_api_path: &Path,
  all_handlers: &mut Vec<HandlerRouteInfo>,
  all_schemas: &mut HashSet<String>
) -> Result<()> {
  let mut pub_mods = Vec::new();
  let mut route_registrations = Vec::new();

  let relative_path = current_dir.strip_prefix(root_api_path).unwrap_or(Path::new(""));
  let module_path_prefix = format!(
    "crate::routes::api{}",
    relative_path.to_str().unwrap().replace(std::path::MAIN_SEPARATOR, "::")
  )
    .trim_end_matches("::")
    .to_string();

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
      let mod_name = file_name;
      pub_mods.push(format!("pub mod {};", mod_name));
      route_registrations.push(format!("    router = {}::register_routes(router);", mod_name));
      generate_mod_for_directory(&path, root_api_path, all_handlers, all_schemas)?;
    } else if path.is_file() && path.extension().map_or(false, |e| e == "rs") {
      let mod_name = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap()
        .replace(['[', ']'], "");
      pub_mods.push(format!("pub mod {};", mod_name));
      route_registrations.push(format!("    router = {}::register_routes(router);", mod_name));

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
    r#"//! THIS FILE IS AUTOMATICALLY GENERATED BY build.rs
//! DO NOT EDIT THIS FILE MANUALLY

{}

use axum::Router;
use std::sync::Arc;
use crate::routes::ChatState;

pub fn register_routes(mut router: Router<Arc<ChatState>>) -> Router<Arc<ChatState>> {{
{}
    router
}}
"#,
    pub_mods.join("\n"),
    route_registrations.join("\n")
  );

  fs::write(current_dir.join("mod.rs"), mod_content)?;
  Ok(())
}

fn generate_root_api_mod(
  api_routes_path: &Path,
  all_handlers: &[HandlerRouteInfo],
  schemas: &HashSet<String>
) -> Result<()> {
  let mut modules = Vec::new();
  for entry in fs::read_dir(api_routes_path)?.flatten() {
    if let Some(name) = entry.file_name().to_str() {
      if name != "mod.rs" && !name.starts_with('.') {
        modules.push(name.strip_suffix(".rs").unwrap_or(name).replace(['[', ']'], ""));
      }
    }
  }
  modules.sort();
  modules.dedup();

  let mut content = String::from(
    "//! THIS FILE IS AUTOMATICALLY GENERATED BY build.rs\n//! DO NOT EDIT THIS FILE MANUALLY\n\n"
  );
  content.push_str(
    "use axum::Router;\nuse std::sync::Arc;\nuse utoipa::OpenApi;\nuse crate::routes::ChatState;\n\n"
  );
  for module in &modules {
    content.push_str(&format!("pub mod {};\n", module));
  }
  content.push_str("\n");

  let all_paths: Vec<String> = all_handlers
    .iter()
    .map(|h| format!("        {}::{}", h.handler_module_path, h.func_name))
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

  content.push_str(
    "pub fn create_api_routes() -> Router<Arc<ChatState>> {\n    register_routes(Router::new())\n}\n"
  );

  fs::write(api_routes_path.join("mod.rs"), content)?;
  Ok(())
}

fn main() -> Result<()> {
  println!("cargo:rerun-if-changed=build.rs");
  let api_routes_path = Path::new("src/routes/api");
  fs::create_dir_all(api_routes_path)?;
  println!("cargo:rerun-if-changed=src/routes/api/");

  let mut all_handlers = Vec::new();
  let mut all_schemas = HashSet::new();

  generate_mod_for_directory(
    api_routes_path,
    api_routes_path,
    &mut all_handlers,
    &mut all_schemas
  )?;
  generate_root_api_mod(api_routes_path, &all_handlers, &all_schemas)?;

  Ok(())
}
