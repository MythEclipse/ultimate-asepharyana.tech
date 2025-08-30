use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Deserialize;
use serde_json::Value;
use std::sync::Arc;
use crate::routes::mod_::ChatState; // Updated path to ChatState

#[derive(Debug, Deserialize)]
pub struct ApiProxyParams {
    url: String,
}

use axum::{routing::get, Router};

pub fn create_routes() -> Router<Arc<ChatState>> {
    Router::new()
        .route("/", get(api_proxy_handler))
}

pub async fn api_proxy_handler(
    Query(params): Query<ApiProxyParams>,
    State(_state): State<Arc<ChatState>>, // State is not used here, but kept for consistency
) -> Response {
    let url = params.url;

    if url.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json("URL is required".into()),
        )
            .into_response();
    }

    // Fetch data from the target URL
    let client = reqwest::Client::new();
    let response = client.get(&url).send().await;

    match response {
        Ok(res) => {
            let status = res.status();
            let body = res.json::<Value>().await;

            match body {
                Ok(json_data) => {
                    (status, Json(json_data)).into_response()
                }
                Err(e) => {
                    eprintln!("Error parsing response body as JSON: {:?}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json("Failed to parse response as JSON".into()),
                    )
                        .into_response()
                }
            }
        }
        Err(e) => {
            eprintln!("Error fetching data from API: {:?}", e);
            (
                StatusCode::BAD_GATEWAY,
                Json("Failed to fetch data from the target API".into()),
            )
                .into_response()
        }
    }
}
