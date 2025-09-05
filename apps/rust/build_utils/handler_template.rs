use std::fs;
use std::path::Path;
use anyhow::{anyhow, Result};
use regex::Regex;
use crate::build_utils::path_utils::{generate_default_description, sanitize_operation_id, sanitize_tag};
use crate::build_utils::constants::DYNAMIC_REGEX;

pub fn generate_handler_template(path: &Path, root_api_path: &Path) -> Result<()> {
    let file_stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| anyhow!("Could not get file stem from {:?}", path))?
        .replace(['[', ']'], "")
        .replace('-', "_");

    let func_name = &file_stem;

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
    let default_tag = {
        let tag_str = sanitize_tag(&route_path_str);
        if tag_str.is_empty() { "api".to_string() } else { tag_str }
    };
    let operation_id = sanitize_operation_id(&route_path_str);

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

    // Check for bracket notation first (legacy)
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

    // If no bracket params found, check for dynamic patterns
    if path_params.is_empty() {
        let dynamic_patterns = ["_id", "id", "slug", "uuid", "key"];
        for pattern in &dynamic_patterns {
            if route_path_str.ends_with(pattern) {
                let param_name = pattern.trim_start_matches('_');
                path_params.push((param_name.to_string(), "String"));
                break;
            }
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

    // Build response data that includes path parameters
    let response_data = if path_params.is_empty() {
        if response_struct_name == "SearchResponse" {
            r#"
            data: vec![],
            total: None,
            page: None,
            per_page: None,"#.to_string()
        } else if response_struct_name == "ListResponse" {
            r#"
            data: vec![],
            total: None,"#.to_string()
        } else {
            r#"
            data: serde_json::json!(null),"#.to_string()
        }
    } else {
        let param_assignments = path_params.iter()
            .map(|(name, _)| format!("\"{}\": \"{}\"", name, name))
            .collect::<Vec<_>>()
            .join(", ");
        if response_struct_name == "ListResponse" || response_struct_name == "SearchResponse" {
            format!(r#"
            data: vec![serde_json::json!({{{}}})],
            total: Some(1),"#, param_assignments)
        } else {
            format!(r#"
            data: serde_json::json!({{{}}}),"#, param_assignments)
        }
    };

    // Build message that includes path parameter info
    let message_content = if path_params.is_empty() {
        format!("\"Hello from {}!\".to_string()", func_name)
    } else {
        let param_info = path_params.iter()
            .map(|(name, _)| format!("{}: {{{}}}", name, name))
            .collect::<Vec<_>>()
            .join(", ");
        format!("format!(\"Hello from {} with parameters: {}\")", func_name, param_info)
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
            message: {},{}
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
        message_content,
        response_data,
        default_description,
        func_name
      );

    fs::write(path, template)?;
    println!("cargo:warning=Generated new handler template for {:?}", path);
    Ok(())
}
