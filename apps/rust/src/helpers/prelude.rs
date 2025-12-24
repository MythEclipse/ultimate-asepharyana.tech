//! Prelude module - common imports for handlers.
//!
//! # Usage
//!
//! ```ignore
//! use rust::helpers::prelude::*;
//! ```

// Re-export common types
pub use axum::{
    extract::{Extension, Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
pub use serde::{Deserialize, Serialize};
pub use tracing::{debug, error, info, warn};

// Re-export our helpers
pub use super::response::{ApiResult, JsonResponse, ErrorResponse};
pub use super::pagination::{Paginated, PaginationParams};

// Re-export common extractors
pub use crate::extractors::{ValidatedJson, ValidatedQuery};
pub use crate::observability::RequestId;

// Re-export database types
pub use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait,
    QueryFilter, QueryOrder, QuerySelect, Set,
};

// Re-export common std types
pub use std::sync::Arc;
