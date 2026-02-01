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
            
            // We need to build the statement. create_table_from_entity returns TableCreateStatement.
            // backend.build(&stmt) returns Statement.
            match db.execute(backend.build(&create_image_cache)).await {
                Ok(_) => info!("Table 'ImageCache' checked/created successfully."),
                Err(e) => tracing::error!("Failed to create ImageCache table: {}", e),
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
