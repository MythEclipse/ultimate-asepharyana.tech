// --- METADATA UNTUK BUILD.RS ---
const ENDPOINT_METHOD: &str = "GET";
const ENDPOINT_PATH: &str = "/api/drivepng";
const ENDPOINT_DESCRIPTION: &str = "DrivePNG API endpoint";
const ENDPOINT_TAG: &str = "drivepng";
const SUCCESS_RESPONSE_BODY: &str = "DrivePngResponse";
const SLUG_DESCRIPTION: &str = "No slug needed for DrivePNG.";
// --- AKHIR METADATA ---

use axum::{
    response::{IntoResponse, Response},
    Json,
    Router,
    routing::get,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use serde_json::json;
use crate::routes::ChatState;
use std::sync::Arc;
use axum::http::StatusCode;

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DrivePngResponse {
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

pub async fn drivepng_handler() -> Response {
    // This is a dummy handler. Replace with actual logic.
    Json(json!({
        "message": "DrivePNG API endpoint"
    }))
    .into_response()
}

pub fn create_routes() -> Router<Arc<ChatState>> {
    Router::new()
        .route("/", get(drivepng_handler))
}
