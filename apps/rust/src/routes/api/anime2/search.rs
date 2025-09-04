//! Handler for the search endpoint.
 #![allow(dead_code)]

 use axum::{response::IntoResponse, routing::get, Json, Router};
 use std::sync::Arc;
 use crate::routes::AppState;
 use serde::{Deserialize, Serialize};
 use utoipa::ToSchema;

 pub const ENDPOINT_METHOD: &str = "get";
 pub const ENDPOINT_PATH: &str = "/anime2/search";
 pub const ENDPOINT_DESCRIPTION: &str = "Description for the search endpoint";
 pub const ENDPOINT_TAG: &str = "anime2.search";
 pub const SUCCESS_RESPONSE_BODY: &str = "Json<SearchResponse>";

 #[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
 pub struct SearchResponse {
     pub message: String,
 }
#[utoipa::path(
    get,
    path = "/api/anime2/search",
    tag = "anime2.search",
    operation_id = "anime2_search",
    responses(
        (status = 200, description = "Description for the search endpoint", body = SearchResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn search() -> impl IntoResponse {
     Json(SearchResponse {
         message: "Hello from search!".to_string(),
     })
 }

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(search))
}