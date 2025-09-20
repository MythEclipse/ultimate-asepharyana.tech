//! Handler for the id endpoint.
    #![allow(dead_code)]

    use crate::middleware::auth::AuthMiddleware;
use crate::utils::auth::Claims;
use axum::Extension;
use axum::{extract::Path, response::IntoResponse, routing::get, Json, Router};
    use std::sync::Arc;
    use crate::routes::AppState;
    use serde::{Deserialize, Serialize};
    use serde_json;
    use utoipa::ToSchema;

    pub const ENDPOINT_METHOD: &str = "get";
    pub const ENDPOINT_PATH: &str = "/test/v3/protected/products/detail/id";
    pub const ENDPOINT_DESCRIPTION: &str = "Handles GET requests for the test/v3/protected/products/detail/id endpoint.";
    pub const ENDPOINT_TAG: &str = "test/v3/protected/products/detail/id";
    pub const OPERATION_ID: &str = "test/v3/protected/products/detail/id";
    pub const SUCCESS_RESPONSE_BODY: &str = "Json<DetailResponse>";

    /// Response structure for the Id endpoint.
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
("id" = String, Path, description = "Parameter")
        ),
        path = "/test/v3/protected/products/detail/id",
        tag = "test/v3/protected/products/detail/id",
        operation_id = "test/v3/protected/products/detail/id",
        responses(
            (status = 200, description = "Handles GET requests for the test/v3/protected/products/detail/id endpoint.", body = DetailResponse),
            (status = 401, description = "Unauthorized", body = String),
            (status = 500, description = "Internal Server Error", body = String)
        )
    security(
        ("ApiKeyAuth" = [])
    ),
    )]
    pub async fn id(Extension(claims): Extension<Claims>, Path(id): Path<String>) -> impl IntoResponse {
        
    
    
        Json(DetailResponse {
            message: format!("Hello from id with parameters: id: {id}"),
            data: serde_json::json!({"id": "id"}),
        })
    }

    /// Handles GET requests for the test/v3/protected/products/detail/id endpoint.
    pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
        let router = router.layer(AuthMiddleware::layer());
        router.route(ENDPOINT_PATH, get(id))
    }
    