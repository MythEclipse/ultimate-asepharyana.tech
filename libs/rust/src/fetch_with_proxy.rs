// Proxy fetch logic with Redis cache, updated for sync Redis API and reqwest API changes.

use reqwest::Client;
use tracing::{info, warn, error};

use crate::redis_client::get_redis_connection;
use crate::scrape_croxy_proxy::scrape_croxy_proxy_cached;
use crate::utils::http::is_internet_baik_block_page;
use crate::utils::error::AppError;

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
    let mut conn = get_redis_connection()?;
    let key = get_fetch_cache_key(slug);
    let cached: Option<String> = redis::cmd("GET").arg(&key).query(&mut conn)?;
    if let Some(cached_str) = cached {
        match serde_json::from_str::<FetchResult>(&cached_str) {
            Ok(parsed) => {
                info!("[fetchWithProxy] Returning cached response for {}", slug);
                Ok(Some(parsed))
            },
            Err(_) => {
                Ok(None)
            }
        }
    } else {
        Ok(None)
    }
}

async fn set_cached_fetch(slug: &str, value: &FetchResult) -> Result<(), AppError> {
    let mut conn = get_redis_connection()?;
    let key = get_fetch_cache_key(slug);
    let json_string = serde_json::to_string(value)?;
    redis::cmd("SET").arg(&key).arg(&json_string).arg("EX").arg(120).query::<()>(&mut conn)?;
    Ok(())
}
// --- REDIS CACHE WRAPPER END ---

pub async fn fetch_with_proxy(slug: &str) -> Result<FetchResult, AppError> {
    if let Ok(Some(cached)) = get_cached_fetch(slug).await {
        return Ok(cached);
    }

    let client = Client::new();
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(reqwest::header::USER_AGENT, reqwest::header::HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3"));
    headers.insert(reqwest::header::CACHE_CONTROL, reqwest::header::HeaderValue::from_static("no-store"));

    match client.get(slug).headers(headers).send().await {
        Ok(res) => {
            info!("[fetchWithProxy] Direct fetch response: url={}, status={}", slug, res.status());
            if res.status().is_success() {
                let content_type = res.headers().get(reqwest::header::CONTENT_TYPE)
                    .and_then(|h| h.to_str().ok())
                    .map(|s| s.to_string());
                let text_data = res.text().await?;

                if is_internet_baik_block_page(&text_data) {
                    warn!("Blocked by internetbaik (direct fetch), trying proxies");
                    let proxy_result = fetch_from_proxies(slug).await?;
                    set_cached_fetch(slug, &proxy_result).await?;
                    Ok(proxy_result)
                } else {
                    let result = FetchResult { data: text_data, content_type };
                    set_cached_fetch(slug, &result).await?;
                    Ok(result)
                }
            } else {
                let error_msg = format!("Direct fetch failed with status {}", res.status());
                error!("Direct fetch failed for {}: {}", slug, error_msg);
                error!("Direct fetch failed, trying proxies");
                let proxy_result = fetch_from_proxies(slug).await?;
                set_cached_fetch(slug, &proxy_result).await?;
                Ok(proxy_result)
            }
        },
        Err(e) => {
            warn!("Direct fetch failed for {}: {:?}", slug, e);
            error!("Direct fetch failed, trying proxies");
            let proxy_result = fetch_from_proxies(slug).await?;
            set_cached_fetch(slug, &proxy_result).await?;
            Ok(proxy_result)
        }
    }
}

pub async fn fetch_with_proxy_only(slug: &str) -> Result<FetchResult, AppError> {
    if let Ok(Some(cached)) = get_cached_fetch(slug).await {
        return Ok(cached);
    }

    let proxy_result = fetch_from_proxies(slug).await?;
    set_cached_fetch(slug, &proxy_result).await?;
    Ok(proxy_result)
}

async fn fetch_from_proxies(slug: &str) -> Result<FetchResult, AppError> {
    info!("Using only croxy proxy for {}", slug);

    // Only use scrapeCroxyProxy
    match scrape_croxy_proxy_cached(slug).await {
        Ok(html) => {
            info!("scrapeCroxyProxy successful for {}", slug);
            Ok(FetchResult { data: html, content_type: Some("text/html".to_string()) })
        },
        Err(e) => {
            error!("scrapeCroxyProxy failed for {}: {:?}", slug, e);
            Err(AppError::Other(format!("Failed to fetch using croxy proxy: {:?}", e)))
        }
    }
}
