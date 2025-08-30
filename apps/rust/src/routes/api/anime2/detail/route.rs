use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use std::sync::Arc;
use crate::routes::mod_::ChatState;
use anime2_service;

pub async fn detail_handler(
    Path(slug): Path<String>,
    State(_state): State<Arc<ChatState>>,
) -> Response {
    match anime2_service::get_anime2_detail(&slug).await {
        Ok(detail) => (StatusCode::OK, Json(detail)).into_response(),
        Err(e) => {
            eprintln!("Anime2 detail error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "message": format!("Failed to fetch anime2 detail: {}", e) })),
            )
                .into_response()
        }
    }
}

use axum::{routing::{get}, Router};

pub fn create_routes() -> Router<Arc<ChatState>> {
    Router::new()
        .route("/:slug", get(detail_handler))
}
