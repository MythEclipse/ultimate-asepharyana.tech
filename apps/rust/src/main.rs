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

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use rust_lib::config::CONFIG_MAP;
use crate::routes::{create_routes, ChatState};
use sqlx::mysql::MySqlPoolOptions;
use std::sync::Arc;
use axum::Router;

mod routes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing::info!("RustExpress starting up...");

    // .env is loaded by rust_lib::config, so no need to load again here

    // Set default values for Rust-specific configs if not present
    let rust_log = CONFIG_MAP
        .get("RUST_LOG")
        .cloned()
        .unwrap_or_else(|| "info".to_string());
    std::env::set_var("RUST_LOG", &rust_log);

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(rust_log))
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Initializing configuration...");

    // Use CONFIG_MAP for JWT_SECRET
    let jwt_secret = CONFIG_MAP
        .get("JWT_SECRET")
        .cloned()
        .expect("JWT_SECRET must be set in the environment");

    // Use CONFIG_MAP for DATABASE_URL
    let database_url = CONFIG_MAP
        .get("DATABASE_URL")
        .cloned()
        .expect("DATABASE_URL must be set in the environment");

    tracing::info!("Connecting to database...");
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;
    tracing::info!("Database connection established.");

    tracing::info!("Running database migrations...");
    // sqlx::migrate!().run(&pool).await?;
    tracing::info!("Database migrations complete.");

    tracing::info!("Creating chat state...");
    let chat_state = Arc::new(ChatState {
        pool: Arc::new(pool),
        clients: Default::default(),
        jwt_secret,
    });

    tracing::info!("Building application routes...");

    let app = Router::new()
        .merge(
            SwaggerUi::new("/docs").url("/api-doc/openapi.json", crate::routes::api::ApiDoc::openapi())
        )
        .merge(create_routes().with_state(chat_state));

    let port = CONFIG_MAP
        .get("PORT")
        .and_then(|s| s.parse::<u16>().ok())
        .unwrap_or(3000);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("Binding server to address: {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("Server listening on {}", listener.local_addr()?);

    axum::serve(listener, app).await?;
    tracing::info!("RustExpress shutting down.");

    Ok(())
}
