use axum::{routing::{get, post, put, delete}, Router};
use std::sync::Arc;
use crate::routes::mod_::ChatState; // Adjusted path to ChatState

use super::register;
use super::imageproxy;
use super::videoproxy;
use super::apiproxy;
use super::compress;
use super::docs;
use super::komik;
use super::uploader;
pub mod anime;
use super::anime2;
use super::sosmed;

pub fn create_api_routes() -> Router<Arc<ChatState>> {
    Router::new()
        .nest("/register", register::create_routes())
        .nest("/imageproxy", imageproxy::create_routes())
        .nest("/videoproxy", videoproxy::create_routes())
        .nest("/apiproxy", apiproxy::create_routes())
        .nest("/compress", compress::create_routes())
        .nest("/docs", docs::create_routes())
        .nest("/komik", komik::create_routes())
        .nest("/uploader", uploader::create_routes())
        .route("/anime/search", get(anime::route::anime_search_handler))
        .route("/anime/detail/:slug", get(anime::route::anime_detail_handler))
        .route("/anime/episode/:episode_url_slug", get(anime::route::anime_episode_handler))
        .nest("/anime2", anime2::create_routes())
        .nest("/sosmed/comments", sosmed::comments::create_routes())
        .nest("/sosmed/likes", sosmed::likes::create_routes())
        .nest("/sosmed/posts", sosmed::posts::create_routes())
}
