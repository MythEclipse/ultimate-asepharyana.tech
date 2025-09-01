// Rate limiter using Redis sorted sets with tracing for all Redis operations.

use redis::{Commands, Connection};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::utils::error::AppError;
use tracing::{info, debug, error};

pub struct RateLimiter {
    conn: Connection,
    limit: usize,
    window_seconds: u64,
}

impl RateLimiter {
    pub fn new(conn: Connection, limit: usize, window_seconds: u64) -> Self {
        info!("Creating RateLimiter: limit={}, window_seconds={}", limit, window_seconds);
        RateLimiter {
            conn,
            limit,
            window_seconds,
        }
    }

    pub fn check(&mut self, key: &str) -> Result<bool, AppError> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let window_start = now - self.window_seconds;

        debug!("Checking rate limit for key: {}, now: {}, window_start: {}", key, now, window_start);

        match self.conn.zrembyscore::<_, _, _, ()>(key, 0, window_start) {
            Ok(_) => debug!("Old entries removed from sorted set for key: {}", key),
            Err(e) => error!("Failed to remove old entries from sorted set for key {}: {:?}", key, e),
        }

        match self.conn.zadd::<_, _, _, ()>(key, now, now) {
            Ok(_) => debug!("Added timestamp {} to sorted set for key: {}", now, key),
            Err(e) => error!("Failed to add timestamp to sorted set for key {}: {:?}", key, e),
        }

        let count: usize = match self.conn.zcard(key) {
            Ok(c) => {
                debug!("Current count for key {}: {}", key, c);
                c
            },
            Err(e) => {
                error!("Failed to get count from sorted set for key {}: {:?}", key, e);
                return Err(AppError::from(e));
            }
        };

        let allowed = count <= self.limit;
        info!("Rate limit check for key {}: count={}, allowed={}", key, count, allowed);

        Ok(allowed)
    }
}
