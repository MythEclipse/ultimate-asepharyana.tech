use crate::routes::api::chat::chat_service::load_messages;

pub async fn chat_handler() -> String {
    load_messages().await
}


pub fn register_routes(router: Router<Arc<ChatState>>) -> Router<Arc<ChatState>> {
    router.route(ENDPOINT_PATH, axum::routing::get(chat_handler))
}