//! Database Query Profiling.
//!
//! Track and analyze database query performance.
//!
//! # Example
//!
//! ```ignore
//! use rust::helpers::query_profiler::{QueryProfiler, QueryLog};
//!
//! let profiler = QueryProfiler::new();
//!
//! profiler.log("SELECT * FROM users WHERE id = ?", duration);
//!
//! // Get slow queries
//! let slow = profiler.slow_queries(100); // > 100ms
//! ```

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock};
use std::time::Duration;

/// Query log entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryLog {
    /// Query string (may be truncated).
    pub query: String,
    /// Execution duration in milliseconds.
    pub duration_ms: u64,
    /// Timestamp.
    pub timestamp: DateTime<Utc>,
    /// Rows affected (if available).
    pub rows_affected: Option<u64>,
    /// Location in code (file:line).
    pub location: Option<String>,
}

/// Query statistics.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QueryStats {
    pub total_queries: u64,
    pub total_duration_ms: u64,
    pub slow_queries: u64,
    pub avg_duration_ms: f64,
    pub max_duration_ms: u64,
}

/// Query profiler.
#[derive(Clone)]
pub struct QueryProfiler {
    logs: Arc<RwLock<Vec<QueryLog>>>,
    stats: Arc<ProfilerStats>,
    enabled: Arc<std::sync::atomic::AtomicBool>,
    slow_threshold_ms: u64,
    max_logs: usize,
}

struct ProfilerStats {
    total_queries: AtomicU64,
    total_duration_ms: AtomicU64,
    slow_queries: AtomicU64,
    max_duration_ms: AtomicU64,
}

impl Default for QueryProfiler {
    fn default() -> Self {
        Self::new()
    }
}

impl QueryProfiler {
    /// Create a new profiler.
    pub fn new() -> Self {
        Self {
            logs: Arc::new(RwLock::new(Vec::new())),
            stats: Arc::new(ProfilerStats {
                total_queries: AtomicU64::new(0),
                total_duration_ms: AtomicU64::new(0),
                slow_queries: AtomicU64::new(0),
                max_duration_ms: AtomicU64::new(0),
            }),
            enabled: Arc::new(std::sync::atomic::AtomicBool::new(true)),
            slow_threshold_ms: 100,
            max_logs: 1000,
        }
    }

    /// Create with custom settings.
    pub fn with_settings(slow_threshold_ms: u64, max_logs: usize) -> Self {
        Self {
            slow_threshold_ms,
            max_logs,
            ..Self::new()
        }
    }

    /// Enable profiling.
    pub fn enable(&self) {
        self.enabled.store(true, Ordering::SeqCst);
    }

    /// Disable profiling.
    pub fn disable(&self) {
        self.enabled.store(false, Ordering::SeqCst);
    }

    /// Check if profiling is enabled.
    pub fn is_enabled(&self) -> bool {
        self.enabled.load(Ordering::SeqCst)
    }

    /// Log a query.
    pub fn log(&self, query: &str, duration: Duration) {
        if !self.is_enabled() {
            return;
        }

        let duration_ms = duration.as_millis() as u64;

        // Update stats
        self.stats.total_queries.fetch_add(1, Ordering::SeqCst);
        self.stats
            .total_duration_ms
            .fetch_add(duration_ms, Ordering::SeqCst);

        if duration_ms > self.slow_threshold_ms {
            self.stats.slow_queries.fetch_add(1, Ordering::SeqCst);
            tracing::warn!(
                "Slow query ({}ms): {}",
                duration_ms,
                truncate_query(query, 200)
            );
        }

        // Update max
        let mut current_max = self.stats.max_duration_ms.load(Ordering::SeqCst);
        while duration_ms > current_max {
            match self.stats.max_duration_ms.compare_exchange(
                current_max,
                duration_ms,
                Ordering::SeqCst,
                Ordering::SeqCst,
            ) {
                Ok(_) => break,
                Err(v) => current_max = v,
            }
        }

        // Add to logs
        let log = QueryLog {
            query: truncate_query(query, 500),
            duration_ms,
            timestamp: Utc::now(),
            rows_affected: None,
            location: None,
        };

        if let Ok(mut logs) = self.logs.write() {
            logs.push(log);
            // Keep only last max_logs entries
            let len = logs.len();
            if len > self.max_logs {
                logs.drain(0..len - self.max_logs);
            }
        }
    }

    /// Log a query with additional info.
    pub fn log_full(
        &self,
        query: &str,
        duration: Duration,
        rows_affected: Option<u64>,
        location: Option<&str>,
    ) {
        if !self.is_enabled() {
            return;
        }

        let duration_ms = duration.as_millis() as u64;

        self.stats.total_queries.fetch_add(1, Ordering::SeqCst);
        self.stats
            .total_duration_ms
            .fetch_add(duration_ms, Ordering::SeqCst);

        if duration_ms > self.slow_threshold_ms {
            self.stats.slow_queries.fetch_add(1, Ordering::SeqCst);
        }

        let log = QueryLog {
            query: truncate_query(query, 500),
            duration_ms,
            timestamp: Utc::now(),
            rows_affected,
            location: location.map(String::from),
        };

        if let Ok(mut logs) = self.logs.write() {
            logs.push(log);
            let len = logs.len();
            if len > self.max_logs {
                logs.drain(0..len - self.max_logs);
            }
        }
    }

    /// Get all query logs.
    pub fn get_logs(&self) -> Vec<QueryLog> {
        self.logs.read().map(|l| l.clone()).unwrap_or_default()
    }

    /// Get slow queries.
    pub fn slow_queries(&self, threshold_ms: u64) -> Vec<QueryLog> {
        self.logs
            .read()
            .map(|logs| {
                logs.iter()
                    .filter(|l| l.duration_ms > threshold_ms)
                    .cloned()
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get query statistics.
    pub fn stats(&self) -> QueryStats {
        let total = self.stats.total_queries.load(Ordering::SeqCst);
        let total_duration = self.stats.total_duration_ms.load(Ordering::SeqCst);

        QueryStats {
            total_queries: total,
            total_duration_ms: total_duration,
            slow_queries: self.stats.slow_queries.load(Ordering::SeqCst),
            avg_duration_ms: if total > 0 {
                total_duration as f64 / total as f64
            } else {
                0.0
            },
            max_duration_ms: self.stats.max_duration_ms.load(Ordering::SeqCst),
        }
    }

    /// Clear logs and reset stats.
    pub fn clear(&self) {
        if let Ok(mut logs) = self.logs.write() {
            logs.clear();
        }
        self.stats.total_queries.store(0, Ordering::SeqCst);
        self.stats.total_duration_ms.store(0, Ordering::SeqCst);
        self.stats.slow_queries.store(0, Ordering::SeqCst);
        self.stats.max_duration_ms.store(0, Ordering::SeqCst);
    }
}

fn truncate_query(query: &str, max_len: usize) -> String {
    let query = query.trim();
    if query.len() <= max_len {
        query.to_string()
    } else {
        format!("{}...", &query[..max_len])
    }
}

/// Global query profiler.
static PROFILER: std::sync::OnceLock<QueryProfiler> = std::sync::OnceLock::new();

/// Initialize global profiler.
pub fn init_profiler() -> &'static QueryProfiler {
    PROFILER.get_or_init(QueryProfiler::new)
}

/// Get global profiler.
pub fn profiler() -> Option<&'static QueryProfiler> {
    PROFILER.get()
}
