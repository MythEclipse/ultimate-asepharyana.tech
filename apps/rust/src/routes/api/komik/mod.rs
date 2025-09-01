//! Komik API module re-exports for all komik endpoints
// This module now exposes OpenAPI documentation for all komik endpoints.

use axum::{routing::get, Router};
use utoipa::OpenApi;
use crate::routes::ChatState;
use std::sync::Arc;

pub mod chapter;
pub mod detail;
pub mod manga;
pub mod manhua;
pub mod manhwa;
pub mod search;

/// Aggregates OpenAPI docs for all komik endpoints.
#[derive(OpenApi)]
#[openapi(
    paths(
        chapter::handler,
        detail::handler,
        manga::handler,
        manhua::handler,
        manhwa::handler,
        search::handler
    ),
    tags(
        (name = "komik", description = "Komik API endpoints")
    )
)]
pub struct KomikApiDoc;

pub fn create_routes() -> Router<Arc<ChatState>> {
    Router::new()
        .route("/chapter", get(chapter::handler))
        .route("/detail", get(detail::handler))
        .route("/manga", get(manga::handler))
        .route("/manhua", get(manhua::handler))
        .route("/manhwa", get(manhwa::handler))
        .route("/search", get(search::handler))
}
