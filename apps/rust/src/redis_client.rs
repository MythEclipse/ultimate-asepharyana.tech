// Redis connection utility with tracing for connection lifecycle and errors.

use crate::config::CONFIG_MAP;
use crate::utils::error::AppError;
use deadpool_redis::{Manager, Pool};
use once_cell::sync::Lazy;
use tracing::{debug, error, info};

// Create a lazy static Redis connection pool
pub static REDIS_POOL: Lazy<Pool> = Lazy::new(|| {
    let host = CONFIG_MAP
        .get("REDIS_HOST")
        .cloned()
        .unwrap_or_else(|| "127.0.0.1".to_string());

    let port = CONFIG_MAP
        .get("REDIS_PORT")
        .cloned()
        .unwrap_or_else(|| "6379".to_string());

    let password = CONFIG_MAP
        .get("REDIS_PASSWORD")
        .cloned()
        .unwrap_or_else(|| "".to_string());

    let redis_url = if password.is_empty() {
        format!("redis://{}:{}", host, port)
    } else {
        format!("redis://:{}@{}:{}", password, host, port)
    };

    info!("Initializing Redis connection pool for URL: {}", redis_url);

    Pool::builder(Manager::new(redis_url).expect("Failed to create Redis manager"))
        .max_size(10) // Set maximum number of connections in the pool
        .build()
        .expect("Failed to create Redis connection pool")
});

// Function to get an async connection from the pool
pub async fn get_redis_conn() -> Result<deadpool_redis::Connection, AppError> {
    debug!("Attempting to get Redis connection from pool.");
    match REDIS_POOL.get().await {
        Ok(conn) => {
            debug!("Successfully retrieved Redis connection from pool.");
            Ok(conn)
        }
        Err(e) => {
            error!("Failed to get Redis connection from pool: {:?}", e);
            Err(AppError::from(e))
        }
    }
}
