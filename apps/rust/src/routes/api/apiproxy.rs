//! Handler for the apiproxy endpoint.
 #![allow(dead_code)]

 use axum::{response::IntoResponse, routing::get, Json, Router};
 use std::sync::Arc;
 use crate::routes::AppState;
 use serde::{Deserialize, Serialize};
 use utoipa::ToSchema;

 pub const ENDPOINT_METHOD: &str = "get";
 pub const ENDPOINT_PATH: &str = "/apiproxy";
 pub const ENDPOINT_DESCRIPTION: &str = "Description for the apiproxy endpoint";
 pub const ENDPOINT_TAG: &str = "apiproxy";
 pub const SUCCESS_RESPONSE_BODY: &str = "Json<ApiproxyResponse>";

 #[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
 pub struct ApiproxyResponse {
     pub message: String,
 }
#[utoipa::path(
    get,
    path = "/api/apiproxy",
    tag = "apiproxy",
    operation_id = "apiproxy",
    responses(
        (status = 200, description = "Description for the apiproxy endpoint", body = ApiproxyResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn apiproxy() -> impl IntoResponse {
     Json(ApiproxyResponse {
         message: "Hello from apiproxy!".to_string(),
     })
 }

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(apiproxy))
}