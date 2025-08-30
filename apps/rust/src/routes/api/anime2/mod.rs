use axum::{Router};
use std::sync::Arc;
use crate::routes::mod_::ChatState;

pub mod search;
pub mod detail;
pub mod episode;

pub fn create_routes() -> Router<Arc<ChatState>> {
    Router::new()
        .nest("/search", search::create_routes())
        .nest("/detail", detail::create_routes())
        .nest("/episode", episode::create_routes())
}
