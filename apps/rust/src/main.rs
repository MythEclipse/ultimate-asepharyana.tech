// Temporary comment to force recompile
#![doc = "Logging Setup"]
//
// This application uses [`tracing`](https://docs.rs/tracing) for structured logging.
// The log level is controlled by the `RUST_LOG` environment variable (e.g., `info`, `debug`, `warn`, `error`).
// Example usage in `.env`:
// RUST_LOG=info
// Logging is initialized from the environment via `tracing_subscriber`.

use std::net::SocketAddr;
use rust_lib::config::CONFIG_MAP;
use crate::routes::api::{ create_api_routes, ApiDoc };
use crate::routes::AppState;
use std::sync::Arc;
use axum::Router;
use tracing_subscriber::EnvFilter;
use utoipa_swagger_ui::SwaggerUi;
use utoipa::OpenApi;
use tower_http::cors::{ Any, CorsLayer };
use rust_lib::headless_chrome::launch_browser;

mod routes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let filter = std::env
    ::var_os("RUST_LOG")
    .and_then(|s| s.into_string().ok())
    .unwrap_or_else(|| "info".to_string());
  tracing_subscriber::fmt().with_env_filter(EnvFilter::new(&filter)).init();

  tracing::info!("RustExpress starting up...");

  let jwt_secret = CONFIG_MAP.get("JWT_SECRET")
    .cloned()
    .unwrap_or_else(|| {
      tracing::warn!(
        "JWT_SECRET not set in environment, using default secret (not recommended for production)"
      );
      "default_secret".to_string()
    });

  tracing::info!("Launching headless browser...");
  let client = match launch_browser(true, None).await {
      Ok(client) => client,
      Err(e) => {
          tracing::error!("Failed to launch browser: {:?}", e);
          tracing::error!("Make sure Chrome and ChromeDriver are installed and WebDriver is running on localhost:4444");
          tracing::error!("Install ChromeDriver: https://chromedriver.chromium.org/downloads");
          std::process::exit(1);
      }
  };

  let app_state = Arc::new(AppState {
    jwt_secret,
    browser_client: Arc::new(client),
  });

  tracing::info!("Building application routes...");
  let cors = CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any);

  let app = Router::new()
    .merge(create_api_routes().with_state(app_state.clone()))
    .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
    .layer(cors);

  let port = 4091;
  let addr = SocketAddr::from(([0, 0, 0, 0], port));
  tracing::info!("Binding server to address: {}", addr);

  // Bind a TcpListener and use axum::serve (keeps compatibility with current axum version)
  let listener = tokio::net::TcpListener::bind(&addr).await?;
  tracing::info!("Server listening on {}", listener.local_addr()?);

  axum::serve(listener, app).await?;
  tracing::info!("RustExpress shutting down.");

  Ok(())
}
