use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, PaginatorTrait, Set};
use chrono::Utc;
use tracing::info;

use crate::entities::chat_room;

/// Check if chat_rooms table is empty and seed default data if needed
pub async fn seed_chat_data_if_empty(db: &DatabaseConnection) -> Result<(), sea_orm::DbErr> {
    // Check if there are any chat rooms using SeaORM
    let count = chat_room::Entity::find().count(db).await?;

    if count == 0 {
        info!("Chat tables are empty, seeding default data...");

        // Insert default rooms
        let room1 = chat_room::ActiveModel {
            id: Set("00000000-0000-0000-0000-000000000001".to_string()),
            name: Set("General".to_string()),
            description: Set(Some("General discussion room for everyone".to_string())),
            is_private: Set(0),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
        };
        room1.insert(db).await?;

        let room2 = chat_room::ActiveModel {
            id: Set("00000000-0000-0000-0000-000000000002".to_string()),
            name: Set("Tech Talk".to_string()),
            description: Set(Some("Discuss technology, programming, and development".to_string())),
            is_private: Set(0),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
        };
        room2.insert(db).await?;

        let room3 = chat_room::ActiveModel {
            id: Set("00000000-0000-0000-0000-000000000003".to_string()),
            name: Set("Random".to_string()),
            description: Set(Some("Random chat and off-topic discussions".to_string())),
            is_private: Set(0),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
        };
        room3.insert(db).await?;

        // Note: ChatMessage table doesn't have room_id or username fields in current schema
        // If you need to seed messages, the schema needs to be updated first
        // Current ChatMessage schema: id, userId, text, email, imageProfile, imageMessage, role, timestamp
        
        info!("✅ Default chat data seeded successfully!");
    } else {
        info!("Chat data already exists, skipping seed");
    }

    Ok(())
}
