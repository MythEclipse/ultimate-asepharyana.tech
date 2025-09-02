//! # Image and Video Compression API
//!
//! This module provides API endpoints for compressing images and videos from URLs.

use axum::{
    extract::Query,
    response::IntoResponse,
    Json,
    Router, // Add Router import
    routing::get,
};
use serde::Deserialize;
use std::sync::Arc;
use crate::routes::ChatState;
use serde_json::json;
use std::str::FromStr;

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
pub enum CompressionSize {
    Kilobytes(u32),
    Percentage(u8),
}

impl ToString for CompressionSize {
    fn to_string(&self) -> String {
        match self {
            CompressionSize::Kilobytes(kb) => format!("{}kb", kb),
            CompressionSize::Percentage(percent) => format!("{}%", percent),
        }
    }
}

impl FromStr for CompressionSize {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.ends_with("kb") {
            let kb_str = s.trim_end_matches("kb");
            let kb = kb_str.parse::<u32>().map_err(|_| format!("Invalid kilobytes value: {}", kb_str))?;
            Ok(CompressionSize::Kilobytes(kb))
        } else if s.ends_with("%") {
            let percent_str = s.trim_end_matches("%");
            let percent = percent_str.parse::<u8>().map_err(|_| format!("Invalid percentage value: {}", percent_str))?;
            if percent > 100 {
                return Err("Percentage cannot be greater than 100".to_string());
            }
            Ok(CompressionSize::Percentage(percent))
        } else {
            Err(format!("Invalid size format: {}. Expected '100kb' or '50%'", s))
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CompressParams {
    url: String,
    #[serde(deserialize_with = "deserialize_compression_size")]
    size: CompressionSize,
}

fn deserialize_compression_size<'de, D>(deserializer: D) -> Result<CompressionSize, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    CompressionSize::from_str(&s).map_err(serde::de::Error::custom)
}

#[utoipa::path(
    get,
    path = "/",
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
    let size = params.size;

    if url.is_empty() {
        return (
            axum::http::StatusCode::BAD_REQUEST,
            Json(json!({"message": "URL parameter is required"})),
        ).into_response();
    }

    // A more robust way to check content type would be to fetch headers or inspect content
    // For now, extending the basic check.
    let is_image = url.ends_with(".jpg")
        || url.ends_with(".jpeg")
        || url.ends_with(".png")
        || url.ends_with(".gif")
        || url.ends_with(".webp")
        || url.ends_with(".bmp")
        || url.ends_with(".tiff")
        || url.ends_with(".tif");


    let result = if is_image {
        compress_service::compress_image_from_url(&url, &size.to_string()).await
    } else {
        compress_service::compress_video_from_url(&url, &size.to_string()).await
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
