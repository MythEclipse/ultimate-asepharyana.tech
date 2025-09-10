use regex::Regex;
use anyhow::{ anyhow, Result };

// Function to parse Query struct and extract query parameters
pub fn parse_query_params(content: &str) -> Result<Vec<(String, String)>> {
  let mut query_params = Vec::new();

  // Find Query struct usage in function signature
  let query_regex = Regex::new(r"Query\((\w+)\):\s*Query<(\w+)>").map_err(|e|
    anyhow!("Invalid query regex pattern: {}", e)
  )?;

  if let Some(cap) = query_regex.captures(content) {
    let _param_name = &cap[1];
    let struct_name = &cap[2];

    // Find the struct definition line
    let struct_line_pattern = format!("struct {} {{", struct_name);
    if let Some(start_pos) = content.find(&struct_line_pattern) {
      // Find the end of the struct
      let struct_content = &content[start_pos..];
      if let Some(end_pos) = struct_content.find('}') {
        let struct_body = &struct_content[struct_line_pattern.len()..end_pos];

        // Parse fields from struct body
        let field_regex = Regex::new(r"pub\s+(\w+):\s*([^,]+),?").map_err(|e|
          anyhow!("Invalid field regex pattern: {}", e)
        )?;

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
  param_type: &str
) -> String {
  let description = match name {
    "id" => format!("Unique identifier for the resource (UUID format recommended)"),
    "slug" =>
      format!("URL-friendly identifier for the resource (typically lowercase with hyphens)"),
    "uuid" => format!("Universally unique identifier for the resource"),
    "user_id" => format!("User identifier (can be UUID, username, or numeric ID)"),
    "chapter_id" => format!("Chapter identifier for manga/comic content"),
    "komik_id" => format!("Comic/manga identifier"),
    "anime_id" => format!("Anime series identifier"),
    "page" => format!("Page number for pagination (starts from 1)"),
    "limit" => format!("Maximum number of items to return (default: 20, max: 100)"),
    "offset" => format!("Number of items to skip for pagination"),
    "search" => format!("Search query string for filtering results"),
    "category" => format!("Category filter for content organization"),
    "status" => format!("Status filter (active, inactive, pending, etc.)"),
    "type" | "r#type" => format!("Content type filter"),
    "sort" => format!("Sort order (asc, desc, or field name)"),
    "order" => format!("Sort direction (ascending or descending)"),
    "file_name" => format!("Name of the file to access or download"),
    "chapter" => format!("Chapter number for content navigation"),
    "episode" => format!("Episode number for series content"),
    _ => {
      // Generate context-aware description based on route path
      if route_path.contains("detail") {
        format!("Identifier for the detailed resource view")
      } else if route_path.contains("search") {
        format!("Search parameter for filtering results")
      } else if route_path.contains("chapter") {
        format!("Chapter-specific identifier")
      } else if route_path.contains("episode") {
        format!("Episode-specific identifier")
      } else {
        format!("Parameter for resource identification")
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

  format!(r#"("{}" = {}, {}, description = "{}", {})"#, name, typ, param_type, description, example)
}

// Parse path parameters from existing function signature
pub fn parse_path_params_from_signature(content: &str) -> Result<Vec<(String, String)>> {
  let mut path_params = Vec::new();

  // Look for Path<...> patterns in function signatures
  let path_regex = Regex::new(r"Path\((\w+)\):\s*Path<([^>]+)>").map_err(|e|
    anyhow!("Invalid path regex pattern: {}", e)
  )?;

  for cap in path_regex.captures_iter(content) {
    let param_name = cap[1].to_string();
    let param_type = cap[2].to_string();
    path_params.push((param_name, param_type));
  }

  Ok(path_params)
}
