use sea_orm::{ConnectionTrait, DatabaseConnection, DbBackend, Statement, Schema};
use tracing::info;
use crate::entities::image_cache;

pub async fn init(db: &DatabaseConnection) -> Result<(), sea_orm::DbErr> {
    info!("Initializing database schema...");
    let backend = db.get_database_backend();
    let schema = Schema::new(backend);

    // 1. Bookmarks (Legacy raw SQL - kept for reference, but we could migrate to Entity)
    // For now, let's keep the existing raw SQL for bookmarks to avoid breaking anything specific,
    // though migrating it to Entity-based creation is recommended.
    // However, since we are tasked to "fix" migration, let's be robust.
    
    match backend {
        DbBackend::MySql => {
            // Ensure ImageCache exists
            let create_image_cache = schema.create_table_from_entity(image_cache::Entity).if_not_exists().to_owned();
            
            // Ensure Social Media Tables exist
            let create_users = schema.create_table_from_entity(crate::entities::user::Entity).if_not_exists().to_owned();
            let create_posts = schema.create_table_from_entity(crate::entities::posts::Entity).if_not_exists().to_owned();
            let create_likes = schema.create_table_from_entity(crate::entities::likes::Entity).if_not_exists().to_owned();
            let create_comments = schema.create_table_from_entity(crate::entities::comments::Entity).if_not_exists().to_owned();
            let create_chat_room = schema.create_table_from_entity(crate::entities::chat_room::Entity).if_not_exists().to_owned();

            // We need to build the statement. create_table_from_entity returns TableCreateStatement.
            // backend.build(&stmt) returns Statement.
            let _ = db.execute(backend.build(&create_image_cache)).await;
            let _ = db.execute(backend.build(&create_image_cache)).await;
            
            if let Err(e) = db.execute(backend.build(&create_users)).await { tracing::error!("Failed to create Users table: {}", e); }
            if let Err(e) = db.execute(backend.build(&create_posts)).await { tracing::error!("Failed to create Posts table: {}", e); }
            if let Err(e) = db.execute(backend.build(&create_likes)).await { tracing::error!("Failed to create Likes table: {}", e); }
            if let Err(e) = db.execute(backend.build(&create_comments)).await { tracing::error!("Failed to create Comments table: {}", e); }
            
            match db.execute(backend.build(&create_chat_room)).await {
                Ok(_) => info!("Social media and chat tables checked/created successfully."),
                Err(e) => tracing::error!("Failed to create ChatRoom table: {}", e),
            }

            // Create bookmarks table (Legacy)
             let sql = r#"
                CREATE TABLE IF NOT EXISTS bookmarks (
                    id VARCHAR(255) NOT NULL PRIMARY KEY,
                    user_id VARCHAR(255) NOT NULL,
                    content_type VARCHAR(50) NOT NULL,
                    slug VARCHAR(255) NOT NULL,
                    title VARCHAR(255) NOT NULL,
                    poster VARCHAR(512) NOT NULL,
                    created_at TIMESTAMP NULL DEFAULT CURRENT_TIMESTAMP,
                    updated_at TIMESTAMP NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
                    INDEX idx_bookmarks_user_id (user_id),
                    INDEX idx_bookmarks_content (content_type, slug)
                ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
            "#;
            match db.execute(Statement::from_string(backend, sql)).await {
                Ok(_) => info!("Table 'bookmarks' checked/created successfully."),
                Err(e) => tracing::error!("Failed to create bookmarks table: {}", e),
            }
        }
        _ => {
            info!("Skipping schema init for non-MySQL backend");
        }
    }
    Ok(())
}
