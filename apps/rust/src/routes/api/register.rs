//! Handler for the register endpoint.
#![allow(dead_code)]

use axum::{ response::IntoResponse, routing::get, Json, Router };
use std::sync::Arc;
use crate::routes::AppState;
use serde::{ Deserialize, Serialize };
use utoipa::ToSchema;

pub const ENDPOINT_METHOD: &str = "get";
pub const ENDPOINT_PATH: &str = "/register";
pub const ENDPOINT_DESCRIPTION: &str = "Description for the register endpoint";
pub const ENDPOINT_TAG: &str = "register";
pub const SUCCESS_RESPONSE_BODY: &str = "Json<RegisterResponse>";

/// Response structure for list endpoints.
/// Replace `serde_json::Value` with your actual data types and implement `utoipa::ToSchema` for complex types.
#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct RegisterResponse {
  /// Success message
  pub message: String,
  /// List of items - replace with actual Vec<T> where T implements ToSchema
  pub data: Vec<serde_json::Value>,
  /// Total number of items
  pub total: Option<u64>,
}
#[utoipa::path(
    get,
    path = "/api/register",
    tag = "register",
    operation_id = "register",
    responses(
        (status = 200, description = "Handles GET requests for the /register endpoint.", body = RegisterResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn register() -> impl IntoResponse {
  Json(RegisterResponse {
    message: "Hello from register!".to_string(),
    data: vec![],
    total: Some(0),
  })
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(register))
}