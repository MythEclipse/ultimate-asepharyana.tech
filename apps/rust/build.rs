use std::collections::HashSet;
use std::fs;
use std::path::Path;

use anyhow::Result;

mod build_utils;
use build_utils::{ mod_generator, openapi_generator };

fn main() -> Result<()> {
  println!("cargo:rerun-if-changed=build.rs");
  let api_routes_path = Path::new("src/routes/api");
  fs::create_dir_all(api_routes_path)?;
  println!("cargo:rerun-if-changed=src/routes/api/");

  let mut all_handlers = Vec::new();
  let mut all_schemas = HashSet::new();

  mod_generator::generate_mod_for_directory(
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

  openapi_generator::generate_root_api_mod(api_routes_path, &modules, &all_handlers, &all_schemas)?;

  Ok(())
}
