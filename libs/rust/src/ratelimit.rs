use redis::{Commands, Connection};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct RateLimiter {
    conn: Connection,
    limit: usize,
    window_seconds: u64,
}

impl RateLimiter {
    pub fn new(conn: Connection, limit: usize, window_seconds: u64) -> Self {
        RateLimiter {
            conn,
            limit,
            window_seconds,
        }
    }

    pub fn check(&mut self, key: &str) -> Result<bool, redis::RedisError> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let window_start = now - self.window_seconds;

        let _: () = self.conn.zrembyscore(key, 0, window_start)?;
        let _: () = self.conn.zadd(key, now, now)?;
        let count: usize = self.conn.zcard(key)?;

        Ok(count <= self.limit)
    }
}
