//! Common types used across build utilities for better type safety.



/// Types of API endpoint templates
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TemplateType {
    /// List endpoint returning multiple items
    List,
    /// Detail endpoint returning a single item
    Detail,
    /// Search endpoint with pagination
    Search,
}

impl TemplateType {
    /// Determine template type from route path
    pub fn from_path(path: &str) -> Self {
        if path.contains("/search") {
            Self::Search
        } else if path.contains('{') || path.contains("/detail") {
            Self::Detail
        } else {
            Self::List
        }
    }

}

/// Information about response structure for templates
#[derive(Debug, Clone)]
pub struct ResponseStructInfo {
    /// Name of the response struct
    pub struct_name: &'static str,
    /// Field definitions for the struct
    pub fields: &'static str,
    /// Type string for SUCCESS_RESPONSE_BODY
    pub success_body: &'static str,
}

impl ResponseStructInfo {
    /// Create from a template type
    pub fn from_template_type(template_type: TemplateType) -> Self {
        match template_type {
            TemplateType::Search => Self {
                struct_name: "SearchResponse",
                fields: r#"
    /// Success message
    pub message: String,
    /// Search results - replace with actual Vec<T> where T implements ToSchema
    pub data: Vec<serde_json::Value>,
    /// Total number of results
    pub total: Option<u64>,
    /// Current page
    pub page: Option<u32>,
    /// Results per page
    pub per_page: Option<u32>,"#,
                success_body: "Json<SearchResponse>",
            },
            TemplateType::Detail => Self {
                struct_name: "DetailResponse",
                fields: r#"
    /// Success message
    pub message: String,
    /// Detailed data - replace with actual T where T implements ToSchema
    pub data: serde_json::Value,"#,
                success_body: "Json<DetailResponse>",
            },
            TemplateType::List => Self {
                struct_name: "ListResponse",
                fields: r#"
    /// Success message
    pub message: String,
    /// List of items - replace with actual Vec<T> where T implements ToSchema
    pub data: Vec<serde_json::Value>,
    /// Total number of items
    pub total: Option<u64>,"#,
                success_body: "Json<ListResponse>",
            },
        }
    }
}


/// Dynamic parameter information
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DynamicParam {
    /// Parameter name (e.g., "id", "slug")
    pub name: String,
    /// Parameter type (e.g., "String", "Vec<String>")
    pub param_type: String,
    /// Whether this is a catch-all parameter ([...param])
    pub is_catch_all: bool,
}

/// Information about a route file for auto-routing
#[derive(Debug, Clone)]
pub struct RouteFileInfo {
    /// Absolute path to the file
    pub file_path: std::path::PathBuf,
    /// Route path (e.g., "/users/{id}")
    pub route_path: String,
    /// Whether this route contains dynamic segments
    pub is_dynamic: bool,
}

impl RouteFileInfo {

    /// Get the file stem (filename without extension)
    pub fn file_stem(&self) -> Option<String> {
        self.file_path
            .file_stem()
            .and_then(|s| s.to_str())
            .map(|s| s.to_string())
    }

    /// Get module name for this route
    pub fn module_name(&self) -> Option<String> {
        self.file_stem().map(|stem| {
            stem.trim_matches(|c| c == '[' || c == ']')
                .replace("...", "")
                .replace('-', "_")
        })
    }
}
