//! Handler for the list endpoint.
    #![allow(dead_code)]

    use crate::middleware::auth::AuthMiddleware;
use crate::utils::auth::Claims;
use axum::Extension;
use axum::{response::IntoResponse, routing::get, Json, Router};
    use std::sync::Arc;
    use crate::routes::AppState;
    use serde::{Deserialize, Serialize};
    use serde_json;
    use utoipa::ToSchema;

    pub const ENDPOINT_METHOD: &str = "get";
    pub const ENDPOINT_PATH: &str = "/test/v3/protected/list";
    pub const ENDPOINT_DESCRIPTION: &str = "Retrieves a list of protected.";
    pub const ENDPOINT_TAG: &str = "test/v3/protected/list";
    pub const OPERATION_ID: &str = "test/v3/protected/list";
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
        params(

        ),
        path = "/test/v3/protected/list",
        tag = "test/v3/protected/list",
        operation_id = "test/v3/protected/list",
        responses(
            (status = 200, description = "Retrieves a list of protected.", body = ListResponse),
            (status = 401, description = "Unauthorized", body = String),
            (status = 500, description = "Internal Server Error", body = String)
        )
    security(
        ("ApiKeyAuth" = [])
    ),
    )]
    pub async fn list(Extension(claims): Extension<Claims>) -> impl IntoResponse {
        
    
    
        Json(ListResponse {
            message: "Hello from list!".to_string(),
            data: vec![],
            total: None,
        })
    }

    /// Retrieves a list of protected.
    pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
        let router = router.layer(AuthMiddleware::layer());
        router.route(ENDPOINT_PATH, get(list))
    }
    