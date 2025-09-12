pub mod api;
use fantoccini::Client as FantocciniClient;
use std::sync::Arc;

#[derive(Clone)]
#[allow(dead_code)]
pub struct AppState {
  pub jwt_secret: String,
  pub browser_client: Arc<FantocciniClient>,
}
