// Proxy fetch logic with Redis cache, updated for sync Redis API and reqwest API changes.

use deadpool_redis::Connection;
use redis::AsyncCommands;
use reqwest::Client;
use tracing::{error, info, warn};

use crate::infra::redis::get_redis_conn;
use crate::utils::error::AppError;
use crate::utils::headers::common_headers;
use crate::utils::http::is_internet_baik_block_page; // Import the new common_headers function

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FetchResult {
    pub data: String,
    pub content_type: Option<String>,
}
// Implement Display for FetchResult to allow .to_string() and formatting
impl std::fmt::Display for FetchResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "FetchResult {{ data: {}, content_type: {} }}",
            self.data,
            self.content_type.as_deref().unwrap_or("None")
        )
    }
}

// --- REDIS CACHE WRAPPER START ---
fn get_fetch_cache_key(slug: &str) -> String {
    format!("fetch:proxy:{slug}")
}

async fn get_cached_fetch(slug: &str) -> Result<Option<FetchResult>, AppError> {
    // Use a static Redis connection pool for reuse
    use once_cell::sync::Lazy;
    static REDIS_POOL: Lazy<tokio::sync::Mutex<Option<Connection>>> =
        Lazy::new(|| tokio::sync::Mutex::new(None));
    let mut pool = REDIS_POOL.lock().await;
    if pool.is_none() {
        *pool = Some(get_redis_conn().await?);
    }
    let conn = pool.as_mut().unwrap();
    let key = get_fetch_cache_key(slug);
    let cached: Option<String> = conn.get(&key).await?;
    if let Some(cached_str) = cached {
        match serde_json::from_str::<FetchResult>(&cached_str) {
            Ok(parsed) => {
                info!("[fetchWithProxy] Returning cached response for {}", slug);
                Ok(Some(parsed))
            }
            Err(_) => Ok(None),
        }
    } else {
        Ok(None)
    }
}

async fn set_cached_fetch(slug: &str, value: &FetchResult) -> Result<(), AppError> {
    let mut conn = get_redis_conn().await?;
    let key = get_fetch_cache_key(slug);
    let json_string = serde_json::to_string(value)?;
    conn.set_ex::<_, _, ()>(&key, &json_string, 120).await?;
    Ok(())
}
// --- REDIS CACHE WRAPPER END ---

pub async fn fetch_with_proxy(slug: &str) -> Result<FetchResult, AppError> {
    if let Ok(Some(cached)) = get_cached_fetch(slug).await {
        return Ok(cached);
    }

    let client = Client::new();
    let headers = common_headers(); // Use the common_headers function

    match client
        .get(slug)
        .headers(headers)
        .timeout(std::time::Duration::from_secs(10))
        .send()
        .await
    {
        Ok(res) => {
            info!(
                "[fetchWithProxy] Direct fetch response: url={}, status={}",
                slug,
                res.status()
            );
            if res.status().is_success() {
                let content_type = res
                    .headers()
                    .get(reqwest::header::CONTENT_TYPE)
                    .and_then(|h| h.to_str().ok())
                    .map(|s| s.to_string());
                let bytes = res.bytes().await?;
                let text_data = if bytes.len() > 2 && bytes[0] == 0x1f && bytes[1] == 0x8b {
                    // Gzip compressed, offload to blocking thread
                    let decompressed = tokio::task::spawn_blocking(move || {
                        use flate2::read::GzDecoder;
                        use std::io::Read;
                        let mut decoder = GzDecoder::new(&bytes[..]);
                        let mut decompressed = Vec::new();
                        decoder
                            .read_to_end(&mut decompressed)
                            .map(|_| decompressed)
                            .map_err(|e| AppError::Other(format!("Decompression failed: {:?}", e)))
                    })
                    .await??;
                    match std::str::from_utf8(&decompressed) {
                        Ok(s) => s.to_string(),
                        Err(_) => String::from_utf8_lossy(&decompressed).to_string(),
                    }
                } else {
                    match std::str::from_utf8(&bytes) {
                        Ok(s) => s.to_string(),
                        Err(_) => {
                            warn!("Response bytes are not valid UTF-8, using lossy conversion");
                            String::from_utf8_lossy(&bytes).to_string()
                        }
                    }
                };

                if is_internet_baik_block_page(&text_data) {
                    warn!("Blocked by internetbaik (direct fetch) for {}", slug);
                    Err(AppError::Other(format!(
                        "Blocked by internetbaik for {}",
                        slug
                    )))
                } else {
                    let result = FetchResult {
                        data: text_data,
                        content_type,
                    };
                    set_cached_fetch(slug, &result).await?;
                    Ok(result)
                }
            } else {
                let error_msg = format!(
                    "Direct fetch failed with status {} for {}",
                    res.status(),
                    slug
                );
                error!("{}", error_msg);
                Err(AppError::Other(error_msg))
            }
        }
        Err(e) => {
            let error_msg = format!("Direct fetch failed for {}: {:?}", slug, e);
            error!("{}", error_msg);
            Err(AppError::Other(error_msg))
        }
    }
}

pub async fn fetch_with_proxy_only(slug: &str) -> Result<FetchResult, AppError> {
    if let Ok(Some(cached)) = get_cached_fetch(slug).await {
        return Ok(cached);
    }

    fetch_from_single_proxy(slug).await
}

async fn fetch_from_single_proxy(slug: &str) -> Result<FetchResult, AppError> {
    let proxy_url_base = "https://my-fetcher-mytheclipse8647-ap12h7hq.apn.leapcell.dev/fetch?url="; // Use only the first proxy
    let client = Client::new();
    let encoded_url = urlencoding::encode(slug);
    let proxy_url = format!("{}{}", proxy_url_base, encoded_url);

    info!(
        "[fetch_from_single_proxy] Attempting to fetch {} via single proxy",
        slug
    );

    match client
        .get(&proxy_url)
        .timeout(std::time::Duration::from_secs(30))
        .send()
        .await
    {
        Ok(res) => {
            info!(
                "[fetch_from_single_proxy] Proxy fetch response for {}: status={}",
                slug,
                res.status()
            );
            if res.status().is_success() {
                let content_type = res
                    .headers()
                    .get(reqwest::header::CONTENT_TYPE)
                    .and_then(|h| h.to_str().ok())
                    .map(|s| s.to_string());
                let data = res.text().await?;

                let result = FetchResult { data, content_type };
                info!(
                    "[fetch_from_single_proxy] Successfully fetched {} via single proxy",
                    slug
                );
                set_cached_fetch(slug, &result).await?;
                Ok(result)
            } else {
                let error_msg = format!(
                    "Single proxy fetch failed with status {} for {}",
                    res.status(),
                    slug
                );
                warn!("{}", error_msg);
                Err(AppError::Other(error_msg))
            }
        }
        Err(e) => {
            let error_msg = format!("Single proxy fetch failed for {}: {:?}", slug, e);
            warn!("{}", error_msg);
            Err(AppError::Other(error_msg))
        }
    }
}
