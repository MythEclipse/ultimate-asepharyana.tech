use axum::{
    routing::get,
    Router,
};
use crate::routes::ChatState;
use std::sync::Arc;
use utoipa::OpenApi;

pub mod chat_service; // Import the chat_service module

/// OpenAPI doc for Chat API
#[derive(OpenApi)]
#[openapi(
    paths(
        // Add paths for your chat API endpoints here if any
    ),
    tags(
        (name = "Chat", description = "Chat API endpoints")
    )
)]
pub struct ChatApiDoc;


pub fn create_routes() -> Router<Arc<ChatState>> {
    Router::new()
        .route("/test", get(|| async { "Hello from Chat API!" })) // Example route
}
