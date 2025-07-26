//! # Logging Setup
//!
//! This application uses [`tracing`](https://docs.rs/tracing) for structured logging.
//! The log level is controlled by the `RUST_LOG` environment variable (e.g., `info`, `debug`, `warn`, `error`).
//! Example usage in `.env`:
//! ```env
//! RUST_LOG=info
//! ```
//! Logging is initialized with `tracing_subscriber::EnvFilter` for environment-based configuration.
//! See the code below for details.
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use crate::config::AppConfig;
use crate::routes::{create_routes, ChatState};
use sqlx::mysql::MySqlPoolOptions;
use std::sync::Arc;

mod config;
mod routes;
mod models;
mod chat_service;
mod pdf_service;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing::info!("RustExpress starting up...");

    // Always load .env from current directory
    match dotenvy::dotenv() {
        Ok(path) => tracing::info!("Loaded environment from {:?}", path),
        Err(e) => tracing::warn!("Could not load .env file: {}", e),
    }

    // Set default values for Rust-specific configs if not present
    if std::env::var("PORT").is_err() {
        tracing::info!("PORT not set, using default 4091");
        std::env::set_var("PORT", "4091");
    }
    if std::env::var("RUST_LOG").is_err() {
        tracing::info!("RUST_LOG not set, using default 'info'");
        std::env::set_var("RUST_LOG", "info");
    }
    // Don't override DATABASE_URL since it should come from root .env

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Initializing configuration...");
    let config = AppConfig::from_env()?;

    tracing::info!("Connecting to database...");
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await?;
    tracing::info!("Database connection established.");

    tracing::info!("Running database migrations...");
    sqlx::migrate!().run(&pool).await?;
    tracing::info!("Database migrations complete.");

    tracing::info!("Creating chat state...");
    let chat_state = Arc::new(ChatState {
        pool: Arc::new(pool),
        clients: Default::default(),
    });

    tracing::info!("Building application routes...");
    let app = create_routes().with_state(chat_state);

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    tracing::info!("Binding server to address: {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("Server listening on {}", listener.local_addr()?);

    axum::serve(listener, app).await?;
    tracing::info!("RustExpress shutting down.");

    Ok(())
}