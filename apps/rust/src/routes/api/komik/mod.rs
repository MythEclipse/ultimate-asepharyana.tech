use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json, Router, routing::get,
};
use serde::Deserialize;
use std::sync::Arc;
use crate::routes::ChatState;
pub mod chapter;
pub mod detail;
pub mod external_link;
pub mod manga_dto;
pub mod komik_service;
pub mod search;
pub use self::komik_service as komik;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct KomikQueryParams {
    pub komik_id: Option<String>,
    pub chapter_url: Option<String>,
    pub page: Option<u32>,
    pub query: Option<String>,
}

#[allow(dead_code)]
pub async fn media_handler(
    Path(function): Path<String>,
    Query(params): Query<KomikQueryParams>,
    State(_state): State<Arc<ChatState>>,
) -> Response {
    let allowed_functions = ["manga", "manhwa", "manhua", "search", "detail", "chapter", "external-link"];
    if !allowed_functions.contains(&function.as_str()) {
        return error_response(StatusCode::BAD_REQUEST, "Invalid function parameter");
    }

    match function.as_str() {
        "detail" => {
            let komik_id = params.komik_id.unwrap_or_else(|| "one-piece".to_string());
            match komik::get_detail(&komik_id).await {
                Ok(data) => (StatusCode::OK, Json(data)).into_response(),
                Err(e) => error_response(StatusCode::INTERNAL_SERVER_ERROR, &format!("Failed to fetch manga detail: {}", e)),
            }
        }
        "chapter" => {
            let chapter_url = params.chapter_url.unwrap_or_default();
            if chapter_url.is_empty() {
                return error_response(StatusCode::BAD_REQUEST, "chapter_url parameter is required");
            }
            match komik::get_chapter(&chapter_url).await {
                Ok(data) => (StatusCode::OK, Json(data)).into_response(),
                Err(e) => error_response(StatusCode::INTERNAL_SERVER_ERROR, &format!("Failed to fetch manga chapter: {}", e)),
            }
        }
        "external-link" => {
            match komik::handle_external_link().await {
                Ok(link) => (StatusCode::OK, Json(serde_json::json!({ "link": link }))).into_response(),
                Err(e) => error_response(StatusCode::INTERNAL_SERVER_ERROR, &format!("Failed to fetch external link: {}", e)),
            }
        }
        _ => {
            // Handles "manga", "manhwa", "manhua", "search"
            let page = params.page.unwrap_or(1);
            let query = params.query.as_deref();
            match komik::handle_list_or_search(&function, page, query).await {
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
                Err(e) => error_response(StatusCode::INTERNAL_SERVER_ERROR, &format!("Failed to fetch komik list/search: {}", e)),
            }
        }
    }
}

fn error_response(status: StatusCode, message: &str) -> Response {
    (
        status,
        Json(serde_json::json!({ "status": false, "message": message })),
    )
        .into_response()
}

#[allow(dead_code)]
pub fn create_routes() -> Router<Arc<ChatState>> {
    Router::new()
        .route("/:function", get(media_handler))
}
