//! API routing module for the application.
//! Organizes and exposes all API endpoints in a modular and idiomatic Rust style.

use axum::Router;
use std::sync::Arc;
use crate::routes::ChatState;

pub mod anime;
pub mod sosmed;
pub mod chat;
pub mod komik;
pub mod compress;
pub mod uploader;

/// Registers all API routes under their respective namespaces.
pub fn create_api_routes() -> Router<Arc<ChatState>> {
    Router::new()
        .nest("/anime", anime::create_routes())
        .nest("/sosmed", sosmed::create_routes())
        .nest("/chat", chat::create_routes())
        .nest("/komik", komik::create_routes())
        .nest("/compress", compress::create_routes())
        .nest("/uploader", uploader::create_routes())
}
