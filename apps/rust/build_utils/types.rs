//! Common types used across build utilities for better type safety.

use std::fmt;

/// HTTP methods supported by the API handlers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Patch,
}

impl HttpMethod {
    /// Parse an HTTP method from a string
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "get" => Some(Self::Get),
            "post" => Some(Self::Post),
            "put" => Some(Self::Put),
            "delete" => Some(Self::Delete),
            "patch" => Some(Self::Patch),
            _ => None,
        }
    }

    /// Get the lowercase string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Get => "get",
            Self::Post => "post",
            Self::Put => "put",
            Self::Delete => "delete",
            Self::Patch => "patch",
        }
    }
}

impl fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Types of API endpoint templates
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TemplateType {
    /// List endpoint returning multiple items
    List,
    /// Detail endpoint returning a single item
    Detail,
    /// Search endpoint with pagination
    Search,
    /// Custom response structure
    Custom,
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

    /// Get the response struct name for this template type
    pub fn response_struct_name(&self) -> &'static str {
        match self {
            Self::List => "ListResponse",
            Self::Detail => "DetailResponse",
            Self::Search => "SearchResponse",
            Self::Custom => "CustomResponse",
        }
    }

    /// Get the success response body type
    pub fn success_response_body(&self) -> &'static str {
        match self {
            Self::List => "Json<ListResponse>",
            Self::Detail => "Json<DetailResponse>",
            Self::Search => "Json<SearchResponse>",
            Self::Custom => "Json<CustomResponse>",
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
            TemplateType::List | TemplateType::Custom => Self {
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

/// Metadata for an API endpoint
#[derive(Debug, Clone)]
pub struct EndpointMetadata {
    pub http_method: String,
    pub route_path: String,
    pub route_tag: String,
    pub operation_id: String,
    pub route_description: String,
    pub response_body: Option<String>,
    pub axum_path: String,
}
