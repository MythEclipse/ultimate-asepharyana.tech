use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct ChatRoom {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_by: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct ChatMessage {
    pub id: String,
    pub room_id: String,
    pub user_id: String,
    pub user_name: String,
    pub content: String,
    pub message_type: String, // "text", "image", "file"
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct RoomMember {
    pub room_id: String,
    pub user_id: String,
    pub user_name: String,
    pub joined_at: DateTime<Utc>,
}

// Request/Response types
#[derive(Debug, Deserialize)]
pub struct CreateRoomRequest {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SendMessageRequest {
    pub content: String,
    pub message_type: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct RoomResponse {
    pub room: ChatRoom,
    pub members: Vec<RoomMember>,
    pub message_count: i64,
}

#[derive(Debug, Serialize)]
pub struct MessagesResponse {
    pub messages: Vec<ChatMessage>,
    pub total: i64,
    pub page: i32,
    pub page_size: i32,
}

#[derive(Debug, Deserialize)]
pub struct MessageQuery {
    pub page: Option<i32>,
    pub page_size: Option<i32>,
}

// WebSocket message types
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WsMessage {
    Join {
        room_id: String,
        user_id: String,
        user_name: String,
    },
    Leave {
        room_id: String,
        user_id: String,
    },
    Message {
        room_id: String,
        message: ChatMessage,
    },
    UserJoined {
        room_id: String,
        user_id: String,
        user_name: String,
    },
    UserLeft {
        room_id: String,
        user_id: String,
        user_name: String,
    },
    Error {
        message: String,
    },
}
