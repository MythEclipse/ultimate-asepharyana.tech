// Redis connection utility with tracing for connection lifecycle and errors.

use redis::{Client, Connection};
use crate::config::CONFIG_MAP;
use crate::utils::error::AppError;
use tracing::{info, error, debug};

pub fn get_redis_connection() -> Result<Connection, AppError> {
    let rest_url = CONFIG_MAP.get("UPSTASH_REDIS_REST_URL")
        .cloned()
        .unwrap_or_else(|| "https://localhost:8080".to_string());

    let token = CONFIG_MAP.get("UPSTASH_REDIS_REST_TOKEN")
        .cloned()
        .unwrap_or_else(|| "".to_string());

    // Convert Upstash REST URL to Redis protocol URL
    // REST URL: https://host
    // Redis URL: rediss://default:token@host:6379
    let host = rest_url.trim_start_matches("https://").trim_start_matches("http://");
    let redis_url = format!("rediss://default:{}@{}:6379", token, host);

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
