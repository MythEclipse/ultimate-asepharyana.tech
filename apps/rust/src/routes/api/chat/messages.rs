use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use std::sync::Arc;
use serde_json::json;

use super::db;
use super::models::{MessageQuery, SendMessageRequest};
use crate::utils::error::AppError;
use crate::routes::AppState;

#[allow(dead_code)]
pub const ENDPOINT_METHOD: &str = "get,post";
#[allow(dead_code)]
pub const ENDPOINT_PATH: &str = "/api/chat/rooms/:id/messages";
#[allow(dead_code)]
pub const ENDPOINT_DESCRIPTION: &str = "Handles chat messages operations";
#[allow(dead_code)]
pub const ENDPOINT_TAG: &str = "chat";
#[allow(dead_code)]
pub const OPERATION_ID: &str = "chat_messages";

#[utoipa::path(
    get,
    path = "/api/chat/rooms/{room_id}/messages",
    tag = "chat",
    operation_id = "get_chat_messages",
    params(
        ("room_id" = String, Path, description = "Room ID")
    ),
    responses(
        (status = 200, description = "List of messages"),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn get_messages_handler(
    State(state): State<Arc<AppState>>,
    Path(room_id): Path<String>,
    Query(query): Query<MessageQuery>,
) -> Result<impl IntoResponse, AppError> {
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(50);

    let (messages, total) = db::get_messages(&state.pool, &room_id, page, page_size).await?;

    Ok(Json(json!({
        "success": true,
        "messages": messages,
        "total": total,
        "page": page,
        "page_size": page_size
    })))
}

#[utoipa::path(
    post,
    path = "/api/chat/rooms/{room_id}/messages",
    tag = "chat",
    operation_id = "send_chat_message",
    params(
        ("room_id" = String, Path, description = "Room ID")
    ),
    responses(
        (status = 200, description = "Message sent successfully"),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn send_message_handler(
    State(state): State<Arc<AppState>>,
    Path(room_id): Path<String>,
    Json(req): Json<SendMessageRequest>,
) -> Result<impl IntoResponse, AppError> {
    // TODO: Get user info from JWT token
    let user_id = "user_123";
    let user_name = "User 123";

    let message = db::send_message(&state.pool, &room_id, user_id, user_name, req).await?;

    // Broadcast via WebSocket
    let ws_msg = super::models::WsMessage::Message {
        room_id: room_id.clone(),
        message: message.clone(),
    };
    let _ = state.chat_tx.send(ws_msg);

    Ok(Json(json!({
        "success": true,
        "message": message
    })))
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router
        .route("/api/chat/rooms/:id/messages", get(get_messages_handler))
        .route("/api/chat/rooms/:id/messages", post(send_message_handler))
}
