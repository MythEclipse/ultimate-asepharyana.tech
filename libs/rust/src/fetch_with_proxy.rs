// Proxy fetch logic with Redis cache, updated for sync Redis API and reqwest API changes.

use reqwest::Client;
use tracing::{ info, warn, error };
use redis::AsyncCommands;

use crate::redis_client::get_redis_conn;
use crate::utils::http::is_internet_baik_block_page;
use crate::utils::error::AppError;
use std::sync::Arc;
use tokio::sync::Mutex as TokioMutex; // Use Tokio Mutex for async operations

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
  let mut conn = get_redis_conn().await?;
  let key = get_fetch_cache_key(slug);
  let cached: Option<String> = conn.get(&key).await?;
  if let Some(cached_str) = cached {
    match serde_json::from_str::<FetchResult>(&cached_str) {
      Ok(parsed) => {
        info!("[fetchWithProxy] Returning cached response for {}", slug);
        Ok(Some(parsed))
      }
      Err(_) => { Ok(None) }
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

pub async fn fetch_with_proxy(
  slug: &str,
  _browser: &Arc<TokioMutex<()>>
) -> Result<FetchResult, AppError> {
  if let Ok(Some(cached)) = get_cached_fetch(slug).await {
    return Ok(cached);
  }

  let client = Client::new();
  let mut headers = reqwest::header::HeaderMap::new();
  headers.insert(
    reqwest::header::ACCEPT,
    reqwest::header::HeaderValue::from_static(
      "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7"
    )
  );
  headers.insert(
    reqwest::header::ACCEPT_LANGUAGE,
    reqwest::header::HeaderValue::from_static("en-US,en;q=0.9,id;q=0.8")
  );
  headers.insert(
    reqwest::header::CACHE_CONTROL,
    reqwest::header::HeaderValue::from_static("no-cache")
  );
  headers.insert(reqwest::header::PRAGMA, reqwest::header::HeaderValue::from_static("no-cache"));
  headers.insert("priority", reqwest::header::HeaderValue::from_static("u=0, i"));
  headers.insert(
    "sec-ch-ua",
    reqwest::header::HeaderValue::from_static(
      "\"Not;A=Brand\";v=\"99\", \"Microsoft Edge\";v=\"139\", \"Chromium\";v=\"139\""
    )
  );
  headers.insert("sec-ch-ua-mobile", reqwest::header::HeaderValue::from_static("?0"));
  headers.insert("sec-ch-ua-platform", reqwest::header::HeaderValue::from_static("\"Windows\""));
  headers.insert("sec-fetch-dest", reqwest::header::HeaderValue::from_static("document"));
  headers.insert("sec-fetch-mode", reqwest::header::HeaderValue::from_static("navigate"));
  headers.insert("sec-fetch-site", reqwest::header::HeaderValue::from_static("none"));
  headers.insert("sec-fetch-user", reqwest::header::HeaderValue::from_static("?1"));
  headers.insert("upgrade-insecure-requests", reqwest::header::HeaderValue::from_static("1"));
  headers.insert(
    reqwest::header::USER_AGENT,
    reqwest::header::HeaderValue::from_static(
      "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/139.0.0.0 Safari/537.36 Edg/139.0.0.0"
    )
  );

  match client.get(slug).headers(headers).timeout(std::time::Duration::from_secs(10)).send().await {
    Ok(res) => {
      info!("[fetchWithProxy] Direct fetch response: url={}, status={}", slug, res.status());
      if res.status().is_success() {
        let content_type = res
          .headers()
          .get(reqwest::header::CONTENT_TYPE)
          .and_then(|h| h.to_str().ok())
          .map(|s| s.to_string());
        let bytes = res.bytes().await?;
        let text_data = if bytes.len() > 2 && bytes[0] == 0x1f && bytes[1] == 0x8b {
          // Gzip compressed
          use flate2::read::GzDecoder;
          use std::io::Read;
          let mut decoder = GzDecoder::new(&bytes[..]);
          let mut decompressed = Vec::new();
          decoder
            .read_to_end(&mut decompressed)
            .map_err(|e| AppError::Other(format!("Decompression failed: {:?}", e)))?;
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
    }
    Err(e) => {
      warn!("Direct fetch failed for {}: {:?}", slug, e);
      error!("Direct fetch failed, trying proxies");
      let proxy_result = fetch_from_proxies(slug).await?;
      set_cached_fetch(slug, &proxy_result).await?;
      Ok(proxy_result)
    }
  }
}

pub async fn fetch_with_proxy_only(
  slug: &str,
  _browser: &Arc<TokioMutex<()>>
) -> Result<FetchResult, AppError> {
  if let Ok(Some(cached)) = get_cached_fetch(slug).await {
    return Ok(cached);
  }

  let proxy_result = fetch_from_proxies(slug).await?;
  set_cached_fetch(slug, &proxy_result).await?;
  Ok(proxy_result)
}

async fn fetch_from_proxies(slug: &str) -> Result<FetchResult, AppError> {
  let proxy_urls = [
    "https://my-fetcher-mytheclipse8647-ap12h7hq.apn.leapcell.dev/fetch?url=",
    // Add more proxy URLs here if needed
  ];

  let client = Client::new();
  let encoded_url = urlencoding::encode(slug);

  for (i, base_proxy_url) in proxy_urls.iter().enumerate() {
    let proxy_url = format!("{}{}", base_proxy_url, encoded_url);
    info!(
      "[fetch_from_proxies] Attempting to fetch {} via proxy {}/{}",
      slug,
      i + 1,
      proxy_urls.len()
    );

    match client.get(&proxy_url).timeout(std::time::Duration::from_secs(30)).send().await {
      Ok(res) => {
        info!(
          "[fetch_from_proxies] Proxy fetch response from {}/{} for {}: status={}",
          i + 1,
          proxy_urls.len(),
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
            "[fetch_from_proxies] Successfully fetched {} via proxy {}/{}",
            slug,
            i + 1,
            proxy_urls.len()
          );
          return Ok(result);
        } else {
          let error_msg = format!(
            "Proxy {}/{} fetch failed with status {}",
            i + 1,
            proxy_urls.len(),
            res.status()
          );
          warn!("Proxy fetch failed for {}: {}", slug, error_msg);
        }
      }
      Err(e) => {
        warn!("Proxy {}/{} fetch failed for {}: {:?}", i + 1, proxy_urls.len(), slug, e);
      }
    }
  }

  error!("All proxy attempts failed for {}", slug);
  Err(AppError::Other("All proxy attempts failed".to_string()))
}
