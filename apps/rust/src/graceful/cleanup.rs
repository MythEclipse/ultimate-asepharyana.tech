//! Graceful shutdown with proper resource cleanup.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::signal;
use tokio::sync::Notify;
use tokio::time::{sleep, Duration};
use tracing::info;

use crate::helpers::memory::MemoryMetrics;
use crate::ws::RoomManager;

/// Graceful shutdown coordinator.
pub struct ShutdownCoordinator {
    /// Shutdown signal flag
    is_shutting_down: Arc<AtomicBool>,
    /// Notify for graceful shutdown
    shutdown_notify: Arc<Notify>,
    /// Room manager for cleanup
    room_manager: Option<Arc<RoomManager>>,
    /// Memory metrics
    metrics: Option<Arc<MemoryMetrics>>,
}

impl ShutdownCoordinator {
    /// Create a new shutdown coordinator.
    pub fn new() -> Self {
        Self {
            is_shutting_down: Arc::new(AtomicBool::new(false)),
            shutdown_notify: Arc::new(Notify::new()),
            room_manager: None,
            metrics: None,
        }
    }

    /// Set room manager for cleanup.
    pub fn with_room_manager(mut self, manager: Arc<RoomManager>) -> Self {
        self.room_manager = Some(manager);
        self
    }

    /// Set memory metrics.
    pub fn with_metrics(mut self, metrics: Arc<MemoryMetrics>) -> Self {
        self.metrics = Some(metrics);
        self
    }

    /// Check if shutdown is in progress.
    pub fn is_shutting_down(&self) -> bool {
        self.is_shutting_down.load(Ordering::Relaxed)
    }

    /// Start shutdown process.
    pub fn initiate_shutdown(&self) {
        info!("🛑 Initiating graceful shutdown...");
        self.is_shutting_down.store(true, Ordering::Relaxed);
        self.shutdown_notify.notify_waiters();
    }

    /// Wait for shutdown signal.
    pub async fn wait_for_shutdown_signal(&self) {
        let ctrl_c = async {
            if let Err(e) = signal::ctrl_c().await {
                tracing::error!("Failed to listen for Ctrl+C: {}", e);
            }
        };

        #[cfg(unix)]
        let terminate = async {
            match signal::unix::signal(signal::unix::SignalKind::terminate()) {
                Ok(mut stream) => {
                    stream.recv().await;
                }
                Err(e) => {
                    tracing::error!("Failed to listen for SIGTERM: {}", e);
                }
            }
        };

        #[cfg(not(unix))]
        let terminate = std::future::pending::<()>();

        tokio::select! {
            _ = ctrl_c => {
                info!("Received Ctrl+C signal");
            }
            _ = terminate => {
                info!("Received SIGTERM signal");
            }
        }

        self.initiate_shutdown();
    }

    /// Perform cleanup operations.
    pub async fn cleanup(&self) {
        info!("🧹 Starting cleanup operations...");

        // Give active requests time to finish
        info!("Waiting for active requests to complete...");
        sleep(Duration::from_secs(5)).await;

        // Clean up empty rooms
        if let Some(ref manager) = self.room_manager {
            let removed = manager.cleanup_empty_rooms();
            info!("Cleaned up {} empty rooms", removed);
        }

        // Log final metrics
        if let Some(ref metrics) = self.metrics {
            info!("Final metrics:");
            metrics.log_metrics();
        }

        info!("✅ Cleanup completed");
    }

    /// Get a handle for checking shutdown status.
    pub fn handle(&self) -> ShutdownHandle {
        ShutdownHandle {
            is_shutting_down: Arc::clone(&self.is_shutting_down),
        }
    }
}

impl Default for ShutdownCoordinator {
    fn default() -> Self {
        Self::new()
    }
}

/// Handle for checking shutdown status.
#[derive(Clone)]
pub struct ShutdownHandle {
    is_shutting_down: Arc<AtomicBool>,
}

impl ShutdownHandle {
    /// Check if shutdown is in progress.
    pub fn is_shutting_down(&self) -> bool {
        self.is_shutting_down.load(Ordering::Relaxed)
    }
}

/// Wait for shutdown signal and perform cleanup.
pub async fn wait_for_shutdown_and_cleanup(
    room_manager: Option<Arc<RoomManager>>,
    metrics: Option<Arc<MemoryMetrics>>,
) {
    let mut coordinator = ShutdownCoordinator::new();

    if let Some(manager) = room_manager {
        coordinator = coordinator.with_room_manager(manager);
    }

    if let Some(m) = metrics {
        coordinator = coordinator.with_metrics(m);
    }

    coordinator.wait_for_shutdown_signal().await;
    coordinator.cleanup().await;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shutdown_coordinator() {
        let coordinator = ShutdownCoordinator::new();
        assert!(!coordinator.is_shutting_down());

        coordinator.initiate_shutdown();
        assert!(coordinator.is_shutting_down());
    }

    #[test]
    fn test_shutdown_handle() {
        let coordinator = ShutdownCoordinator::new();
        let handle = coordinator.handle();

        assert!(!handle.is_shutting_down());

        coordinator.initiate_shutdown();
        assert!(handle.is_shutting_down());
    }
}
