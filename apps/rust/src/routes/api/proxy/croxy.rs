use axum::{ extract::{ Query, State }, Router, routing::get };
use std::sync::Arc;
use serde::Deserialize; // Keep Deserialize for ProxyParams

use utoipa::ToSchema; // Keep ToSchema for ProxyParams

use rust_lib::error::ErrorResponse;

use crate::routes::AppState;

pub const ENDPOINT_PATH: &str = "/api/proxy/croxy";

#[derive(Deserialize, ToSchema, Debug)]
pub struct ProxyParams {}

#[utoipa::path(
    get,
    path = "/api/proxy/croxy",
    tag = "proxy",
    operation_id = "proxy_croxy",
    responses(
        (status = 200, description = "Handles GET requests for the /api/proxy/croxy endpoint.", body = String),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
#[axum::debug_handler]
pub async fn croxy(
  Query(_params): Query<ProxyParams>,
  State(_state): State<Arc<AppState>>
) -> Result<String, ErrorResponse> {
  Err(ErrorResponse { error: "Proxy functionality has been removed".to_string() })
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(croxy))
}