use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub port: u16,
    pub database_url: String,
    pub env: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, envy::Error> {
        // Always load .env when loading config
        match dotenvy::dotenv() {
            Ok(path) => tracing::info!("Loaded environment from {:?}", path),
            Err(e) => tracing::warn!("Could not load .env file: {}", e),
        }

        tracing::info!("Loading configuration from environment variables...");
        // Try to load from envy first, with fallbacks
        let mut config: AppConfig = match envy::from_env() {
            Ok(cfg) => {
                tracing::info!("Loaded config from environment: {:?}", cfg);
                cfg
            }
            Err(e) => {
                tracing::warn!("Failed to load config from environment, using defaults: {}", e);
                AppConfig {
                    port: 4091,
                    database_url: "mysql://root:password@localhost:3306/rustexpress".to_string(),
                    env: "development".to_string(),
                }
            }
        };

        // Override with direct environment variable access for compatibility
        if let Ok(port_str) = env::var("PORT") {
            match port_str.parse::<u16>() {
                Ok(port) => {
                    tracing::info!("Overriding port from env: {}", port);
                    config.port = port;
                }
                Err(e) => {
                    tracing::error!("Failed to parse PORT env variable: {}", e);
                }
            }
        }

        if let Ok(db_url) = env::var("DATABASE_URL") {
            if !db_url.is_empty() {
                tracing::info!("Overriding database_url from env: {}", db_url);
                config.database_url = db_url;
            }
        }

        if let Ok(node_env) = env::var("NODE_ENV") {
            tracing::info!("Overriding env from env: {}", node_env);
            config.env = node_env;
        }

        tracing::info!("Final configuration: {:?}", config);
        Ok(config)
    }
}