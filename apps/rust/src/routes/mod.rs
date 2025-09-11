pub mod api;
use rust_lib::headless_chrome::BrowserPool;
use std::sync::Arc;

#[derive(Clone)]
#[allow(dead_code)]
pub struct AppState {
  pub jwt_secret: String,
  pub browser_pool: Arc<BrowserPool>,
}
