use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
    Router,
    routing::get,
};
use serde_json::json;
use crate::routes::api::komik::komik;

pub async fn external_link_handler() -> Response {
    match komik::handle_external_link().await {
        Ok(link) => (StatusCode::OK, Json(link)).into_response(),
        Err(e) => {
            eprintln!("Error fetching external link: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "message": "Failed to fetch external link" })),
            )
                .into_response()
        }
    }
}

pub fn create_routes() -> Router {
    Router::new()
        .route("/", get(external_link_handler))
}
