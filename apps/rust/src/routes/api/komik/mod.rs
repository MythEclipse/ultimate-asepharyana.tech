use axum::{Router};
use std::sync::Arc;
use crate::routes::ChatState;

pub mod detail;
pub mod chapter;
pub mod manga;
pub mod manhwa;
pub mod manhua;
pub mod search;
pub mod external_link;
pub mod manga_dto;
pub mod komik_service;
pub use self::komik_service as komik;

pub fn create_routes() -> Router<Arc<ChatState>> {
    Router::new()
        .nest("/detail", detail::create_routes())
        .nest("/chapter", chapter::create_routes())
        .nest("/manga", manga::create_routes())
        .nest("/manhwa", manhwa::create_routes())
        .nest("/manhua", manhua::create_routes())
        .nest("/search", search::create_routes())
        .nest("/external-link", external_link::create_routes())
}
