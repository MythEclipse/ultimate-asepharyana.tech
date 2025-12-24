//! WebSocket message types.

use serde::{Deserialize, Serialize};

/// WebSocket event types.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum WsEvent {
    /// Connection established
    Connected,
    /// Message received
    Message,
    /// User joined a room
    Join,
    /// User left a room
    Leave,
    /// Ping/pong for keepalive
    Ping,
    Pong,
    /// Error occurred
    Error,
    /// Broadcast to all
    Broadcast,
    /// Private message to user
    Private,
}

/// WebSocket message wrapper.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsMessage<T = serde_json::Value> {
    /// Event type
    pub event: WsEvent,
    /// Message payload
    pub data: T,
    /// Target room (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room: Option<String>,
    /// Target user ID (for private messages)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    /// Sender ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    /// Timestamp
    #[serde(default = "default_timestamp")]
    pub timestamp: String,
}

fn default_timestamp() -> String {
    chrono::Utc::now().to_rfc3339()
}

impl<T: Serialize> WsMessage<T> {
    /// Create a new message.
    pub fn new(event: WsEvent, data: T) -> Self {
        Self {
            event,
            data,
            room: None,
            to: None,
            from: None,
            timestamp: default_timestamp(),
        }
    }

    /// Set room.
    pub fn room(mut self, room: impl Into<String>) -> Self {
        self.room = Some(room.into());
        self
    }

    /// Set recipient.
    pub fn to(mut self, user_id: impl Into<String>) -> Self {
        self.to = Some(user_id.into());
        self
    }

    /// Set sender.
    pub fn from(mut self, user_id: impl Into<String>) -> Self {
        self.from = Some(user_id.into());
        self
    }

    /// Convert to JSON string.
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }
}

impl WsMessage<serde_json::Value> {
    /// Parse from JSON string.
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

// Convenience constructors
impl WsMessage<serde_json::Value> {
    /// Create a connected message.
    pub fn connected() -> Self {
        Self::new(WsEvent::Connected, serde_json::json!({"status": "connected"}))
    }

    /// Create an error message.
    pub fn error(msg: impl Into<String>) -> Self {
        Self::new(WsEvent::Error, serde_json::json!({"error": msg.into()}))
    }

    /// Create a pong response.
    pub fn pong() -> Self {
        Self::new(WsEvent::Pong, serde_json::json!({}))
    }
}
