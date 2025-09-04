//! Handler for the search endpoint.
 #![allow(dead_code)]

 use axum::{response::IntoResponse, routing::get, Json, Router};
 use std::sync::Arc;
 use crate::routes::AppState;
 use serde::{Deserialize, Serialize};
 use utoipa::ToSchema;

 pub const ENDPOINT_METHOD: &str = "get";
 pub const ENDPOINT_PATH: &str = "/anime/search";
 pub const ENDPOINT_DESCRIPTION: &str = "Description for the search endpoint";
 pub const ENDPOINT_TAG: &str = "anime.search";
 pub const SUCCESS_RESPONSE_BODY: &str = "Json<SearchResponse>";

 /// Response structure for search endpoints.
/// Replace `serde_json::Value` with your actual data types and implement `utoipa::ToSchema` for complex types.
#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct SearchResponse {
    /// Success message
    pub message: String,
    /// Search results - replace with actual Vec<T> where T implements ToSchema
    pub data: Vec<serde_json::Value>,
    /// Total number of results
    pub total: Option<u64>,
    /// Current page
    pub page: Option<u32>,
    /// Results per page
    pub per_page: Option<u32>,
}
#[utoipa::path(
    get,
    path = "/api/anime/search",
    tag = "anime.search",
    operation_id = "anime_search",
    responses(
        (status = 200, description = "Searches for anime based on query parameters.", body = SearchResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn search() -> impl IntoResponse {
      Json(SearchResponse {
          message: "Hello from search!".to_string(),
          data: vec![],
          total: None,
          page: None,
          per_page: None,
      })
  }

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(search))
}