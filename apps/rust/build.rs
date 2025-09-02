use std::collections::HashSet;
use std::fs;
use std::path::{ Path, PathBuf };
use regex::Regex;

fn main() {
  println!("cargo:rerun-if-changed=src/routes/api/");

  let api_routes_path = Path::new("src/routes/api");
  let mut modules = Vec::new();

  if api_routes_path.is_dir() {
    if let Ok(entries) = fs::read_dir(api_routes_path) {
      for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
          if let Some(dir_name) = path.file_name().and_then(|s| s.to_str()) {
            modules.push(dir_name.to_string());
          }
        }
      }
    }
  }
  modules.sort();

  let mut all_api_docs = Vec::new();

  for module in &modules {
    println!("cargo:rerun-if-changed=src/routes/api/{}/", module);
    let module_path = api_routes_path.join(module);
    let mut handlers = HashSet::new();
    let mut schemas = HashSet::new();

    process_module_directory(
      &module_path,
      module,
      &mut handlers,
      &mut schemas,
      &format!("{}::", module)
    );

    if handlers.is_empty() {
      continue;
    }

    let pascal_case_module = to_pascal_case(module);

    let mut sorted_handlers: Vec<String> = handlers.into_iter().collect();
    sorted_handlers.sort();
    let mut sorted_schemas: Vec<String> = schemas.into_iter().collect();
    sorted_schemas.sort();

    let api_doc_struct = format!(
      "#[derive(utoipa::OpenApi)]\n#[openapi(\n    paths({}),\n    components(schemas({})),\n    tags((\n        name = \"{}\", description = \"{} endpoints\"\n    ))\n)]\npub struct {}ApiDoc;",
      sorted_handlers.join(",\n        "),
      sorted_schemas.join(", "),
      module,
      module,
      pascal_case_module
    );

    all_api_docs.push((module.clone(), pascal_case_module));

    let module_mod_path = module_path.join("mod.rs");
    let mut module_mod_content = fs::read_to_string(&module_mod_path).unwrap_or_default();

    let start_marker = "// START: UTOIPA DOCS";
    let end_marker = "// END: UTOIPA DOCS";
    let new_doc_block = format!("{}\n{}\n{}", start_marker, api_doc_struct, end_marker);

    if let Some(start) = module_mod_content.find(start_marker) {
      if let Some(end) = module_mod_content.find(end_marker) {
        if start < end {
          module_mod_content.replace_range(start..end + end_marker.len(), &new_doc_block);
        } else {
          module_mod_content.push_str(&format!("\n\n{}", new_doc_block));
        }
      } else {
        module_mod_content.push_str(&format!("\n\n{}", new_doc_block));
      }
    } else {
      module_mod_content.push_str(&format!("\n\n{}", new_doc_block));
    }

    fs::write(&module_mod_path, module_mod_content).unwrap();
  }

  generate_main_mod_file(api_routes_path, &modules, &all_api_docs);
}

fn process_module_directory(
  dir_path: &Path,
  module_name: &str,
  handlers: &mut HashSet<String>,
  schemas: &mut HashSet<String>,
  current_path_prefix: &str
) {
  if let Ok(entries) = fs::read_dir(dir_path) {
    for entry in entries.flatten() {
      let path = entry.path();
      let file_name_str = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("");

      if path.is_dir() {
        let sub_module_name = path
          .file_name()
          .and_then(|s| s.to_str())
          .unwrap();
        process_module_directory(
          &path,
          module_name,
          handlers,
          schemas,
          &format!("{}{}{}", current_path_prefix, sub_module_name, "::")
        );
      } else if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("rs") {
        update_handler_file(
          &path,
          module_name,
          file_name_str,
          handlers,
          schemas,
          current_path_prefix
        );
      }
    }
  }
}

