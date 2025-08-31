// Handles GET /api/proxy: Proxies a remote URL provided as a `url` query parameter, mirroring the Next.js handler logic.
// Uses reqwest for HTTP requests and axum for routing and response construction.

use axum::{
    extract::Query,
    http::{HeaderMap, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
struct ProxyParams {
    url: Option<String>,
}

pub fn router() -> Router {
    Router::new().route("/api/proxy", get(proxy_get))
}

async fn proxy_get(Query(params): Query<ProxyParams>) -> impl IntoResponse {
    let Some(target_url) = params.url else {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Missing url parameter" })),
        );
    };

    let client = Client::new();
    let resp = match client.get(&target_url).send().await {
        Ok(r) => r,
        Err(e) => {
            return (
                StatusCode::BAD_GATEWAY,
                Json(json!({
                    "error": "Failed to fetch URL",
                    "details": e.to_string(),
                    "status": 502,
                })),
            );
        }
    };

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("text/plain")
        .to_string();

    let mut headers = HeaderMap::new();
    headers.insert(
        "content-type",
        HeaderValue::from_str(&content_type).unwrap_or(HeaderValue::from_static("text/plain")),
    );
    headers.insert(
        "X-Proxy-Used",
        HeaderValue::from_static("reqwest"),
    );

    // Try to handle JSON responses as JSON, otherwise return as bytes/text
    if content_type.contains("application/json") {
        match resp.bytes().await {
            Ok(bytes) => {
                let json_result = serde_json::from_slice::<serde_json::Value>(&bytes);
                match json_result {
                    Ok(json_val) => {
                        return (StatusCode::OK, headers, Json(json_val)).into_response();
                    }
                    Err(_) => {
                        // Fallback: return as text
                        return (
                            StatusCode::OK,
                            headers,
                            bytes,
                        )
                            .into_response();
                    }
                }
            }
            Err(e) => {
                return (
                    StatusCode::BAD_GATEWAY,
                    Json(json!({
                        "error": "Failed to read response body",
                        "details": e.to_string(),
                        "status": 502,
                    })),
                );
            }
        }
    } else {
        // For other content types, return as bytes
        match resp.bytes().await {
            Ok(bytes) => (StatusCode::OK, headers, bytes).into_response(),
            Err(e) => (
                StatusCode::BAD_GATEWAY,
                Json(json!({
                    "error": "Failed to read response body",
                    "details": e.to_string(),
                    "status": 502,
                })),
            )
                .into_response(),
        }
    }
}
