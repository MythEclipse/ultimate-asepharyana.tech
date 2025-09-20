use super::param_parsing::generate_detailed_param_doc;

// Enhanced function to generate detailed utoipa macro with comprehensive parameter documentation
pub fn generate_utoipa_macro(
  http_method: &str,
  route_path: &str,
  route_tag: &str,
  response_body: Option<&str>,
  route_description: &str,
  operation_id: &str,
  path_params: &[(String, String)],
  query_params: &[(String, String)],
  is_protected: bool
) -> String {
  let method_ident = match http_method.to_uppercase().as_str() {
    "POST" => "post",
    "PUT" => "put",
    "DELETE" => "delete",
    "PATCH" => "patch",
    "HEAD" => "head",
    "OPTIONS" => "options",
    "TRACE" => "trace",
    _ => "get", // Default to GET
  };

  let params_str = if path_params.is_empty() && query_params.is_empty() {
    String::new()
  } else {
    let mut params: Vec<String> = Vec::new();

    // Add path parameters
    for (name, typ) in path_params {
      let param_doc = generate_detailed_param_doc(name, typ, route_path, "Path");
      params.push(param_doc);
    }

    // Add query parameters
    for (name, typ) in query_params {
      let param_doc = generate_detailed_param_doc(name, typ, route_path, "Query");
      params.push(param_doc);
    }

    if params.is_empty() {
      String::new()
    } else {
      format!(",\n    params(\n        {}\n    )", params.join(",\n        "))
    }
  };

  let body_str = if let Some(body) = response_body {
    format!(", body = {}", body)
  } else {
    String::new()
  };

  let security_str = if is_protected {
    ",\n    security((\"ApiKeyAuth\" = []))"
  } else {
    ""
  };

  format!(
    r#"#[utoipa::path(
    {}{}{},
    path = "{}",
    tag = "{}",
    operation_id = "{}",
    responses(
        (status = 200, description = "{}"{}),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]"#,
    method_ident,
    params_str,
    security_str,
    route_path,
    route_tag,
    operation_id,
    route_description,
    body_str
  )
}
