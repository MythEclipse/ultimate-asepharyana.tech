use reqwest::{Client, Proxy};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tracing::{info, warn, error};
use url::Url;
use regex::Regex;

use crate::redis_client::get_redis_connection;
use crate::scrape_croxy_proxy::{scrape_croxy_proxy, scrape_croxy_proxy_cached};
use crate::utils::http::is_internet_baik_block_page;
use crate::error::AppError;

const DEFAULT_PROXY_LIST_URL: &str = "https://www.proxy-list.download/api/v1/get?type=https";

#[derive(Debug, Clone)]
pub struct FetchResult {
    pub data: String,
    pub content_type: Option<String>,
}

fn get_proxy_list_url() -> String {
    std::env::var("PROXY_LIST_URL").unwrap_or_else(|_| DEFAULT_PROXY_LIST_URL.to_string())
}

fn parse_proxy_line(line: &str) -> Option<String> {
    let trimmed = line.trim();
    if trimmed.is_empty() || trimmed.starts_with('#') {
        return None;
    }
    if Regex::new(r"^(http|https|socks4|socks5)://").unwrap().is_match(trimmed) {
        return Some(trimmed.to_string());
    }
    if Regex::new(r"^[^:]+:\d+$").unwrap().is_match(trimmed) {
        return Some(format!("https://{}", trimmed));
    }
    None
}

async fn get_proxies() -> Result<Vec<String>, AppError> {
    let client = Client::new();
    let res = client.get(&get_proxy_list_url()).send().await?;
    if !res.status().is_ok() {
        let error_msg = format!("Failed to fetch proxy list: {}", res.status());
        error!("Error fetching proxy list: {}", error_msg);
        return Err(error_msg.into());
    }
    let data = res.text().await?;
    let proxies = data
        .lines()
        .filter_map(parse_proxy_line)
        .take(10)
        .collect();
    Ok(proxies)
}

static mut CACHED_PROXIES: Option<Vec<String>> = None;
static mut CACHE_TIMESTAMP: Instant = Instant::now();
const CACHE_DURATION: Duration = Duration::from_secs(6 * 60);

async fn get_cached_proxies() -> Result<Vec<String>, AppError> {
    let now = Instant::now();
    unsafe {
        if CACHED_PROXIES.is_none() || now.duration_since(CACHE_TIMESTAMP) > CACHE_DURATION {
            CACHED_PROXIES = Some(get_proxies().await?);
            CACHE_TIMESTAMP = now;
        }
        Ok(CACHED_PROXIES.clone().unwrap_or_default())
    }
}

// --- REDIS CACHE WRAPPER START ---
fn get_fetch_cache_key(slug: &str) -> String {
    format!("fetch:proxy:{}", slug)
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
    redis::cmd("SET").arg(&key).arg(&json_string).arg("EX").arg(120).query(&mut conn)?;
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
            if res.status().is_ok() {
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
    let mut last_error: Option<AppError> = None;
    let proxies = get_cached_proxies().await?;

    for proxy_url_str in proxies {
        info!("[DEBUG] Proxy loop: Trying proxy {}", proxy_url_str);
        let client_builder = Client::builder();

        let client = match Proxy::all(&proxy_url_str) {
            Ok(proxy) => client_builder.proxy(proxy).build()?,
            Err(e) => {
                warn!("Invalid proxy URL {}: {:?}", proxy_url_str, e);
                last_error = Some(AppError::UrlParseError(e.into()));
                continue;
            }
        };

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(reqwest::header::USER_AGENT, reqwest::header::HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3"));

        match tokio::time::timeout(Duration::from_secs(6), client.get(slug).headers(headers).send()).await {
            Ok(Ok(res)) => {
                info!("[ProxyListOnly] Proxy fetch response: url={}, proxy={}, status={}", slug, proxy_url_str, res.status());
                if res.status().is_ok() {
                    let content_type = res.headers().get(reqwest::header::CONTENT_TYPE)
                        .and_then(|h| h.to_str().ok())
                        .map(|s| s.to_string());
                    let text_data = res.text().await?;

                    if is_internet_baik_block_page(&text_data) {
                        warn!("[ProxyListOnly] Blocked by internetbaik (proxy {}), trying next proxy", proxy_url_str);
                        continue;
                    }
                    return Ok(FetchResult { data: text_data, content_type });
                }
            },
            Ok(Err(e)) => {
                last_error = Some(AppError::ReqwestError(e));
                warn!("[ProxyListOnly] Proxy fetch failed for {} via {}: {:?}", slug, proxy_url_str, last_error);
            },
            Err(_) => {
                last_error = Some(AppError::TimeoutError("Proxy request timed out".to_string()));
                warn!("[ProxyListOnly] Proxy fetch timed out for {} via {}", slug, proxy_url_str);
            }
        }
    }

    error!("Failed to fetch from all proxies for {}: {:?}", slug, last_error);

    // Fallback: try scrapeCroxyProxy as last resort
    info!("Trying scrapeCroxyProxy fallback...");
    match scrape_croxy_proxy_cached(slug).await {
        Ok(html) => {
            info!("scrapeCroxyProxy fallback successful.");
            Ok(FetchResult { data: html, content_type: Some("text/html".to_string()) })
        },
        Err(e) => {
            error!("scrapeCroxyProxy fallback failed: {:?}", e);
            Err(last_error.unwrap_or_else(|| AppError::Other("Failed to fetch from all proxies and scrapeCroxyProxy".to_string())))
        }
    }
}
