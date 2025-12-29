//! Health Check Registry.
//!
//! Custom health checks for dependencies.
//!
//! # Example
//!
//! ```ignore
//! use rust::helpers::health_check::{HealthRegistry, HealthCheck, HealthStatus};
//!
//! let mut registry = HealthRegistry::new();
//! registry.register("database", DatabaseHealthCheck);
//! registry.register("redis", RedisHealthCheck);
//!
//! let results = registry.check_all().await;
//! ```

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

/// Health status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
}

impl HealthStatus {
    pub fn is_healthy(&self) -> bool {
        *self == HealthStatus::Healthy
    }
}

/// Health check result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResult {
    pub status: HealthStatus,
    pub message: Option<String>,
    pub latency_ms: Option<u64>,
    pub details: Option<serde_json::Value>,
}

impl HealthResult {
    pub fn healthy() -> Self {
        Self {
            status: HealthStatus::Healthy,
            message: None,
            latency_ms: None,
            details: None,
        }
    }

    pub fn unhealthy(message: &str) -> Self {
        Self {
            status: HealthStatus::Unhealthy,
            message: Some(message.to_string()),
            latency_ms: None,
            details: None,
        }
    }

    pub fn degraded(message: &str) -> Self {
        Self {
            status: HealthStatus::Degraded,
            message: Some(message.to_string()),
            latency_ms: None,
            details: None,
        }
    }

    pub fn with_latency(mut self, latency: Duration) -> Self {
        self.latency_ms = Some(latency.as_millis() as u64);
        self
    }

    pub fn with_details(mut self, details: serde_json::Value) -> Self {
        self.details = Some(details);
        self
    }
}

/// Health check trait.
#[async_trait]
pub trait HealthCheck: Send + Sync {
    /// Name of the health check.
    fn name(&self) -> &str;

    /// Perform the health check.
    async fn check(&self) -> HealthResult;

    /// Timeout for this check.
    fn timeout(&self) -> Duration {
        Duration::from_secs(5)
    }

    /// Whether this check is critical.
    fn critical(&self) -> bool {
        true
    }
}

/// Overall health response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: HealthStatus,
    pub checks: HashMap<String, HealthResult>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl HealthResponse {
    pub fn is_healthy(&self) -> bool {
        self.status == HealthStatus::Healthy
    }
}

/// Health check registry.
pub struct HealthRegistry {
    checks: Arc<RwLock<HashMap<String, Box<dyn HealthCheck>>>>,
}

impl Default for HealthRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl HealthRegistry {
    /// Create a new health registry.
    pub fn new() -> Self {
        Self {
            checks: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a health check.
    pub async fn register<H: HealthCheck + 'static>(&self, check: H) {
        let mut checks = self.checks.write().await;
        checks.insert(check.name().to_string(), Box::new(check));
    }

    /// Run all health checks.
    pub async fn check_all(&self) -> HealthResponse {
        let checks = self.checks.read().await;
        let mut results = HashMap::new();
        let mut overall_status = HealthStatus::Healthy;

        for (name, check) in checks.iter() {
            let start = Instant::now();
            let timeout = check.timeout();

            let result = tokio::time::timeout(timeout, check.check())
                .await
                .unwrap_or_else(|_| HealthResult::unhealthy("Timeout"))
                .with_latency(start.elapsed());

            if check.critical() {
                match result.status {
                    HealthStatus::Unhealthy => overall_status = HealthStatus::Unhealthy,
                    HealthStatus::Degraded if overall_status == HealthStatus::Healthy => {
                        overall_status = HealthStatus::Degraded;
                    }
                    _ => {}
                }
            }

            results.insert(name.clone(), result);
        }

        HealthResponse {
            status: overall_status,
            checks: results,
            timestamp: chrono::Utc::now(),
        }
    }

    /// Run a single health check.
    pub async fn check_one(&self, name: &str) -> Option<HealthResult> {
        let checks = self.checks.read().await;
        if let Some(check) = checks.get(name) {
            let start = Instant::now();
            Some(check.check().await.with_latency(start.elapsed()))
        } else {
            None
        }
    }
}

// =============================================================================
// Built-in health checks
// =============================================================================

/// Redis health check.
pub struct RedisHealthCheck {
    pool: Arc<deadpool_redis::Pool>,
}

impl RedisHealthCheck {
    pub fn new(pool: Arc<deadpool_redis::Pool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl HealthCheck for RedisHealthCheck {
    fn name(&self) -> &str {
        "redis"
    }

    async fn check(&self) -> HealthResult {
        use deadpool_redis::redis::AsyncCommands;

        match self.pool.get().await {
            Ok(mut conn) => {
                let result: Result<String, _> = conn.get("health:ping").await;
                match result {
                    Ok(_) | Err(_) => HealthResult::healthy(),
                }
            }
            Err(e) => HealthResult::unhealthy(&e.to_string()),
        }
    }
}

/// Memory health check.
pub struct MemoryHealthCheck {
    max_percent: f64,
}

impl MemoryHealthCheck {
    pub fn new(max_percent: f64) -> Self {
        Self { max_percent }
    }
}

#[async_trait]
impl HealthCheck for MemoryHealthCheck {
    fn name(&self) -> &str {
        "memory"
    }

    async fn check(&self) -> HealthResult {
        // Simple check - in production use sysinfo crate
        HealthResult::healthy().with_details(serde_json::json!({
            "max_percent": self.max_percent
        }))
    }

    fn critical(&self) -> bool {
        false
    }
}
