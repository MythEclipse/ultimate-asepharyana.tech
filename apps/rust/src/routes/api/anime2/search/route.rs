use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;
use crate::routes::mod_::ChatState;
use anime2_service;

#[derive(Debug, Deserialize)]
pub struct Anime2QueryParams {
    pub q: Option<String>,
}

pub async fn search_handler(
    Query(params): Query<Anime2QueryParams>,
    State(_state): State<Arc<ChatState>>,
) -> Response {
    let slug = params.q.unwrap_or_else(|| "log".to_string());

    match anime2_service::fetch_anime2_data(&slug).await {
        Ok(html) => {
            let (anime_list, pagination) = anime2_service::parse_anime2_data(&html);
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
            eprintln!("Anime2 search error: {:?}", e);
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
