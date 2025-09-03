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
use rust_lib::config::CONFIG_MAP;
use crate::routes::{create_routes, ChatState};
use std::sync::Arc;
use axum::Router;

mod routes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing::info!("RustExpress starting up...");

    // Use CONFIG_MAP for JWT_SECRET
    let jwt_secret = CONFIG_MAP
        .get("JWT_SECRET")
        .cloned()
        .expect("JWT_SECRET must be set in the environment");

    tracing::info!("Creating app state..."); // Changed from chat state
    let chat_state = Arc::new(ChatState {
        jwt_secret,
    });

    tracing::info!("Building application routes...");

    let app = Router::new()
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
