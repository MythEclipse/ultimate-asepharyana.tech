use axum::Router;
use std::sync::Arc;
use crate::routes::ChatState;

pub fn create_routes() -> Router<Arc<ChatState>> {
    Router::new()
}
