use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use std::sync::Arc;

use crate::routes::ChatState;
use crate::routes::api::anime::otakudesu_service::{fetch_anime_page_ongoing, parse_anime_page_ongoing};

#[utoipa::path(
    get,
    path = "/api/anime/ongoing-anime/{slug}",
    params(
        ("slug" = String, Path, description = "Anime page slug")
    ),
    responses(
        (status = 200, description = "List of ongoing anime"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Anime"
)]
pub async fn ongoing_anime_handler(
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
