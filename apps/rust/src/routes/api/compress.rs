//! Handler for the compress endpoint.
#![allow(dead_code)]

use axum::{ response::IntoResponse, routing::get, Json, Router };
use std::sync::Arc;
use crate::routes::AppState;
use serde::{ Deserialize, Serialize };
use utoipa::ToSchema;

pub const ENDPOINT_METHOD: &str = "get";
pub const ENDPOINT_PATH: &str = "/compress";
pub const ENDPOINT_DESCRIPTION: &str = "Description for the compress endpoint";
pub const ENDPOINT_TAG: &str = "compress";
pub const SUCCESS_RESPONSE_BODY: &str = "Json<CompressResponse>";

/// Response structure for list endpoints.
/// Replace `serde_json::Value` with your actual data types and implement `utoipa::ToSchema` for complex types.
#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct CompressResponse {
  /// Success message
  pub message: String,
  /// List of items - replace with actual Vec<T> where T implements ToSchema
  pub data: Vec<serde_json::Value>,
  /// Total number of items
  pub total: Option<u64>,
}
#[utoipa::path(
    get,
    path = "/api/compress",
    tag = "compress",
    operation_id = "compress",
    responses(
        (status = 200, description = "Handles GET requests for the /compress endpoint.", body = CompressResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn compress() -> impl IntoResponse {
  Json(CompressResponse {
    message: "Hello from compress!".to_string(),
    data: vec![],
    total: Some(0),
  })
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(compress))
}