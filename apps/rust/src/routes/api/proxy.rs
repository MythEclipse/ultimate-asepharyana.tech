//! Handler for the proxy endpoint.
#![allow(dead_code)]

use axum::{
  extract::Query,
  http::{ header, HeaderMap, StatusCode },
  response::{ IntoResponse, Response },
  routing::get,
  Json,
  Router,
};
use std::sync::Arc;
use crate::routes::AppState;
use serde::{ Deserialize, Serialize };
use serde_json;
use utoipa::ToSchema;
use rust_lib::fetch_with_proxy::fetch_with_proxy;
use tracing::{ error, info };

pub const ENDPOINT_METHOD: &str = "get";
pub const ENDPOINT_PATH: &str = "/api/proxy";
pub const ENDPOINT_DESCRIPTION: &str = "Handles GET requests for the proxy endpoint.";
pub const ENDPOINT_TAG: &str = "proxy";
pub const OPERATION_ID: &str = "proxy";
pub const SUCCESS_RESPONSE_BODY: &str = "String";

#[derive(Deserialize, ToSchema)]
pub struct ProxyQuery {
  pub url: Option<String>,
}

/// Error response structure
#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct ErrorResponse {
  pub error: String,
  pub details: Option<String>,
  pub status: Option<u16>,
}

#[utoipa::path(
    get,
    params(
        ("url" = Option<String>, Query, description = "Parameter for resource identification", example = "sample_value")
    ),
    path = "/api/proxy",
    tag = "proxy",
    operation_id = "proxy",
    responses(
        (status = 200, description = "Handles GET requests for the proxy endpoint.", body = String),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn proxy(Query(params): Query<ProxyQuery>) -> Result<
  Response,
  (StatusCode, Json<ErrorResponse>)
> {
  let url = params.url.ok_or((
    StatusCode::BAD_REQUEST,
    Json(ErrorResponse {
      error: "Missing url parameter".to_string(),
      details: None,
      status: Some(400),
    }),
  ))?;

  info!("Proxying request to: {}", url);

  match fetch_with_proxy(&url).await {
    Ok(result) => {
      let mut headers = HeaderMap::new();
      headers.insert("X-Proxy-Used", "fetchWithProxy".parse().unwrap());

      // Handle JSON responses
      if let Some(content_type) = &result.content_type {
        if content_type.contains("application/json") {
          // Try to parse as JSON
          match serde_json::from_str::<serde_json::Value>(&result.data) {
            Ok(parsed) => {
              headers.insert(header::CONTENT_TYPE, "application/json".parse().unwrap());
              return Ok((StatusCode::OK, headers, Json(parsed)).into_response());
            }
            Err(_) => {
              // Fallback to text response
              headers.insert(header::CONTENT_TYPE, content_type.parse().unwrap());
              return Ok((StatusCode::OK, headers, result.data).into_response());
            }
          }
        } else {
          // Non-JSON response
          if let Ok(content_type_header) = content_type.parse() {
            headers.insert(header::CONTENT_TYPE, content_type_header);
          } else {
            headers.insert(header::CONTENT_TYPE, "text/plain".parse().unwrap());
          }
          return Ok((StatusCode::OK, headers, result.data).into_response());
        }
      } else {
        // No content type specified
        headers.insert(header::CONTENT_TYPE, "text/plain".parse().unwrap());
        return Ok((StatusCode::OK, headers, result.data).into_response());
      }
    }
    Err(e) => {
      error!("Failed to fetch URL {}: {:?}", url, e);
      Err((
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(ErrorResponse {
          error: "Failed to fetch URL".to_string(),
          details: Some(format!("{:?}", e)),
          status: Some(500),
        }),
      ))
    }
  }
}

/// Handles GET requests for the proxy endpoint.

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(proxy))
}