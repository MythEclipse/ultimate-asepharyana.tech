//! Demo handler to test auto-routing system.

use axum::{extract::Path, response::IntoResponse, routing::get, Json, Router};
use std::sync::Arc;
use crate::routes::AppState;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub const ENDPOINT_METHOD: &str = "get";
pub const ENDPOINT_PATH: &str = "/demo/[testid]";

/// Demo response
#[derive(Serialize, Deserialize, ToSchema)]
pub struct DemoResponse {
    pub message: String,
    pub test_id: String,
}

#[utoipa::path(
    get,
    params(
        ("testid" = String, Path, description = "Parameter for resource identification", example = "sample_value")
    ),
    path = "/api//demo/[testid]",
    tag = "demo",
    operation_id = "demo_testid",
    responses(
        (status = 200, description = "Handles GET requests for the /api//demo/{testid} endpoint."),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn test_auto_route(Path(testid): Path<String>) -> impl IntoResponse {
    Json(DemoResponse {
        message: "Auto-routing works!".to_string(),
        test_id: testid,
    })
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(test_auto_route))
}