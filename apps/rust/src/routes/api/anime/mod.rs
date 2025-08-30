use axum::{Router};
use std::sync::Arc;
use crate::routes::ChatState;

pub mod search;
pub mod detail;
pub mod episode;
pub mod anime_service; // otakudesu
pub mod anime_dto;     // otakudesu
pub mod alqanime_service;
pub mod alqanime_dto;

// NOTE: The /search endpoint now supports ?status=complete or ?status=ongoing for filtering.

pub fn create_routes() -> Router<Arc<ChatState>> {
    Router::new()
        .nest("/search", search::create_routes())
        .nest("/detail", detail::create_routes())
        .nest("/episode", episode::create_routes())
}
