use sqlx::MySqlPool;
use uuid::Uuid;
use chrono::Utc;

use super::models::{ChatRoom, ChatMessage, RoomMember, CreateRoomRequest, SendMessageRequest};
use crate::utils::error::AppError;

#[utoipa::path(
    get,
    path = "/api/chat/db",
    tag = "chat",
    operation_id = "chat_db",
    responses(
        (status = 200, description = "Handles GET requests for the /api/chat/db endpoint."),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn create_room(
    pool: &MySqlPool,
    user_id: &str,
    req: CreateRoomRequest,
) -> Result<ChatRoom, AppError> {
    let room_id = Uuid::new_v4().to_string();
    let now = Utc::now();

    let room = sqlx::query_as::<_, ChatRoom>(
        r#"
        INSERT INTO chat_rooms (id, name, description, created_by, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING *
        "#,
    )
    .bind(&room_id)
    .bind(&req.name)
    .bind(&req.description)
    .bind(user_id)
    .bind(now)
    .bind(now)
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    // Add creator as member
    sqlx::query(
        r#"
        INSERT INTO room_members (room_id, user_id, user_name, joined_at)
        VALUES ($1, $2, $3, $4)
        "#,
    )
    .bind(&room_id)
    .bind(user_id)
    .bind(user_id) // Using user_id as name for now, should be fetched from users table
    .bind(now)
    .execute(pool)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(room)
}

pub async fn get_rooms(pool: &MySqlPool) -> Result<Vec<ChatRoom>, AppError> {
    let rooms = sqlx::query_as::<_, ChatRoom>(
        r#"
        SELECT * FROM chat_rooms
        ORDER BY updated_at DESC
        LIMIT 50
        "#,
    )
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(rooms)
}

pub async fn get_room(pool: &MySqlPool, room_id: &str) -> Result<Option<ChatRoom>, AppError> {
    let room = sqlx::query_as::<_, ChatRoom>(
        r#"
        SELECT * FROM chat_rooms
        WHERE id = $1
        "#,
    )
    .bind(room_id)
    .fetch_optional(pool)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(room)
}

pub async fn join_room(
    pool: &MySqlPool,
    room_id: &str,
    user_id: &str,
    user_name: &str,
) -> Result<(), AppError> {
    let now = Utc::now();

    sqlx::query(
        r#"
        INSERT INTO room_members (room_id, user_id, user_name, joined_at)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (room_id, user_id) DO NOTHING
        "#,
    )
    .bind(room_id)
    .bind(user_id)
    .bind(user_name)
    .bind(now)
    .execute(pool)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(())
}

pub async fn leave_room(
    pool: &MySqlPool,
    room_id: &str,
    user_id: &str,
) -> Result<(), AppError> {
    sqlx::query(
        r#"
        DELETE FROM room_members
        WHERE room_id = $1 AND user_id = $2
        "#,
    )
    .bind(room_id)
    .bind(user_id)
    .execute(pool)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(())
}

pub async fn get_room_members(
    pool: &MySqlPool,
    room_id: &str,
) -> Result<Vec<RoomMember>, AppError> {
    let members = sqlx::query_as::<_, RoomMember>(
        r#"
        SELECT * FROM room_members
        WHERE room_id = $1
        ORDER BY joined_at
        "#,
    )
    .bind(room_id)
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(members)
}

pub async fn send_message(
    pool: &MySqlPool,
    room_id: &str,
    user_id: &str,
    user_name: &str,
    req: SendMessageRequest,
) -> Result<ChatMessage, AppError> {
    let message_id = Uuid::new_v4().to_string();
    let now = Utc::now();
    let message_type = req.message_type.unwrap_or_else(|| "text".to_string());

    let message = sqlx::query_as::<_, ChatMessage>(
        r#"
        INSERT INTO chat_messages (id, room_id, user_id, user_name, content, message_type, created_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING *
        "#,
    )
    .bind(&message_id)
    .bind(room_id)
    .bind(user_id)
    .bind(user_name)
    .bind(&req.content)
    .bind(&message_type)
    .bind(now)
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    // Update room's updated_at
    sqlx::query(
        r#"
        UPDATE chat_rooms
        SET updated_at = $1
        WHERE id = $2
        "#,
    )
    .bind(now)
    .bind(room_id)
    .execute(pool)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(message)
}

pub async fn get_messages(
    pool: &MySqlPool,
    room_id: &str,
    page: i32,
    page_size: i32,
) -> Result<(Vec<ChatMessage>, i64), AppError> {
    let offset = (page - 1) * page_size;

    let messages = sqlx::query_as::<_, ChatMessage>(
        r#"
        SELECT * FROM chat_messages
        WHERE room_id = $1
        ORDER BY created_at DESC
        LIMIT $2 OFFSET $3
        "#,
    )
    .bind(room_id)
    .bind(page_size as i64)
    .bind(offset as i64)
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    let total: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*) FROM chat_messages
        WHERE room_id = $1
        "#,
    )
    .bind(room_id)
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok((messages, total.0))
}

pub async fn get_message_count(pool: &MySqlPool, room_id: &str) -> Result<i64, AppError> {
    let count: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*) FROM chat_messages
        WHERE room_id = $1
        "#,
    )
    .bind(room_id)
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(count.0)
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(create_room))
}