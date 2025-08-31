use redis::{Client, Connection};
use std::env;
use crate::error::AppError;

pub fn get_redis_connection() -> Result<Connection, AppError> {
    let redis_url = env::var("UPSTASH_REDIS_REST_URL")
        .unwrap_or_else(|_| "redis://127.0.0.1/".to_string()); // Default to localhost if not set

    let client = Client::open(redis_url)?;
    Ok(client.get_connection()?)
}
