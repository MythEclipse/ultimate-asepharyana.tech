//! Async utilities and helpers.

use std::future::Future;
use std::time::Duration;
use tokio::time::timeout;

/// Run with timeout.
pub async fn with_timeout<T, F>(duration: Duration, future: F) -> Result<T, tokio::time::error::Elapsed>
where
    F: Future<Output = T>,
{
    timeout(duration, future).await
}

/// Run with timeout in seconds.
pub async fn timeout_secs<T, F>(secs: u64, future: F) -> Result<T, tokio::time::error::Elapsed>
where
    F: Future<Output = T>,
{
    timeout(Duration::from_secs(secs), future).await
}

/// Run with timeout in milliseconds.
pub async fn timeout_ms<T, F>(ms: u64, future: F) -> Result<T, tokio::time::error::Elapsed>
where
    F: Future<Output = T>,
{
    timeout(Duration::from_millis(ms), future).await
}

/// Sleep for duration.
pub async fn sleep(duration: Duration) {
    tokio::time::sleep(duration).await;
}

/// Sleep for seconds.
pub async fn sleep_secs(secs: u64) {
    tokio::time::sleep(Duration::from_secs(secs)).await;
}

/// Sleep for milliseconds.
pub async fn sleep_ms(ms: u64) {
    tokio::time::sleep(Duration::from_millis(ms)).await;
}

/// Run multiple futures concurrently and collect results.
pub async fn join_all<T, F>(futures: Vec<F>) -> Vec<T>
where
    F: Future<Output = T>,
{
    futures::future::join_all(futures).await
}

/// Run futures with concurrency limit.
pub async fn join_all_limited<T, F, Fut>(
    items: Vec<T>,
    concurrency: usize,
    f: F,
) -> Vec<Fut::Output>
where
    F: Fn(T) -> Fut,
    Fut: Future,
{
    use futures::stream::{self, StreamExt};
    
    stream::iter(items)
        .map(f)
        .buffer_unordered(concurrency)
        .collect()
        .await
}

/// Race multiple futures, returning first to complete.
pub async fn race<T, F1, F2>(f1: F1, f2: F2) -> T
where
    F1: Future<Output = T>,
    F2: Future<Output = T>,
{
    tokio::select! {
        result = f1 => result,
        result = f2 => result,
    }
}

/// Retry a future with delay between attempts.
pub async fn simple_retry<T, E, F, Fut>(
    attempts: usize,
    delay: Duration,
    mut f: F,
) -> Result<T, E>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T, E>>,
{
    let mut last_error = None;
    for i in 0..attempts {
        match f().await {
            Ok(result) => return Ok(result),
            Err(e) => {
                last_error = Some(e);
                if i < attempts - 1 {
                    tokio::time::sleep(delay).await;
                }
            }
        }
    }
    Err(last_error.unwrap())
}

/// Run in blocking thread pool.
pub async fn spawn_blocking<F, R>(f: F) -> Result<R, tokio::task::JoinError>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    tokio::task::spawn_blocking(f).await
}

/// Run as a background task (fire and forget).
pub fn spawn<F>(future: F)
where
    F: Future<Output = ()> + Send + 'static,
{
    tokio::spawn(future);
}

/// Debounce - only run after delay with no new calls.
pub struct Debouncer {
    delay: Duration,
}

impl Debouncer {
    pub fn new(delay: Duration) -> Self {
        Self { delay }
    }

    pub async fn debounce<F, Fut>(&self, f: F)
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = ()>,
    {
        tokio::time::sleep(self.delay).await;
        f().await;
    }
}
