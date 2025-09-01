//! # Proxy API
//!
//! This module provides a proxy endpoint for fetching external URLs.

use axum::{
    extract::{Query, Path},
    response::{IntoResponse, Response},
    Json,
    Router, // Add Router import
    routing::get, // Add get import
};
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;
use http::HeaderValue;
use crate::routes::ChatState;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
struct ProxyParams {
    url: Option<String>,
}

pub fn create_routes() -> Router<Arc<ChatState>> {
    Router::new()
        .route("/", get(proxy_get))
}

#[utoipa::path(
    get,
    path = "/api/proxy",
    responses(
        (status = 200, description = "Proxy successful"),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error")
    ),
    params(
        ("url" = String, Query, description = "URL to proxy")
    ),
    tag = "Proxy"
)]
pub async fn proxy_get(Query(params): Query<ProxyParams>) -> impl IntoResponse {
    let url = match params.url {
        Some(u) => u,
        None => {
            return (
                axum::http::StatusCode::BAD_REQUEST,
                Json(json!({"message": "URL parameter is missing"})),
            ).into_response();
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

            axum_response.body(body.into()).unwrap().into_response()
        }
        Err(e) => {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": format!("Failed to proxy URL: {}", e)})),
            ).into_response()
        }
    }
}
