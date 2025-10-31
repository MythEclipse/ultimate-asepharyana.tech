use crate::build_utils::constants::DYNAMIC_REGEX;
use regex::Regex;

pub fn extract_path_params(axum_path: &str) -> Vec<(String, String)> {
    let re = Regex::new(r"\{([^}]+)\}").unwrap();
    re.captures_iter(axum_path)
        .map(|cap| {
            let param = &cap[1];
            if param.starts_with("...") {
                (
                    param.strip_prefix("...").unwrap().to_string(),
                    "Vec<String>".to_string(),
                )
            } else {
                (param.to_string(), "String".to_string())
            }
        })
        .collect()
}

pub fn parse_path_params_from_signature(
    content: &str,
) -> Result<Vec<(String, String)>, anyhow::Error> {
    // Look for Path parameters directly
    let param_regex = Regex::new(r"Path\((\w+)\):\s*Path<([^>]+)>")
        .map_err(|e| anyhow::anyhow!("Invalid path regex pattern: {}", e))?;
    Ok(param_regex
        .captures_iter(content)
        .map(|cap| {
            let name = cap[1].to_string();
            let typ = cap[2].to_string();
            (name, typ)
        })
        .collect())
}

pub fn generate_default_description(path_str: &str, method: &str) -> String {
    let path_segments: Vec<&str> = path_str.trim_matches('/').split('/').collect();
    let last_segment = path_segments.last().unwrap_or(&"");
    let second_last_segment = if path_segments.len() > 1 {
        path_segments[path_segments.len() - 2]
    } else {
        ""
    };

    match *last_segment {
        "search" => format!(
            "Searches for {} based on query parameters.",
            second_last_segment
        ),
        "detail" => format!(
            "Retrieves details for a specific {} by ID.",
            second_last_segment
        ),
        "list" => format!("Retrieves a list of {}.", second_last_segment),
        "index" => {
            if second_last_segment.is_empty() {
                format!(
                    "Handles {} requests for the root API endpoint.",
                    method.to_uppercase()
                )
            } else {
                format!(
                    "Handles {} requests for the {} index endpoint.",
                    method.to_uppercase(),
                    second_last_segment
                )
            }
        }
        "[slug]" => format!(
            "Retrieves details for a specific {} by slug.",
            second_last_segment
        ),
        "[[...file]]" => format!(
            "Handles file operations (upload/download) for {}.",
            second_last_segment
        ),
        _ => format!(
            "Handles {} requests for the {} endpoint.",
            method.to_uppercase(),
            path_str
        ),
    }
}

pub fn sanitize_operation_id(path_str: &str) -> String {
    let s = path_str.replace([std::path::MAIN_SEPARATOR, '-'], "_");
    let s = DYNAMIC_REGEX
        .replace_all(&s, |caps: &regex::Captures| {
            let inner = &caps[1];
            if inner.starts_with("...") {
                "_catch_all".to_string()
            } else {
                format!("_{}", inner)
            }
        })
        .to_string();
    s.trim_matches('_').replace("__", "_")
}

pub fn sanitize_tag(path_str: &str) -> String {
    // Only take the first part of the path before any separator
    let first_part = path_str
        .split(std::path::MAIN_SEPARATOR)
        .next()
        .unwrap_or("");
    let s = first_part.replace('-', "_");
    let s = DYNAMIC_REGEX
        .replace_all(&s, |caps: &regex::Captures| {
            let inner = &caps[1];
            if inner.starts_with("...") {
                "_catch_all".to_string()
            } else {
                format!("_{}", inner)
            }
        })
        .to_string();
    s.trim_matches('_').to_string()
}

pub fn is_dynamic_route_content(content: &str) -> bool {
    content.contains("//! DYNAMIC_ROUTE")
}
