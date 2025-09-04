//! Handler for the posts endpoint.
#![allow(dead_code)]

use axum::{ response::IntoResponse, routing::get, Json, Router };
use std::sync::Arc;
use crate::routes::AppState;
use serde::{ Deserialize, Serialize };
use utoipa::ToSchema;

pub const ENDPOINT_METHOD: &str = "get";
pub const ENDPOINT_PATH: &str = "/sosmed/posts";
pub const ENDPOINT_DESCRIPTION: &str = "Description for the posts endpoint";
pub const ENDPOINT_TAG: &str = "sosmed.posts";
pub const SUCCESS_RESPONSE_BODY: &str = "Json<PostsResponse>";

/// Response structure for list endpoints.
/// Replace `serde_json::Value` with your actual data types and implement `utoipa::ToSchema` for complex types.
#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct PostsResponse {
  /// Success message
  pub message: String,
  /// List of items - replace with actual Vec<T> where T implements ToSchema
  pub data: Vec<serde_json::Value>,
  /// Total number of items
  pub total: Option<u64>,
}
#[utoipa::path(
    get,
    path = "/api/sosmed/posts",
    tag = "sosmed.posts",
    operation_id = "sosmed_posts",
    responses(
        (status = 200, description = "Handles GET requests for the /sosmed/posts endpoint.", body = PostsResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn posts() -> impl IntoResponse {
  Json(PostsResponse {
    message: "Hello from posts!".to_string(),
    data: vec![],
    total: Some(0),
  })
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(posts))
}