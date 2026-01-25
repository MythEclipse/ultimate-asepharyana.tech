//! Graceful shutdown implementation.

use std::future::Future;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::signal;
use tokio::sync::broadcast;
use tracing::info;

/// Graceful shutdown controller.
pub struct GracefulShutdown {
    shutdown_tx: broadcast::Sender<()>,
    is_shutting_down: Arc<AtomicBool>,
}

impl GracefulShutdown {
    /// Create a new graceful shutdown controller.
    pub fn new() -> Self {
        let (shutdown_tx, _) = broadcast::channel(1);
        Self {
            shutdown_tx,
            is_shutting_down: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Check if shutdown has been initiated.
    pub fn is_shutting_down(&self) -> bool {
        self.is_shutting_down.load(Ordering::SeqCst)
    }

    /// Get a receiver for shutdown notifications.
    pub fn subscribe(&self) -> broadcast::Receiver<()> {
        self.shutdown_tx.subscribe()
    }

    /// Initiate shutdown.
    pub fn shutdown(&self) {
        if !self.is_shutting_down.swap(true, Ordering::SeqCst) {
            info!("🛑 Initiating graceful shutdown...");
            let _ = self.shutdown_tx.send(());
        }
    }

    /// Wait for shutdown signal and then gracefully drain.
    pub async fn wait_for_shutdown(&self, drain_timeout: Duration) {
        // Wait for shutdown signal
        shutdown_signal().await;

        self.shutdown();

        // Give time for in-flight requests to complete
        info!(
            "⏳ Waiting {}s for in-flight requests...",
            drain_timeout.as_secs()
        );
        tokio::time::sleep(drain_timeout).await;

        info!("✅ Graceful shutdown complete");
    }
}

impl Default for GracefulShutdown {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for GracefulShutdown {
    fn clone(&self) -> Self {
        Self {
            shutdown_tx: self.shutdown_tx.clone(),
            is_shutting_down: Arc::clone(&self.is_shutting_down),
        }
    }
}

/// Wait for a shutdown signal (SIGTERM, SIGINT, or Ctrl+C).
pub async fn shutdown_signal() {
    let ctrl_c = async {
        if let Err(e) = signal::ctrl_c().await {
            tracing::error!("Failed to install Ctrl+C handler: {}", e);
        }
    };

    #[cfg(unix)]
    let terminate = async {
        match signal::unix::signal(signal::unix::SignalKind::terminate()) {
            Ok(mut stream) => {
                stream.recv().await;
            }
            Err(e) => {
                tracing::error!("Failed to install SIGTERM handler: {}", e);
            }
        }
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            info!("📥 Received Ctrl+C signal");
        }
        _ = terminate => {
            info!("📥 Received SIGTERM signal");
        }
    }
}

/// Create a shutdown future that can be used with axum's serve.
pub fn create_shutdown_signal() -> impl Future<Output = ()> + Send + 'static {
    async {
        shutdown_signal().await;
    }
}
