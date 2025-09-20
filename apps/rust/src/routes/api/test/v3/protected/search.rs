//! Handler for the search endpoint.
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
    pub const ENDPOINT_PATH: &str = "/test/v3/protected/search";
    pub const ENDPOINT_DESCRIPTION: &str = "Searches for protected based on query parameters.";
    pub const ENDPOINT_TAG: &str = "test/v3/protected/search";
    pub const OPERATION_ID: &str = "test/v3/protected/search";
    pub const SUCCESS_RESPONSE_BODY: &str = "Json<SearchResponse>";

    /// Response structure for the Search endpoint.
    /// Replace `serde_json::Value` with your actual data types and implement `utoipa::ToSchema` for complex types.
    #[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
    pub struct SearchResponse {
    /// Success message
    pub message: String,
    /// Search results - replace with actual Vec<T> where T implements ToSchema
    pub data: Vec<serde_json::Value>,
    /// Total number of results
    pub total: Option<u64>,
    /// Current page
    pub page: Option<u32>,
    /// Results per page
    pub per_page: Option<u32>,
    }

    #[utoipa::path(
        get,
        params(

        ),
        path = "/test/v3/protected/search",
        tag = "test/v3/protected/search",
        operation_id = "test/v3/protected/search",
        responses(
            (status = 200, description = "Searches for protected based on query parameters.", body = SearchResponse),
            (status = 401, description = "Unauthorized", body = String),
            (status = 500, description = "Internal Server Error", body = String)
        )
    security(
        ("ApiKeyAuth" = [])
    ),
    )]
    pub async fn search(Extension(claims): Extension<Claims>) -> impl IntoResponse {
        
    
    
        Json(SearchResponse {
            message: "Hello from search!".to_string(),
            data: vec![],
            total: None,
            page: None,
            per_page: None,
        })
    }

    /// Searches for protected based on query parameters.
    pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
        let router = router.layer(AuthMiddleware::layer());
        router.route(ENDPOINT_PATH, get(search))
    }
    