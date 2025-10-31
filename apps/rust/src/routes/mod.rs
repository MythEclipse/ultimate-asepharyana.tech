#![allow(unused_imports)]
pub mod api;
pub mod ws;
use crate::routes::api::komik2;
use axum::Router;
use std::sync::Arc;

use deadpool_redis::Pool;
use tracing::info;

#[derive(Clone)]
#[allow(dead_code)]
pub struct AppState {
    pub jwt_secret: String,
    pub redis_pool: Pool,
    pub db: sqlx::MySqlPool,
    pub pool: sqlx::MySqlPool,
    pub chat_tx: tokio::sync::broadcast::Sender<crate::routes::ws::models::WsMessage>,
}
