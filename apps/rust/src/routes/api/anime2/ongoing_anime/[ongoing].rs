pub async fn ongoing_handler() -> String {
    "hello test".to_string()
}


pub fn register_routes(router: Router<Arc<ChatState>>) -> Router<Arc<ChatState>> {
    router.route(ENDPOINT_PATH, axum::routing::get(ongoing_handler))
}