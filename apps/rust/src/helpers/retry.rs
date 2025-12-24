//! HTTP retry utilities with exponential backoff.

use backoff::ExponentialBackoff;
use std::time::Duration;

/// Default retry configuration for HTTP requests.
pub fn default_backoff() -> ExponentialBackoff {
    ExponentialBackoff {
        initial_interval: Duration::from_millis(500),
        max_interval: Duration::from_secs(10),
        multiplier: 2.0,
        max_elapsed_time: Some(Duration::from_secs(30)),
        ..Default::default()
    }
}

/// Create a custom exponential backoff.
pub fn custom_backoff(
    initial_ms: u64,
    max_secs: u64,
    multiplier: f64,
    max_elapsed_secs: u64,
) -> ExponentialBackoff {
    ExponentialBackoff {
        initial_interval: Duration::from_millis(initial_ms),
        max_interval: Duration::from_secs(max_secs),
        multiplier,
        max_elapsed_time: Some(Duration::from_secs(max_elapsed_secs)),
        ..Default::default()
    }
}

/// Quick backoff for fast retries (3 attempts, 100ms initial).
pub fn quick_backoff() -> ExponentialBackoff {
    ExponentialBackoff {
        initial_interval: Duration::from_millis(100),
        max_interval: Duration::from_secs(1),
        multiplier: 2.0,
        max_elapsed_time: Some(Duration::from_secs(5)),
        ..Default::default()
    }
}

/// Slow backoff for long operations (10 attempts, 1s initial).
pub fn slow_backoff() -> ExponentialBackoff {
    ExponentialBackoff {
        initial_interval: Duration::from_secs(1),
        max_interval: Duration::from_secs(30),
        multiplier: 2.0,
        max_elapsed_time: Some(Duration::from_secs(120)),
        ..Default::default()
    }
}

/// Make an error transient (will be retried).
pub fn transient<E>(err: E) -> backoff::Error<E> {
    backoff::Error::transient(err)
}

/// Make an error permanent (will NOT be retried).
pub fn permanent<E>(err: E) -> backoff::Error<E> {
    backoff::Error::permanent(err)
}

// Re-export retry function for convenience
pub use backoff::future::retry;
