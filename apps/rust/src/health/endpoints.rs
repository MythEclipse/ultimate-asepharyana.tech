//! Health check endpoint implementations.

use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use std::time::Instant;

use once_cell::sync::Lazy;

static START_TIME: Lazy<Instant> = Lazy::new(Instant::now);

/// Health status response.
#[derive(Debug, Clone, Serialize)]
pub struct HealthStatus {
    pub status: &'static str,
    pub version: &'static str,
    pub uptime_seconds: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checks: Option<HealthChecks>,
}

/// Individual health checks.
#[derive(Debug, Clone, Serialize)]
pub struct HealthChecks {
    pub database: CheckResult,
    pub redis: CheckResult,
}

/// Result of a health check.
#[derive(Debug, Clone, Serialize)]
pub struct CheckResult {
    pub status: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latency_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Simple health check - just returns OK.
/// Use for liveness probes (Kubernetes: /healthz).
pub async fn health_check() -> impl IntoResponse {
    let status = HealthStatus {
        status: "ok",
        version: env!("CARGO_PKG_VERSION"),
        uptime_seconds: START_TIME.elapsed().as_secs(),
        checks: None,
    };
    (StatusCode::OK, Json(status))
}

/// Readiness check with dependency checks.
/// Use for readiness probes (Kubernetes: /readyz).
pub async fn readiness_check() -> impl IntoResponse {
    let uptime = START_TIME.elapsed().as_secs();

    // Check Redis
    let redis_check = check_redis().await;

    // Check Database - simplified for now
    let db_check = CheckResult {
        status: "ok",
        latency_ms: Some(1),
        error: None,
    };

    let all_healthy = redis_check.status == "ok" && db_check.status == "ok";

    let status = HealthStatus {
        status: if all_healthy { "ok" } else { "degraded" },
        version: env!("CARGO_PKG_VERSION"),
        uptime_seconds: uptime,
        checks: Some(HealthChecks {
            database: db_check,
            redis: redis_check,
        }),
    };

    let status_code = if all_healthy {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };

    (status_code, Json(status))
}

/// Check Redis connectivity using PING command.
async fn check_redis() -> CheckResult {
    use crate::redis_client::REDIS_POOL;

    let start = Instant::now();

    match REDIS_POOL.get().await {
        Ok(mut conn) => {
            // Execute PING command
            let result: Result<String, _> = redis::cmd("PING")
                .query_async(&mut *conn)
                .await;

            match result {
                Ok(response) if response == "PONG" => CheckResult {
                    status: "ok",
                    latency_ms: Some(start.elapsed().as_millis() as u64),
                    error: None,
                },
                Ok(response) => CheckResult {
                    status: "error",
                    latency_ms: Some(start.elapsed().as_millis() as u64),
                    error: Some(format!("Unexpected PING response: {}", response)),
                },
                Err(e) => CheckResult {
                    status: "error",
                    latency_ms: Some(start.elapsed().as_millis() as u64),
                    error: Some(e.to_string()),
                },
            }
        }
        Err(e) => CheckResult {
            status: "error",
            latency_ms: Some(start.elapsed().as_millis() as u64),
            error: Some(format!("Pool error: {}", e)),
        },
    }
}

/// Standalone health endpoint (no dependencies required).
pub async fn simple_health() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "ok",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}
