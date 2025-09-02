//! # Image and Video Compression API
//!
//! This module provides API endpoints for compressing images and videos from URLs.

use axum::{
    extract::{Query, State},
    response::{IntoResponse, Response},
    Json,
    Router, // Add Router import
    routing::get, // Add get import
};
use serde::Deserialize;
use std::sync::Arc;
use crate::routes::ChatState;
use serde_json::json;

pub mod compress_service;

/// OpenAPI doc for Compression API
#[derive(utoipa::OpenApi)]
#[openapi(
    paths(
        compress_handler
    ),
    tags(
        (name = "Compression", description = "Image and video compression API")
    )
)]
pub struct CompressApiDoc;

#[derive(Debug, Deserialize)]
pub struct CompressParams {
    url: String,
    size: String, // e.g. "100kb" or "50%"
}

#[utoipa::path(
    get,
    path = "/api/compress",
    responses(
        (status = 200, description = "Compression successful", body = String),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error")
    ),
    params(
        ("url" = String, Query, description = "URL of the image or video to compress"),
        ("size" = String, Query, description = "Target size or percentage (e.g., '100kb', '50%')")
    ),
    tag = "Compression"
)]
pub async fn compress_handler(Query(params): Query<CompressParams>) -> impl IntoResponse {
    let url = params.url;
    let size_param = params.size;

    if url.is_empty() || size_param.is_empty() {
        return (
            axum::http::StatusCode::BAD_REQUEST,
            Json(json!({"message": "URL and size parameters are required"})),
        ).into_response();
    }

    let is_image = url.ends_with(".jpg")
        || url.ends_with(".jpeg")
        || url.ends_with(".png")
        || url.ends_with(".gif")
        || url.ends_with(".webp");

    let result = if is_image {
        compress_service::compress_image_from_url(&url, &size_param).await
    } else {
        compress_service::compress_video_from_url(&url, &size_param).await
    };

    match result {
        Ok(compressed_url) => (
            axum::http::StatusCode::OK,
            Json(json!({"compressed_url": compressed_url})),
        )
            .into_response(),
        Err(e) => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"message": format!("Compression failed: {}", e)})),
        )
            .into_response(),
    }
}

pub fn create_routes() -> Router<Arc<ChatState>> {
    Router::new()
        .route("/", get(compress_handler))
}
