use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
    Router,
    routing::get,
};
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;
use crate::routes::ChatState;
use crate::routes::api::komik::komik;

#[derive(Debug, Deserialize)]
pub struct KomikQueryParams {
    pub chapter_url: Option<String>,
}

pub async fn chapter_handler(
    Query(params): Query<KomikQueryParams>,
    State(_state): State<Arc<ChatState>>,
) -> Response {
    if let Some(chapter_url) = params.chapter_url {
        match komik::get_chapter(&chapter_url).await {
            Ok(chapter) => (StatusCode::OK, Json(chapter)).into_response(),
            Err(e) => {
                eprintln!("Error fetching komik chapter: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "message": "Failed to fetch manga chapter" })),
                )
                    .into_response()
            }
        }
    } else {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({ "message": "chapter_url parameter is required for chapter" })),
        )
            .into_response()
    }
}

pub fn create_routes() -> Router<Arc<ChatState>> {
    Router::new()
        .route("/", get(chapter_handler))
}
