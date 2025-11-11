#![allow(unused_imports)]
pub mod api;
pub mod ws;
use crate::routes::api::komik2;
use axum::Router;
use std::sync::Arc;

use deadpool_redis::Pool;
use sea_orm::DatabaseConnection;
use tracing::info;

#[derive(Clone)]
#[allow(dead_code)]
pub struct AppState {
    pub jwt_secret: String,
    pub redis_pool: Pool,
    pub db: DatabaseConnection,
    pub pool: DatabaseConnection, // SeaORM connection (alias for compatibility)
    pub chat_tx: tokio::sync::broadcast::Sender<crate::routes::ws::models::WsMessage>,
}

impl AppState {
    /// Get SeaORM database connection
    pub fn sea_orm(&self) -> &DatabaseConnection {
        &self.db
    }
}

