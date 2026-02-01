use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};

pub fn common_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36"));
    headers.insert("Referer", HeaderValue::from_static("https://google.com"));
    headers
}

pub fn common_image_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36"));
    headers.insert("Accept", HeaderValue::from_static("image/avif,image/webp,image/apng,image/svg+xml,image/*,*/*;q=0.8"));
    headers
}

pub fn is_internet_baik_block_page(content: &str) -> bool {
    content.contains("Internet Baik") || content.contains("TrustPositif") || content.contains("Mercusuar")
}
