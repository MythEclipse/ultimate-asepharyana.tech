// Komik base URL logic with Redis lock, updated for sync Redis API and correct imports.

use std::sync::Arc;
use tokio::sync::Mutex as TokioMutex; // Use Tokio Mutex for async operations
use tracing::error;

use crate::utils::error::AppError;

pub async fn get_dynamic_komik_base_url(
  _browser: &Arc<TokioMutex<()>>
) -> Result<String, AppError> {
  error!("Komik base URL functionality disabled - browser not available");
  Err(AppError::Other("Browser functionality has been removed".to_string()))
}

pub async fn get_cached_komik_base_url(
  _browser: &Arc<TokioMutex<()>>,
  _force_refresh: bool
) -> Result<String, AppError> {
  error!("Komik base URL functionality disabled - browser not available");
  Err(AppError::Other("Browser functionality has been removed".to_string()))
}
