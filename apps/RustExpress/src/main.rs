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
    // Read the .env file to get the path to the main environment file
    if let Ok(env_content) = std::fs::read_to_string(".env") {
        let env_path = env_content.trim();
        if let Err(e) = dotenvy::from_filename(env_path) {
            tracing::warn!("Could not load {}: {}", env_path, e);
        }
    } else {
        // Fallback to local .env if available
        dotenvy::dotenv().ok();
    }

    // Set default values for Rust-specific configs if not present
    if std::env::var("PORT").is_err() {
        std::env::set_var("PORT", "4091");
    }
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    // Don't override DATABASE_URL since it should come from root .env

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = AppConfig::from_env()?;

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await?;

    sqlx::migrate!().run(&pool).await?;

    let chat_state = Arc::new(ChatState {
        pool: Arc::new(pool),
        clients: Default::default(),
    });

    let app = create_routes().with_state(chat_state);

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}