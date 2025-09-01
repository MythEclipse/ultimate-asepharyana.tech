//! API module re-exports for all implemented endpoints
// This module now exposes OpenAPI documentation for all API groups.

use utoipa::OpenApi;

pub mod komik;
pub mod anime;
pub mod anime2;
pub mod uploader;
pub mod proxy;
pub mod compress;
pub mod drivepng;

/// Aggregates OpenAPI docs for all API groups.
#[derive(OpenApi)]
#[openapi(
    tags(
        (name = "api", description = "Root API module")
    )
)]
pub struct ApiDoc;
