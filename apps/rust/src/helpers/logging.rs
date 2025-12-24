//! Logging utilities and helpers.

use std::time::Instant;
use tracing::{debug, error, info, warn};

/// Log entry with timing.
pub struct TimedOperation {
    name: String,
    start: Instant,
}

impl TimedOperation {
    /// Start a timed operation.
    pub fn start(name: impl Into<String>) -> Self {
        let name = name.into();
        info!("Starting: {}", name);
        Self {
            name,
            start: Instant::now(),
        }
    }

    /// Complete the operation and log duration.
    pub fn complete(self) {
        let duration = self.start.elapsed();
        info!("Completed: {} in {:?}", self.name, duration);
    }

    /// Complete with custom message.
    pub fn complete_with(self, message: &str) {
        let duration = self.start.elapsed();
        info!("{}: {} in {:?}", self.name, message, duration);
    }

    /// Fail the operation.
    pub fn fail(self, error: &str) {
        let duration = self.start.elapsed();
        error!("Failed: {} - {} after {:?}", self.name, error, duration);
    }

    /// Get elapsed time.
    pub fn elapsed(&self) -> std::time::Duration {
        self.start.elapsed()
    }
}

/// Log request info.
pub fn log_request(method: &str, path: &str, status: u16, duration_ms: u128) {
    let level = if status >= 500 {
        "ERROR"
    } else if status >= 400 {
        "WARN"
    } else {
        "INFO"
    };

    match level {
        "ERROR" => error!("[{}] {} {} - {}ms", status, method, path, duration_ms),
        "WARN" => warn!("[{}] {} {} - {}ms", status, method, path, duration_ms),
        _ => info!("[{}] {} {} - {}ms", status, method, path, duration_ms),
    }
}

/// Log with context.
#[macro_export]
macro_rules! log_ctx {
    ($level:ident, $ctx:expr, $($arg:tt)*) => {
        tracing::$level!(context = $ctx, $($arg)*);
    };
}

/// Log and return error.
pub fn log_error<E: std::fmt::Display>(context: &str, error: E) -> E {
    error!("[{}] Error: {}", context, error);
    error
}

/// Log and return error, mapped.
pub fn log_and_map<E, F, R>(context: &str, error: E, mapper: F) -> R
where
    E: std::fmt::Display,
    F: FnOnce(E) -> R,
{
    error!("[{}] Error: {}", context, error);
    mapper(error)
}

/// Create a span for tracing.
#[macro_export]
macro_rules! span {
    ($name:expr) => {
        tracing::info_span!($name)
    };
    ($name:expr, $($field:tt)*) => {
        tracing::info_span!($name, $($field)*)
    };
}

/// Performance logger for expensive operations.
pub struct PerfLogger {
    name: String,
    start: Instant,
    threshold_ms: u128,
}

impl PerfLogger {
    pub fn new(name: impl Into<String>, threshold_ms: u128) -> Self {
        Self {
            name: name.into(),
            start: Instant::now(),
            threshold_ms,
        }
    }

    pub fn checkpoint(&self, label: &str) {
        let elapsed = self.start.elapsed().as_millis();
        debug!("[PERF] {} - {}: {}ms", self.name, label, elapsed);
    }
}

impl Drop for PerfLogger {
    fn drop(&mut self) {
        let elapsed = self.start.elapsed().as_millis();
        if elapsed > self.threshold_ms {
            warn!(
                "[PERF] {} slow operation: {}ms (threshold: {}ms)",
                self.name, elapsed, self.threshold_ms
            );
        }
    }
}

/// Debug print for development.
#[cfg(debug_assertions)]
#[macro_export]
macro_rules! debug_print {
    ($($arg:tt)*) => {
        eprintln!("[DEBUG] {}", format!($($arg)*));
    };
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! debug_print {
    ($($arg:tt)*) => {};
}
