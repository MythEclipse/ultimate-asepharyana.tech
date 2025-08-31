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
pub mod complete_anime;
pub mod full;
pub mod otakudesu_service; // New module for Otakudesu service
pub mod ongoing_anime;

// NOTE: The /search endpoint now supports ?status=complete or ?status=ongoing for filtering.

pub mod anime_detail_dto;

pub fn create_routes() -> Router<Arc<ChatState>> {
    Router::new()
        .route("/complete-anime/:slug", axum::routing::get(complete_anime::complete_anime_handler))
        .route("/ongoing-anime/:slug", axum::routing::get(ongoing_anime::ongoing_anime_handler))
        .route("/full/:slug", axum::routing::get(full::full_anime_handler))
}
