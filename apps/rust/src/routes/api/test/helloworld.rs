//! Handler for the helloworld endpoint.
use axum::{response::IntoResponse, routing::get, Json, Router};
use std::sync::Arc;
use crate::routes::ChatState;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// --- API METADATA ---
pub const ENDPOINT_METHOD: &str = "get";
pub const ENDPOINT_PATH: &str = "/test/helloworld";
pub const ENDPOINT_DESCRIPTION: &str = "Description for the helloworld endpoint";
pub const ENDPOINT_TAG: &str = "test";
pub const SUCCESS_RESPONSE_BODY: &str = "Json<HelloworldResponse>";

// --- RESPONSE STRUCTURE ---
#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct HelloworldResponse {
    pub message: String,
}

// --- HANDLER FUNCTION ---
// The Utoipa macro is automatically generated/updated by build.rs
pub async fn helloworld() -> impl IntoResponse {
    Json(HelloworldResponse {
        message: "Hello from helloworld!".to_string(),
    })
}

// --- ROUTE REGISTRATION ---
// This function is automatically generated/updated by build.rs
pub fn register_routes(router: Router<Arc<ChatState>>) -> Router<Arc<ChatState>> {
    router.route(ENDPOINT_PATH, get(helloworld))
}
