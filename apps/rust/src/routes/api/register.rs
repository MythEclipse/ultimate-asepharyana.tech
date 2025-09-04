//! Handler for the register endpoint.
 #![allow(dead_code)]

 use axum::{response::IntoResponse, routing::get, Json, Router};
 use std::sync::Arc;
 use crate::routes::AppState;
 use serde::{Deserialize, Serialize};
 use utoipa::ToSchema;

 pub const ENDPOINT_METHOD: &str = "get";
 pub const ENDPOINT_PATH: &str = "/register";
 pub const ENDPOINT_DESCRIPTION: &str = "Description for the register endpoint";
 pub const ENDPOINT_TAG: &str = "api";
 pub const SUCCESS_RESPONSE_BODY: &str = "Json<RegisterResponse>";

 #[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
 pub struct RegisterResponse {
     pub message: String,
 }
#[utoipa::path(
    get,
    path = "/api/register",
    tag = "api",
    operation_id = "register",
    responses(
        (status = 200, description = "Description for the register endpoint", body = RegisterResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn register() -> impl IntoResponse {
     Json(RegisterResponse {
         message: "Hello from register!".to_string(),
     })
 }

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(register))
}