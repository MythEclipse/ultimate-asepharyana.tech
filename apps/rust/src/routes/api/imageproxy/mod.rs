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

#[derive(Debug, Deserialize)]
pub struct ImageProxyParams {
    url: String,
}

use axum::Router;
use axum::routing::get;

pub fn create_routes() -> Router<Arc<ChatState>> {
    Router::new()
        .route("/", get(image_proxy_handler))
}

pub async fn image_proxy_handler(
    Query(params): Query<ImageProxyParams>,
    State(_state): State<Arc<ChatState>>, // State is not used here, but kept for consistency
) -> impl IntoResponse {
    let url = params.url;

    if url.is_empty() {
        let mut headers = HeaderMap::new();
        headers.insert("Access-Control-Allow-Origin", HeaderValue::from_static("*"));
        headers.insert("Access-Control-Allow-Methods", HeaderValue::from_static("GET, POST, PUT, DELETE, OPTIONS"));
        headers.insert("Access-Control-Allow-Headers", HeaderValue::from_static("Content-Type, Authorization"));

        return (
            StatusCode::BAD_REQUEST,
            headers,
            Json(json!({ "error": "URL is required" })),
        ).into_response();
    }

    // Fetch the image
    let client = reqwest::Client::new();
    let response = client.get(&url).send().await;

    match response {
        Ok(res) => {
            let status = res.status();
            let headers = res.headers().clone();
            let body = res.bytes().await;

            match body {
                Ok(bytes) => {
                    let mut response_builder = Response::builder().status(status);

                    // Copy all headers from the fetched response
                    for (key, value) in headers.iter() {
                        response_builder = response_builder.header(key, value);
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
                    let mut headers = HeaderMap::new();
                    headers.insert("Access-Control-Allow-Origin", HeaderValue::from_static("*"));
                    headers.insert("Access-Control-Allow-Methods", HeaderValue::from_static("GET, POST, PUT, DELETE, OPTIONS"));
                    headers.insert("Access-Control-Allow-Headers", HeaderValue::from_static("Content-Type, Authorization"));
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        headers,
                        Json(json!({ "error": "Error reading response body" })),
                    ).into_response()
                }
            }
        }
        Err(e) => {
            eprintln!("Error fetching image: {:?}", e);
            let mut headers = HeaderMap::new();
            headers.insert("Access-Control-Allow-Origin", HeaderValue::from_static("*"));
            headers.insert("Access-Control-Allow-Methods", HeaderValue::from_static("GET, POST, PUT, DELETE, OPTIONS"));
            headers.insert("Access-Control-Allow-Headers", HeaderValue::from_static("Content-Type, Authorization"));
            (
                StatusCode::BAD_GATEWAY,
                headers,
                Json(json!({ "error": "Error fetching image" })),
            ).into_response()
        }
    }
}
