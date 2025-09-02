//! Komik API module re-exports for all komik endpoints
// This module now exposes OpenAPI documentation for all komik endpoints.

use axum::{routing::get, Router};
use crate::routes::ChatState;
use std::sync::Arc;

pub mod chapter;
pub mod detail;
pub mod manga;
pub mod manhua;
pub mod manhwa;
pub mod search;

/// Aggregates OpenAPI docs for all komik endpoints.

pub fn create_routes() -> Router<Arc<ChatState>> {
    Router::new()
        .route("/chapter", get(chapter::chapter_handler))
        .route("/detail", get(detail::detail_handler))
        .route("/manga", get(manga::manga_handler))
        .route("/manhua", get(manhua::manhua_handler))
        .route("/manhwa", get(manhwa::manhwa_handler))
        .route("/search", get(search::search_handler))
}
