//! Handler for the slug endpoint.
 #![allow(dead_code)]

 use axum::{response::IntoResponse, routing::get, Json, Router};
 use std::sync::Arc;
 use crate::routes::AppState;
 use serde::{Deserialize, Serialize};
 use utoipa::ToSchema;

 pub const ENDPOINT_METHOD: &str = "get";
 pub const ENDPOINT_PATH: &str = "/anime/full/{slug}";
 pub const ENDPOINT_DESCRIPTION: &str = "Description for the slug endpoint";
 pub const ENDPOINT_TAG: &str = "anime.full.slug";
 pub const SUCCESS_RESPONSE_BODY: &str = "Json<SlugResponse>";

 #[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
 pub struct SlugResponse {
     pub message: String,
 }
#[utoipa::path(
    get,
    path = "/api/anime/full/{slug}",
    tag = "anime.full.slug",
    operation_id = "anime_full_slug",
    responses(
        (status = 200, description = "Description for the slug endpoint", body = SlugResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn slug() -> impl IntoResponse {
     Json(SlugResponse {
         message: "Hello from slug!".to_string(),
     })
 }

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(slug))
}