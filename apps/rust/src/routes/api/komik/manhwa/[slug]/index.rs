//! Handler for the manhwa endpoint.
#![allow(dead_code)]

use axum::{extract::{Path, Query}, response::IntoResponse, routing::get, Json, Router};
use std::sync::Arc;
use crate::routes::AppState;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub const ENDPOINT_METHOD: &str = "get";
pub const ENDPOINT_PATH: &str = "/komik/manhwa/{slug}";
pub const ENDPOINT_DESCRIPTION: &str = "Mengambil detail manhwa berdasarkan slug dan parameter kueri lainnya. Mendukung pagination dan pengurutan.";
pub const ENDPOINT_TAG: &str = "komik.manhwa";
pub const SUCCESS_RESPONSE_BODY: &str = "Json<ManhwaResponse>";

/// Query parameters for manhwa endpoint
#[derive(Deserialize, ToSchema)]
pub struct ManhwaQuery {
    /// Page number for pagination (required)
    pub page: i32,
    /// Order parameter for sorting (optional)
    pub order: Option<String>,
}

/// Individual manhwa item structure
#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct ManhwaItem {
    /// Title of the manhwa
    pub title: String,
    /// Image URL of the manhwa
    pub image: String,
    /// Latest chapter information
    pub chapter: String,
    /// Rating/score of the manhwa
    pub score: String,
    /// Release date
    pub date: String,
    /// Type of manhwa
    pub r#type: String,
    /// Unique identifier for the manhwa
    pub komik_id: String,
}

/// Pagination information structure
#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct PaginationInfo {
    /// Current page number
    pub current_page: i32,
    /// Last visible page number
    pub last_visible_page: i32,
    /// Whether there's a next page
    pub has_next_page: bool,
    /// Next page number (nullable)
    pub next_page: Option<i32>,
    /// Whether there's a previous page
    pub has_previous_page: bool,
    /// Previous page number (nullable)
    pub previous_page: Option<i32>,
}

/// Response structure for manhwa endpoint
#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct ManhwaResponse {
    /// List of manhwa items
    pub data: Vec<ManhwaItem>,
    /// Pagination information
    pub pagination: PaginationInfo,
}

#[utoipa::path(
    get,
    params(
        ("slug" = String, Path, description = "The slug identifier")
    ),
    path = "/api/komik/manhwa/{slug}",
    tag = "komik.manhwa",
    operation_id = "komik_manhwa",
    responses(
        (status = 200, description = "Mengambil detail manhwa berdasarkan slug dan parameter kueri lainnya. Mendukung pagination dan pengurutan.", body = ManhwaResponse),
        (status = 400, description = "Bad request - invalid parameters", body = String),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn manhwa(Path(slug): Path<String>, Query(params): Query<ManhwaQuery>) -> impl IntoResponse {
    // Validate page parameter
    if params.page < 1 {
        return Json(serde_json::json!({
            "message": "Page must be greater than 0"
        })).into_response();
    }

    // Validate order parameter if provided
    if let Some(ref order) = params.order {
        let valid_orders = ["update", "popular", "titleasc", "titledesc"];
        if !valid_orders.contains(&order.as_str()) {
            return Json(serde_json::json!({
                "message": "Invalid order parameter. Must be one of: update, popular, titleasc, titledesc"
            })).into_response();
        }
    }

    // Mock data - replace with actual implementation
    let mock_data = vec![
        ManhwaItem {
            title: "Sample Manhwa 1".to_string(),
            image: "https://example.com/manhwa1.jpg".to_string(),
            chapter: "Chapter 1".to_string(),
            score: "8.5".to_string(),
            date: "2024-01-01".to_string(),
            r#type: "Manhwa".to_string(),
            komik_id: "manhwa-1".to_string(),
        },
        ManhwaItem {
            title: "Sample Manhwa 2".to_string(),
            image: "https://example.com/manhwa2.jpg".to_string(),
            chapter: "Chapter 5".to_string(),
            score: "9.2".to_string(),
            date: "2024-01-15".to_string(),
            r#type: "Manhwa".to_string(),
            komik_id: "manhwa-2".to_string(),
        },
    ];

    // Calculate pagination
    let total_pages = 10; // Mock total pages
    let has_next = params.page < total_pages;
    let has_prev = params.page > 1;

    let pagination = PaginationInfo {
        current_page: params.page,
        last_visible_page: total_pages,
        has_next_page: has_next,
        next_page: if has_next { Some(params.page + 1) } else { None },
        has_previous_page: has_prev,
        previous_page: if has_prev { Some(params.page - 1) } else { None },
    };

    let response = ManhwaResponse {
        data: mock_data,
        pagination,
    };

    Json(response).into_response()
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(manhwa))
}
