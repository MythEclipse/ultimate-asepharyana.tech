use axum::{routing::{post, delete}, Router};
use std::sync::Arc;
use crate::routes::mod_::ChatState;

pub mod route;

pub fn create_routes() -> Router<Arc<ChatState>> {
    Router::new()
        .route("/", post(route::likes_post_handler))
        .route("/", delete(route::likes_delete_handler))
}
