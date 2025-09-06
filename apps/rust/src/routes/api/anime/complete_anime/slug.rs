//! Handler for the slug endpoint.
    #![allow(dead_code)]

    use axum::{extract::Path, response::IntoResponse, routing::get, Json, Router};
    use std::sync::Arc;
    use crate::routes::AppState;
    use serde::{Deserialize, Serialize};
    use serde_json;
    use utoipa::ToSchema;

    pub const ENDPOINT_METHOD: &str = "get";
    pub const ENDPOINT_PATH: &str = "/anime/complete-anime/{slug}";
    pub const ENDPOINT_DESCRIPTION: &str = "Handles GET requests for the anime/complete-anime/slug endpoint.";
    pub const ENDPOINT_TAG: &str = "anime";
    pub const OPERATION_ID: &str = "anime_complete_anime_slug";
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
    path = "/api/anime/complete-anime/{slug}",
    tag = "anime",
    operation_id = "anime_complete_anime_slug",
    responses(
        (status = 200, description = "Handles GET requests for the anime/complete-anime/slug endpoint.", body = ListResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn slug(Path(slug): Path<String>) -> impl IntoResponse {
        Json(ListResponse {
            message: format!("Hello from slug with parameters: slug: {slug}"),
            data: vec![serde_json::json!({"slug": "slug"})],
            total: Some(1),
        })
    }

    /// Handles GET requests for the anime/complete-anime/slug endpoint.

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(slug))
}