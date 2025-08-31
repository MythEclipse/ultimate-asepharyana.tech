use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json, Router, routing::get,
};
use serde::Deserialize;
use std::sync::Arc;
use crate::routes::ChatState;
pub mod detail;
pub mod chapter;
pub mod search;
pub mod external_link;
pub mod manga_dto;
pub mod komik_service;
pub use self::komik_service as komik;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct KomikQueryParams {
    pub page: Option<u32>,
    pub query: Option<String>,
    pub komik_id: Option<String>,
    pub chapter_url: Option<String>,
}

#[allow(dead_code)]
pub async fn media_handler(
    Path(media_type): Path<String>,
    Query(params): Query<KomikQueryParams>,
    State(_state): State<Arc<ChatState>>,
) -> Response {
    let allowed_types = ["manga", "manhwa", "manhua", "search", "detail", "chapter", "external-link"];
    if !allowed_types.contains(&media_type.as_str()) {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "status": false, "message": "Invalid type parameter" })),
        )
            .into_response();
    }

    match media_type.as_str() {
        "detail" => {
            let komik_id = params.komik_id.unwrap_or_else(|| "one-piece".to_string());
            match komik::get_detail(&komik_id).await {
                Ok(data) => (StatusCode::OK, Json(data)).into_response(),
                Err(e) => {
                    eprintln!("Error fetching komik detail: {:?}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(serde_json::json!({ "status": false, "message": "Failed to fetch manga detail" })),
                    )
                        .into_response()
                }
            }
        }
        "chapter" => {
            let chapter_url = params.chapter_url.unwrap_or_default();
            if chapter_url.is_empty() {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(serde_json::json!({ "status": false, "message": "chapter_url parameter is required" })),
                )
                    .into_response();
            }
            match komik::get_chapter(&chapter_url).await {
                Ok(data) => (StatusCode::OK, Json(data)).into_response(),
                Err(e) => {
                    eprintln!("Error fetching komik chapter: {:?}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(serde_json::json!({ "status": false, "message": "Failed to fetch manga chapter" })),
                    )
                        .into_response()
                }
            }
        }
        "external-link" => {
            match komik::handle_external_link().await {
                Ok(link) => (StatusCode::OK, Json(serde_json::json!({ "link": link }))).into_response(),
                Err(e) => {
                    eprintln!("Error fetching external link: {:?}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(serde_json::json!({ "status": false, "message": "Failed to fetch external link" })),
                    )
                        .into_response()
                }
            }
        }
        _ => {
            // Handles "manga", "manhwa", "manhua", "search"
            let page = params.page.unwrap_or(1);
            let query = params.query.as_deref();
            match komik::handle_list_or_search(&media_type, page, query).await {
                Ok(data) => {
                    if data["data"].is_null() || data["data"].as_array().map_or(true, |arr| arr.is_empty()) {
                        (
                            StatusCode::NOT_FOUND,
                            Json(serde_json::json!({ "status": false, "message": "No data found", "data": [], "pagination": data["pagination"] })),
                        )
                            .into_response()
                    } else {
                        (
                            StatusCode::OK,
                            Json(serde_json::json!({ "data": data["data"], "pagination": data["pagination"] })),
                        )
                            .into_response()
                    }
                }
                Err(e) => {
                    eprintln!("Error fetching komik list/search: {:?}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(serde_json::json!({ "status": false, "message": "Failed to fetch komik list/search" })),
                    )
                        .into_response()
                }
            }
        }
    }
}

#[allow(dead_code)]
pub fn create_routes() -> Router<Arc<ChatState>> {
    Router::new()
        .route("/:type", get(media_handler))
}
