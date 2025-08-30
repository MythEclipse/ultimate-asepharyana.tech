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
use rust_lib::services::komik;

#[derive(Debug, Deserialize)]
pub struct KomikQueryParams {
    pub page: Option<u32>,
    pub query: Option<String>,
}

pub async fn search_handler(
    Query(params): Query<KomikQueryParams>,
    State(_state): State<Arc<ChatState>>,
) -> Response {
    let page = params.page.unwrap_or(1);
    let query = params.query.as_deref();

    match komik::handle_list_or_search("search", page, query).await {
        Ok((data, pagination)) => (
            StatusCode::OK,
            Json(json!({ "data": data, "pagination": pagination })),
        )
            .into_response(),
        Err(e) => {
            eprintln!("Error fetching komik list/search: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "message": "Failed to fetch manga list/search" })),
            )
                .into_response()
        }
    }
}

pub fn create_routes() -> Router<Arc<ChatState>> {
    Router::new()
        .route("/", get(search_handler))
}
