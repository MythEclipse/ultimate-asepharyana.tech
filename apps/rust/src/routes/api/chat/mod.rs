//! Chat API module. Exposes chat-related endpoints and router.
// This module now exposes OpenAPI documentation for all chat endpoints.

use axum::Router;
use std::sync::Arc;
use crate::routes::ChatState;
use utoipa::OpenApi;

pub mod chat_service;
pub mod chat_message_dto;
pub use self::chat_service as chat;

/// Aggregates OpenAPI docs for all chat endpoints.
#[derive(OpenApi)]
#[openapi(
    paths(
        chat_service::chat_handler
    ),
    tags(
        (name = "chat", description = "Chat API endpoints")
    )
)]
pub struct ChatApiDoc;

/// Returns the router for chat endpoints.
pub fn create_routes() -> Router<Arc<ChatState>> {
    chat::create_routes()
}
