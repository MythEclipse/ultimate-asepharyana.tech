//! Memory monitoring and leak detection utilities.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::time::{interval, Duration};
use tracing::{info, warn};

/// Global counters for tracking allocations.
#[derive(Debug, Clone)]
pub struct MemoryMetrics {
    pub active_websockets: Arc<AtomicUsize>,
    pub active_tasks: Arc<AtomicUsize>,
    pub active_browser_tabs: Arc<AtomicUsize>,
    pub active_rooms: Arc<AtomicUsize>,
    pub total_connections: Arc<AtomicUsize>,
}

impl Default for MemoryMetrics {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryMetrics {
    /// Create new memory metrics tracker.
    pub fn new() -> Self {
        Self {
            active_websockets: Arc::new(AtomicUsize::new(0)),
            active_tasks: Arc::new(AtomicUsize::new(0)),
            active_browser_tabs: Arc::new(AtomicUsize::new(0)),
            active_rooms: Arc::new(AtomicUsize::new(0)),
            total_connections: Arc::new(AtomicUsize::new(0)),
        }
    }

    /// Increment WebSocket counter.
    pub fn websocket_connected(&self) {
        self.active_websockets.fetch_add(1, Ordering::Relaxed);
        self.total_connections.fetch_add(1, Ordering::Relaxed);
    }

    /// Decrement WebSocket counter.
    pub fn websocket_disconnected(&self) {
        self.active_websockets.fetch_sub(1, Ordering::Relaxed);
    }

    /// Increment task counter.
    pub fn task_spawned(&self) {
        self.active_tasks.fetch_add(1, Ordering::Relaxed);
    }

    /// Decrement task counter.
    pub fn task_completed(&self) {
        self.active_tasks.fetch_sub(1, Ordering::Relaxed);
    }

    /// Increment browser tab counter.
    pub fn tab_acquired(&self) {
        self.active_browser_tabs.fetch_add(1, Ordering::Relaxed);
    }

    /// Decrement browser tab counter.
    pub fn tab_released(&self) {
        self.active_browser_tabs.fetch_sub(1, Ordering::Relaxed);
    }

    /// Set room count.
    pub fn set_room_count(&self, count: usize) {
        self.active_rooms.store(count, Ordering::Relaxed);
    }

    /// Get current metrics.
    pub fn snapshot(&self) -> MetricsSnapshot {
        MetricsSnapshot {
            active_websockets: self.active_websockets.load(Ordering::Relaxed),
            active_tasks: self.active_tasks.load(Ordering::Relaxed),
            active_browser_tabs: self.active_browser_tabs.load(Ordering::Relaxed),
            active_rooms: self.active_rooms.load(Ordering::Relaxed),
            total_connections: self.total_connections.load(Ordering::Relaxed),
        }
    }

    /// Log current metrics.
    pub fn log_metrics(&self) {
        let snapshot = self.snapshot();
        info!(
            "Memory Metrics - WS: {}, Tasks: {}, Tabs: {}, Rooms: {}, Total Conn: {}",
            snapshot.active_websockets,
            snapshot.active_tasks,
            snapshot.active_browser_tabs,
            snapshot.active_rooms,
            snapshot.total_connections
        );
    }

    /// Check for potential memory leaks.
    pub fn check_for_leaks(&self) {
        let snapshot = self.snapshot();

        if snapshot.active_websockets > 1000 {
            warn!(
                "⚠️  High WebSocket count: {} (possible leak)",
                snapshot.active_websockets
            );
        }

        if snapshot.active_tasks > 5000 {
            warn!(
                "⚠️  High task count: {} (possible leak)",
                snapshot.active_tasks
            );
        }

        if snapshot.active_browser_tabs > 50 {
            warn!(
                "⚠️  High browser tab count: {} (possible leak)",
                snapshot.active_browser_tabs
            );
        }

        if snapshot.active_rooms > 100 {
            warn!(
                "⚠️  High room count: {} (possible leak)",
                snapshot.active_rooms
            );
        }
    }
}

/// Snapshot of metrics at a point in time.
#[derive(Debug, Clone)]
pub struct MetricsSnapshot {
    pub active_websockets: usize,
    pub active_tasks: usize,
    pub active_browser_tabs: usize,
    pub active_rooms: usize,
    pub total_connections: usize,
}

/// Start periodic memory monitoring task.
pub fn start_memory_monitor(metrics: MemoryMetrics, interval_secs: u64) {
    tokio::spawn(async move {
        let mut interval = interval(Duration::from_secs(interval_secs));

        loop {
            interval.tick().await;
            metrics.log_metrics();
            metrics.check_for_leaks();
        }
    });
}

/// RAII guard for tracking task lifetime.
pub struct TaskGuard {
    metrics: Arc<MemoryMetrics>,
}

impl TaskGuard {
    /// Create a new task guard.
    pub fn new(metrics: Arc<MemoryMetrics>) -> Self {
        metrics.task_spawned();
        Self { metrics }
    }
}

impl Drop for TaskGuard {
    fn drop(&mut self) {
        self.metrics.task_completed();
    }
}

/// RAII guard for tracking WebSocket lifetime.
pub struct WebSocketGuard {
    metrics: Arc<MemoryMetrics>,
}

impl WebSocketGuard {
    /// Create a new WebSocket guard.
    pub fn new(metrics: Arc<MemoryMetrics>) -> Self {
        metrics.websocket_connected();
        Self { metrics }
    }
}

impl Drop for WebSocketGuard {
    fn drop(&mut self) {
        self.metrics.websocket_disconnected();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_tracking() {
        let metrics = MemoryMetrics::new();

        metrics.websocket_connected();
        assert_eq!(metrics.active_websockets.load(Ordering::Relaxed), 1);

        metrics.websocket_disconnected();
        assert_eq!(metrics.active_websockets.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn test_task_guard() {
        let metrics = Arc::new(MemoryMetrics::new());

        {
            let _guard = TaskGuard::new(metrics.clone());
            assert_eq!(metrics.active_tasks.load(Ordering::Relaxed), 1);
        }

        assert_eq!(metrics.active_tasks.load(Ordering::Relaxed), 0);
    }
}
