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

/// OpenAPI doc for DrivePNG API
#[derive(utoipa::OpenApi)]
#[openapi(
    paths(
        drivepng_handler
    ),
    tags(
        (name = "DrivePNG", description = "DrivePNG API")
    )
)]
pub struct DrivePngApiDoc;

#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "DrivePNG API response", body = String),
        (status = 500, description = "Internal server error")
    ),
    tag = "DrivePNG"
)]
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
