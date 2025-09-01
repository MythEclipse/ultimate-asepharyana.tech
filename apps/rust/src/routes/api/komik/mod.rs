//! Komik API module re-exports for all komik endpoints
// This module now exposes OpenAPI documentation for all komik endpoints.

use utoipa::OpenApi;

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
        chapter::chapter_handler,
        detail::detail_handler,
        manga::manga_handler,
        manhua::manhua_handler,
        manhwa::manhwa_handler,
        search::search_handler
    ),
    tags(
        (name = "komik", description = "Komik API endpoints")
    )
)]
pub struct KomikApiDoc;
