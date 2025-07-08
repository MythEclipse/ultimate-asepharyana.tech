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
        // Try to load from envy first, with fallbacks
        let mut config: AppConfig = envy::from_env().unwrap_or_else(|_| AppConfig {
            port: 4091,
            database_url: "mysql://root:password@localhost:3306/rustexpress".to_string(),
            env: "development".to_string(),
        });

        // Override with direct environment variable access for compatibility
        if let Ok(port_str) = env::var("PORT") {
            if let Ok(port) = port_str.parse::<u16>() {
                config.port = port;
            }
        }

        if let Ok(db_url) = env::var("DATABASE_URL") {
            if !db_url.is_empty() {
                config.database_url = db_url;
            }
        }

        if let Ok(node_env) = env::var("NODE_ENV") {
            config.env = node_env;
        }

        Ok(config)
    }
}