//! Handler for the key endpoint.
    #![allow(dead_code)]

    use axum::{extract::Path, response::IntoResponse, routing::get, Json, Router};
    use std::sync::Arc;
    use crate::routes::AppState;
    use serde::{Deserialize, Serialize};
    use serde_json;
    use utoipa::ToSchema;

    pub const ENDPOINT_METHOD: &str = "get";
    pub const ENDPOINT_PATH: &str = "/test/v2/posts/detail/key";
    pub const ENDPOINT_DESCRIPTION: &str = "Handles GET requests for the test/v2/posts/detail/key endpoint.";
    pub const ENDPOINT_TAG: &str = "test/v2/posts/detail/key";
    pub const OPERATION_ID: &str = "test/v2/posts/detail/key";
    pub const SUCCESS_RESPONSE_BODY: &str = "Json<DetailResponse>";

    /// Response structure for the Key endpoint.
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
("key" = String, Path, description = "Parameter")
        ),
        path = "/test/v2/posts/detail/key",
        tag = "test/v2/posts/detail/key",
        operation_id = "test/v2/posts/detail/key",
        responses(
            (status = 200, description = "Handles GET requests for the test/v2/posts/detail/key endpoint.", body = DetailResponse),
            (status = 401, description = "Unauthorized", body = String),
            (status = 500, description = "Internal Server Error", body = String)
        )
    )]
    pub async fn key(Path(key): Path<String>) -> impl IntoResponse {
        
        Json(DetailResponse {
            message: format!("Hello from key with parameters: key: {key}"),
            data: serde_json::json!({"key": "key"}),
        })
    }

    /// Handles GET requests for the test/v2/posts/detail/key endpoint.
    pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
        
        router.route(ENDPOINT_PATH, get(key))
    }
    