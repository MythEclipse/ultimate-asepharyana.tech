// --- METADATA UNTUK BUILD.RS ---
const ENDPOINT_METHOD: &str = "GET";
const ENDPOINT_PATH: &str = "/api/anime/detail";
const ENDPOINT_DESCRIPTION: &str = "A simple test endpoint.";
const ENDPOINT_TAG: &str = "test";
const SUCCESS_RESPONSE_BODY: &str = "String";
// --- AKHIR METADATA ---

pub async fn detail_handler() -> String {
    "hello test".to_string()
}


pub fn register_routes(router: Router<Arc<ChatState>>) -> Router<Arc<ChatState>> {
    router.route(ENDPOINT_PATH, axum::routing::GET(detail_handler))
}