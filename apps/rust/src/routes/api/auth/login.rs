//! Handler for the login endpoint.
    #![allow(dead_code)]

    use axum::{response::IntoResponse, routing::get, Json, Router};
    use std::sync::Arc;
    use crate::routes::AppState;
    use serde::{Deserialize, Serialize};
    use serde_json;
    use utoipa::ToSchema;

    pub const ENDPOINT_METHOD: &str = "get";
    pub const ENDPOINT_PATH: &str = "/auth/login";
    pub const ENDPOINT_DESCRIPTION: &str = "Handles GET requests for the auth/login endpoint.";
    pub const ENDPOINT_TAG: &str = "auth/login";
    pub const OPERATION_ID: &str = "auth/login";
    pub const SUCCESS_RESPONSE_BODY: &str = "Json<ListResponse>";

    /// Response structure for the Login endpoint.
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
    path = "/api//auth/login",
    tag = "auth/login",
    operation_id = "auth/login",
    responses(
        (status = 200, description = "Handles GET requests for the auth/login endpoint.", body = ListResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
    pub async fn login() -> impl IntoResponse {

        Json(ListResponse {
            message: "Hello from login!".to_string(),
            data: vec![],
            total: None,
        })
    }

    /// Handles GET requests for the auth/login endpoint.

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(login))
}
