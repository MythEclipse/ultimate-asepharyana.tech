//! Handler for the videoproxy endpoint.
#![allow(dead_code)]

use axum::{extract::Path,  response::IntoResponse, routing::get, Json, Router };
use std::sync::Arc;
use crate::routes::AppState;
use serde::{ Deserialize, Serialize };
use utoipa::ToSchema;

pub const ENDPOINT_METHOD: &str = "get";
pub const ENDPOINT_PATH: &str = "/{videoproxy}";
pub const ENDPOINT_DESCRIPTION: &str = "Description for the videoproxy endpoint";
pub const ENDPOINT_TAG: &str = "videoproxy";
pub const SUCCESS_RESPONSE_BODY: &str = "Json<VideoproxyResponse>";

/// Response structure for list endpoints.
/// Replace `serde_json::Value` with your actual data types and implement `utoipa::ToSchema` for complex types.
#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct VideoproxyResponse {
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
        ("videoproxy" = String, Path, description = "The videoproxy identifier")
    ),
    path = "/api/{videoproxy}",
    tag = "videoproxy",
    operation_id = "videoproxy",
    responses(
        (status = 200, description = "Handles GET requests for the /{videoproxy} endpoint.", body = VideoproxyResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn videoproxy(Path(videoproxy): Path<String>) -> impl IntoResponse {
  Json(VideoproxyResponse {
    message: "Hello from videoproxy!".to_string(),
    data: vec![],
    total: Some(0),
  })
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(videoproxy))
}