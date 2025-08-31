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
            let anime_data = AnimeDetailResponseData {
                title: detail.title,
                alternative_title: detail.alternative_title,
                poster: detail.poster,
                r#type: detail.r#type,
                release_date: detail.release_date,
                status: detail.status,
                synopsis: detail.synopsis,
                studio: detail.studio,
                genres: detail.genres.into_iter().map(|g| Genre {
                    name: g.name,
                    slug: g.slug,
                    anime_url: g.anime_url,
                }).collect(),
                producers: detail.producers,
                recommendations: detail.recommendations.into_iter().map(|r| Recommendation {
                    title: r.title,
                    slug: r.slug,
                    poster: r.poster,
                    status: r.status,
                    r#type: r.r#type,
                }).collect(),
                batch: detail.batch.into_iter().map(|b| EpisodeListItem {
                    episode: b.episode,
                    slug: b.slug,
                }).collect(),
                episode_lists: detail.episode_lists.into_iter().map(|e| EpisodeListItem {
                    episode: e.episode,
                    slug: e.slug,
                }).collect(),
            };

            let response = AnimeDetailResponse {
                status: "Ok".to_string(),
                data: anime_data,
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
