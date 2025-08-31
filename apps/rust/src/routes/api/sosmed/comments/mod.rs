use axum::{Router};
use std::sync::Arc;
use crate::routes::ChatState;

pub mod route;

pub fn create_routes() -> Router<Arc<ChatState>> {
    Router::new()
        .route("/", post(route::comments_post_handler))
        .route("/", get(route::comments_get_handler))
        .route("/", put(route::comments_put_handler))
        .route("/", delete(route::comments_delete_handler))
}
