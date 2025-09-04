//! Routing module for the application.
//! All environment variables (including JWT secret) must be loaded via rust_lib::config::CONFIG_MAP

use axum::{
  routing::get,
  Router,
  response::Redirect,
  extract::State,
  response::IntoResponse,
  Json,
};
use std::sync::Arc;
use serde_json::json;
use utoipa_swagger_ui::SwaggerUi;
use crate::routes::api::ApiDoc;

pub mod api;
use utoipa::OpenApi;

pub struct AppState {
  /// JWT secret loaded from CONFIG_MAP
  #[allow(dead_code)]
  pub jwt_secret: String,
}

pub fn create_routes() -> Router<Arc<AppState>> {
  Router::new()
    .route("/", get(root_handler))
    .route("/api/health", get(health_check))
    .route("/api/status", get(status_handler))
    .nest("/api", api::create_api_routes())
    .merge(SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi()))
}

async fn root_handler() -> impl IntoResponse {
  Redirect::permanent("https://asepharyana.tech/chat")
}

// Health check endpoint
async fn health_check() -> impl IntoResponse {
  Json(
    json!({
        "status": "healthy",
        "service": "RustExpress",
        "version": "0.1.0",
        "timestamp": chrono::Utc::now().to_rfc3339()
    })
  )
}

async fn status_handler(State(_state): State<Arc<AppState>>) -> impl IntoResponse {
  // Simplified status as database is no longer directly accessed here
  Json(
    json!({
        "status": "running",
        "service": "RustExpress (Rust migration of Express.js)",
        "features": ["simplified_api_handlers"]
    })
  )
}
