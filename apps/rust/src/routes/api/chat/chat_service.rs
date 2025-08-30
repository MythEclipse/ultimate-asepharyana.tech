use sqlx::MySqlPool;
use crate::routes::api::chat::chat_message_dto::ChatMessage;
use anyhow::Result;

pub async fn save_message(pool: &MySqlPool, message: &ChatMessage) -> Result<ChatMessage> {
    // MySQL doesn't support RETURNING, so we do INSERT then SELECT
    sqlx::query(
        "INSERT INTO ChatMessage (id, userId, text, email, imageProfile, imageMessage, role, timestamp) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&message.id)
    .bind(&message.user_id)
    .bind(&message.text)
    .bind(&message.email)
    .bind(&message.image_profile)
    .bind(&message.image_message)
    .bind(&message.role)
    .bind(&message.timestamp)
    .execute(pool)
    .await
    .map_err(|e| anyhow::anyhow!("Failed to insert message: {}", e))?;

    // Now fetch the inserted message
    sqlx::query_as::<_, ChatMessage>(
        "SELECT id, userId, text, email, imageProfile, imageMessage, role, timestamp FROM ChatMessage WHERE id = ?"
    )
    .bind(&message.id)
    .fetch_one(pool)
    .await
    .map_err(|e| anyhow::anyhow!("Failed to fetch saved message: {}", e))
}

pub async fn load_messages(pool: &MySqlPool, limit: u32) -> Result<Vec<ChatMessage>> {
    sqlx::query_as::<_, ChatMessage>(
        "SELECT id, userId, text, email, imageProfile, imageMessage, role, timestamp FROM ChatMessage ORDER BY timestamp DESC LIMIT ?"
    )
    .bind(limit)
    .fetch_all(pool)
    .await
    .map_err(|e| anyhow::anyhow!("Failed to load messages: {}", e))
}
