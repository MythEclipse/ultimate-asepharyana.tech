//! Handler for the slug endpoint.
    #![allow(dead_code)]

    use axum::{extract::Path, response::IntoResponse, routing::get, Json, Router};
    use std::sync::Arc;
    use crate::routes::AppState;
    use serde::{Deserialize, Serialize};
    use serde_json;
    use utoipa::ToSchema;

    pub const ENDPOINT_METHOD: &str = "get";
    pub const ENDPOINT_PATH: &str = "/anime2/detail/{slug}";
    pub const ENDPOINT_DESCRIPTION: &str = "Handles GET requests for the anime2/detail/slug endpoint.";
    pub const ENDPOINT_TAG: &str = "anime2.detail.slug";
    pub const OPERATION_ID: &str = "anime2_detail_slug";
    pub const SUCCESS_RESPONSE_BODY: &str = "Json<DetailResponse>";

    /// Response structure for the Slug endpoint.
    /// Replace `serde_json::Value` with your actual data types and implement `utoipa::ToSchema` for complex types.
    #[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
    pub struct DetailResponse {
    /// Success message
    pub message: String,
    /// Detailed data - replace with actual T where T implements ToSchema
    pub data: serde_json::Value,
    }
#[utoipa::path(
    get,
    params(
        ("slug" = String, Path, description = "The slug identifier")
    ),
    path = "/api/anime2/detail/{slug}",
    tag = "anime2.detail.slug",
    operation_id = "anime2_detail_slug",
    responses(
        (status = 200, description = "Handles GET requests for the anime2/detail/slug endpoint.", body = DetailResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn slug(Path(slug): Path<String>) -> impl IntoResponse {
        Json(DetailResponse {
            message: "Hello from slug!".to_string(),
            data: serde_json::json!(null),
        })
    }

    /// Handles GET requests for the anime2/detail/slug endpoint.

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(slug))
}