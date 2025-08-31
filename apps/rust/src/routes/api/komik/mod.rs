use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json, Router, routing::get,
};
use serde::Deserialize;
use std::sync::Arc;
use crate::routes::ChatState;
pub mod detail;
pub mod chapter;
pub mod search;
pub mod external_link;
pub mod manga_dto;
pub mod komik_service;
pub use self::komik_service as komik;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct KomikQueryParams {
    pub page: Option<u32>,
    pub query: Option<String>,
}

#[allow(dead_code)]
pub async fn media_handler(
    Path(media_type): Path<String>,
    Query(params): Query<KomikQueryParams>,
    State(_state): State<Arc<ChatState>>,
) -> Response {
    let allowed_types = ["manga", "manhwa", "manhua"];
    if !allowed_types.contains(&media_type.as_str()) {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "message": "Invalid komik type" })),
        )
            .into_response();
    }
    let page = params.page.unwrap_or(1);
    let query = params.query.as_deref();
    match komik::handle_list_or_search(&media_type, page, query).await {
        Ok((data, pagination)) => (
            StatusCode::OK,
            Json(serde_json::json!({ "data": data, "pagination": pagination })),
        )
            .into_response(),
        Err(e) => {
            eprintln!("Error fetching komik list/search: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "message": "Failed to fetch komik list/search" })),
            )
                .into_response()
        }
    }
}

#[allow(dead_code)]
pub fn create_routes() -> Router<Arc<ChatState>> {
    Router::new()
        .nest("/detail", detail::create_routes())
        .nest("/chapter", chapter::create_routes())
        .nest("/search", search::create_routes())
        .nest("/external-link", external_link::create_routes())
        .route("/:type", get(media_handler))
}
