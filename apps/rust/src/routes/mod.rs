pub mod api;
use chromiumoxide::Browser;
use std::sync::Arc;

#[derive(Clone)]
#[allow(dead_code)]
pub struct AppState {
  pub jwt_secret: String,
  pub browser: Arc<Browser>,
}
