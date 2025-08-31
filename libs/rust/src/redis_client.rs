use redis::{Client, Connection, RedisResult};
use std::env;

pub fn get_redis_connection() -> RedisResult<Connection> {
    let redis_url = env::var("UPSTASH_REDIS_REST_URL")
        .unwrap_or_else(|_| "redis://127.0.0.1/".to_string()); // Default to localhost if not set

    let client = Client::open(redis_url)?;
    client.get_connection()
}
