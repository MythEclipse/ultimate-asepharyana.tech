use redis::{Client, Connection};
use crate::config::CONFIG_MAP;
use crate::utils::error::AppError;

pub fn get_redis_connection() -> Result<Connection, AppError> {
    let redis_url = CONFIG_MAP.get("UPSTASH_REDIS_REST_URL")
        .cloned()
        .unwrap_or_else(|| "redis://127.0.0.1/".to_string()); // Default to localhost if not set

    let client = Client::open(redis_url)?;
    Ok(client.get_connection()?)
}
