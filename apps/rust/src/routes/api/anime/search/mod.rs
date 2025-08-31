use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use std::sync::Arc;
use crate::routes::api::anime::anime_service::{fetch_anime_data, parse_anime_data};

pub async fn search_handler(
    Path(slug): Path<String>,
) -> Response {
    let slug = if slug.is_empty() { "one".to_string() } else { slug };

    // There is no fetch_anime_data_with_status, using fetch_anime_data instead
    match fetch_anime_data(&slug).await {
        Ok(html) => {
            let (anime_list, pagination) = parse_anime_data(&html, &slug);
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
                Json(json!({
                    "message": "Failed to process request",
                    "error": format!("{}", e)
                })),
            )
                .into_response()
        }
    }
}

use axum::{routing::{get}, Router};

pub fn create_routes() -> Router {
    Router::new()
        .route("/:slug", get(search_handler))
}
