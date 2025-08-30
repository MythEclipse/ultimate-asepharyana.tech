use axum::{
    extract::{Path, State},
    http::{HeaderMap, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use std::sync::Arc;
use crate::routes::mod_::ChatState;
use reqwest::Client;

use once_cell::sync::Lazy;
use std::env;

static PRODUCTION_URL: Lazy<String> = Lazy::new(|| {
    env::var("PRODUCTION_URL").unwrap_or_else(|_| "https://asepharyana.tech".to_string())
});

pub async fn uploader_get_handler(
    Path(file_name): Path<String>,
    State(_state): State<Arc<ChatState>>, // State is not used here, but kept for consistency
) -> Response {
    if file_name.is_empty() {
        return (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "File not found" })),
        )
            .into_response();
    }

    let original_url = format!("https://pomf2.lain.la/f/{}", file_name);
    let client = Client::new();

    match client.get(&original_url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36")
        .send()
        .await
    {
        Ok(res) => {
            let status = res.status();
            let content_type = res.headers().get(reqwest::header::CONTENT_TYPE)
                .and_then(|v| v.to_str().ok())
                .unwrap_or("application/octet-stream")
                .to_string();
            let content_length = res.headers().get(reqwest::header::CONTENT_LENGTH)
                .and_then(|v| v.to_str().ok())
                .unwrap_or("0")
                .to_string();

            let body = res.bytes().await;

            match body {
                Ok(bytes) => {
                    Response::builder()
                        .status(status)
                        .header("Content-Type", content_type)
                        .header("Content-Disposition", format!("attachment; filename=\"{}\"", file_name))
                        .header("Content-Length", content_length)
                        .header("Cache-Control", "public, max-age=31536000")
                        .body(bytes.into())
                        .unwrap_or_else(|e| {
                            eprintln!("Error building response: {:?}", e);
                            StatusCode::INTERNAL_SERVER_ERROR.into_response()
                        })
                }
                Err(e) => {
                    eprintln!("Error reading response body: {:?}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({ "error": "Failed to fetch file" })),
                    )
                        .into_response()
                }
            }
        }
        Err(e) => {
            eprintln!("Uploader GET error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Failed to fetch file" })),
            )
                .into_response()
        }
    }
}

use axum::{routing::{get}, Router};

pub fn create_routes() -> Router<Arc<ChatState>> {
    Router::new()
        .route("/", get(uploader_get_handler))
}
