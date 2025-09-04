//! Handler for the manga endpoint.
#![allow(dead_code)]

use axum::{extract::{Path, Query}, response::IntoResponse, routing::get, Json, Router};
use std::sync::Arc;
use crate::routes::AppState;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub const ENDPOINT_METHOD: &str = "get";
pub const ENDPOINT_PATH: &str = "/komik/manga/{slug}";
pub const ENDPOINT_DESCRIPTION: &str = "Mengambil detail manga berdasarkan slug dan parameter kueri lainnya. Mendukung pagination dan pengurutan.";
pub const ENDPOINT_TAG: &str = "komik.manga";
pub const SUCCESS_RESPONSE_BODY: &str = "Json<MangaResponse>";

/// Query parameters for manga endpoint
#[derive(Deserialize, ToSchema)]
pub struct MangaQuery {
    /// Page number for pagination (required)
    pub page: i32,
    /// Order parameter for sorting (optional)
    pub order: Option<String>,
}

/// Individual manga item structure
#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct MangaItem {
    /// Title of the manga
    pub title: String,
    /// Image URL of the manga
    pub image: String,
    /// Latest chapter information
    pub chapter: String,
    /// Rating/score of the manga
    pub score: String,
    /// Release date
    pub date: String,
    /// Type of manga
    pub r#type: String,
    /// Unique identifier for the manga
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

/// Response structure for manga endpoint
#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct MangaResponse {
    /// List of manga items
    pub data: Vec<MangaItem>,
    /// Pagination information
    pub pagination: PaginationInfo,
}

#[utoipa::path(
    get,
    params(
        ("slug" = String, Path, description = "The slug identifier")
    ),
    path = "/api/komik/manga/{slug}",
    tag = "komik.manga",
    operation_id = "komik_manga",
    responses(
        (status = 200, description = "Mengambil detail manga berdasarkan slug dan parameter kueri lainnya. Mendukung pagination dan pengurutan.", body = MangaResponse),
        (status = 400, description = "Bad request - invalid parameters", body = String),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn manga(Path(slug): Path<String>, Query(params): Query<MangaQuery>) -> impl IntoResponse {
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
        MangaItem {
            title: "Sample Manga 1".to_string(),
            image: "https://example.com/image1.jpg".to_string(),
            chapter: "Chapter 1".to_string(),
            score: "8.5".to_string(),
            date: "2024-01-01".to_string(),
            r#type: "Manga".to_string(),
            komik_id: "sample-1".to_string(),
        },
        MangaItem {
            title: "Sample Manga 2".to_string(),
            image: "https://example.com/image2.jpg".to_string(),
            chapter: "Chapter 5".to_string(),
            score: "9.2".to_string(),
            date: "2024-01-15".to_string(),
            r#type: "Manga".to_string(),
            komik_id: "sample-2".to_string(),
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

    let response = MangaResponse {
        data: mock_data,
        pagination,
    };

    Json(response).into_response()
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(manga))
}
