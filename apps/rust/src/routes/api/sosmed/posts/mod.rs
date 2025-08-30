use axum::{routing::{post, get, put, delete}, Router};
use std::sync::Arc;
use crate::routes::mod_::ChatState;

pub mod route;

pub fn create_routes() -> Router<Arc<ChatState>> {
    Router::new()
        .route("/", post(route::posts_post_handler))
        .route("/", get(route::posts_get_handler))
        .route("/", put(route::posts_put_handler))
        .route("/", delete(route::posts_delete_handler))
}
