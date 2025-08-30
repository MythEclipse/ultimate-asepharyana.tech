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
    pub komik_id: Option<String>,
}

pub async fn detail_handler(
    Query(params): Query<KomikQueryParams>,
    State(_state): State<Arc<ChatState>>,
) -> Response {
    if let Some(komik_id) = params.komik_id {
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
    } else {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({ "message": "komik_id parameter is required for detail" })),
        )
            .into_response()
    }
}

pub fn create_routes() -> Router<Arc<ChatState>> {
    Router::new()
        .route("/", get(detail_handler))
}
