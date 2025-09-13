use axum::{ extract::{ Query, State }, Router, routing::get };
use std::sync::Arc;
use serde::Deserialize; // Keep Deserialize for ProxyParams

use utoipa::ToSchema; // Keep ToSchema for ProxyParams

use rust_lib::scrape_croxy_proxy::scrape_croxy_proxy;
use rust_lib::error::ErrorResponse;

use crate::routes::AppState;

pub const ENDPOINT_PATH: &str = "/api/proxy/croxy";

#[derive(Deserialize, ToSchema, Debug)]
pub struct ProxyParams {
  /// URL to proxy through CroxyProxy
  pub url: String,
}

#[utoipa::path(
    get,
    params(
        ("url" = String, Query, description = "Parameter for resource identification", example = "sample_value")
    ),
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
  Query(params): Query<ProxyParams>,
  State(state): State<Arc<AppState>>
) -> Result<String, ErrorResponse> {
  let browser_arc = Arc::clone(&state.browser);
  let browser_pool = rust_lib::headless_chrome::BrowserPool::from_arc(browser_arc);
  let html = scrape_croxy_proxy(&browser_pool, &params.url).await?;

  Ok(html)
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(croxy))
}