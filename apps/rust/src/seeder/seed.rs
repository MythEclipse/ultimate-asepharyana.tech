use chrono::Utc;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, PaginatorTrait, Set};
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
            description: Set(Some(
                "Discuss technology, programming, and development".to_string(),
            )),
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

    // Seed Social Media Data (Users & Posts)
    // Check if users exist
    let user_count = crate::entities::user::Entity::find().count(db).await?;
    if user_count == 0 {
        info!("Seeding social media users and posts...");
        
        // 1. Create Users
        let users = vec![
            ("u1", "Architect", "https://api.dicebear.com/7.x/avataaars/svg?seed=Architect"),
            ("u2", "System", "https://api.dicebear.com/7.x/avataaars/svg?seed=System"),
            ("u3", "Explorer", "https://api.dicebear.com/7.x/avataaars/svg?seed=Explorer"),
            ("u4", "Protocol", "https://api.dicebear.com/7.x/avataaars/svg?seed=Protocol"),
        ];

        for (id, name, image) in users {
            let user = crate::entities::user::ActiveModel {
                id: Set(id.to_string()),
                name: Set(Some(name.to_string())),
                image: Set(Some(image.to_string())),
                role: Set("user".to_string()),
                ..Default::default()
            };
            let _ = user.insert(db).await;
        }

        // 2. Create Posts
        let posts = vec![
            ("1", "u1", "Just deployed the new quantum bridge interface. The glassmorphism is real.", Some("https://images.unsplash.com/photo-1451187580459-43490279c0fa"), "2024-01-01T10:00:00Z"),
            ("2", "u2", "Systems nominal. Digital destiny is loading...", None, "2024-01-01T11:00:00Z"),
            ("3", "u3", "Exploring the void. The scroll observer is detecting life forms.", None, "2024-01-01T12:00:00Z"),
            ("4", "u4", "Staggered reveal successful. Initializing heart explosion protocol.", Some("https://images.unsplash.com/photo-1534972195531-d756b9bfa9f2"), "2024-01-01T13:00:00Z"),
        ];

        for (id, user_id, content, image_url, created_at) in posts {
            let post = crate::entities::posts::ActiveModel {
                id: Set(id.to_string()),
                user_id: Set(user_id.to_string()),
                author_id: Set(user_id.to_string()),
                content: Set(content.to_string()),
                image_url: Set(image_url.map(|s| s.to_string())),
                created_at: Set(created_at.parse().unwrap_or(Utc::now())),
                updated_at: Set(Utc::now()),
                ..Default::default()
            };
            let _ = post.insert(db).await;
        }
        
        info!("✅ Social media data seeded successfully!");
    }

    Ok(())
}
