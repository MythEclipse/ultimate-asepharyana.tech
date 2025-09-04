//! Handler for the proxy endpoint.
 #![allow(dead_code)]

 use axum::{response::IntoResponse, routing::get, Json, Router};
 use std::sync::Arc;
 use crate::routes::AppState;
 use serde::{Deserialize, Serialize};
 use utoipa::ToSchema;

 pub const ENDPOINT_METHOD: &str = "get";
 pub const ENDPOINT_PATH: &str = "/proxy";
 pub const ENDPOINT_DESCRIPTION: &str = "Description for the proxy endpoint";
 pub const ENDPOINT_TAG: &str = "api";
 pub const SUCCESS_RESPONSE_BODY: &str = "Json<ProxyResponse>";

 #[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
 pub struct ProxyResponse {
     pub message: String,
 }
#[utoipa::path(
    get,
    path = "/api/proxy",
    tag = "api",
    operation_id = "proxy",
    responses(
        (status = 200, description = "Description for the proxy endpoint", body = ProxyResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn proxy() -> impl IntoResponse {
     Json(ProxyResponse {
         message: "Hello from proxy!".to_string(),
     })
 }

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(proxy))
}