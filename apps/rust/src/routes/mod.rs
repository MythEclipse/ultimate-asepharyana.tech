pub mod api;
use headless_chrome::browser::Browser;
use std::sync::Arc;
use tokio::sync::Mutex as TokioMutex; // Use Tokio Mutex for async operations

#[derive(Clone)]
#[allow(dead_code)]
pub struct AppState {
  pub jwt_secret: String,
  pub browser: Arc<TokioMutex<Browser>>, // Use Tokio Mutex
}
