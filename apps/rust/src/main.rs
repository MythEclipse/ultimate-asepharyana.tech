#![doc = "Logging Setup"]
extern crate lazy_static;
// This application uses [`tracing`](https://docs.rs/tracing) for structured logging.
// The log level is controlled by the `RUST_LOG` environment variable (e.g., `info`, `debug`, `warn`, `error`).
// Example usage in `.env`:
// RUST_LOG=info
// Logging is initialized from the environment via `tracing_subscriber`.

use std::net::SocketAddr;

use std::sync::Arc;

use axum::Router;


use sea_orm::{Database, DatabaseConnection};
use tower_http::compression::{CompressionLayer, CompressionLevel};
use tower_http::cors::CorsLayer;
use tracing_subscriber::EnvFilter;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use rustexpress::core::config::CONFIG;
use rustexpress::infra::redis::REDIS_POOL;

use rustexpress::routes::api::{create_api_routes, ApiDoc};
use rustexpress::routes::AppState;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing with config log level
    let filter = &CONFIG.log_level;
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new(filter))
        .init();

    tracing::info!("🚀 RustExpress starting up...");
    tracing::info!("   Environment: {}", CONFIG.environment);

    // Log thread configuration
    let worker_threads = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);
    tracing::info!(
        "   Tokio Worker Threads: (Defaulting to CPU cores: {})",
        worker_threads
    );

    // Get JWT secret from config (validated at startup)
    let jwt_secret = CONFIG.jwt_secret.clone();

    // Initialize Redis pool early
    let _ = REDIS_POOL.get().await;

    // Initialize browser pool for anime2 scraping
    tracing::info!("Initializing browser pool...");
    let mut browser_config = rustexpress::browser::BrowserPoolConfig::default();
    browser_config.headless = true;
    browser_config.sandbox = false; // CRITICAL: Fix for VPS/Docker crash running as root
    match rustexpress::browser::pool::init_browser_pool(browser_config).await {
        Ok(_) => tracing::info!("✓ Browser pool initialized"),
        Err(e) => {
            tracing::error!("⚠️ Failed to initialize browser pool: {}", e);
            // Non-fatal, continue startup in headless environment
        }
    }
    tracing::info!("✓ Browser pool initialized");

    // SeaORM connection using validated config
    let mut opt = sea_orm::ConnectOptions::new(CONFIG.database_url.clone());
    opt.max_connections(CONFIG.db.max_connections)
        .min_connections(CONFIG.db.min_connections)
        .connect_timeout(std::time::Duration::from_secs(
            CONFIG.db.connect_timeout_seconds,
        ))
        .idle_timeout(std::time::Duration::from_secs(
            CONFIG.db.idle_timeout_seconds,
        ))
        .acquire_timeout(std::time::Duration::from_secs(
            CONFIG.db.acquire_timeout_seconds,
        ))
        .max_lifetime(std::time::Duration::from_secs(
            CONFIG.db.max_lifetime_seconds,
        ))
        .sqlx_logging(CONFIG.log_level == "debug"); // Only log SQL in debug mode

    let db: DatabaseConnection = Database::connect(opt)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to connect to MySQL database: {}", e))?;
    tracing::info!("✓ SeaORM database connection established");

    // Initialize Schema (Bookmarks)
    if let Err(e) = rustexpress::infra::db_setup::init(&db).await {
        tracing::error!("Failed to init DB schema: {}", e);
    }

    // Seed default chat data if tables are empty
    if let Err(e) = rustexpress::seeder::seed::seed_chat_data_if_empty(&db).await {
        tracing::warn!("Failed to seed chat data: {}", e);
    }

    // Create broadcast channel for WebSocket chat messages
    let (chat_tx, _) = tokio::sync::broadcast::channel(1000);

    let db_arc = Arc::new(db);

    // Limit background uploads to 5 as requested by user
    let semaphore_permit = CONFIG.image_processing_concurrency;
    tracing::info!(
        "Initializing image processing semaphore with {} permits (Global Upload Limit)",
        semaphore_permit
    );
    let image_processing_semaphore = Arc::new(tokio::sync::Semaphore::new(semaphore_permit));

    // Initialize RoomManager for WebSocket rooms
    let room_manager = Arc::new(rustexpress::ws::room::RoomManager::new());
    tracing::info!("✓ WebSocket RoomManager initialized");

    let app_state = Arc::new(AppState {
        jwt_secret,
        redis_pool: REDIS_POOL.clone(),
        db: db_arc.clone(),
        pool: db_arc.clone(),
        chat_tx,
        image_processing_semaphore,
        room_manager: room_manager.clone(),
    });

    // Initialize scheduler for background tasks
    tracing::info!("Initializing scheduler...");
    let scheduler = rustexpress::scheduler::Scheduler::new()
        .await
        .expect("Failed to create scheduler");

    // Add cache cleanup task (runs daily at 2 AM)
    let cache_cleanup = rustexpress::scheduler::CleanupOldCache::new(db_arc.clone());
    scheduler
        .add(cache_cleanup)
        .await
        .expect("Failed to add cache cleanup task");

    // Add room cleanup task (runs every 5 minutes)
    let room_cleanup = rustexpress::scheduler::CleanupEmptyRooms::new(room_manager.clone());
    scheduler
        .add(room_cleanup)
        .await
        .expect("Failed to add room cleanup task");

    // Start scheduler
    scheduler.start().await.expect("Failed to start scheduler");
    tracing::info!("✓ Scheduler started with 2 tasks");

    tracing::info!("Building application routes...");
    let cors = CorsLayer::permissive();

    let app = Router::new()
        .merge(create_api_routes().with_state(app_state.clone()))
        .merge(
            rustexpress::routes::ws::register_routes(Router::new()).with_state(app_state.clone()),
        )
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(CompressionLayer::new().quality(CompressionLevel::Fastest))
        .layer(cors);

    let port = CONFIG.server_port;
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("Binding server to address: {}", addr);

    // Bind a TcpListener and use axum::serve (keeps compatibility with current axum version)
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("Server listening on {}", listener.local_addr()?);

    axum::serve(listener, app.into_make_service()).await?;
    tracing::info!("RustExpress shutting down.");

    Ok(())
}
