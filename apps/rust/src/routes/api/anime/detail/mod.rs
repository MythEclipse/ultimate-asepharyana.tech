use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use crate::routes::api::anime::anime_detail_dto::{AnimeDetailResponse, AnimeDetailResponseData, EpisodeListItem, Genre, Recommendation};
use crate::routes::api::anime::anime_service::get_anime_detail;

pub async fn detail_handler(
    Path(slug): Path<String>,
) -> Response {
    match get_anime_detail(&slug).await {
        Ok(detail) => {
            let response = AnimeDetailResponse {
                status: "Ok".to_string(),
                data: detail,
            };
            (StatusCode::OK, Json(response)).into_response()
        },
        Err(e) => {
            eprintln!("Anime detail error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "status": "Error", "message": format!("Failed to fetch anime detail: {}", e) })),
            )
                .into_response()
        }
    }
}

use axum::{routing::{get}, Router};

pub fn create_routes() -> Router<std::sync::Arc<crate::routes::ChatState>> {
    Router::new()
        .route("/:slug", get(detail_handler))
}
