use axum::{
    routing::get,
    Router,
};
use crate::routes::ChatState;
use std::sync::Arc;

pub mod chat_service; // Import the chat_service module

/// OpenAPI doc for Chat API


pub fn create_routes() -> Router<Arc<ChatState>> {
    Router::new()
        .route("/test", get(|| async { "Hello from Chat API!" }))
}
