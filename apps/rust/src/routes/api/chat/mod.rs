//! Chat API module. Exposes chat-related endpoints and router.

use axum::Router;
use std::sync::Arc;
use crate::routes::ChatState;

pub mod chat_service;
pub mod chat_message_dto;
pub use self::chat_service as chat;

/// Returns the router for chat endpoints.
pub fn create_routes() -> Router<Arc<ChatState>> {
    chat::create_routes()
}
