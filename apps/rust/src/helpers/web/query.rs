//! Query builder helpers for SeaORM.
//!
//! Provides utilities for pagination, filtering, and sorting queries.
//!
//! # Example
//!
//! ```ignore
//! use rustexpress::helpers::query::{QueryBuilder, Pagination, SortOrder};
//!
//! let query = User::find()
//!     .apply_pagination(Pagination::new(1, 20))
//!     .apply_sort("created_at", SortOrder::Desc);
//! ```

use sea_orm::{entity::prelude::*, Order, QuerySelect, Select};
use serde::{Deserialize, Serialize};

/// Pagination parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination {
    /// Current page (1-indexed).
    pub page: u64,
    /// Items per page.
    pub per_page: u64,
    /// Total items (optional, set after query).
    #[serde(skip_deserializing)]
    pub total: Option<u64>,
}

impl Pagination {
    /// Create new pagination.
    pub fn new(page: u64, per_page: u64) -> Self {
        Self {
            page: page.max(1),
            per_page: per_page.clamp(1, 100),
            total: None,
        }
    }

    /// Default pagination (page 1, 20 per page).
    pub fn default_page() -> Self {
        Self::new(1, 20)
    }

    /// Calculate offset for query.
    pub fn offset(&self) -> u64 {
        (self.page - 1) * self.per_page
    }

    /// Calculate total pages.
    pub fn total_pages(&self) -> u64 {
        match self.total {
            Some(total) => (total as f64 / self.per_page as f64).ceil() as u64,
            None => 0,
        }
    }

    /// Check if there's a next page.
    pub fn has_next(&self) -> bool {
        self.page < self.total_pages()
    }

    /// Check if there's a previous page.
    pub fn has_prev(&self) -> bool {
        self.page > 1
    }

    /// Set total after counting.
    pub fn with_total(mut self, total: u64) -> Self {
        self.total = Some(total);
        self
    }
}

impl Default for Pagination {
    fn default() -> Self {
        Self::default_page()
    }
}

/// Sort order.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SortOrder {
    Asc,
    Desc,
}

impl From<SortOrder> for Order {
    fn from(order: SortOrder) -> Self {
        match order {
            SortOrder::Asc => Order::Asc,
            SortOrder::Desc => Order::Desc,
        }
    }
}

/// Sort parameter.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sort {
    pub field: String,
    pub order: SortOrder,
}

impl Sort {
    pub fn new(field: &str, order: SortOrder) -> Self {
        Self {
            field: field.to_string(),
            order,
        }
    }

    pub fn asc(field: &str) -> Self {
        Self::new(field, SortOrder::Asc)
    }

    pub fn desc(field: &str) -> Self {
        Self::new(field, SortOrder::Desc)
    }
}

/// Filter operator.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FilterOp {
    Eq,
    Ne,
    Gt,
    Gte,
    Lt,
    Lte,
    Like,
    In,
    IsNull,
    IsNotNull,
}

/// Filter parameter.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Filter {
    pub field: String,
    pub op: FilterOp,
    pub value: serde_json::Value,
}

impl Filter {
    pub fn eq(field: &str, value: impl Into<serde_json::Value>) -> Self {
        Self {
            field: field.to_string(),
            op: FilterOp::Eq,
            value: value.into(),
        }
    }

    pub fn ne(field: &str, value: impl Into<serde_json::Value>) -> Self {
        Self {
            field: field.to_string(),
            op: FilterOp::Ne,
            value: value.into(),
        }
    }

    pub fn like(field: &str, value: &str) -> Self {
        Self {
            field: field.to_string(),
            op: FilterOp::Like,
            value: serde_json::Value::String(value.to_string()),
        }
    }

    pub fn is_null(field: &str) -> Self {
        Self {
            field: field.to_string(),
            op: FilterOp::IsNull,
            value: serde_json::Value::Null,
        }
    }

    pub fn is_not_null(field: &str) -> Self {
        Self {
            field: field.to_string(),
            op: FilterOp::IsNotNull,
            value: serde_json::Value::Null,
        }
    }
}

/// Query parameters combining pagination, sort, and filters.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QueryParams {
    #[serde(default)]
    pub pagination: Pagination,
    #[serde(default)]
    pub sort: Option<Sort>,
    #[serde(default)]
    pub filters: Vec<Filter>,
    #[serde(default)]
    pub search: Option<String>,
}

impl QueryParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn page(mut self, page: u64) -> Self {
        self.pagination.page = page.max(1);
        self
    }

    pub fn per_page(mut self, per_page: u64) -> Self {
        self.pagination.per_page = per_page.clamp(1, 100);
        self
    }

    pub fn sort_by(mut self, field: &str, order: SortOrder) -> Self {
        self.sort = Some(Sort::new(field, order));
        self
    }

    pub fn filter(mut self, filter: Filter) -> Self {
        self.filters.push(filter);
        self
    }

    pub fn search(mut self, query: &str) -> Self {
        self.search = Some(query.to_string());
        self
    }
}

/// Paginated result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResult<T> {
    pub data: Vec<T>,
    pub pagination: PaginationMeta,
}

/// Pagination metadata for response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationMeta {
    pub page: u64,
    pub per_page: u64,
    pub total: u64,
    pub total_pages: u64,
    pub has_next: bool,
    pub has_prev: bool,
}

impl<T> PaginatedResult<T> {
    pub fn new(data: Vec<T>, pagination: Pagination) -> Self {
        let total = pagination.total.unwrap_or(0);
        let total_pages = pagination.total_pages();

        Self {
            data,
            pagination: PaginationMeta {
                page: pagination.page,
                per_page: pagination.per_page,
                total,
                total_pages,
                has_next: pagination.has_next(),
                has_prev: pagination.has_prev(),
            },
        }
    }
}

/// Extension trait for applying pagination to queries.
pub trait PaginateExt<E: EntityTrait> {
    fn apply_pagination(self, pagination: &Pagination) -> Self;
}

impl<E: EntityTrait> PaginateExt<E> for Select<E> {
    fn apply_pagination(self, pagination: &Pagination) -> Self {
        self.offset(pagination.offset()).limit(pagination.per_page)
    }
}
