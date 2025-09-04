//! Handler for the posts endpoint.
 #![allow(dead_code)]

 use axum::{response::IntoResponse, routing::get, Json, Router};
 use std::sync::Arc;
 use crate::routes::AppState;
 use serde::{Deserialize, Serialize};
 use utoipa::ToSchema;

 pub const ENDPOINT_METHOD: &str = "get";
 pub const ENDPOINT_PATH: &str = "/sosmed/posts";
 pub const ENDPOINT_DESCRIPTION: &str = "Description for the posts endpoint";
 pub const ENDPOINT_TAG: &str = "sosmed";
 pub const SUCCESS_RESPONSE_BODY: &str = "Json<PostsResponse>";

 #[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
 pub struct PostsResponse {
     pub message: String,
 }
#[utoipa::path(
    get,
    path = "/api/sosmed/posts",
    tag = "sosmed",
    operation_id = "sosmed_posts",
    responses(
        (status = 200, description = "Description for the posts endpoint", body = PostsResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn posts() -> impl IntoResponse {
     Json(PostsResponse {
         message: "Hello from posts!".to_string(),
     })
 }

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(posts))
}