pub mod api;
use axum::Router;
use std::sync::Arc;
use crate::routes::api::komik2;

use deadpool_redis::Pool;
use tracing::info;

#[derive(Clone)]
#[allow(dead_code)]
pub struct AppState {
  pub jwt_secret: String,
  pub redis_pool: Pool,
}
