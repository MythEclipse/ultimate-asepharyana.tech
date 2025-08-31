// Komik base URL logic with Redis lock, updated for sync Redis API and correct imports.

use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, debug, warn, error};
use scraper::{Html, Selector};
use regex::Regex;
use base64::{engine::general_purpose, Engine as _}; // Updated for deprecation
use url::Url;

use crate::redis_client::get_redis_connection;
use crate::fetch_with_proxy::fetch_with_proxy;
use crate::utils::error::AppError;

// --- SINGLE FLIGHT LOGIC WITH REDIS LOCK START ---
// Using a static Mutex for single-flight promise simulation
static KOMIK_BASE_URL_PROMISE: once_cell::sync::Lazy<Arc<Mutex<Option<String>>>> =
    once_cell::sync::Lazy::new(|| Arc::new(Mutex::new(None)));

const KOMIK_BASE_URL_LOCK_KEY: &str = "komik:baseurl:lock";
const KOMIK_BASE_URL_KEY: &str = "komik:baseurl";

fn acquire_redis_lock(key: &str, ttl_seconds: usize) -> Result<bool, AppError> {
    let mut conn = get_redis_connection()?;
    let acquired: bool = redis::cmd("SET")
        .arg(key)
        .arg("locked")
        .arg("NX")
        .arg("EX")
        .arg(ttl_seconds)
        .query::<()>(&mut conn)
        .is_ok();
    Ok(acquired)
}

fn release_redis_lock(key: &str) -> Result<(), AppError> {
    let mut conn = get_redis_connection()?;
    redis::cmd("DEL").arg(key).query::<()>(&mut conn)?;
    Ok(())
}

async fn sleep_ms(ms: u64) {
    tokio::time::sleep(tokio::time::Duration::from_millis(ms)).await;
}

async fn fetch_with_proxy_only_wrapper(url: &str) -> Result<String, AppError> {
    debug!("[fetchWithProxyOnlyWrapper] Fetching {}", url);
    let response = fetch_with_proxy(url).await?;
    info!("[fetchWithProxyOnlyWrapper] Fetched {}", url);
    Ok(response.data)
}

pub async fn get_dynamic_komik_base_url() -> Result<String, AppError> {
    let mut promise_guard = KOMIK_BASE_URL_PROMISE.lock().await;
    if let Some(url) = promise_guard.as_ref() {
        debug!("[getDynamicKomikBaseUrl] Returning in-flight promise");
        return Ok(url.clone());
    }

    let lock_ttl = 10; // 10 seconds
    let wait_interval = 200; // ms
    let max_wait = 10000; // 10 seconds
    let mut waited = 0;

    // Try to acquire lock
    while !acquire_redis_lock(KOMIK_BASE_URL_LOCK_KEY, lock_ttl)? {
        debug!("[getDynamicKomikBaseUrl] Waiting for Redis lock...");
        sleep_ms(wait_interval).await;
        waited += wait_interval;
        if waited >= max_wait {
            warn!("[getDynamicKomikBaseUrl] Waited too long for lock, proceeding anyway");
            return Err(AppError::Other("Waited too long for Redis lock".to_string()));
        }
        // Check if value is already cached by other process
        let mut conn = get_redis_connection()?;
        let cached: Option<String> = redis::cmd("GET").arg(KOMIK_BASE_URL_KEY).query(&mut conn)?;
        if let Some(cached_url) = cached {
            if !cached_url.contains(".cz") {
                info!("[getDynamicKomikBaseUrl] Found cached base URL while waiting for lock: {}", cached_url);
                *promise_guard = Some(cached_url.clone());
                return Ok(cached_url);
            }
        }
    }

    let result = (async {
        debug!("[getDynamicKomikBaseUrl] Fetching komik base URL");
        let body = fetch_with_proxy_only_wrapper("https://komikindo.cz/").await?;
        let document = Html::parse_document(&body);

        let website_btn_selector = Selector::parse("a.elementskit-btn").unwrap();
        let mut org_link = String::new();

        for element in document.select(&website_btn_selector) {
            let href = element.value().attr("href").unwrap_or_default();
            let cpo_original_value_of_href = element.value().attr("__cporiginalvalueofhref").unwrap_or_default();

            let re_komikindo = Regex::new(r"komikindo\.(?!cz)").unwrap();
            if re_komikindo.is_match(href) || re_komikindo.is_match(cpo_original_value_of_href) {
                org_link = if !cpo_original_value_of_href.is_empty() {
                    cpo_original_value_of_href.to_string()
                } else {
                    href.to_string()
                };
                break;
            }
        }

        // If link is an IP, decode from query string __cpo
        let re_ip = Regex::new(r"^\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}").unwrap();
        if re_ip.is_match(&org_link) {
            if let Ok(parsed_url) = Url::parse(&org_link) {
                if let Some(cpo) = parsed_url.query_pairs().find(|(key, _)| key == "__cpo").map(|(_, value)| value) {
                    match general_purpose::STANDARD.decode(&*cpo) {
                        Ok(decoded_bytes) => {
                            if let Ok(decoded_str) = String::from_utf8(decoded_bytes) {
                                org_link = decoded_str;
                            } else {
                                error!("[getDynamicKomikBaseUrl] Failed to decode __cpo to UTF-8: {}", cpo);
                            }
                        },
                        Err(e) => error!("[getDynamicKomikBaseUrl] Failed to base64 decode __cpo: {}", e),
                    }
                }
            }
        }

        if org_link.is_empty() || org_link.contains(".cz") {
            error!("[getDynamicKomikBaseUrl] Failed to fetch komik base URL selain cz");
            return Err(AppError::Other("Failed to fetch komik base URL selain cz".to_string()));
        }
        info!("[getDynamicKomikBaseUrl] Got base URL: {}", org_link);
        let final_url = org_link.trim_end_matches('/').to_string();
        // Cache the result immediately for other waiters
        let mut conn = get_redis_connection()?;
        redis::cmd("SET").arg(KOMIK_BASE_URL_KEY).arg(&final_url).arg("EX").arg(60 * 60 * 24 * 30).query::<()>(&mut conn)?;
        Ok(final_url)
    }).await;

    *promise_guard = result.as_ref().ok().cloned();
    let _ = release_redis_lock(KOMIK_BASE_URL_LOCK_KEY); // Release lock regardless of success or failure

    result
}

pub async fn get_cached_komik_base_url(force_refresh: bool) -> Result<String, AppError> {
    if !force_refresh {
        let mut conn = get_redis_connection()?;
        let cached: Option<String> = redis::cmd("GET").arg(KOMIK_BASE_URL_KEY).query(&mut conn)?;
        if let Some(cached_url) = cached {
            if !cached_url.contains(".cz") {
                info!("[getCachedKomikBaseUrl] Using cached base URL: {}", cached_url);
                return Ok(cached_url);
            }
        }
    }
    // Fetch new value and cache it
    let url = get_dynamic_komik_base_url().await?;
    info!("[getCachedKomikBaseUrl] Refreshed and cached base URL: {}", url);
    Ok(url)
}
