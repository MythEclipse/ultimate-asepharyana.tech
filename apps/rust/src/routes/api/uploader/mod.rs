use axum::{Router};
use std::sync::Arc;
use crate::routes::mod_::ChatState;

pub mod route;
pub mod file;

pub fn create_routes() -> Router<Arc<ChatState>> {
    Router::new()
        .nest("/", route::create_routes())
        .nest("/:file", file::create_routes())
}
