//! Rate limiter middleware using the governor crate.
//!
//! Provides token-bucket based rate limiting with configurable limits.
//! Default: 1000 requests per second (1ms minimum interval).

use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use governor::{
    clock::DefaultClock,
    state::{InMemoryState, NotKeyed},
    Quota, RateLimiter as GovernorRateLimiter,
};
use once_cell::sync::Lazy;
use serde_json::json;
use std::{num::NonZeroU32, sync::Arc, time::Duration};
use tracing::warn;

/// Global rate limiter instance.
/// Configured for 1000 requests per second (1ms minimum interval).
static GLOBAL_LIMITER: Lazy<Arc<GovernorRateLimiter<NotKeyed, InMemoryState, DefaultClock>>> =
    Lazy::new(|| {
        // 1000 requests per second = 1 request per millisecond
        let quota = Quota::with_period(Duration::from_millis(1))
            .unwrap()
            .allow_burst(NonZeroU32::new(1000).unwrap());

        Arc::new(GovernorRateLimiter::direct(quota))
    });

/// Rate limiter configuration.
#[derive(Debug, Clone)]
pub struct RateLimiterConfig {
    /// Requests per second
    pub requests_per_second: u32,
    /// Burst size (max requests that can be made instantly)
    pub burst_size: u32,
}

impl Default for RateLimiterConfig {
    fn default() -> Self {
        Self {
            requests_per_second: 1000,
            burst_size: 1000,
        }
    }
}

/// Create a custom rate limiter with specific configuration.
pub fn create_rate_limiter(
    config: RateLimiterConfig,
) -> Arc<GovernorRateLimiter<NotKeyed, InMemoryState, DefaultClock>> {
    let period_ms = 1000 / config.requests_per_second;
    let quota = Quota::with_period(Duration::from_millis(period_ms as u64))
        .unwrap()
        .allow_burst(NonZeroU32::new(config.burst_size).unwrap());

    Arc::new(GovernorRateLimiter::direct(quota))
}

/// Rate limiting middleware using the global limiter.
///
/// Returns 429 Too Many Requests if the limit is exceeded.
pub async fn rate_limit_middleware(req: Request, next: Next) -> Response {
    match GLOBAL_LIMITER.check() {
        Ok(_) => next.run(req).await,
        Err(_) => {
            warn!("Rate limit exceeded");
            (
                StatusCode::TOO_MANY_REQUESTS,
                Json(json!({
                    "error": "Too many requests",
                    "code": "RATE_LIMIT_EXCEEDED",
                    "retry_after_ms": 1
                })),
            )
                .into_response()
        }
    }
}
