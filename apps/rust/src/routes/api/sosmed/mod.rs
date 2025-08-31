//! Social Media API module router.
//!
//! This module aggregates all social media-related API routes (comments, likes, etc.)
//! and exposes them under a unified router for integration into the main API.

use axum::Router;
use std::sync::Arc;
use crate::routes::ChatState;

mod comments;
mod likes;

pub fn create_routes() -> Router<Arc<ChatState>> {
    Router::new()
        .nest("/comments", crate::routes::api::sosmed::comments::create_routes())
        .nest("/likes", crate::routes::api::sosmed::likes::create_routes())
}
