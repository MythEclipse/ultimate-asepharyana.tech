use crate::routes::api::compress::compress_service::compress_service;

pub async fn compress_handler() -> String {
    compress_service().await
}


pub fn register_routes(router: Router<Arc<ChatState>>) -> Router<Arc<ChatState>> {
    router.route(ENDPOINT_PATH, axum::routing::get(compress_handler))
}