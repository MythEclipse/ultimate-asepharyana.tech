use sqlx::MySqlPool;
use tracing::info;

/// Check if chat_rooms table is empty and seed default data if needed
pub async fn seed_chat_data_if_empty(pool: &MySqlPool) -> Result<(), sqlx::Error> {
    // Check if there are any chat rooms
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM chat_rooms")
        .fetch_one(pool)
        .await?;

    if count.0 == 0 {
        info!("Chat tables are empty, seeding default data...");

        // Insert default rooms
        sqlx::query(
            r#"
            INSERT INTO chat_rooms (id, name, description, created_by, created_at) VALUES
            ('00000000-0000-0000-0000-000000000001', 'General', 'General discussion room for everyone', 'system', NOW()),
            ('00000000-0000-0000-0000-000000000002', 'Tech Talk', 'Discuss technology, programming, and development', 'system', NOW()),
            ('00000000-0000-0000-0000-000000000003', 'Random', 'Random chat and off-topic discussions', 'system', NOW())
            "#
        )
        .execute(pool)
        .await?;

        // Insert welcome messages
        sqlx::query(
            r#"
            INSERT INTO chat_messages (id, room_id, user_id, username, message, created_at) VALUES
            ('00000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000001', 'system', 'System', 'Welcome to the General chat room! Feel free to discuss anything here.', NOW()),
            ('00000000-0000-0000-0000-000000000002', '00000000-0000-0000-0000-000000000002', 'system', 'System', 'Welcome to Tech Talk! Share your knowledge and learn from others.', NOW()),
            ('00000000-0000-0000-0000-000000000003', '00000000-0000-0000-0000-000000000003', 'system', 'System', 'Welcome to Random! This is a place for casual conversations.', NOW())
            "#
        )
        .execute(pool)
        .await?;

        info!("✅ Default chat data seeded successfully!");
    } else {
        info!("Chat data already exists, skipping seed");
    }

    Ok(())
}
