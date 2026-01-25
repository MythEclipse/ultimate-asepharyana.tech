pub mod chat;
pub mod models;

use crate::routes::AppState;
use axum::Router;
use std::sync::Arc;

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    chat::register_routes(router)
}
