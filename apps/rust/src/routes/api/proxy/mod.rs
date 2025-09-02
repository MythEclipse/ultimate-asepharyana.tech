// --- METADATA UNTUK BUILD.RS ---
const ENDPOINT_METHOD: &str = "GET";
const ENDPOINT_PATH: &str = "/api/proxy";
const ENDPOINT_DESCRIPTION: &str = "Proxy endpoint for fetching external URLs.";
const ENDPOINT_TAG: &str = "proxy";
const SUCCESS_RESPONSE_BODY: &str = "BinaryDataResponse";
const URL_DESCRIPTION: &str = "URL to proxy.";
// --- AKHIR METADATA ---

use axum::{
    extract::Query,
    response::{IntoResponse, Response},
    Json,
    Router,
    routing::get,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use http::HeaderValue;
use crate::routes::ChatState;
use std::sync::Arc;
use utoipa::ToSchema;
use axum::http::StatusCode;

#[derive(Debug, Deserialize, ToSchema)]
pub struct ProxyParams {
    pub url: Option<String>,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct BinaryDataResponse {
    // This struct will not be directly serialized as JSON, but represents the binary data
    // For OpenAPI, we might want to describe it as a file or stream.
    // For now, it's a placeholder to satisfy the schema requirement.
    pub message: String,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    pub message: String,
    pub error: String,
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(self)).into_response()
    }
}

pub fn create_routes() -> Router<Arc<ChatState>> {
    Router::new()
        .route("/", get(proxy_get))
}

pub async fn proxy_get(Query(params): Query<ProxyParams>) -> impl IntoResponse {
    let url = match params.url {
        Some(u) => u,
        None => {
            return ErrorResponse {
                message: "URL parameter is missing".to_string(),
                error: "Missing parameter".to_string(),
            }.into_response();
        }
    };

    let client = Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/107.0.0.0 Safari/537.36")
        .build()
        .unwrap();

    match client.get(&url).send().await {
        Ok(response) => {
            let status = response.status();
            let headers = response.headers().clone(); // Clone headers to use after consuming response
            let content_type = headers.get(http::header::CONTENT_TYPE)
                .and_then(|h| h.to_str().ok())
                .map(|s| s.to_string());

            let body = response.bytes().await.unwrap_or_default();

            let mut axum_response = Response::builder()
                .status(status);

            if let Some(ct) = content_type {
                axum_response = axum_response.header(http::header::CONTENT_TYPE, HeaderValue::from_str(&ct).unwrap());
            }

            axum_response.body(axum::body::Body::from(body)).unwrap().into_response()
        }
        Err(e) => {
            ErrorResponse {
                message: format!("Failed to proxy URL: {}", e),
                error: e.to_string(),
            }.into_response()
        }
    }
}
