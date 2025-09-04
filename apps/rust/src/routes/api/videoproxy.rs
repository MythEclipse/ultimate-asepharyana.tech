//! Handler for the videoproxy endpoint.
 #![allow(dead_code)]

 use axum::{response::IntoResponse, routing::get, Json, Router};
 use std::sync::Arc;
 use crate::routes::AppState;
 use serde::{Deserialize, Serialize};
 use utoipa::ToSchema;

 pub const ENDPOINT_METHOD: &str = "get";
 pub const ENDPOINT_PATH: &str = "/videoproxy";
 pub const ENDPOINT_DESCRIPTION: &str = "Description for the videoproxy endpoint";
 pub const ENDPOINT_TAG: &str = "videoproxy";
 pub const SUCCESS_RESPONSE_BODY: &str = "Json<VideoproxyResponse>";

 #[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
 pub struct VideoproxyResponse {
     pub message: String,
 }
#[utoipa::path(
    get,
    path = "/api/videoproxy",
    tag = "videoproxy",
    operation_id = "videoproxy",
    responses(
        (status = 200, description = "Description for the videoproxy endpoint", body = VideoproxyResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn videoproxy() -> impl IntoResponse {
     Json(VideoproxyResponse {
         message: "Hello from videoproxy!".to_string(),
     })
 }

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(videoproxy))
}