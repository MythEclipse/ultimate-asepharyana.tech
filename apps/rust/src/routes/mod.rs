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

pub fn register_komik2_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
  info!("Registering komik2 routes via komik2::mod.rs");
  komik2::register_routes(router)
}
