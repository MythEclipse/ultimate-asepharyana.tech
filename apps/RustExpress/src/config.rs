use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub PORT: u16,
    pub DATABASE_URL: String,
    pub NODE_ENV: String,
    pub JWT_SECRET: String,
    pub NEXTAUTH_SECRET: String,
    pub NEXTAUTH_URL: String,
    pub SECRET: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, envy::Error> {
        // Try to load from envy first, with fallbacks
        let mut config: AppConfig = envy::from_env().unwrap_or_else(|_| AppConfig {
            PORT: 4091,
            DATABASE_URL: "mysql://root:password@localhost:3306/rustexpress".to_string(),
            NODE_ENV: "development".to_string(),
            JWT_SECRET: "changeme".to_string(),
            NEXTAUTH_SECRET: "changeme".to_string(),
            NEXTAUTH_URL: "http://localhost:3000".to_string(),
            SECRET: "changeme".to_string(),
        });

        // Override with direct environment variable access for compatibility
        if let Ok(port_str) = env::var("PORT") {
            if let Ok(port) = port_str.parse::<u16>() {
                config.PORT = port;
            }
        }

        if let Ok(db_url) = env::var("DATABASE_URL") {
            if !db_url.is_empty() {
                config.DATABASE_URL = db_url;
            }
        }

        if let Ok(node_env) = env::var("NODE_ENV") {
            config.NODE_ENV = node_env;
        }

        if let Ok(jwt_secret) = env::var("JWT_SECRET") {
            config.JWT_SECRET = jwt_secret;
        }

        if let Ok(nextauth_secret) = env::var("NEXTAUTH_SECRET") {
            config.NEXTAUTH_SECRET = nextauth_secret;
        }

        if let Ok(nextauth_url) = env::var("NEXTAUTH_URL") {
            config.NEXTAUTH_URL = nextauth_url;
        }

        if let Ok(secret) = env::var("SECRET") {
            config.SECRET = secret;
        }

        Ok(config)
    }
}