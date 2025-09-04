//! Handler for the apiproxy endpoint.
#![allow(dead_code)]

use axum::{ response::IntoResponse, routing::get, Json, Router };
use std::sync::Arc;
use crate::routes::AppState;
use serde::{ Deserialize, Serialize };
use utoipa::ToSchema;

pub const ENDPOINT_METHOD: &str = "get";
pub const ENDPOINT_PATH: &str = "/apiproxy";
pub const ENDPOINT_DESCRIPTION: &str = "Description for the apiproxy endpoint";
pub const ENDPOINT_TAG: &str = "apiproxy";
pub const SUCCESS_RESPONSE_BODY: &str = "Json<ApiproxyResponse>";

/// Response structure for list endpoints.
/// Replace `serde_json::Value` with your actual data types and implement `utoipa::ToSchema` for complex types.
#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct ApiproxyResponse {
  /// Success message
  pub message: String,
  /// List of items - replace with actual Vec<T> where T implements ToSchema
  pub data: Vec<serde_json::Value>,
  /// Total number of items
  pub total: Option<u64>,
}
#[utoipa::path(
    get,
    path = "/api/apiproxy",
    tag = "apiproxy",
    operation_id = "apiproxy",
    responses(
        (status = 200, description = "Handles GET requests for the /apiproxy endpoint.", body = ApiproxyResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn apiproxy() -> impl IntoResponse {
  Json(ApiproxyResponse {
    message: "Hello from apiproxy!".to_string(),
    data: vec![],
    total: Some(0),
  })
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(apiproxy))
}