//! API module re-exports for all implemented endpoints
// This module now exposes OpenAPI documentation for all API groups.

use utoipa::OpenApi;

pub mod komik;
pub mod anime;
pub mod anime2;
pub mod uploader;
pub mod proxy;
pub mod compress;
pub mod drivepng;
pub mod chat;

/// Aggregates OpenAPI docs for all API groups.
#[derive(OpenApi)]
#[openapi(
    tags(
        (name = "api", description = "Root API module")
    ),
    nest(
        (path = "/api/komik", api = komik::KomikApiDoc),
        (path = "/api/anime", api = anime::AnimeApiDoc),
        (path = "/api/anime2", api = anime2::Anime2ApiDoc),
        (path = "/api/uploader", api = uploader::UploaderApiDoc),
        (path = "/api/proxy", api = proxy::ProxyApiDoc),
        (path = "/api/compress", api = compress::CompressApiDoc),
        (path = "/api/drivepng", api = drivepng::DrivePngApiDoc),
        (path = "/api/chat", api = chat::ChatApiDoc)
    )
)]
pub struct ApiDoc;

use axum::Router;
use crate::routes::ChatState;
use std::sync::Arc;

pub fn create_api_routes() -> Router<Arc<ChatState>> {
    Router::new()
        .nest("/komik", komik::create_routes())
        .nest("/anime", anime::create_routes())
        .nest("/anime2", anime2::create_routes())
        .nest("/uploader", uploader::create_routes())
        .nest("/proxy", proxy::create_routes())
        .nest("/compress", compress::create_routes())
        .nest("/drivepng", drivepng::create_routes())
        .nest("/chat", chat::create_routes())
}
