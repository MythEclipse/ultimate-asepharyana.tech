use std::env;
use std::fs;
use std::path::PathBuf;
use std::process;

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() < 3 || args.len() > 4 {
    eprintln!("❌ Usage: cargo run --bin scaffold -- <module_name> <endpoint_name> [endpoint_path]");
    eprintln!("   Example 1 (default path): cargo run --bin scaffold -- anime search");
    eprintln!("   Example 2 (custom path): cargo run --bin scaffold -- user profile /api/user/{{id}}/profile");
    process::exit(1);
  }

  let module_name = &args[1];
  let endpoint_name = &args[2];
  let custom_endpoint_path = if args.len() == 4 {
    Some(args[3].clone())
  } else {
    None
  };

  let mut dir_path = PathBuf::from("src");
  dir_path.push("routes");
  dir_path.push("api");
  dir_path.push(module_name);
  dir_path.push(endpoint_name);

  // Define paths for the handler file
  let mut handler_file_path = dir_path.clone();
  handler_file_path.push(format!("{}_handler.rs", endpoint_name));

  // Check if handler file already exists
  if handler_file_path.exists() {
    println!("⚠️ Handler file already exists at {:?}. No changes were made.", handler_file_path);
    process::exit(0);
  }

  if let Err(e) = fs::create_dir_all(&dir_path) {
    eprintln!("Failed to create directory {:?}: {}", dir_path, e);
    process::exit(1);
  }

  let pascal_endpoint = to_pascal_case(endpoint_name);

  let endpoint_path = custom_endpoint_path.unwrap_or_else(|| {
    format!("/api/{}/{}/{{slug}}", module_name, endpoint_name)
  });

  let handler_template = format!(
    r#"// --- METADATA UNTUK BUILD.RS ---
const ENDPOINT_METHOD: &str = "GET";
const ENDPOINT_PATH: &str = "{}";
const ENDPOINT_DESCRIPTION: &str = "Description for {} endpoint.";
const ENDPOINT_TAG: &str = "{}";
const SUCCESS_RESPONSE_BODY: &str = "{}Response";
const SLUG_DESCRIPTION: &str = "Description for the slug parameter.";
// --- AKHIR METADATA ---

use axum::{{
    extract::Path,
    response::{{IntoResponse, Response}},
    Json,
}};
use serde::{{Deserialize, Serialize}};
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct {}Data {{
    // TODO: Definisikan field data Anda di sini
    pub message: String,
}}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct {}Response {{
    pub status: &'static str,
    pub data: {}Data,
}}

/// {}
pub async fn {}_handler(Path(slug): Path<String>) -> Response {{
    // TODO: Implementasikan logika handler Anda di sini
    let response_data = {}Data {{
        message: format!("Data for slug: {{}}", slug),
    }};

    let response = {}Response {{
        status: "Ok",
        data: response_data,
    }};

    Json(response).into_response()
}}
"#,
    endpoint_path,
    endpoint_name,
    module_name,
    pascal_endpoint,
    pascal_endpoint,
    pascal_endpoint,
    pascal_endpoint,
    endpoint_name,
    endpoint_name,
    pascal_endpoint,
    pascal_endpoint
  );

  // Write the handler file
  if let Err(e) = fs::write(&handler_file_path, handler_template) {
    eprintln!("Failed to write to handler file {:?}: {}", handler_file_path, e);
    process::exit(1);
  }

  println!("✅ Endpoint template created successfully at: {:?}", dir_path);
}

fn to_pascal_case(s: &str) -> String {
  s.split('_')
    .map(|word| {
      let mut chars = word.chars();
      match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
      }
    })
    .collect()
}
