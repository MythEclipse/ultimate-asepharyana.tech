use anyhow::{anyhow, Result};
use regex::Regex;

pub fn enhance_response_struct(content: &str, axum_path: &str) -> Result<String> {
    let struct_regex = Regex::new(
        r"(?ms)#\[derive\([^)]*\)\]\s*pub struct (\w+Response)\s*\{\s*pub message: String,\s*\}",
    )
    .map_err(|e| anyhow!("Failed to create response struct regex: {}", e))?;

    if let Some(cap) = struct_regex.captures(content) {
        let struct_name = &cap[1];
        let enhanced_struct = if axum_path.contains("/search") {
            format!(
                r#"/// Response structure for search endpoints.
/// Replace `serde_json::Value` with your actual data types and implement `utoipa::ToSchema` for complex types.
#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct {} {{
    /// Success message
    pub message: String,
    /// Search results - replace with actual Vec<T> where T implements ToSchema
    pub data: Vec<serde_json::Value>,
    /// Total number of results
    pub total: Option<u64>,
    /// Current page
    pub page: Option<u32>,
    /// Results per page
    pub per_page: Option<u32>,
}}"#,
                struct_name
            )
        } else if axum_path.contains('{') || axum_path.contains("/detail") {
            format!(
                r#"/// Response structure for detail endpoints.
/// Replace `serde_json::Value` with your actual data type and implement `utoipa::ToSchema` for complex types.
#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct {} {{
    /// Success message
    pub message: String,
    /// Detailed data - replace with actual T where T implements ToSchema
    pub data: serde_json::Value,
}}"#,
                struct_name
            )
        } else {
            format!(
                r#"/// Response structure for list endpoints.
/// Replace `serde_json::Value` with your actual data types and implement `utoipa::ToSchema` for complex types.
#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct {} {{
    /// Success message
    pub message: String,
    /// List of items - replace with actual Vec<T> where T implements ToSchema
    pub data: Vec<serde_json::Value>,
    /// Total number of items
    pub total: Option<u64>,
}}"#,
                struct_name
            )
        };

        let old_struct_regex = Regex::new(
            r"(?ms)#\[derive\([^)]*\)\]\s*pub struct \w+Response\s*\{\s*pub message: String,\s*\}",
        )
        .map_err(|e| anyhow!("Failed to create old struct regex: {}", e))?;

        Ok(old_struct_regex
            .replace(content, &enhanced_struct)
            .to_string())
    } else {
        Ok(content.to_string())
    }
}
