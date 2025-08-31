use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
    Router,
    routing::get,
};
use serde_json::json;
use std::sync::Arc;
use crate::routes::ChatState;
use crate::routes::api::komik::komik;
use axum::extract::Path;

#[allow(dead_code)]
pub async fn detail_handler(
    Path(komik_id): Path<String>,
    State(_state): State<Arc<ChatState>>,
) -> Response {
    match komik::get_detail(&komik_id).await {
        Ok(detail) => (StatusCode::OK, Json(detail)).into_response(),
        Err(e) => {
            eprintln!("Error fetching komik detail: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "message": "Failed to fetch manga detail" })),
            )
                .into_response()
        }
    }
}

#[allow(dead_code)]
pub fn create_routes() -> Router<Arc<ChatState>> {
    Router::new()
        .route("/:komik_id", get(detail_handler))
}
