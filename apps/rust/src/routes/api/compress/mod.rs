use axum::{
    extract::{Query, State},
    http::{HeaderMap, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;
use crate::routes::ChatState; // Updated path to ChatState
use rust_lib::services::compress;

#[derive(Debug, Deserialize)]
pub struct CompressParams {
    url: String,
    size: String, // Can be "100kb" or "50%"
}

use axum::{routing::get, Router};

pub fn create_routes() -> Router<Arc<ChatState>> {
    Router::new()
        .route("/", get(compress_handler))
}

pub async fn compress_handler(
    Query(params): Query<CompressParams>,
    State(_state): State<Arc<ChatState>>, // State is not used here, but kept for consistency
) -> Response {
    let url = params.url;
    let size_param = params.size;

    if url.is_empty() || size_param.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Parameter url dan size diperlukan" })),
        )
            .into_response();
    }

    // TODO: Implement queueing mechanism if needed

    // Determine if it's an image or video based on URL extension
    let extension = url.split('.').last().unwrap_or("").to_lowercase();

    if ["jpg", "jpeg", "png"].contains(&extension.as_str()) {
        // Image compression logic
        match compress::compress_image_from_url(&url, &size_param).await {
            Ok(cdn_link) => {
                (
                    StatusCode::OK,
                    Json(json!({ "link": cdn_link })),
                )
                    .into_response()
            }
            Err(e) => {
                eprintln!("Image compression error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": "Kompresi gambar gagal" })),
                )
                    .into_response()
            }
        }
    } else if ["mp4", "mov", "avi"].contains(&extension.as_str()) {
        // Video compression logic
        match compress::compress_video_from_url(&url, &size_param).await {
            Ok(cdn_link) => {
                (
                    StatusCode::OK,
                    Json(json!({ "link": cdn_link })),
                )
                    .into_response()
            }
            Err(e) => {
                eprintln!("Video compression error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": "Kompresi video gagal" })),
                )
                    .into_response()
            }
        }
    } else {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Format tidak didukung" })),
        )
            .into_response()
    }
}
