use axum::extract::ws::Message;
use serde::{Deserialize, Serialize};
use sqlx::{MySqlPool, query, query_as};
use tokio::sync::mpsc;
use std::sync::Arc;
use chrono::{Utc, DateTime};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: Option<i64>,
    pub username: String,
    pub message: String,
    pub timestamp: DateTime<Utc>,
}

pub async fn load_messages(pool: Arc<MySqlPool>) -> anyhow::Result<Vec<ChatMessage>> {
    let messages = query_as!(ChatMessage, "SELECT id, username, message, timestamp FROM chat_messages ORDER BY timestamp ASC LIMIT 100")
        .fetch_all(&*pool)
        .await?;
    Ok(messages)
}

pub async fn save_message(pool: Arc<MySqlPool>, message: &ChatMessage) -> anyhow::Result<()> {
    query!(
        "INSERT INTO chat_messages (username, message, timestamp) VALUES (?, ?, ?)",
        message.username,
        message.message,
        message.timestamp
    )
    .execute(&*pool)
    .await?;
    Ok(())
}

pub async fn broadcast_message(
    message: ChatMessage,
    clients: &mut Vec<mpsc::UnboundedSender<Message>>,
) {
    let json_message = serde_json::to_string(&message).expect("Failed to serialize chat message");
    let msg = Message::Text(json_message.into());

    clients.retain(|client_tx| client_tx.send(msg.clone()).is_ok());
}
