use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use std::sync::Arc;
use crate::routes::ChatState;
use crate::routes::api::anime2::anime2_service;
use crate::routes::api::anime2::anime2;

pub async fn episode_handler(
    Path(episode_url_slug): Path<String>,
    State(_state): State<Arc<ChatState>>,
) -> Response {
    let episode_url = format!("https://alqanime.net/episode/{}", episode_url_slug);

    match anime2::get_anime2_episode_images(&episode_url).await {
        Ok(images) => (StatusCode::OK, Json(images)).into_response(),
        Err(e) => {
            eprintln!("Anime2 episode images error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "message": format!("Failed to fetch anime2 episode images: {}", e) })),
            )
                .into_response()
        }
    }
}

use axum::{routing::{get}, Router};

pub fn create_routes() -> Router<Arc<ChatState>> {
    Router::new()
        .route("/:episode_url_slug", get(episode_handler))
}
