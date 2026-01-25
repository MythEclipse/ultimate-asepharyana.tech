use crate::build_utils::constants::DYNAMIC_REGEX;
use crate::build_utils::path_utils::{
    generate_default_description, sanitize_operation_id, sanitize_tag,
};
use crate::build_utils::types::{ResponseStructInfo, TemplateType};
use regex::Regex;

pub fn generate_template_content(
    path: &std::path::Path,
    root_api_path: &std::path::Path,
    protected: bool,
) -> anyhow::Result<String> {
    let file_stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| anyhow::anyhow!("Could not get file stem from {:?}", path))?
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
        route_path_str = route_path_str
            .strip_suffix("/index")
            .unwrap_or("")
            .to_string();
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
        if tag_str.is_empty() {
            "api".to_string()
        } else {
            tag_str
        }
    };
    let operation_id = sanitize_operation_id(&route_path_str);

    let template_type = TemplateType::from_path(&axum_path);
    let response_info = ResponseStructInfo::from_template_type(template_type);

    let path_params = extract_path_params_from_route(&route_path_str);

    let func_signature = build_function_signature(func_name, &path_params, protected);

    let response_data = build_response_data(response_info.struct_name, &path_params);

    let message_content = build_message_content(func_name, &path_params);

    let imports = build_imports(&path_params, protected);

    let security = if protected {
        r#"
    security(
        ("ApiKeyAuth" = [])
    ),"#
    } else {
        ""
    };
    let middleware_layer = if protected {
        "let router = router.layer(AuthMiddleware::layer());"
    } else {
        ""
    };

    let auth_verification = if protected {
        r#"
    "#
    } else {
        ""
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

    #[utoipa::path(
        get,
        params(
{}
        ),
        path = "/{}",
        tag = "{}",
        operation_id = "{}",
        responses(
            (status = 200, description = "{}", body = {}),
            (status = 401, description = "Unauthorized", body = String),
            (status = 500, description = "Internal Server Error", body = String)
        ){}
    )]
    {} {{
        {}{}
        Json({} {{
            message: {},{}
        }})
    }}

    /// {}
    pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {{
        {middleware_layer}
        router.route(ENDPOINT_PATH, get({}))
    }}
    "#,
        func_name,
        imports,
        axum_path,
        default_description,
        default_tag,
        operation_id,
        response_info.success_body,
        pascal_case_name,
        response_info.struct_name,
        response_info.fields,
        if path_params.is_empty() {
            "".to_string()
        } else {
            path_params
                .iter()
                .map(|(name, _)| {
                    format!(r#"("{}" = String, Path, description = "Parameter")"#, name)
                })
                .collect::<Vec<_>>()
                .join(",\n        ")
        },
        axum_path,
        default_tag,
        operation_id,
        default_description,
        response_info.struct_name,
        security,
        func_signature,
        auth_verification,
        if protected { "\n    " } else { "" },
        response_info.struct_name,
        message_content,
        response_data,
        default_description,
        func_name
    );

    Ok(template)
}

pub fn extract_path_params_from_route(route_path_str: &str) -> Vec<(String, String)> {
    let mut path_params = Vec::new();

    // Check for bracket notation first (legacy)
    for cap in DYNAMIC_REGEX.captures_iter(route_path_str) {
        let param = &cap[1];
        if param.starts_with("...") {
            // Catch-all parameter
            let param_name = param.strip_prefix("...").unwrap_or(param);
            path_params.push((param_name.to_string(), "Vec<String>".to_string()));
        } else {
            path_params.push((param.to_string(), "String".to_string()));
        }
    }

    // If no bracket params found, check for dynamic patterns
    if path_params.is_empty() {
        let dynamic_patterns = ["_id", "id", "slug", "uuid", "key"];
        for pattern in &dynamic_patterns {
            if route_path_str.ends_with(pattern) {
                let param_name = pattern.trim_start_matches('_');
                path_params.push((param_name.to_string(), "String".to_string()));
                break;
            }
        }
    }
    path_params
}

pub fn build_function_signature(
    func_name: &str,
    path_params: &[(String, String)],
    protected: bool,
) -> String {
    let mut params = Vec::new();

    if protected {
        params.push("Extension(claims): Extension<Claims>".to_string());
    }

    if path_params.is_empty() {
        if params.is_empty() {
            format!("pub async fn {}() -> impl IntoResponse", func_name)
        } else {
            format!(
                "pub async fn {}({}) -> impl IntoResponse",
                func_name,
                params.join(", ")
            )
        }
    } else {
        let path_params_str = path_params
            .iter()
            .map(|(name, typ)| format!("Path({}): Path<{}>", name, typ))
            .collect::<Vec<_>>()
            .join(", ");
        params.push(path_params_str);
        format!(
            "pub async fn {}({}) -> impl IntoResponse",
            func_name,
            params.join(", ")
        )
    }
}

pub fn build_response_data(response_struct_name: &str, path_params: &[(String, String)]) -> String {
    if path_params.is_empty() {
        if response_struct_name == "SearchResponse" {
            r#"
            data: vec![],
            total: None,
            page: None,
            per_page: None,"#
                .to_string()
        } else if response_struct_name == "ListResponse" {
            r#"
            data: vec![],
            total: None,"#
                .to_string()
        } else {
            r#"
            data: serde_json::json!(null),"#
                .to_string()
        }
    } else {
        let param_assignments = path_params
            .iter()
            .map(|(name, _)| format!("\"{}\": \"{}\"", name, name))
            .collect::<Vec<_>>()
            .join(", ");
        if response_struct_name == "ListResponse" || response_struct_name == "SearchResponse" {
            format!(
                r#"
            data: vec![serde_json::json!({{{}}})],
            total: Some(1),"#,
                param_assignments
            )
        } else {
            format!(
                r#"
            data: serde_json::json!({{{}}}),"#,
                param_assignments
            )
        }
    }
}

pub fn build_message_content(func_name: &str, path_params: &[(String, String)]) -> String {
    if path_params.is_empty() {
        format!("\"Hello from {}!\".to_string()", func_name)
    } else {
        let param_info = path_params
            .iter()
            .map(|(name, _)| format!("{}: {{{}}}", name, name))
            .collect::<Vec<_>>()
            .join(", ");
        format!(
            "format!(\"Hello from {} with parameters: {}\")",
            func_name, param_info
        )
    }
}

pub fn build_imports(path_params: &[(String, String)], protected: bool) -> String {
    let mut imports = Vec::new();

    if protected {
        imports.push("use crate::middleware::auth::AuthMiddleware;".to_string());
        imports.push("use crate::utils::auth::Claims;".to_string());
        imports.push("use axum::Extension;".to_string());
    }

    if path_params.is_empty() {
        imports.push("use axum::{response::IntoResponse, routing::get, Json, Router};".to_string());
    } else {
        imports.push(
            "use axum::{extract::Path, response::IntoResponse, routing::get, Json, Router};"
                .to_string(),
        );
    }

    imports.join("\n")
}
