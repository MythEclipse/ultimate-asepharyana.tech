#![doc = "Logging Setup"]
extern crate lazy_static;
// Temporary comment to force recompile
//
// This application uses [`tracing`](https://docs.rs/tracing) for structured logging.
// The log level is controlled by the `RUST_LOG` environment variable (e.g., `info`, `debug`, `warn`, `error`).
// Example usage in `.env`:
// RUST_LOG=info
// Logging is initialized from the environment via `tracing_subscriber`.

use std::net::SocketAddr;

use std::sync::Arc;

use axum::Router;
use http::{header, Method};
use sea_orm::{Database, DatabaseConnection};
use tower_http::cors::CorsLayer;
use tracing_subscriber::EnvFilter;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use rust::config::CONFIG;
use rust::redis_client::REDIS_POOL;

use rust::routes::api::{create_api_routes, ApiDoc};
use rust::routes::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing with config log level
    let filter = &CONFIG.log_level;
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new(filter))
        .init();

    tracing::info!("🚀 RustExpress starting up...");
    tracing::info!("   Environment: {}", CONFIG.environment);

    // Get JWT secret from config (validated at startup)
    let jwt_secret = CONFIG.jwt_secret.clone();

    // Initialize Redis pool early
    let _ = REDIS_POOL.get().await;

    // SeaORM connection using validated config
    let db: DatabaseConnection = Database::connect(&CONFIG.database_url)
        .await
        .expect("Failed to connect to MySQL database with SeaORM");
    tracing::info!("✓ SeaORM database connection established");

    // Seed default chat data if tables are empty
    if let Err(e) = rust::seed::seed_chat_data_if_empty(&db).await {
        tracing::warn!("Failed to seed chat data: {}", e);
    }

    // Create broadcast channel for WebSocket chat messages
    let (chat_tx, _) = tokio::sync::broadcast::channel(1000);

    let db_arc = Arc::new(db);
 
    let app_state = Arc::new(AppState {
        jwt_secret,
        redis_pool: REDIS_POOL.clone(),
        db: db_arc.clone(),
        pool: db_arc.clone(),
        chat_tx,
    });

    tracing::info!("Building application routes...");
    let allowed_origins = [
        "http://localhost:4090".parse().unwrap(),
        "http://localhost:3000".parse().unwrap(),
        "https://solid.asepharyana.tech".parse().unwrap(),
        "https://asepharyana.tech".parse().unwrap(),
    ];
    let cors = CorsLayer::new()
        .allow_origin(allowed_origins)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
            Method::PATCH,
        ])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION, header::ACCEPT])
        .allow_credentials(true);

    let app = Router::new()
        .merge(create_api_routes().with_state(app_state.clone()))
        .merge(rust::routes::ws::register_routes(Router::new()).with_state(app_state.clone()))
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(cors);

    let port = 4091;
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("Binding server to address: {}", addr);

    // Bind a TcpListener and use axum::serve (keeps compatibility with current axum version)
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("Server listening on {}", listener.local_addr()?);

    axum::serve(listener, app.into_make_service()).await?;
    tracing::info!("RustExpress shutting down.");

    Ok(())
}
