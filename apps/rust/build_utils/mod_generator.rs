use std::collections::HashSet;
use std::fs;
use std::path::{ Path, PathBuf };
use anyhow::{ Result, Context };
use crate::build_utils::handler_updater::{ HandlerRouteInfo, update_handler_file };
use crate::build_utils::path_utils::is_dynamic_route_content;
use super::BuildOperation;

fn is_rust_keyword(s: &str) -> bool {
  matches!(
    s,
    "as" |
      "break" |
      "const" |
      "continue" |
      "crate" |
      "else" |
      "enum" |
      "extern" |
      "false" |
      "fn" |
      "for" |
      "if" |
      "impl" |
      "in" |
      "let" |
      "loop" |
      "match" |
      "mod" |
      "move" |
      "mut" |
      "pub" |
      "ref" |
      "return" |
      "self" |
      "Self" |
      "static" |
      "struct" |
      "super" |
      "trait" |
      "true" |
      "type" |
      "unsafe" |
      "use" |
      "where" |
      "while" |
      "async" |
      "await" |
      "dyn" |
      "abstract" |
      "become" |
      "box" |
      "do" |
      "final" |
      "macro" |
      "override" |
      "priv" |
      "typeof" |
      "unsized" |
      "virtual" |
      "yield"
  )
}

fn sanitize_module_name(name: &str) -> String {
  let sanitized = name.trim_matches(|c| (c == '[' || c == ']')).replace('-', "_");
  if is_rust_keyword(&sanitized) {
    format!("r#{}", sanitized)
  } else {
    sanitized
  }
}

fn compute_module_path_prefix(current_dir: &Path, root_api_path: &Path) -> Result<String> {
  let relative_path = current_dir.strip_prefix(root_api_path).unwrap_or(Path::new(""));
  let relative_path_str = relative_path
    .to_str()
    .ok_or_else(|| anyhow::anyhow!("Invalid path encoding for directory: {:?}", current_dir))?
    .replace(std::path::MAIN_SEPARATOR, "::")
    .replace('-', "_");

  let module_path_prefix = if relative_path_str.is_empty() {
    "crate::routes::api".to_string()
  } else {
    let sanitized_segments: Vec<String> = relative_path_str
      .split("::")
      .map(|s| sanitize_module_name(&s.replace("[", "").replace("]", "")))
      .collect();
    format!("crate::routes::api::{}", sanitized_segments.join("::"))
  };

  Ok(module_path_prefix)
}

fn process_directory_entries(
  current_dir: &Path,
  root_api_path: &Path,
  module_path_prefix: &str,
  all_handlers: &mut Vec<HandlerRouteInfo>,
  all_schemas: &mut HashSet<String>,
  modules: &mut Vec<String>,
  pub_mods: &mut Vec<String>,
  route_registrations: &mut Vec<String>
) -> Result<()> {
  let mut entries: Vec<PathBuf> = fs
    ::read_dir(current_dir)
    .with_context(|| format!("Failed to read directory: {:?}", current_dir))?
    .filter_map(Result::ok)
    .map(|e| e.path())
    .collect();
  entries.sort();

  for path in entries {
    let file_name = path
      .file_name()
      .and_then(|s| s.to_str())
      .unwrap_or("");
    if file_name.starts_with('.') || file_name == "mod.rs" || file_name == "test" {
      continue;
    }

    let mod_name = sanitize_module_name(file_name);

    if path.is_dir() {
      // Recursively generate mod.rs for the subdirectory
      let has_routes = generate_mod_for_directory(
        &path,
        root_api_path,
        all_handlers,
        all_schemas,
        modules
      )?;

      if has_routes {
        pub_mods.push(format!("pub mod {};", mod_name));
        route_registrations.push(mod_name.clone());

        // If this is the root level, add to modules
        if current_dir == root_api_path {
          modules.push(mod_name.clone());
        }
      }
    } else if path.is_file() && path.extension().map_or(false, |e| e == "rs") {
      let file_content = fs::read_to_string(&path)?;
      let is_dynamic = is_dynamic_route_content(&file_content);

      let file_stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| anyhow::anyhow!("Invalid file stem for: {:?}", path))?;

      let mod_name_from_file_stem = sanitize_module_name(file_stem);
      pub_mods.push(format!("pub mod {};", mod_name_from_file_stem));

      // For dynamic routes, register the route using the dynamic segment
      if is_dynamic {
        // Assuming dynamic routes are always in the format "some_name.rs" where "some_name" is the parameter
        route_registrations.push(format!("{}/{{{}}}", current_dir.strip_prefix(root_api_path)?.to_str().unwrap().replace("\\", "/"), file_stem));
      } else {
        route_registrations.push(mod_name_from_file_stem.clone());
      }

      if
        let Some(handler_info) = update_handler_file(
          &path,
          all_schemas,
          module_path_prefix,
          root_api_path
        ).with_context(|| format!("Failed to update handler file: {:?}", path))?
      {
        all_handlers.push(handler_info);
      }

      // If this is the root level, add to modules
      if current_dir == root_api_path {
        modules.push(mod_name_from_file_stem.clone());
      }
    }
  }

  Ok(())
}

fn build_route_registration_body(route_registrations: &[String]) -> String {
  if route_registrations.is_empty() {
    "router".to_string()
  } else {
    route_registrations
      .iter()
      .rev()
      .fold("router".to_string(), |acc, reg| {
        format!("{}::register_routes({})", reg, acc)
      })
  }
}

fn generate_mod_content(pub_mods: &[String], body: &str) -> String {
  format!(
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
    body
  )
}

fn cleanup_empty_directory(
  current_dir: &Path,
  root_api_path: &Path,
  has_routes: bool
) -> Result<()> {
  // If this directory has no routes and is not the root, delete it
  if !has_routes && current_dir != root_api_path {
    fs
      ::remove_dir_all(current_dir)
      .with_context(|| format!("Failed to remove empty directory: {:?}", current_dir))?;
  }
  Ok(())
}

pub fn generate_mod_for_directory(
  current_dir: &Path,
  root_api_path: &Path,
  all_handlers: &mut Vec<HandlerRouteInfo>,
  all_schemas: &mut HashSet<String>,
  modules: &mut Vec<String>
) -> Result<bool> {
  let mut pub_mods = Vec::new();
  let mut route_registrations = Vec::new();

  let module_path_prefix = compute_module_path_prefix(current_dir, root_api_path)?;

  process_directory_entries(
    current_dir,
    root_api_path,
    &module_path_prefix,
    all_handlers,
    all_schemas,
    modules,
    &mut pub_mods,
    &mut route_registrations
  )?;

  let body = build_route_registration_body(&route_registrations);
  let mod_content = generate_mod_content(&pub_mods, &body);

  fs
    ::write(current_dir.join("mod.rs"), mod_content)
    .with_context(|| format!("Failed to write mod.rs for directory: {:?}", current_dir))?;

  let has_routes = !pub_mods.is_empty();

  cleanup_empty_directory(current_dir, root_api_path, has_routes)?;

  Ok(has_routes)
}
