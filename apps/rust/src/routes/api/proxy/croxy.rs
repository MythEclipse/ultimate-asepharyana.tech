use axum::{
    extract::{Query, State},
    response::Response,
    Router,
};
use http::StatusCode;
use serde::Deserialize;
use std::sync::Arc;
use utoipa::ToSchema;

use crate::infra::proxy::fetch_with_proxy;
use crate::routes::AppState;
use crate::core::error::AppError;


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
    _: State<Arc<AppState>>,
    Query(params): Query<ProxyParams>,
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
            Err(AppError::Other(format!(
                "Failed to fetch URL via proxy: {}",
                e
            )))
        }
    }
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router
}