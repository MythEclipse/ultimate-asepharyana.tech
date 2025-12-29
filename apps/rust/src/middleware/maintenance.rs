//! Maintenance mode middleware.
//!
//! Enable/disable application access during maintenance.
//!
//! # Example
//!
//! ```ignore
//! use rust::middleware::maintenance::{MaintenanceMode, MaintenanceConfig};
//!
//! let maintenance = MaintenanceMode::new(redis_pool);
//!
//! // Enable maintenance
//! maintenance.enable("Updating database...").await?;
//!
//! // Disable maintenance
//! maintenance.disable().await?;
//!
//! // Use as middleware
//! app.layer(axum::middleware::from_fn(maintenance.middleware()));
//! ```

use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{Html, IntoResponse, Response},
};
use deadpool_redis::{redis::AsyncCommands, Pool};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::Arc;

const MAINTENANCE_KEY: &str = "app:maintenance";

/// Maintenance mode configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaintenanceConfig {
    /// Whether maintenance mode is enabled.
    pub enabled: bool,
    /// Message to display during maintenance.
    pub message: String,
    /// Retry-After header value in seconds.
    pub retry_after: Option<u64>,
    /// Allowed IP addresses (bypass maintenance).
    pub allowed_ips: HashSet<String>,
    /// Allowed paths (bypass maintenance).
    pub allowed_paths: HashSet<String>,
    /// Secret token to bypass maintenance.
    pub secret: Option<String>,
}

impl Default for MaintenanceConfig {
    fn default() -> Self {
        let mut allowed_paths = HashSet::new();
        allowed_paths.insert("/health".to_string());
        allowed_paths.insert("/api/health".to_string());

        Self {
            enabled: false,
            message: "We are currently performing maintenance. Please try again later.".to_string(),
            retry_after: Some(300),
            allowed_ips: HashSet::new(),
            allowed_paths,
            secret: None,
        }
    }
}

/// Maintenance mode error.
#[derive(Debug, thiserror::Error)]
pub enum MaintenanceError {
    #[error("Redis error: {0}")]
    RedisError(String),
    #[error("Serialization error: {0}")]
    SerializationError(String),
}

/// Maintenance mode manager.
#[derive(Clone)]
pub struct MaintenanceMode {
    pool: Arc<Pool>,
}

impl MaintenanceMode {
    /// Create a new maintenance mode manager.
    pub fn new(pool: Arc<Pool>) -> Self {
        Self { pool }
    }

    /// Enable maintenance mode.
    pub async fn enable(&self, message: &str) -> Result<(), MaintenanceError> {
        let config = MaintenanceConfig {
            enabled: true,
            message: message.to_string(),
            ..Default::default()
        };
        self.set_config(&config).await
    }

    /// Enable with custom config.
    pub async fn enable_with_config(
        &self,
        config: MaintenanceConfig,
    ) -> Result<(), MaintenanceError> {
        let mut config = config;
        config.enabled = true;
        self.set_config(&config).await
    }

    /// Disable maintenance mode.
    pub async fn disable(&self) -> Result<(), MaintenanceError> {
        let mut conn = self.pool.get().await.map_err(|e| {
            tracing::error!("Redis connection error: {}", e);
            MaintenanceError::RedisError(e.to_string())
        })?;

        conn.del::<_, ()>(MAINTENANCE_KEY).await.map_err(|e| {
            tracing::error!("Redis del error: {}", e);
            MaintenanceError::RedisError(e.to_string())
        })?;

        tracing::info!("Maintenance mode disabled");
        Ok(())
    }

    /// Get current config.
    pub async fn get_config(&self) -> Result<Option<MaintenanceConfig>, MaintenanceError> {
        let mut conn = self.pool.get().await.map_err(|e| {
            tracing::error!("Redis connection error: {}", e);
            MaintenanceError::RedisError(e.to_string())
        })?;

        let json: Option<String> = conn.get(MAINTENANCE_KEY).await.map_err(|e| {
            tracing::error!("Redis get error: {}", e);
            MaintenanceError::RedisError(e.to_string())
        })?;

        match json {
            Some(j) => {
                let config: MaintenanceConfig = serde_json::from_str(&j)
                    .map_err(|e| MaintenanceError::SerializationError(e.to_string()))?;
                Ok(Some(config))
            }
            None => Ok(None),
        }
    }

    /// Set maintenance config.
    async fn set_config(&self, config: &MaintenanceConfig) -> Result<(), MaintenanceError> {
        let mut conn = self.pool.get().await.map_err(|e| {
            tracing::error!("Redis connection error: {}", e);
            MaintenanceError::RedisError(e.to_string())
        })?;

        let json = serde_json::to_string(config)
            .map_err(|e| MaintenanceError::SerializationError(e.to_string()))?;

        conn.set::<_, _, ()>(MAINTENANCE_KEY, &json)
            .await
            .map_err(|e| {
                tracing::error!("Redis set error: {}", e);
                MaintenanceError::RedisError(e.to_string())
            })?;

        tracing::info!("Maintenance mode enabled: {}", config.message);
        Ok(())
    }

    /// Check if maintenance is enabled.
    pub async fn is_enabled(&self) -> bool {
        match self.get_config().await {
            Ok(Some(config)) => config.enabled,
            _ => false,
        }
    }
}

/// Maintenance mode middleware function.
pub fn maintenance_middleware(
    pool: Arc<Pool>,
) -> impl Fn(Request, Next) -> std::pin::Pin<Box<dyn std::future::Future<Output = Response> + Send>>
       + Clone
       + Send {
    move |req: Request, next: Next| {
        let pool = pool.clone();
        Box::pin(async move {
            let maintenance = MaintenanceMode::new(pool);

            let config = match maintenance.get_config().await {
                Ok(Some(c)) if c.enabled => c,
                _ => return next.run(req).await,
            };

            // Check allowed paths
            let path = req.uri().path();
            if config.allowed_paths.iter().any(|p| path.starts_with(p)) {
                return next.run(req).await;
            }

            // Check secret bypass
            if let Some(secret) = &config.secret {
                if let Some(header) = req.headers().get("X-Maintenance-Secret") {
                    if header.to_str().unwrap_or("") == secret {
                        return next.run(req).await;
                    }
                }
            }

            // Check allowed IPs
            if let Some(addr) = req
                .headers()
                .get("X-Forwarded-For")
                .and_then(|h| h.to_str().ok())
                .and_then(|s| s.split(',').next())
            {
                if config.allowed_ips.contains(addr.trim()) {
                    return next.run(req).await;
                }
            }

            // Return maintenance page
            let mut response = Html(maintenance_page(&config.message)).into_response();
            *response.status_mut() = StatusCode::SERVICE_UNAVAILABLE;

            if let Some(retry) = config.retry_after {
                response
                    .headers_mut()
                    .insert("Retry-After", retry.to_string().parse().unwrap());
            }

            response
        })
    }
}

/// Generate maintenance page HTML.
fn maintenance_page(message: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Maintenance</title>
    <style>
        body {{
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            display: flex;
            justify-content: center;
            align-items: center;
            min-height: 100vh;
            margin: 0;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
        }}
        .container {{
            text-align: center;
            padding: 2rem;
        }}
        h1 {{ font-size: 3rem; margin-bottom: 1rem; }}
        p {{ font-size: 1.2rem; opacity: 0.9; }}
        .icon {{ font-size: 4rem; margin-bottom: 1rem; }}
    </style>
</head>
<body>
    <div class="container">
        <div class="icon">🔧</div>
        <h1>Under Maintenance</h1>
        <p>{}</p>
    </div>
</body>
</html>"#,
        message
    )
}
