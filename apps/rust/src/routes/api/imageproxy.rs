//! Handler for the imageproxy endpoint.
 #![allow(dead_code)]

 use axum::{response::IntoResponse, routing::get, Json, Router};
 use std::sync::Arc;
 use crate::routes::AppState;
 use serde::{Deserialize, Serialize};
 use utoipa::ToSchema;

 pub const ENDPOINT_METHOD: &str = "get";
 pub const ENDPOINT_PATH: &str = "/imageproxy";
 pub const ENDPOINT_DESCRIPTION: &str = "Description for the imageproxy endpoint";
 pub const ENDPOINT_TAG: &str = "imageproxy";
 pub const SUCCESS_RESPONSE_BODY: &str = "Json<ImageproxyResponse>";

 #[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
 pub struct ImageproxyResponse {
     pub message: String,
 }
#[utoipa::path(
    get,
    path = "/api/imageproxy",
    tag = "imageproxy",
    operation_id = "imageproxy",
    responses(
        (status = 200, description = "Description for the imageproxy endpoint", body = ImageproxyResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn imageproxy() -> impl IntoResponse {
     Json(ImageproxyResponse {
         message: "Hello from imageproxy!".to_string(),
     })
 }

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(imageproxy))
}