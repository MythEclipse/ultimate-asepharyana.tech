//! Komik API module: routes and handlers for manga, manhwa, manhua, search, detail, and chapter endpoints.

use axum::{
    extract::{Query, State},
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

/// Query parameters for Komik endpoints.
#[derive(Debug, Deserialize)]
pub struct KomikQueryParams {
    pub komik_id: Option<String>,
    pub chapter_url: Option<String>,
    pub page: Option<u32>,
    pub query: Option<String>,
}

// --- OpenAPI-compliant handler for GET /api/komik/manga ---
#[derive(Debug, Deserialize)]
struct MangaListQuery {
    page: Option<u32>,
    order: Option<String>,
}

async fn manga_list_handler(
    Query(params): Query<MangaListQuery>,
    State(_state): State<Arc<ChatState>>,
) -> Response {
    let page = match params.page {
        Some(p) if p > 0 => p,
        _ => {
            return error_response(
                StatusCode::BAD_REQUEST,
                "Missing or invalid 'page' query parameter",
            );
        }
    };

    let allowed_orders = ["update", "popular", "titleasc", "titledesc"];
    if let Some(ref order) = params.order {
        if !allowed_orders.contains(&order.as_str()) {
            return error_response(
                StatusCode::BAD_REQUEST,
                "Invalid 'order' query parameter",
            );
        }
    }

    let result = komik::handle_list_or_search("manga", page, None).await;
    match result {
        Ok(data) => {
            let mapped_data: Vec<serde_json::Value> = data["data"]
                .as_array()
                .unwrap_or(&vec![])
                .iter()
                .map(|item| {
                    serde_json::json!({
                        "title": item.get("title").unwrap_or(&serde_json::Value::Null),
                        "image": item.get("poster").unwrap_or(&serde_json::Value::Null),
                        "chapter": item.get("chapter").unwrap_or(&serde_json::Value::Null),
                        "score": item.get("score").unwrap_or(&serde_json::Value::Null),
                        "date": item.get("date").unwrap_or(&serde_json::Value::Null),
                        "type": item.get("manga_type").unwrap_or(&serde_json::Value::Null),
                        "komik_id": item.get("slug").unwrap_or(&serde_json::Value::Null),
                    })
                })
                .collect();

            let pagination = &data["pagination"];
            let response = serde_json::json!({
                "data": mapped_data,
                "pagination": {
                    "current_page": pagination.get("current_page").unwrap_or(&serde_json::Value::Null),
                    "last_visible_page": pagination.get("last_visible_page").unwrap_or(&serde_json::Value::Null),
                    "has_next_page": pagination.get("has_next_page").unwrap_or(&serde_json::Value::Null),
                    "next_page": pagination.get("next_page").unwrap_or(&serde_json::Value::Null),
                    "has_previous_page": pagination.get("has_previous_page").unwrap_or(&serde_json::Value::Null),
                    "previous_page": pagination.get("previous_page").unwrap_or(&serde_json::Value::Null),
                }
            });
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            &format!("Internal server error: {}", e),
        ),
    }
}

// --- OpenAPI-compliant handler for GET /api/komik/manhwa ---
#[derive(Debug, Deserialize)]
struct ManhwaListQuery {
    page: Option<u32>,
    order: Option<String>,
}

async fn manhwa_list_handler(
    Query(params): Query<ManhwaListQuery>,
    State(_state): State<Arc<ChatState>>,
) -> Response {
    let page = match params.page {
        Some(p) if p > 0 => p,
        _ => {
            return error_response(
                StatusCode::BAD_REQUEST,
                "Missing or invalid 'page' query parameter",
            );
        }
    };

    let allowed_orders = ["update", "popular", "titleasc", "titledesc"];
    if let Some(ref order) = params.order {
        if !allowed_orders.contains(&order.as_str()) {
            return error_response(
                StatusCode::BAD_REQUEST,
                "Invalid 'order' query parameter",
            );
        }
    }

    let result = komik::handle_list_or_search("manhwa", page, None).await;
    match result {
        Ok(data) => {
            let mapped_data: Vec<serde_json::Value> = data["data"]
                .as_array()
                .unwrap_or(&vec![])
                .iter()
                .map(|item| {
                    serde_json::json!({
                        "title": item.get("title").unwrap_or(&serde_json::Value::Null),
                        "image": item.get("poster").unwrap_or(&serde_json::Value::Null),
                        "chapter": item.get("chapter").unwrap_or(&serde_json::Value::Null),
                        "score": item.get("score").unwrap_or(&serde_json::Value::Null),
                        "date": item.get("date").unwrap_or(&serde_json::Value::Null),
                        "type": item.get("manga_type").unwrap_or(&serde_json::Value::Null),
                        "komik_id": item.get("slug").unwrap_or(&serde_json::Value::Null),
                    })
                })
                .collect();

            let pagination = &data["pagination"];
            let response = serde_json::json!({
                "data": mapped_data,
                "pagination": {
                    "current_page": pagination.get("current_page").unwrap_or(&serde_json::Value::Null),
                    "last_visible_page": pagination.get("last_visible_page").unwrap_or(&serde_json::Value::Null),
                    "has_next_page": pagination.get("has_next_page").unwrap_or(&serde_json::Value::Null),
                    "next_page": pagination.get("next_page").unwrap_or(&serde_json::Value::Null),
                    "has_previous_page": pagination.get("has_previous_page").unwrap_or(&serde_json::Value::Null),
                    "previous_page": pagination.get("previous_page").unwrap_or(&serde_json::Value::Null),
                }
            });
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            &format!("Internal server error: {}", e),
        ),
    }
}

// --- OpenAPI-compliant handler for GET /api/komik/manhua ---
#[derive(Debug, Deserialize)]
struct ManhuaListQuery {
    page: Option<u32>,
    order: Option<String>,
}

async fn manhua_list_handler(
    Query(params): Query<ManhuaListQuery>,
    State(_state): State<Arc<ChatState>>,
) -> Response {
    let page = match params.page {
        Some(p) if p > 0 => p,
        _ => {
            return error_response(
                StatusCode::BAD_REQUEST,
                "Missing or invalid 'page' query parameter",
            );
        }
    };

    let allowed_orders = ["update", "popular", "titleasc", "titledesc"];
    if let Some(ref order) = params.order {
        if !allowed_orders.contains(&order.as_str()) {
            return error_response(
                StatusCode::BAD_REQUEST,
                "Invalid 'order' query parameter",
            );
        }
    }

    let result = komik::handle_list_or_search("manhua", page, None).await;
    match result {
        Ok(data) => {
            let mapped_data: Vec<serde_json::Value> = data["data"]
                .as_array()
                .unwrap_or(&vec![])
                .iter()
                .map(|item| {
                    serde_json::json!({
                        "title": item.get("title").unwrap_or(&serde_json::Value::Null),
                        "image": item.get("poster").unwrap_or(&serde_json::Value::Null),
                        "chapter": item.get("chapter").unwrap_or(&serde_json::Value::Null),
                        "score": item.get("score").unwrap_or(&serde_json::Value::Null),
                        "date": item.get("date").unwrap_or(&serde_json::Value::Null),
                        "type": item.get("manga_type").unwrap_or(&serde_json::Value::Null),
                        "komik_id": item.get("slug").unwrap_or(&serde_json::Value::Null),
                    })
                })
                .collect();

            let pagination = &data["pagination"];
            let response = serde_json::json!({
                "data": mapped_data,
                "pagination": {
                    "current_page": pagination.get("current_page").unwrap_or(&serde_json::Value::Null),
                    "last_visible_page": pagination.get("last_visible_page").unwrap_or(&serde_json::Value::Null),
                    "has_next_page": pagination.get("has_next_page").unwrap_or(&serde_json::Value::Null),
                    "next_page": pagination.get("next_page").unwrap_or(&serde_json::Value::Null),
                    "has_previous_page": pagination.get("has_previous_page").unwrap_or(&serde_json::Value::Null),
                    "previous_page": pagination.get("previous_page").unwrap_or(&serde_json::Value::Null),
                }
            });
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            &format!("Internal server error: {}", e),
        ),
    }
}

// --- OpenAPI-compliant handler for GET /api/komik/search ---
#[derive(Debug, Deserialize)]
struct KomikSearchQuery {
    query: Option<String>,
}

async fn komik_search_handler(
    Query(params): Query<KomikSearchQuery>,
    State(_state): State<Arc<ChatState>>,
) -> Response {
    let result = komik::handle_list_or_search("search", 1, params.query.as_deref()).await;
    match result {
        Ok(data) => {
            let mapped_data: Vec<serde_json::Value> = data["data"]
                .as_array()
                .unwrap_or(&vec![])
                .iter()
                .map(|item| {
                    serde_json::json!({
                        "title": item.get("title").unwrap_or(&serde_json::Value::Null),
                        "image": item.get("poster").unwrap_or(&serde_json::Value::Null),
                        "chapter": item.get("chapter").unwrap_or(&serde_json::Value::Null),
                        "score": item.get("score").unwrap_or(&serde_json::Value::Null),
                        "date": item.get("date").unwrap_or(&serde_json::Value::Null),
                        "type": item.get("manga_type").unwrap_or(&serde_json::Value::Null),
                        "komik_id": item.get("slug").unwrap_or(&serde_json::Value::Null),
                    })
                })
                .collect();

            let prev_page = data.get("pagination")
                .and_then(|p| p.get("has_previous_page"))
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            let next_page = data.get("pagination")
                .and_then(|p| p.get("has_next_page"))
                .and_then(|v| v.as_bool())
                .unwrap_or(false);

            let response = serde_json::json!({
                "data": mapped_data,
                "prevPage": prev_page,
                "nextPage": next_page
            });
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            &format!("Internal server error: {}", e),
        ),
    }
}

// --- OpenAPI-compliant handler for GET /api/komik/detail ---
#[derive(Debug, Deserialize)]
struct KomikDetailQuery {
    komik_id: Option<String>,
}

async fn komik_detail_handler(
    Query(params): Query<KomikDetailQuery>,
    State(_state): State<Arc<ChatState>>,
) -> Response {
    let komik_id = match params.komik_id {
        Some(ref id) if !id.trim().is_empty() => id.clone(),
        _ => {
            return error_response(
                StatusCode::BAD_REQUEST,
                "Missing or invalid 'komik_id' query parameter",
            );
        }
    };

    match komik::get_detail(&komik_id).await {
        Ok(data) => {
            let response = serde_json::json!({
                "status": "success",
                "data": data
            });
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            &format!("Failed to fetch manga detail: {}", e),
        ),
    }
}

// --- OpenAPI-compliant handler for GET /api/komik/chapter ---
#[derive(Debug, Deserialize)]
struct KomikChapterQuery {
    chapter_url: Option<String>,
}

async fn komik_chapter_handler(
    Query(params): Query<KomikChapterQuery>,
    State(_state): State<Arc<ChatState>>,
) -> Response {
    let chapter_url = match params.chapter_url {
        Some(ref url) if !url.trim().is_empty() => url.clone(),
        _ => {
            return error_response(
                StatusCode::BAD_REQUEST,
                "Missing or invalid 'chapter_url' query parameter",
            );
        }
    };

    match komik::get_chapter(&chapter_url).await {
        Ok(data) => {
            let response = serde_json::json!({
                "message": "success",
                "data": data
            });
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            &format!("Failed to fetch manga chapter: {}", e),
        ),
    }
}

/// Returns a standardized error response.
fn error_response(status: StatusCode, message: &str) -> Response {
    (
        status,
        Json(serde_json::json!({ "message": message })),
    )
        .into_response()
}

/// Main router for Komik endpoints.
pub fn create_routes() -> Router<Arc<ChatState>> {
    Router::new()
        .route("/manga", get(manga_list_handler))
        .route("/manhwa", get(manhwa_list_handler))
        .route("/manhua", get(manhua_list_handler))
        .route("/search", get(komik_search_handler))
        .route("/detail", get(komik_detail_handler))
        .route("/chapter", get(komik_chapter_handler))
        .route("/:function", get(media_handler))
}

/// Handler for dynamic media endpoints.
pub async fn media_handler(
    axum::extract::Path(function): axum::extract::Path<String>,
    Query(params): Query<KomikQueryParams>,
    State(_state): State<Arc<ChatState>>,
) -> Response {
    let allowed_functions = [
        "manga", "manhwa", "manhua", "search", "detail", "chapter", "external-link",
    ];
    if !allowed_functions.contains(&function.as_str()) {
        return error_response(StatusCode::BAD_REQUEST, "Invalid function parameter");
    }

    match function.as_str() {
        "detail" => {
            let komik_id = params.komik_id.unwrap_or_else(|| "one-piece".to_string());
            match komik::get_detail(&komik_id).await {
                Ok(data) => (StatusCode::OK, Json(data)).into_response(),
                Err(e) => error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    &format!("Failed to fetch manga detail: {}", e),
                ),
            }
        }
        "chapter" => {
            let chapter_url = params.chapter_url.unwrap_or_default();
            if chapter_url.is_empty() {
                return error_response(StatusCode::BAD_REQUEST, "chapter_url parameter is required");
            }
            match komik::get_chapter(&chapter_url).await {
                Ok(data) => (StatusCode::OK, Json(data)).into_response(),
                Err(e) => error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    &format!("Failed to fetch manga chapter: {}", e),
                ),
            }
        }
        "external-link" => match komik::handle_external_link().await {
            Ok(link) => (StatusCode::OK, Json(serde_json::json!({ "link": link }))).into_response(),
            Err(e) => error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                &format!("Failed to fetch external link: {}", e),
            ),
        },
        _ => {
            let page = params.page.unwrap_or(1);
            let query = params.query.as_deref();
            match komik::handle_list_or_search(&function, page, query).await {
                Ok(data) => {
                    if data["data"].is_null()
                        || data["data"]
                            .as_array()
                            .map_or(true, |arr| arr.is_empty())
                    {
                        (
                            StatusCode::NOT_FOUND,
                            Json(serde_json::json!({
                                "status": false,
                                "message": "No data found",
                                "data": [],
                                "pagination": data["pagination"]
                            })),
                        )
                            .into_response()
                    } else {
                        (
                            StatusCode::OK,
                            Json(serde_json::json!({
                                "data": data["data"],
                                "pagination": data["pagination"]
                            })),
                        )
                            .into_response()
                    }
                }
                Err(e) => error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    &format!("Failed to fetch komik list/search: {}", e),
                ),
            }
        }
    }
}
