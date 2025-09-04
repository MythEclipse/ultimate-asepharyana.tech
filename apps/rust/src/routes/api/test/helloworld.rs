//! Handler for the helloworld endpoint.
 #![allow(dead_code)]

 use axum::{response::IntoResponse, routing::get, Json, Router};
 use std::sync::Arc;
 use crate::routes::AppState;
 use serde::{Deserialize, Serialize};
 use utoipa::ToSchema;

 pub const ENDPOINT_METHOD: &str = "get";
 pub const ENDPOINT_PATH: &str = "test/helloworld";
 pub const ENDPOINT_DESCRIPTION: &str = "Description for the helloworld endpoint";
 pub const ENDPOINT_TAG: &str = "test";
 pub const SUCCESS_RESPONSE_BODY: &str = "Json<HelloworldResponse>";

 #[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
 pub struct HelloworldResponse {
     pub message: String,
 }
#[utoipa::path(
    get,
    path = "/apitest/helloworld",
    tag = "test",
    responses(
        (status = 200, description = "Description for the helloworld endpoint", body = HelloworldResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn helloworld() -> impl IntoResponse {
     Json(HelloworldResponse {
         message: "Hello from helloworld!".to_string(),
     })
 }

 













pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(helloworld))
}