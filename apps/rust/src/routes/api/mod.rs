use axum::{Router};
use std::sync::Arc;
use crate::routes::ChatState;

pub mod anime;
pub mod anime2;
pub mod sosmed;

pub fn create_api_routes() -> Router<Arc<ChatState>> {
    Router::new()
        .nest("/anime", anime::create_routes())
        .nest("/anime2", anime2::create_routes())
        .nest("/sosmed", sosmed::create_routes())
}
