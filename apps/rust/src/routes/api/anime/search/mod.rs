// This file is created to resolve the 'file not found for module `search`' error.
pub fn create_routes() -> axum::Router<std::sync::Arc<crate::routes::ChatState>> {
    axum::Router::new()
}
