pub mod api;

use deadpool_redis::Pool;

#[derive(Clone)]
#[allow(dead_code)]
pub struct AppState {
  pub jwt_secret: String,
  pub redis_pool: Pool,
}
