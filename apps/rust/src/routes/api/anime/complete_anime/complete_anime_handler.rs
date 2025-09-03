// --- METADATA UNTUK BUILD.RS ---
const ENDPOINT_METHOD: &str = "GET";
const ENDPOINT_PATH: &str = "/api/anime/complete_anime";
const ENDPOINT_DESCRIPTION: &str = "A simple test endpoint.";
const ENDPOINT_TAG: &str = "test";
const SUCCESS_RESPONSE_BODY: &str = "String";
// --- AKHIR METADATA ---

pub async fn complete_anime_handler() -> String {
    "hello test".to_string()
}
