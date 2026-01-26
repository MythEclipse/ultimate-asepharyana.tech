// Proxy fetch logic with Redis cache AND Request Coalescing (SingleFlight)
// Updated for sync Redis API, reqwest API changes, and concurrency optimization.

use dashmap::DashMap;
use once_cell::sync::Lazy;
use redis::AsyncCommands;
use tokio::sync::broadcast;
use tracing::{debug, error, warn};

use crate::helpers::cache_ttl::CACHE_TTL_VERY_SHORT;
use crate::infra::http_client::http_client;
use crate::infra::redis::get_redis_conn;
use crate::utils::error::AppError;
use crate::utils::headers::common_headers;
use crate::utils::http::is_internet_baik_block_page;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FetchResult {
    pub data: String,
    pub content_type: Option<String>,
}

// Implement Display to allow .to_string()
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

// Global In-Flight Request Map for Request Coalescing
// Maps URL slug -> Broadcast Sender
static IN_FLIGHT: Lazy<DashMap<String, broadcast::Sender<Result<FetchResult, String>>>> =
    Lazy::new(DashMap::new);

// --- REDIS CACHE WRAPPER START ---
fn get_fetch_cache_key(slug: &str) -> String {
    format!("fetch:proxy:{slug}")
}

async fn get_cached_fetch(slug: &str) -> Result<Option<FetchResult>, AppError> {
    let mut conn = get_redis_conn().await?;
    let key = get_fetch_cache_key(slug);

    let cached: Option<String> = conn.get(&key).await?;

    if let Some(cached_str) = cached {
        match serde_json::from_str::<FetchResult>(&cached_str) {
            Ok(parsed) => {
                debug!("[fetchWithProxy] Returning cached response for {}", slug);
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

    // Use standardized TTL
    conn.set_ex::<_, _, ()>(&key, &json_string, CACHE_TTL_VERY_SHORT)
        .await?;
    Ok(())
}
// --- REDIS CACHE WRAPPER END ---

/// Main entry point: Fetches with proxy, using Cache and Request Coalescing
pub async fn fetch_with_proxy(slug: &str) -> Result<FetchResult, AppError> {
    // 1. Try Cache First
    if let Ok(Some(cached)) = get_cached_fetch(slug).await {
        return Ok(cached);
    }

    // 2. Request Coalescing (SingleFlight)
    // Check if there is already an in-flight request for this slug
    let tx = {
        if let Some(in_flight) = IN_FLIGHT.get(slug) {
            debug!("[Coalesce] Joining in-flight request for {}", slug);
            in_flight.value().clone()
        } else {
            // No in-flight request, create a new channel
            let (tx, _) = broadcast::channel(1); // Capacity 1 is enough for single result
            IN_FLIGHT.insert(slug.to_string(), tx.clone());
            debug!("[Coalesce] Starting leader request for {}", slug);

            // We are the leader, we must execute the fetch
            // Spawn the fetch task so we don't block holding the map lock (though insert is fast)
            // But actually we are not holding the lock here anymore.

            // Clone for the async block
            let slug_clone = slug.to_string();
            let tx_clone = tx.clone();

            tokio::spawn(async move {
                let result = perform_fetch(&slug_clone).await;

                // Map AppError to String for broadcast (since AppError might not be Clone)
                // FetchResult is Clone.
                let broadcast_result = match &result {
                    Ok(res) => Ok(res.clone()),
                    Err(e) => Err(e.to_string()),
                };

                // Remove from map BEFORE broadcasting to allow retries if needed
                IN_FLIGHT.remove(&slug_clone);

                // Broadcast result to all waiting subscribers
                let _ = tx_clone.send(broadcast_result);
            });

            tx
        }
    };

    // 3. Wait for result (Leader or Follower)
    let mut rx = tx.subscribe();
    match rx.recv().await {
        Ok(Ok(res)) => Ok(res),
        Ok(Err(e_str)) => Err(AppError::Other(e_str)),
        Err(e) => {
            warn!("[Coalesce] Receive mismatch for {}: {:?}", slug, e);
            Err(AppError::Other("Request coalescing error".to_string()))
        }
    }
}

/// The actual fetch logic (Direct -> Retry)
async fn perform_fetch(slug: &str) -> Result<FetchResult, AppError> {
    // Use shared global HTTP client
    let client = http_client().client();
    let headers = common_headers();

    match client
        .get(slug)
        .headers(headers)
        .send() // Timeout handled by client
        .await
    {
        Ok(res) => {
            debug!(
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

                // Check if response is Gzip compressed (magic header 1f 8b)
                let text_data = if bytes.len() > 2 && bytes[0] == 0x1f && bytes[1] == 0x8b {
                    // Gzip compressed, offload decompression to blocking thread
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
                    // Cache the success result
                    if let Err(e) = set_cached_fetch(slug, &result).await {
                        warn!("Failed to cache result for {}: {:?}", slug, e);
                    }
                    Ok(result)
                }
            } else {
                let error_msg = format!(
                    "Direct fetch failed with status {} for {}",
                    res.status(),
                    slug
                );
                if res.status().is_server_error() {
                    error!("{}", error_msg);
                } else {
                    warn!("{}", error_msg);
                }
                Err(AppError::Other(error_msg))
            }
        }
        Err(e) => {
            let error_msg = format!("Direct fetch failed for {}: {:?}", slug, e);
            warn!("{}", error_msg);
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
    let proxy_url_base = "https://my-fetcher-mytheclipse8647-ap12h7hq.apn.leapcell.dev/fetch?url=";

    // Use shared client
    let client = http_client().client();
    let encoded_url = urlencoding::encode(slug);
    let proxy_url = format!("{}{}", proxy_url_base, encoded_url);

    debug!(
        "[fetch_from_single_proxy] Attempting to fetch {} via single proxy",
        slug
    );

    match client.get(&proxy_url).send().await {
        Ok(res) => {
            debug!(
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
                debug!(
                    "[fetch_from_single_proxy] Successfully fetched {} via single proxy",
                    slug
                );

                if let Err(e) = set_cached_fetch(slug, &result).await {
                    warn!("Failed to cache proxy result for {}: {:?}", slug, e);
                }

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
