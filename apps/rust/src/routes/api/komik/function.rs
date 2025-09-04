//! Handler for the function endpoint.
#![allow(dead_code)]

use axum::{extract::Path,  response::IntoResponse, routing::get, Json, Router };
use std::sync::Arc;
use crate::routes::AppState;
use serde::{ Deserialize, Serialize };
use utoipa::ToSchema;

pub const ENDPOINT_METHOD: &str = "get";
pub const ENDPOINT_PATH: &str = "/komik/{function}";
pub const ENDPOINT_DESCRIPTION: &str = "Description for the function endpoint";
pub const ENDPOINT_TAG: &str = "komik.function";
pub const SUCCESS_RESPONSE_BODY: &str = "Json<FunctionResponse>";

/// Response structure for list endpoints.
/// Replace `serde_json::Value` with your actual data types and implement `utoipa::ToSchema` for complex types.
#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct FunctionResponse {
  /// Success message
  pub message: String,
  /// List of items - replace with actual Vec<T> where T implements ToSchema
  pub data: Vec<serde_json::Value>,
  /// Total number of items
  pub total: Option<u64>,
}
#[utoipa::path(
    get,
    params(
        ("function" = String, Path, description = "The function identifier")
    ),
    path = "/api/komik/{function}",
    tag = "komik.function",
    operation_id = "komik_function",
    responses(
        (status = 200, description = "Handles GET requests for the /komik/{function} endpoint.", body = FunctionResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn function(Path(function): Path<String>) -> impl IntoResponse {
  Json(FunctionResponse {
    message: "Hello from function!".to_string(),
    data: vec![],
    total: Some(0),
  })
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(function))
}