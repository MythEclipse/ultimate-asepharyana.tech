//! Handler for the list endpoint.
    #![allow(dead_code)]

    use axum::{response::IntoResponse, routing::get, Json, Router};
    use std::sync::Arc;
    use crate::routes::AppState;
    use serde::{Deserialize, Serialize};
    use serde_json;
    use utoipa::ToSchema;

    pub const ENDPOINT_METHOD: &str = "get";
    pub const ENDPOINT_PATH: &str = "/test/items/list";
    pub const ENDPOINT_DESCRIPTION: &str = "Retrieves a list of items.";
    pub const ENDPOINT_TAG: &str = "test.items.list";
    pub const OPERATION_ID: &str = "test_items_list";
    pub const SUCCESS_RESPONSE_BODY: &str = "Json<ListResponse>";

    /// Response structure for the List endpoint.
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
    path = "/api/test/items/list",
    tag = "test.items.list",
    operation_id = "test_items_list",
    responses(
        (status = 200, description = "Retrieves a list of items.", body = ListResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn list() -> impl IntoResponse {
        Json(ListResponse {
            message: "Hello from list!".to_string(),
            data: vec![],
            total: None,
        })
    }

    /// Retrieves a list of items.

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(list))
}