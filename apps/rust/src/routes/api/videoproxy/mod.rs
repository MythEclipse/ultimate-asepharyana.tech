use axum::{
    extract::{Query, State},
    http::{HeaderMap, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use serde::Deserialize;
use std::sync::Arc;
use crate::routes::ChatState; // Updated path to ChatState

#[derive(Debug, Deserialize)]
pub struct VideoProxyParams {
    url: String,
}

use axum::{routing::get, Router};

pub fn create_routes() -> Router<Arc<ChatState>> {
    Router::new()
        .route("/", get(video_proxy_handler))
}

pub async fn video_proxy_handler(
    Query(params): Query<VideoProxyParams>,
    State(_state): State<Arc<ChatState>>, // State is not used here, but kept for consistency
) -> Response {
    let url = params.url;

    if url.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            "URL is required".to_string(),
        )
            .into_response();
    }

    // Fetch the video
    let client = reqwest::Client::new();
    let response = client.get(&url).send().await;

    match response {
        Ok(res) => {
            let status = res.status();
            let headers = res.headers().clone();
            let body = res.bytes().await;

            match body {
                Ok(bytes) => {
                    // Check content type for video
                    let content_type = headers.get(reqwest::header::CONTENT_TYPE)
                        .and_then(|v| v.to_str().ok())
                        .unwrap_or("");

                    if !content_type.starts_with("video/") {
                        eprintln!("URL does not point to a video: {}", url);
                        return (
                            StatusCode::BAD_REQUEST,
                            "URL does not point to a video".to_string(),
                        )
                            .into_response();
                    }

                    let mut response_builder = Response::builder().status(status);

                    // Copy relevant headers from the fetched response
                    if let Some(content_type) = headers.get(reqwest::header::CONTENT_TYPE) {
                        response_builder = response_builder.header(reqwest::header::CONTENT_TYPE, content_type);
                    }
                    if let Some(cache_control) = headers.get(reqwest::header::CACHE_CONTROL) {
                        response_builder = response_builder.header(reqwest::header::CACHE_CONTROL, cache_control);
                    }
                    if let Some(expires) = headers.get(reqwest::header::EXPIRES) {
                        response_builder = response_builder.header(reqwest::header::EXPIRES, expires);
                    }

                    // Add CORS headers
                    response_builder = response_builder
                        .header("Access-Control-Allow-Origin", "*")
                        .header("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS")
                        .header("Access-Control-Allow-Headers", "Content-Type, Authorization");

                    response_builder.body(bytes.into()).unwrap_or_else(|e| {
                        eprintln!("Error building response: {:?}", e);
                        StatusCode::INTERNAL_SERVER_ERROR.into_response()
                    })
                }
                Err(e) => {
                    eprintln!("Error reading response body: {:?}", e);
                    StatusCode::INTERNAL_SERVER_ERROR.into_response()
                }
            }
        }
        Err(e) => {
            eprintln!("Error fetching video: {:?}", e);
            StatusCode::BAD_GATEWAY.into_response()
        }
    }
}
