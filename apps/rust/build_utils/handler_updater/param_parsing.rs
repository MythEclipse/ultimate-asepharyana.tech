use anyhow::{anyhow, Result};
use regex::Regex;

// Function to parse Query struct and extract query parameters
pub fn parse_query_params(content: &str) -> Result<Vec<(String, String)>> {
    let mut query_params = Vec::new();

    // Find Query struct usage in function signature
    let query_regex = Regex::new(r"Query\((\w+)\):\s*Query<(\w+)>")
        .map_err(|e| anyhow!("Invalid query regex pattern: {}", e))?;

    if let Some(cap) = query_regex.captures(content) {
        let _param_name = &cap[1];
        let struct_name = &cap[2];

        // Find the struct definition line (with optional pub)
        let pub_struct_pattern = format!("pub struct {} {{", struct_name);
        let struct_pattern = format!("struct {} {{", struct_name);
        let start_pos = content
            .find(&pub_struct_pattern)
            .or_else(|| content.find(&struct_pattern));
        if let Some(start_pos) = start_pos {
            // Determine which pattern was found and its length
            let pattern_len = if content[start_pos..].starts_with(&pub_struct_pattern) {
                pub_struct_pattern.len()
            } else {
                struct_pattern.len()
            };
            // Find the end of the struct
            let struct_content = &content[start_pos..];
            if let Some(end_pos) = struct_content.find('}') {
                let struct_body = &struct_content[pattern_len..end_pos];

                // Parse fields from struct body
                let field_regex = Regex::new(r"(?:pub\s+)?(\w+):\s*([^,]+),?")
                    .map_err(|e| anyhow!("Invalid field regex pattern: {}", e))?;

                for field_cap in field_regex.captures_iter(struct_body) {
                    let field_name = field_cap[1].to_string();
                    let field_type = field_cap[2].trim().to_string();
                    query_params.push((field_name, field_type));
                }
            }
        }
    }

    Ok(query_params)
}

// Generate detailed parameter documentation based on parameter name and context
pub fn generate_detailed_param_doc(
    name: &str,
    typ: &str,
    route_path: &str,
    param_type: &str,
) -> String {
    let description = match name {
        "id" => "Unique identifier for the resource (UUID format recommended)".to_string(),
        "slug" => "URL-friendly identifier for the resource (typically lowercase with hyphens)"
            .to_string(),
        "uuid" => "Universally unique identifier for the resource".to_string(),
        "user_id" => "User identifier (can be UUID, username, or numeric ID)".to_string(),
        "chapter_id" => "Chapter identifier for manga/comic content".to_string(),
        "komik_id" => "Comic/manga identifier".to_string(),
        "anime_id" => "Anime series identifier".to_string(),
        "page" => "Page number for pagination (starts from 1)".to_string(),
        "limit" => "Maximum number of items to return (default: 20, max: 100)".to_string(),
        "offset" => "Number of items to skip for pagination".to_string(),
        "search" => "Search query string for filtering results".to_string(),
        "category" => "Category filter for content organization".to_string(),
        "status" => "Status filter (active, inactive, pending, etc.)".to_string(),
        "type" | "r#type" => "Content type filter".to_string(),
        "sort" => "Sort order (asc, desc, or field name)".to_string(),
        "order" => "Sort direction (ascending or descending)".to_string(),
        "file_name" => "Name of the file to access or download".to_string(),
        "chapter" => "Chapter number for content navigation".to_string(),
        "episode" => "Episode number for series content".to_string(),
        _ => {
            // Generate context-aware description based on route path
            if route_path.contains("detail") {
                "Identifier for the detailed resource view".to_string()
            } else if route_path.contains("search") {
                "Search parameter for filtering results".to_string()
            } else if route_path.contains("chapter") {
                "Chapter-specific identifier".to_string()
            } else if route_path.contains("episode") {
                "Episode-specific identifier".to_string()
            } else {
                "Parameter for resource identification".to_string()
            }
        }
    };

    let example = match name {
        "id" | "uuid" => r#"example = "550e8400-e29b-41d4-a716-446655440000""#,
        "slug" => {
            // Use "1" as default for complete-anime and ongoing-anime slug parameters
            if route_path.contains("complete-anime") || route_path.contains("ongoing-anime") {
                r#"example = "1""#
            } else {
                r#"example = "naruto-shippuden-episode-1""#
            }
        }
        "page" => r#"example = 1, minimum = 1"#,
        "limit" => r#"example = 20, minimum = 1, maximum = 100"#,
        "offset" => r#"example = 0, minimum = 0"#,
        "search" => r#"example = "naruto""#,
        "chapter" => r#"example = "15""#,
        "episode" => r#"example = "24""#,
        "file_name" => r#"example = "document.pdf""#,
        _ => r#"example = "sample_value""#,
    };

    format!(
        r#"("{}" = {}, {}, description = "{}", {})"#,
        name, typ, param_type, description, example
    )
}
