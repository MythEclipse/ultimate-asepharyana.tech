pub mod api;
pub mod ws;
use std::sync::Arc;

use deadpool_redis::Pool;
use sea_orm::DatabaseConnection;

#[allow(dead_code)]
pub struct AppState {
    pub jwt_secret: String,
    pub redis_pool: Pool,
    pub db: Arc<DatabaseConnection>,
    pub pool: Arc<DatabaseConnection>, // SeaORM connection (alias for compatibility)
    pub chat_tx: tokio::sync::broadcast::Sender<crate::routes::ws::models::WsMessage>,
    pub image_processing_semaphore: Arc<tokio::sync::Semaphore>,
}

impl AppState {
    /// Get SeaORM database connection
    pub fn sea_orm(&self) -> &DatabaseConnection {
        &self.db
    }
}
