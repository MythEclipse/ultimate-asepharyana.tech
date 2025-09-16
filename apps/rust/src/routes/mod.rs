pub mod api;
use axum::Router;
use std::sync::Arc;
use crate::routes::api::komik2::detail::register_routes as register_komik2_detail_routes;
use crate::routes::api::komik2::chapter::register_routes as register_komik2_chapter_routes;

use deadpool_redis::Pool;

#[derive(Clone)]
#[allow(dead_code)]
pub struct AppState {
  pub jwt_secret: String,
  pub redis_pool: Pool,
}

pub fn register_komik2_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
  let router = register_komik2_detail_routes(router);
  let router = register_komik2_chapter_routes(router);
  router
}
