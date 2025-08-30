use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;
use crate::routes::ChatState;
use rust_lib::services::anime;

#[derive(Debug, Deserialize)]
pub struct AnimeQueryParams {
    pub q: Option<String>,
}

pub async fn search_handler(
    Query(params): Query<AnimeQueryParams>,
    State(_state): State<Arc<ChatState>>,
) -> Response {
    let slug = params.q.unwrap_or_else(|| "one".to_string());

    match anime::fetch_anime_data(&slug).await {
        Ok(html) => {
            let (anime_list, pagination) = anime::parse_anime_data(&html, &slug);
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
        Err(e) => {
            eprintln!("Anime search error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "message": format!("Failed to process request: {}", e) })),
            )
                .into_response()
        }
    }
}

use axum::{routing::{get}, Router};

pub fn create_routes() -> Router<Arc<ChatState>> {
    Router::new()
        .route("/", get(search_handler))
}
