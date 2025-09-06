//! Handler for the product_id endpoint.
    #![allow(dead_code)]

    use axum::{extract::Path, response::IntoResponse, routing::get, Json, Router};
    use std::sync::Arc;
    use crate::routes::AppState;
    use serde::{Deserialize, Serialize};
    use serde_json;
    use utoipa::ToSchema;

    pub const ENDPOINT_METHOD: &str = "get";
    pub const ENDPOINT_PATH: &str = "/products/detail/{id}";
    pub const ENDPOINT_DESCRIPTION: &str = "Handles GET requests for the products/detail/product_id endpoint.";
    pub const ENDPOINT_TAG: &str = "products";
    pub const OPERATION_ID: &str = "products_detail_product_id";
    pub const SUCCESS_RESPONSE_BODY: &str = "Json<DetailResponse>";

    /// Response structure for the ProductId endpoint.
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
        ("id" = String, Path, description = "The id identifier")
    ),
    path = "/api/products/detail/{id}",
    tag = "products",
    operation_id = "products_detail_product_id",
    responses(
        (status = 200, description = "Handles GET requests for the products/detail/product_id endpoint.", body = DetailResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn product_id(Path(id): Path<String>) -> impl IntoResponse {
        Json(DetailResponse {
            message: format!("Hello from product_id with parameters: id: {id}"),
            data: serde_json::json!({"id": "id"}),
        })
    }

    /// Handles GET requests for the products/detail/product_id endpoint.

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(product_id))
}