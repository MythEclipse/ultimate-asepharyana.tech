//! Type-safe application configuration.
//!
//! This module provides a strongly-typed configuration system that:
//! - Loads from environment variables and optional TOML files
//! - Fails fast at startup if required variables are missing
//! - Supports hierarchical configuration (default -> environment-specific)

use config::{Config, ConfigError, Environment, File};
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::env;

/// Application configuration loaded at startup.
/// All fields are required unless marked as `Option<T>`.
#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    /// Database connection URL (MySQL)
    pub database_url: String,

    /// Secret key for JWT signing
    pub jwt_secret: String,

    /// Redis connection URL
    #[serde(default)]
    pub redis_url: String,

    /// Server port to bind to
    #[serde(default = "default_port")]
    pub server_port: u16,

    /// Environment (development, staging, production)
    #[serde(default = "default_env")]
    pub environment: String,

    /// Allowed CORS origins (comma-separated)
    #[serde(default)]
    pub cors_origins: Vec<String>,

    /// Log level (trace, debug, info, warn, error)
    #[serde(default = "default_log_level")]
    pub log_level: String,

    /// SMTP configuration for emails (optional)
    pub smtp: Option<SmtpConfig>,
}

/// SMTP configuration for sending emails
#[derive(Debug, Clone, Deserialize)]
pub struct SmtpConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub from_email: String,
    pub from_name: String,
}

fn default_port() -> u16 {
    4091
}

fn default_env() -> String {
    "development".to_string()
}

fn default_log_level() -> String {
    "info".to_string()
}

impl AppConfig {
    /// Load configuration from environment and optional config files.
    ///
    /// Priority (highest to lowest):
    /// 1. Environment variables (prefixed with APP_)
    /// 2. `config/{environment}.toml`
    /// 3. `config/default.toml`
    pub fn load() -> Result<Self, ConfigError> {
        // Load .env file first
        if let Err(e) = dotenvy::dotenv() {
            tracing::debug!("Could not load .env file: {}", e);
        }

        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let config = Config::builder()
            // Start with default config file
            .add_source(File::with_name("config/default").required(false))
            // Layer on environment-specific values
            .add_source(File::with_name(&format!("config/{}", run_mode)).required(false))
            // Add environment variables (with APP_ prefix)
            .add_source(
                Environment::with_prefix("APP")
                    .separator("__")
                    .try_parsing(true)
                    .list_separator(","),
            )
            // Map legacy env vars to new config structure
            .set_override_option("database_url", env::var("DATABASE_URL").ok())?
            .set_override_option("jwt_secret", env::var("JWT_SECRET").ok())?
            .set_override_option("redis_url", env::var("REDIS_URL").ok())?
            .build()?;

        config.try_deserialize()
    }

    /// Check if running in production mode
    pub fn is_production(&self) -> bool {
        self.environment == "production"
    }

    /// Check if running in development mode
    pub fn is_development(&self) -> bool {
        self.environment == "development"
    }
}

/// Global configuration instance, loaded once at startup.
/// Panics if configuration is invalid - this is intentional for fail-fast behavior.
pub static CONFIG: Lazy<AppConfig> = Lazy::new(|| {
    AppConfig::load().unwrap_or_else(|e| {
        eprintln!("❌ Failed to load configuration: {}", e);
        eprintln!("   Make sure all required environment variables are set:");
        eprintln!("   - DATABASE_URL");
        eprintln!("   - JWT_SECRET");
        eprintln!("   - REDIS_URL (or APP_REDIS_URL)");
        std::process::exit(1);
    })
});

// ============================================================================
// Legacy compatibility layer
// ============================================================================

use std::collections::HashMap;

/// Legacy CONFIG_MAP for backward compatibility.
/// New code should use `CONFIG` directly.
#[deprecated(note = "Use CONFIG struct directly for type safety")]
pub static CONFIG_MAP: Lazy<HashMap<String, String>> = Lazy::new(|| {
    // Load .env
    let _ = dotenvy::dotenv();

    let mut map = HashMap::new();
    for (key, value) in env::vars() {
        // Skip variables with control characters
        if value.contains('\r') || value.contains('\n') || value.contains('\x1b') {
            continue;
        }
        map.insert(key, value);
    }
    map
});
