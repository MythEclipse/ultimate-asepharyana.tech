//! # drivepng API
//!
//! This module provides API endpoints for drivepng.

use axum::{
    response::{IntoResponse, Response},
    Json,
    Router,
    routing::get,
};
use serde_json::json;
use crate::routes::ChatState;
use std::sync::Arc;

pub async fn drivepng_handler() -> Response {
    // This is a dummy handler. Replace with actual logic.
    Json(json!({
        "message": "DrivePNG API endpoint"
    }))
    .into_response()
}

pub fn create_routes() -> Router<Arc<ChatState>> {
    Router::new()
        .route("/", get(drivepng_handler))
}
