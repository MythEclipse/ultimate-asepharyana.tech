// Redis connection utility with tracing for connection lifecycle and errors.

use redis::{Client, Connection};
use crate::config::CONFIG_MAP;
use crate::utils::error::AppError;
use tracing::{info, error, debug};

pub fn get_redis_connection() -> Result<Connection, AppError> {
    let host = CONFIG_MAP.get("REDIS_HOST")
        .cloned()
        .unwrap_or_else(|| "127.0.0.1".to_string());

    let port = CONFIG_MAP.get("REDIS_PORT")
        .cloned()
        .unwrap_or_else(|| "6379".to_string());

    let password = CONFIG_MAP.get("REDIS_PASSWORD")
        .cloned()
        .unwrap_or_else(|| "mysecretpassword".to_string());

    // Construct Redis URL with password
    // Format: redis://:password@host:port
    let redis_url = format!("redis://:{}@{}:{}", password, host, port);

    debug!("Attempting to connect to Redis at URL: {}", redis_url);

    let client = match Client::open(redis_url.clone()) {
        Ok(c) => {
            info!("Redis client created successfully for URL: {}", redis_url);
            c
        },
        Err(e) => {
            error!("Failed to create Redis client for URL {}: {:?}", redis_url, e);
            return Err(AppError::from(e));
        }
    };

    match client.get_connection() {
        Ok(conn) => {
            info!("Redis connection established.");
            Ok(conn)
        },
        Err(e) => {
            error!("Failed to get Redis connection: {:?}", e);
            Err(AppError::from(e))
        }
    }
}
