//! Automatic OpenAPI annotation generator.
//!
//! This module automatically generates utoipa::path annotations
//! from function signatures, eliminating manual boilerplate.

use regex::Regex;

/// Parse function signature and extract OpenAPI metadata
#[derive(Debug, Clone)]
pub struct FunctionSignature {
    pub name: String,
    pub params: Vec<Parameter>,
    pub return_type: Option<String>,
    pub is_async: bool,
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub param_type: ParameterType,
    pub rust_type: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParameterType {
    Path,
    Query,
    Json,
    Extension,
    State,
}

impl FunctionSignature {
    /// Parse a function signature from source code
    pub fn parse(func_code: &str) -> Option<Self> {
        // Extract function name
        let name_regex = Regex::new(r"(?:pub\s+)?(?:async\s+)?fn\s+(\w+)").ok()?;
        let name = name_regex.captures(func_code)?.get(1)?.as_str().to_string();

        let is_async = func_code.contains("async fn");

        // Extract parameters
        let params = Self::extract_parameters(func_code);

        // Extract return type
        let return_type = Self::extract_return_type(func_code);

        Some(FunctionSignature {
            name,
            params,
            return_type,
            is_async,
        })
    }

    fn extract_parameters(func_code: &str) -> Vec<Parameter> {
        let mut params = Vec::new();

        // Match Path<T>
        if let Ok(re) = Regex::new(r"Path<\s*(\w+)\s*>") {
            for cap in re.captures_iter(func_code) {
                if let Some(type_match) = cap.get(1) {
                    params.push(Parameter {
                        name: "path_param".to_string(), // Will be refined
                        param_type: ParameterType::Path,
                        rust_type: type_match.as_str().to_string(),
                    });
                }
            }
        }

        // Match Query<T>
        if let Ok(re) = Regex::new(r"Query<\s*(\w+)\s*>") {
            for cap in re.captures_iter(func_code) {
                if let Some(type_match) = cap.get(1) {
                    params.push(Parameter {
                        name: "query_params".to_string(),
                        param_type: ParameterType::Query,
                        rust_type: type_match.as_str().to_string(),
                    });
                }
            }
        }

        // Match Json<T>
        if let Ok(re) = Regex::new(r"Json<\s*(\w+)\s*>") {
            for cap in re.captures_iter(func_code) {
                if let Some(type_match) = cap.get(1) {
                    // Skip if this is in return type
                    if !func_code.contains(&format!("-> Json<{}>", type_match.as_str()))
                        && !func_code.contains(&"-> impl IntoResponse".to_string())
                    {
                        params.push(Parameter {
                            name: "body".to_string(),
                            param_type: ParameterType::Json,
                            rust_type: type_match.as_str().to_string(),
                        });
                    }
                }
            }
        }

        params
    }

    fn extract_return_type(func_code: &str) -> Option<String> {
        // Match Json<T> in return type
        if let Ok(re) = Regex::new(r"->\s*Json<\s*(\w+)\s*>") {
            if let Some(cap) = re.captures(func_code) {
                return Some(cap.get(1)?.as_str().to_string());
            }
        }

        // Match impl IntoResponse (generic)
        if func_code.contains("-> impl IntoResponse") {
            return Some("IntoResponse".to_string());
        }

        None
    }
}

/// Generate utoipa::path annotation from signature and route info
pub fn generate_utoipa_annotation(
    sig: &FunctionSignature,
    route_path: &str,
    http_method: &str,
    tag: &str,
) -> String {
    let mut annotation = format!(
        r#"#[utoipa::path(
    {},
    path = "{}",
    tag = "{}","#,
        http_method, route_path, tag
    );

    // Add parameters
    if !sig.params.is_empty() {
        annotation.push_str("\n    params(");

        let param_strs: Vec<String> = sig
            .params
            .iter()
            .filter(|p| p.param_type == ParameterType::Path || p.param_type == ParameterType::Query)
            .map(|p| {
                let param_location = match p.param_type {
                    ParameterType::Path => "Path",
                    ParameterType::Query => "Query",
                    _ => "Path",
                };

                // Try to extract param name from route path
                let param_name =
                    extract_param_name_from_path(route_path).unwrap_or_else(|| p.name.clone());

                format!(
                    r#"
        ("{}" = {}, {}, description = "{}")"#,
                    param_name, p.rust_type, param_location, param_name
                )
            })
            .collect();

        annotation.push_str(&param_strs.join(","));
        annotation.push_str("\n    ),");
    }

    // Add request body
    for param in &sig.params {
        if param.param_type == ParameterType::Json {
            annotation.push_str(&format!(
                r#"
    request_body = {},"#,
                param.rust_type
            ));
            break;
        }
    }

    // Add responses
    annotation.push_str("\n    responses(");

    if let Some(return_type) = &sig.return_type {
        if return_type != "IntoResponse" {
            annotation.push_str(&format!(
                r#"
        (status = 200, description = "Success", body = {}),"#,
                return_type
            ));
        } else {
            annotation.push_str(
                r#"
        (status = 200, description = "Success"),"#,
            );
        }
    } else {
        annotation.push_str(
            r#"
        (status = 200, description = "Success"),"#,
        );
    }

    annotation.push_str(
        r#"
        (status = 500, description = "Internal Server Error")
    )
)]"#,
    );

    annotation
}

/// Extract parameter name from route path
/// e.g., "/users/{id}" -> Some("id")
fn extract_param_name_from_path(path: &str) -> Option<String> {
    let re = Regex::new(r"\{(\w+)\}").ok()?;
    re.captures(path)?.get(1).map(|m| m.as_str().to_string())
}

/// Auto-generate ToSchema derive for structs
pub fn add_to_schema_derive(struct_code: &str) -> String {
    // Check if already has ToSchema
    if struct_code.contains("ToSchema") {
        return struct_code.to_string();
    }

    // Find derive attribute
    if let Some(derive_pos) = struct_code.find("#[derive(") {
        // Insert ToSchema into existing derive
        if let Some(close_paren) = struct_code[derive_pos..].find(")]") {
            let insert_pos = derive_pos + close_paren;
            let mut result = struct_code.to_string();
            result.insert_str(insert_pos, ", ToSchema");
            return result;
        }
    }

    // No derive found, add new one
    if let Some(struct_pos) = struct_code.find("pub struct") {
        let mut result = struct_code.to_string();
        result.insert_str(struct_pos, "#[derive(ToSchema)]\n");
        return result;
    }

    struct_code.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_function() {
        let code = r#"
pub async fn get_user(Path(id): Path<String>) -> Json<UserResponse> {
    // ...
}
"#;

        let sig = FunctionSignature::parse(code).unwrap();
        assert_eq!(sig.name, "get_user");
        assert!(sig.is_async);
        assert_eq!(sig.params.len(), 1);
        assert_eq!(sig.params[0].param_type, ParameterType::Path);
        assert_eq!(sig.return_type, Some("UserResponse".to_string()));
    }

    #[test]
    fn test_extract_param_name() {
        assert_eq!(
            extract_param_name_from_path("/users/{id}"),
            Some("id".to_string())
        );
        assert_eq!(
            extract_param_name_from_path("/posts/{slug}/comments"),
            Some("slug".to_string())
        );
    }

    #[test]
    fn test_add_to_schema() {
        let code = r#"#[derive(Serialize, Deserialize)]
pub struct User {}"#;

        let result = add_to_schema_derive(code);
        assert!(result.contains("ToSchema"));
    }
}
