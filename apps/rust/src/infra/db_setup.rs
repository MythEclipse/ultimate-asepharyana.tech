use sea_orm::{ConnectionTrait, DatabaseConnection, DbBackend, Statement};
use tracing::info;

pub async fn init(db: &DatabaseConnection) -> Result<(), sea_orm::DbErr> {
    info!("Initializing database schema...");
    let backend = db.get_database_backend();
    match backend {
        DbBackend::MySql => {
            // Create bookmarks table
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
