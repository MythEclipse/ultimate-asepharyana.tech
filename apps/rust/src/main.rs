#![doc = "Logging Setup"]
//
// This application uses [`tracing`](https://docs.rs/tracing) for structured logging.
// The log level is controlled by the `RUST_LOG` environment variable (e.g., `info`, `debug`, `warn`, `error`).
// Example usage in `.env`:
// RUST_LOG=info
// Logging is initialized from the environment via `tracing_subscriber`.

use std::net::SocketAddr;
use rust_lib::config::CONFIG_MAP;
use crate::routes::api::{create_api_routes, ApiDoc};
use crate::routes::AppState;
use std::sync::Arc;
use axum::Router;
use tracing_subscriber::EnvFilter;
use utoipa_swagger_ui::SwaggerUi;
use utoipa::OpenApi;

mod routes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  // Initialize tracing from RUST_LOG (or defaults)
  tracing_subscriber::fmt().with_env_filter(EnvFilter::from_default_env()).init();

  tracing::info!("RustExpress starting up...");

  // Use CONFIG_MAP for JWT_SECRET
  let jwt_secret = CONFIG_MAP.get("JWT_SECRET")
    .cloned()
    .expect("JWT_SECRET must be set in the environment");

  tracing::info!("Creating app state...");
  let app_state = Arc::new(AppState {
    jwt_secret,
  });

  tracing::info!("Building application routes...");
  let app = Router::new()
    .nest("/api", create_api_routes().with_state(app_state.clone()))
    .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()));

  let port = CONFIG_MAP.get("PORT")
    .and_then(|s| s.parse::<u16>().ok())
    .unwrap_or(3000);
  let addr = SocketAddr::from(([0, 0, 0, 0], port));
  tracing::info!("Binding server to address: {}", addr);

  // Bind a TcpListener and use axum::serve (keeps compatibility with current axum version)
  let listener = tokio::net::TcpListener::bind(&addr).await?;
  tracing::info!("Server listening on {}", listener.local_addr()?);

  axum::serve(listener, app).await?;
  tracing::info!("RustExpress shutting down.");

  Ok(())
}