fn update_handler_file(
  path: &PathBuf,
  module_name: &str,
  _file_stem: &str,
  handlers: &mut HashSet<String>,
  schemas: &mut HashSet<String>,
  current_path_prefix: &str
) {
  let mut file_content = fs::read_to_string(path).unwrap();

  let meta_regex = Regex::new(
    r#"(?s)const\s+ENDPOINT_METHOD\s*:\s*&str\s*=\s*"([^"]*)";\s*const\s+ENDPOINT_PATH\s*:\s*&str\s*=\s*"([^"]*)";\s*const\s+ENDPOINT_DESCRIPTION\s*:\s*&str\s*=\s*"([^"]*)";\s*const\s+ENDPOINT_TAG\s*:\s*&str\s*=\s*"([^"]*)";\s*const\s+SUCCESS_RESPONSE_BODY\s*:\s*&str\s*=\s*"([^"]*)";\s*const\s+SLUG_DESCRIPTION\s*:\s*&str\s*=\s*"([^"]*)";"#
  ).unwrap();
  let handler_regex = Regex::new(r"pub async fn\s+([a-zA-Z0-9_]+)\s*\(").unwrap();
  let schema_regex = Regex::new(r"#\[derive\(utoipa::ToSchema\)\].*?pub struct (\w+)").unwrap();

  for cap in schema_regex.captures_iter(&file_content) {
    let schema_name = format!("{}{}", current_path_prefix, &cap[1]);
    schemas.insert(schema_name);
  }

  if let Some(meta_caps) = meta_regex.captures(&file_content) {
    if let Some(handler_caps) = handler_regex.captures(&file_content) {
      let func_name = &handler_caps[1];
      let full_handler_path = format!("{}{}", current_path_prefix, func_name);
      handlers.insert(full_handler_path);

      let method = meta_caps[1].to_lowercase();
      let api_path = &meta_caps[2];
      let description = &meta_caps[3];
      let tag = &meta_caps[4];
      let success_body = &meta_caps[5];
      let slug_description = &meta_caps[6];

      let params_block = if api_path.contains("{slug}") {
        format!(r#"params(("slug" = String, Path, description = "{}"))"#, slug_description)
      } else {
        "".to_string()
      };

      let utoipa_macro_content = format!(
        r#"{}, path = "{}", tag = "{}", responses((status = 200, description = "Success", body = {}), (status = 500, description = "Internal Server Error")), {}"#,
        method,
        api_path,
        tag,
        success_body,
        params_block
      ).replace(", ,", ",");

      let new_doc_block = format!(
        r#"/// {}
#[utoipa::path({})]"#,
        description,
        utoipa_macro_content.trim_end_matches(',')
      );

      let doc_block_regex = Regex::new(
        &format!(r"(?s)^///.*?\n#\[utoipa::path\(.*?\)]\n(pub async fn {})", func_name)
      ).unwrap();

      let original_declaration = format!("pub async fn {}", func_name);
      let new_declaration_with_doc = format!("{}\n{}", new_doc_block, original_declaration);

      let mut needs_update = false;
      if let Some(existing_block) = doc_block_regex.find(&file_content) {
        if existing_block.as_str() != new_declaration_with_doc {
          file_content = file_content.replace(existing_block.as_str(), &new_declaration_with_doc);
          needs_update = true;
        }
      } else {
        if let Some(func_match) = file_content.find(&original_declaration) {
          file_content.replace_range(
            func_match..func_match + original_declaration.len(),
            &new_declaration_with_doc
          );
          needs_update = true;
        }
      }

      if needs_update {
        fs::write(path, file_content).unwrap();
      }
    }
  }
}

fn generate_main_mod_file(
  api_routes_path: &Path,
  modules: &[String],
  all_api_docs: &[(String, String)]
) {
  let mut mod_content = String::new();
  mod_content.push_str("//! THIS FILE IS AUTOMATICALLY GENERATED BY build.rs\n");
  mod_content.push_str("//! DO NOT EDIT THIS FILE MANUALLY\n\n");
  mod_content.push_str("use axum::Router;\n");
  mod_content.push_str("use std::sync::Arc;\n");
  mod_content.push_str("use utoipa::OpenApi;\n");
  mod_content.push_str("use crate::routes::ChatState;\n\n");

  for module in modules {
    mod_content.push_str(&format!("pub mod {};\n", module));
  }
  mod_content.push_str("\n");

  mod_content.push_str("#[derive(OpenApi)]\n#[openapi(\n");
  let nest_entries: Vec<String> = all_api_docs
    .iter()
    .map(|(module, pascal_case)| {
      format!("    (path = \"/api/{}\", api = {}::{}ApiDoc)", module, module, pascal_case)
    })
    .collect();

  if !nest_entries.is_empty() {
    mod_content.push_str("    nest(\n");
    mod_content.push_str(&nest_entries.join(",\n"));
    mod_content.push_str("\n    ),\n");
  }

  mod_content.push_str(
    "    paths(),\n    components(),\n    tags((\n        name = \"api\", description = \"Main API\"\n    ))\n"
  );
  mod_content.push_str(")]\npub struct ApiDoc;\n\n");

  mod_content.push_str("pub fn create_api_routes() -> Router<Arc<ChatState>> {\n");
  let router_content = modules
    .iter()
    .map(|m| format!("        .nest(\"/{}\", {}::create_routes())", m, m))
    .collect::<Vec<_>>()
    .join("\n");
  if router_content.is_empty() {
    mod_content.push_str("    Router::new()\n");
  } else {
    mod_content.push_str("    Router::new()\n");
    mod_content.push_str(&router_content);
    mod_content.push_str("\n");
  }
  mod_content.push_str("}\n");

  let out_path = api_routes_path.join("mod.rs");
  fs::write(out_path, mod_content).unwrap();
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
