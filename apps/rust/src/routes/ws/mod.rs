pub mod chat;
pub mod models;

use axum::Router;
use std::sync::Arc;
use crate::routes::AppState;

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    chat::register_routes(router)
}

