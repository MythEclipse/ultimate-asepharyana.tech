//! Handler for the function endpoint.
 #![allow(dead_code)]

 use axum::{response::IntoResponse, routing::get, Json, Router};
 use std::sync::Arc;
 use crate::routes::AppState;
 use serde::{Deserialize, Serialize};
 use utoipa::ToSchema;

 pub const ENDPOINT_METHOD: &str = "get";
 pub const ENDPOINT_PATH: &str = "/komik/function";
 pub const ENDPOINT_DESCRIPTION: &str = "Description for the function endpoint";
 pub const ENDPOINT_TAG: &str = "komik.function";
 pub const SUCCESS_RESPONSE_BODY: &str = "Json<FunctionResponse>";

 #[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
 pub struct FunctionResponse {
     pub message: String,
 }
#[utoipa::path(
    get,
    path = "/api/komik/function",
    tag = "komik.function",
    operation_id = "komik_function",
    responses(
        (status = 200, description = "Description for the function endpoint", body = FunctionResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn function() -> impl IntoResponse {
     Json(FunctionResponse {
         message: "Hello from function!".to_string(),
     })
 }

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(function))
}