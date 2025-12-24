//! Pagination helpers.

use serde::{Deserialize, Serialize};

/// Pagination query parameters.
#[derive(Debug, Clone, Deserialize)]
pub struct PaginationParams {
    /// Page number (1-indexed).
    #[serde(default = "default_page")]
    pub page: u64,
    /// Items per page.
    #[serde(default = "default_limit")]
    pub limit: u64,
    /// Sort field.
    #[serde(default)]
    pub sort: Option<String>,
    /// Sort order (asc/desc).
    #[serde(default = "default_order")]
    pub order: String,
}

fn default_page() -> u64 { 1 }
fn default_limit() -> u64 { 20 }
fn default_order() -> String { "asc".to_string() }

impl PaginationParams {
    /// Calculate offset for database query.
    pub fn offset(&self) -> u64 {
        (self.page.saturating_sub(1)) * self.limit
    }

    /// Check if ascending order.
    pub fn is_asc(&self) -> bool {
        self.order.to_lowercase() == "asc"
    }
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self {
            page: default_page(),
            limit: default_limit(),
            sort: None,
            order: default_order(),
        }
    }
}

/// Paginated response wrapper.
#[derive(Debug, Clone, Serialize)]
pub struct Paginated<T> {
    pub data: Vec<T>,
    pub pagination: PaginationMeta,
}

/// Pagination metadata.
#[derive(Debug, Clone, Serialize)]
pub struct PaginationMeta {
    pub page: u64,
    pub limit: u64,
    pub total: u64,
    pub total_pages: u64,
    pub has_next: bool,
    pub has_prev: bool,
}

impl<T> Paginated<T> {
    /// Create a paginated response.
    pub fn new(data: Vec<T>, page: u64, limit: u64, total: u64) -> Self {
        let total_pages = (total + limit - 1) / limit;
        Self {
            data,
            pagination: PaginationMeta {
                page,
                limit,
                total,
                total_pages,
                has_next: page < total_pages,
                has_prev: page > 1,
            },
        }
    }

    /// Create from params and total.
    pub fn from_params(data: Vec<T>, params: &PaginationParams, total: u64) -> Self {
        Self::new(data, params.page, params.limit, total)
    }
}

/// Trait to easily paginate database results.
pub trait Paginatable<T> {
    fn paginate(self, params: &PaginationParams, total: u64) -> Paginated<T>;
}

impl<T> Paginatable<T> for Vec<T> {
    fn paginate(self, params: &PaginationParams, total: u64) -> Paginated<T> {
        Paginated::from_params(self, params, total)
    }
}
