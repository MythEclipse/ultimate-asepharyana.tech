//! Handler for the slug endpoint.
    #![allow(dead_code)]

    use axum::{extract::Path, response::IntoResponse, routing::get, Json, Router};
    use std::sync::Arc;
    use crate::routes::AppState;
    use serde::{Deserialize, Serialize};
    use serde_json;
    use utoipa::ToSchema;

    pub const ENDPOINT_METHOD: &str = "get";
    pub const ENDPOINT_PATH: &str = "/anime2/ongoing_anime/{slug}";
    pub const ENDPOINT_DESCRIPTION: &str = "Handles GET requests for the anime2/ongoing_anime/slug endpoint.";
    pub const ENDPOINT_TAG: &str = "anime2.ongoing_anime.slug";
    pub const OPERATION_ID: &str = "anime2_ongoing_anime_slug";
    pub const SUCCESS_RESPONSE_BODY: &str = "Json<ListResponse>";

    /// Response structure for the Slug endpoint.
    /// Replace `serde_json::Value` with your actual data types and implement `utoipa::ToSchema` for complex types.
    #[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
    pub struct ListResponse {
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
        ("slug" = String, Path, description = "The slug identifier")
    ),
    path = "/api/anime2/ongoing_anime/{slug}",
    tag = "anime2.ongoing_anime.slug",
    operation_id = "anime2_ongoing_anime_slug",
    responses(
        (status = 200, description = "Handles GET requests for the anime2/ongoing_anime/slug endpoint.", body = ListResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn slug(Path(slug): Path<String>) -> impl IntoResponse {
        Json(ListResponse {
            message: "Hello from slug!".to_string(),
            data: vec![],
            total: None,
        })
    }

    /// Handles GET requests for the anime2/ongoing_anime/slug endpoint.

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(slug))
}