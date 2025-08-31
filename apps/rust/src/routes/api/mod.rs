use axum::{Router};
use std::sync::Arc;
use crate::routes::ChatState;

pub mod anime;
// pub mod anime2;
pub mod sosmed;
pub mod chat;
pub mod komik;
pub mod compress;
pub mod uploader;
//// DEBUG: nextjs_lib_api module is missing, commenting out to fix build.
/// pub mod nextjs_lib_api;
// DEBUG: nextjs_lib_api module is declared but missing. Commenting out to fix build.
//// pub mod nextjs_lib_api;

pub fn create_api_routes() -> Router<Arc<ChatState>> {
    Router::new()
        .nest("/anime", anime::create_routes())
        // .nest("/anime2", anime2::create_routes())
        .nest("/sosmed", sosmed::create_routes())
}
