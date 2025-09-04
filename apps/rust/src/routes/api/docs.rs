//! Handler for the docs endpoint.
 #![allow(dead_code)]

 use axum::{response::IntoResponse, routing::get, Json, Router};
 use std::sync::Arc;
 use crate::routes::AppState;
 use serde::{Deserialize, Serialize};
 use utoipa::ToSchema;

 pub const ENDPOINT_METHOD: &str = "get";
 pub const ENDPOINT_PATH: &str = "/docs";
 pub const ENDPOINT_DESCRIPTION: &str = "Description for the docs endpoint";
 pub const ENDPOINT_TAG: &str = "docs";
 pub const SUCCESS_RESPONSE_BODY: &str = "Json<DocsResponse>";

 #[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
 pub struct DocsResponse {
     pub message: String,
 }
#[utoipa::path(
    get,
    path = "/api/docs",
    tag = "docs",
    operation_id = "docs",
    responses(
        (status = 200, description = "Description for the docs endpoint", body = DocsResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn docs() -> impl IntoResponse {
     Json(DocsResponse {
         message: "Hello from docs!".to_string(),
     })
 }

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(docs))
}