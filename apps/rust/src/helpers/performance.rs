//! Performance optimization utilities and best practices guide.

/// Performance optimization tips for Rust application
///
/// 1. **Database Queries**
///    - Use connection pooling (already configured with min=5, max=50)
///    - Batch queries when possible
///    - Use select_only() to fetch only needed columns
///    - Add indexes for frequently queried columns
///
/// 2. **Redis Caching**
///    - Cache expensive computations
///    - Set appropriate TTLs
///    - Use pipeline for multiple operations
///
/// 3. **Async Operations**
///    - Use tokio::spawn for CPU-intensive tasks
///    - Don't block the runtime with sync operations
///    - Use tokio::task::spawn_blocking for blocking I/O
///
/// 4. **Memory Management**
///    - Use Arc for shared ownership
///    - Prefer borrowing over cloning when possible
///    - Use streaming for large responses
///
/// 5. **Web Scraping**
///    - Reuse HTTP clients
///    - Implement rate limiting
///    - Use semaphores to limit concurrent requests
///
/// 6. **Error Handling**
///    - Use Result types for recoverable errors
///    - Log errors appropriately
///    - Return structured error responses
use std::future::Future;
use std::time::Instant;
use tracing::{info, warn};

/// Measure execution time of an async operation
pub async fn measure_time<F, Fut, T>(name: &str, f: F) -> T
where
    F: FnOnce() -> Fut,
    Fut: Future<Output = T>,
{
    let start = Instant::now();
    let result = f().await;
    let duration = start.elapsed();

    if duration.as_millis() > 100 {
        warn!("{} took {:?}", name, duration);
    } else {
        info!("{} took {:?}", name, duration);
    }

    result
}

/// Performance monitoring macro
#[macro_export]
macro_rules! measure {
    ($name:expr, $block:expr) => {{
        let start = std::time::Instant::now();
        let result = $block;
        let duration = start.elapsed();
        if duration.as_millis() > 100 {
            tracing::warn!("{} took {:?}", $name, duration);
        } else {
            tracing::debug!("{} took {:?}", $name, duration);
        }
        result
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_measure_time() {
        let result = measure_time("test_operation", || async {
            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
            42
        })
        .await;

        assert_eq!(result, 42);
    }
}
