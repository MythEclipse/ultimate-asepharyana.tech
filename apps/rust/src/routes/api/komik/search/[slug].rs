use axum::{
    extract::Path,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SearchData {
    pub message: String,
    pub slug: String,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SearchResponse {
    pub status: &'static str,
    pub data: SearchData,
}

pub async fn slug_handler(Path(slug): Path<String>) -> Response {
    let response_data = SearchData {
        message: format!("Search results for komik: {}", slug),
        slug: slug.clone(),
    };

    let response = SearchResponse {
        status: "Ok",
        data: response_data,
    };

    Json(response).into_response()
}


pub fn register_routes(router: Router<Arc<ChatState>>) -> Router<Arc<ChatState>> {
    router.route(ENDPOINT_PATH, axum::routing::get(slug_handler))
}