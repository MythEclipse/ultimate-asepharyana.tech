//! Handlers and state for chat routes
//! All environment variables (including JWT secret) must be loaded via rust_lib::config::CONFIG_MAP

use axum::{
    extract::{State, WebSocketUpgrade},
    response::{IntoResponse, Redirect},
    extract::ws::{Message, WebSocket},
    Json,
    Router,
};
use sqlx::MySqlPool;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use serde_json::json;
use http::StatusCode;
use futures::StreamExt;
use crate::routes::api::chat::chat_message_dto::ChatMessage;
use rust_lib::config::CONFIG_MAP;

// Chat state struct
pub struct ChatState {
    pub pool: Arc<MySqlPool>,
    pub clients: Mutex<Vec<mpsc::UnboundedSender<Message>>>,
    /// JWT secret loaded from CONFIG_MAP
    pub jwt_secret: String,
}

// Root handler
pub async fn root_handler() -> impl IntoResponse {
    Redirect::permanent("https://asepharyana.tech/chat")
}

// Health check endpoint
pub async fn health_check() -> impl IntoResponse {
    Json(json!({
        "status": "ok"
    }))
}

// Status endpoint for monitoring
pub async fn status_handler(State(state): State<Arc<ChatState>>) -> impl IntoResponse {
    // Check database connection
    let db_status = match sqlx::query("SELECT 1").execute(state.pool.as_ref()).await {
        Ok(_) => "connected",
        Err(_) => "disconnected",
    };
    Json(json!({
        "database": db_status
    }))
}

// WebSocket handler
pub async fn ws_handler(ws: WebSocketUpgrade, State(state): State<Arc<ChatState>>) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

// WebSocket socket handler
pub async fn handle_socket(socket: WebSocket, state: Arc<ChatState>) {
    let (mut sender, mut receiver) = socket.split();

    // Example: load messages and send to client
    if let Ok(messages) = chat_service::load_messages(&state.pool, 50).await {
        for msg in messages {
            let _ = sender.send(Message::Text(msg.content)).await;
        }
    }

    // Example: receive and save messages
    while let Some(Ok(Message::Text(text))) = receiver.next().await {
        if let Ok(chat_message) = serde_json::from_str::<ChatMessage>(&text) {
            let _ = chat_service::save_message(&state.pool, &chat_message).await;
        }
    }
}
