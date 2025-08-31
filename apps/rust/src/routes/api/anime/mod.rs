use axum::{Router};
use std::sync::Arc;
use crate::routes::ChatState;

pub mod search;
pub mod detail;
pub mod episode;
pub mod anime_service; // otakudesu
pub mod anime_dto;     // otakudesu
pub mod alqanime_service;
pub mod alqanime_dto;
pub mod complete_anime;
pub mod full;
pub mod otakudesu_service; // New module for Otakudesu service

// NOTE: The /search endpoint now supports ?status=complete or ?status=ongoing for filtering.

pub mod anime_detail_dto;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

use crate::routes::ChatState;
use crate::routes::api::komik::manga_dto::Pagination;
use crate::routes::api::anime::otakudesu_service::{fetch_anime_page_ongoing, parse_anime_page_ongoing, AnimeItem};

pub async fn get_ongoing_anime(
    Path(slug): Path<String>,
    State(_state): State<Arc<ChatState>>,
) -> Response {
    let html = match fetch_anime_page_ongoing(&slug).await {
        Ok(html) => html,
        Err(e) => {
            eprintln!("Error fetching anime page: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "message": format!("Failed to fetch anime data: {}", e) })),
            )
                .into_response();
        }
    };

    let (anime_list, pagination) = parse_anime_page_ongoing(&html, &slug);

    (
        StatusCode::OK,
        Json(json!({
            "status": "Ok",
            "data": anime_list,
            "pagination": pagination,
        })),
    )
        .into_response()
}

pub fn create_routes() -> Router<Arc<ChatState>> {
    Router::new()
        .route("/complete-anime/:slug", axum::routing::get(complete_anime::complete_anime_handler))
        .route("/ongoing-anime/:slug", axum::routing::get(get_ongoing_anime))
        .route("/full/:slug", axum::routing::get(full::full_anime_handler))
}
