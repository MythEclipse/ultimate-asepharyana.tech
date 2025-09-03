use crate::routes::api::drivepng::drivepng_service::drivepng_service;

pub async fn drivepng_handler() -> String {
    drivepng_service().await
}


pub fn register_routes(router: Router<Arc<ChatState>>) -> Router<Arc<ChatState>> {
    router.route(ENDPOINT_PATH, axum::routing::get(drivepng_handler))
}