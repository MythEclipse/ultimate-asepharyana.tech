//! Prometheus metrics endpoint and utilities.

use axum::{http::StatusCode, response::IntoResponse};
use metrics::{counter, gauge, histogram};
use metrics_exporter_prometheus::{Matcher, PrometheusBuilder, PrometheusHandle};
use once_cell::sync::OnceCell;
use std::time::Instant;
use tracing::info;

static METRICS_HANDLE: OnceCell<PrometheusHandle> = OnceCell::new();

/// Set up the Prometheus metrics exporter.
/// Call this once at application startup.
pub fn setup_metrics() -> anyhow::Result<()> {
    let builder = PrometheusBuilder::new();

    // Configure histogram buckets for request latency
    let builder = builder.set_buckets_for_metric(
        Matcher::Full("http_request_duration_seconds".to_string()),
        &[
            0.001, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,
        ],
    )?;

    let handle = builder.install_recorder()?;

    METRICS_HANDLE
        .set(handle)
        .map_err(|_| anyhow::anyhow!("Metrics already initialized"))?;

    info!("📊 Prometheus metrics initialized");
    Ok(())
}

/// Handler for the /metrics endpoint.
pub struct MetricsHandler;

impl MetricsHandler {
    /// Render metrics for Prometheus scraping.
    pub async fn handle() -> impl IntoResponse {
        match METRICS_HANDLE.get() {
            Some(handle) => {
                let metrics = handle.render();
                (StatusCode::OK, metrics)
            }
            None => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Metrics not initialized".to_string(),
            ),
        }
    }
}

// ============================================================================
// Metrics Recording Helpers
// ============================================================================

/// Record an HTTP request.
pub fn record_http_request(method: &str, path: &str, status: u16, duration_secs: f64) {
    let labels = [
        ("method", method.to_string()),
        ("path", path.to_string()),
        ("status", status.to_string()),
    ];

    counter!("http_requests_total", &labels).increment(1);
    histogram!("http_request_duration_seconds", &labels).record(duration_secs);
}

/// Record active connections.
pub fn set_active_connections(count: usize) {
    gauge!("http_connections_active").set(count as f64);
}

/// Record database pool stats.
pub fn record_db_pool_stats(active: u32, idle: u32, max: u32) {
    gauge!("db_pool_connections_active").set(active as f64);
    gauge!("db_pool_connections_idle").set(idle as f64);
    gauge!("db_pool_connections_max").set(max as f64);
}

/// Record Redis pool stats.
pub fn record_redis_pool_stats(size: usize, available: usize) {
    gauge!("redis_pool_connections_size").set(size as f64);
    gauge!("redis_pool_connections_available").set(available as f64);
}

/// Record a background job.
pub fn record_job(job_type: &str, success: bool, duration_secs: f64) {
    let labels = [
        ("job_type", job_type.to_string()),
        ("success", success.to_string()),
    ];

    counter!("jobs_processed_total", &labels).increment(1);
    histogram!("job_duration_seconds", &labels).record(duration_secs);
}

/// Request timing helper.
pub struct RequestTimer {
    start: Instant,
    method: String,
    path: String,
}

impl RequestTimer {
    pub fn new(method: &str, path: &str) -> Self {
        Self {
            start: Instant::now(),
            method: method.to_string(),
            path: path.to_string(),
        }
    }

    pub fn record(self, status: u16) {
        let duration = self.start.elapsed().as_secs_f64();
        record_http_request(&self.method, &self.path, status, duration);
    }
}
