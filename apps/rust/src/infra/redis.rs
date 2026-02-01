//! Redis connection utility with tracing for connection lifecycle and errors.
//!
//! Uses the type-safe CONFIG for Redis connection parameters.

use crate::core::config::CONFIG;
use crate::core::error::AppError;
use deadpool_redis::{Manager, Pool};
use once_cell::sync::Lazy;
use tracing::{debug, error, info};

/// Create a lazy static Redis connection pool.
/// Uses REDIS_URL from config, or falls back to constructing from host/port.
pub static REDIS_POOL: Lazy<Pool> = Lazy::new(|| {
    // Use redis_url if available, otherwise construct from environment
    let redis_url = if !CONFIG.redis_url.is_empty() {
        CONFIG.redis_url.clone()
    } else {
        // Fallback to legacy env vars for backward compatibility
        let host = std::env::var("REDIS_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = std::env::var("REDIS_PORT").unwrap_or_else(|_| "6379".to_string());
        let password = std::env::var("REDIS_PASSWORD").unwrap_or_default();

        if password.is_empty() {
            format!("redis://{}:{}", host, port)
        } else {
            format!("redis://:{}@{}:{}", password, host, port)
        }
    };

    info!("Initializing Redis connection pool for URL: {}", redis_url);

    Pool::builder(Manager::new(redis_url).expect("Failed to create Redis manager"))
        .max_size(100)  // Increased from 50 for high-traffic
        .wait_timeout(Some(std::time::Duration::from_secs(5))) // Add wait timeout
        .runtime(deadpool_redis::Runtime::Tokio1)
        .build()
        .expect("Failed to create Redis connection pool")
});

/// Get an async connection from the pool with retry backoff.
pub async fn get_redis_conn() -> Result<deadpool_redis::Connection, AppError> {
    let mut retries = 5;
    let mut wait = std::time::Duration::from_millis(100);

    loop {
        match REDIS_POOL.get().await {
            Ok(conn) => {
                debug!("Successfully retrieved Redis connection from pool.");
                return Ok(conn);
            }
            Err(e) => {
                if retries <= 0 {
                    error!("Failed to get Redis connection after retries: {:?}", e);
                    return Err(AppError::from(e));
                }
                debug!("Redis connection failed, retrying in {:?}: {:?}", wait, e);
                tokio::time::sleep(wait).await;
                wait = std::cmp::min(wait * 2, std::time::Duration::from_secs(5));
                retries -= 1;
            }
        }
    }
}
