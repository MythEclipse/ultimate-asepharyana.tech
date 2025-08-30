use axum::{Router};
use std::sync::Arc;
use crate::routes::ChatState;

pub mod search;
pub mod detail;
pub mod episode;
pub mod anime2_service;
pub mod anime2_dto;
pub use self::anime2_service as anime2;

pub fn create_routes() -> Router<Arc<ChatState>> {
    Router::new()
        .nest("/search", search::create_routes())
        .nest("/detail", detail::create_routes())
        .nest("/episode", episode::create_routes())
}
