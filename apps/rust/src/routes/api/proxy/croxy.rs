use axum::{ extract::Query, Json, Router, Extension, routing::get };
use std::sync::Arc;
use serde::{ Deserialize, Serialize };

use utoipa::ToSchema;

use rust_lib::scrape_croxy_proxy::scrape_croxy_proxy;
use rust_lib::error::ErrorResponse;

use crate::routes::AppState;

pub const ENDPOINT_PATH: &str = "/proxy/croxy";

#[derive(Deserialize, ToSchema, Debug)]
pub struct ProxyParams {
  /// URL to proxy through CroxyProxy
  pub url: String,
}

#[derive(Serialize, ToSchema, Debug, Clone)]
pub struct ProxyResponse {
  /// Status message
  pub message: String,
  /// HTML content fetched through CroxyProxy
  pub html: String,
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
  Extension(state): Extension<Arc<AppState>>
) -> Result<Json<ProxyResponse>, ErrorResponse> {
  let browser_arc = Arc::clone(&state.browser);
  let html = scrape_croxy_proxy(&browser_arc, &params.url).await?;

  Ok(
    Json(ProxyResponse {
      message: format!("Successfully proxied URL: {}", params.url),
      html,
    })
  )
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(croxy))
}