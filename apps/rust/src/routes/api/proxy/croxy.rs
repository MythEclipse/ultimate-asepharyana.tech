use axum::{ extract::{ Query, State }, response::Response, routing::get, Router };
use http::StatusCode;
use serde::Deserialize;
use utoipa::ToSchema;
use std::sync::Arc;

use crate::fetch_with_proxy::fetch_with_proxy;
use crate::routes::AppState;
use crate::utils::error::AppError;

#[allow(dead_code)]
pub const ENDPOINT_METHOD: &str = "get";
#[allow(dead_code)]
pub const ENDPOINT_PATH: &str = "/api/proxy/croxy";
#[allow(dead_code)]
pub const ENDPOINT_DESCRIPTION: &str = "Handles GET requests for the proxy endpoint.";
#[allow(dead_code)]
pub const ENDPOINT_TAG: &str = "proxy";
#[allow(dead_code)]
pub const OPERATION_ID: &str = "fetch_with_proxy_only";
#[allow(dead_code)]
pub const SUCCESS_RESPONSE_BODY: &str = "Vec<u8>";

#[derive(Debug, Deserialize, ToSchema)]
pub struct ProxyParams {
  url: String,
}

/// Handles GET requests for the proxy endpoint.
#[utoipa::path(
    get,
    params(
        ("url" = String, Query, description = "Parameter for resource identification", example = "sample_value")
    ),
    path = "/api/proxy/croxy",
    tag = "proxy",
    operation_id = "fetch_with_proxy_only",
    responses(
        (status = 200, description = "Handles GET requests for the proxy endpoint.", body = Vec<u8>),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn fetch_with_proxy_only(
  _state: State<Arc<AppState>>,
  Query(params): Query<ProxyParams>
) -> Result<Response, AppError> {
  let slug = params.url;
  match fetch_with_proxy(&slug).await {
    Ok(fetch_result) => {
      let mut response_builder = Response::builder().status(StatusCode::OK);

      if let Some(content_type) = fetch_result.content_type {
        response_builder = response_builder.header("Content-Type", content_type);
      }

      Ok(response_builder.body(fetch_result.data.into())?)
    }
    Err(e) => {
      eprintln!("Proxy fetch error: {:?}", e);
      Err(AppError::Other(format!("Failed to fetch URL via proxy: {}", e)))
    }
  }
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(fetch_with_proxy_only))
}