//! Handler for the comments endpoint.
 #![allow(dead_code)]

 use axum::{response::IntoResponse, routing::get, Json, Router};
 use std::sync::Arc;
 use crate::routes::AppState;
 use serde::{Deserialize, Serialize};
 use utoipa::ToSchema;

 pub const ENDPOINT_METHOD: &str = "get";
 pub const ENDPOINT_PATH: &str = "/sosmed/comments";
 pub const ENDPOINT_DESCRIPTION: &str = "Description for the comments endpoint";
 pub const ENDPOINT_TAG: &str = "sosmed";
 pub const SUCCESS_RESPONSE_BODY: &str = "Json<CommentsResponse>";

 #[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
 pub struct CommentsResponse {
     pub message: String,
 }
#[utoipa::path(
    get,
    path = "/api/sosmed/comments",
    tag = "sosmed",
    operation_id = "sosmed_comments",
    responses(
        (status = 200, description = "Description for the comments endpoint", body = CommentsResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn comments() -> impl IntoResponse {
     Json(CommentsResponse {
         message: "Hello from comments!".to_string(),
     })
 }

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(comments))
}