// CroxyProxy scraping and caching logic using shared chromiumoxide library and Redis.
// Updated to use shared browser instance.

use std::time::Instant;
use tracing::{ info, warn, error };
use crate::redis_client::get_redis_connection;
use crate::utils::error::AppError;
use fantoccini::{ Client, Locator };

const CROXY_PROXY_URL: &str = "https://www.croxyproxy.com/";
const URL_INPUT_SELECTOR: &str = "input#url";
const SUBMIT_BUTTON_SELECTOR: &str = "#requestSubmit";
const MAX_RETRIES: u8 = 1;

pub async fn scrape_croxy_proxy(
  client: &Client,
  target_url: &str
) -> Result<String, AppError> {
  let start_time = Instant::now();
  info!("Scraping {} with CroxyProxy", target_url);

  let mut html_content = String::new();

  for attempt in 1..=MAX_RETRIES {
    info!("Attempt {}/{}", attempt, MAX_RETRIES);
    match client.goto(CROXY_PROXY_URL).await {
      Ok(_) => {
        let url_input = client
          .find(Locator::Css(URL_INPUT_SELECTOR)).await
          .map_err(|e| AppError::FantocciniError(format!("{e:?}")))?;
        url_input
          .send_keys(target_url).await
          .map_err(|e| AppError::FantocciniError(format!("{e:?}")))?;

        let submit_button = client
          .find(Locator::Css(SUBMIT_BUTTON_SELECTOR)).await
          .map_err(|e| AppError::FantocciniError(format!("{e:?}")))?;
        submit_button
          .click().await
          .map_err(|e| AppError::FantocciniError(format!("{e:?}")))?;

        let page_content = client
          .source().await
          .map_err(|e| AppError::FantocciniError(format!("{e:?}")))?;
        let page_text = page_content.to_lowercase();

        let current_url = client.current_url().await;
        let is_error_url = if let Ok(url_string) = current_url {
          url_string.as_str().contains("/requests?fso=")
        } else {
          false
        };
        let has_error_text =
          page_text.contains("your session has outdated") ||
          page_text.contains("something went wrong");

        if is_error_url || has_error_text {
          warn!("Error detected (URL: {}, Text: {}). Retrying...", is_error_url, has_error_text);
          continue; // Retry
        }

        if page_text.contains("proxy is launching") {
          info!("Proxy launching page detected. Waiting for final redirect...");
          info!("Redirected successfully to: {:?}", client.current_url().await);
        } else {
          info!("Mapped directly to: {:?}", client.current_url().await);
        }

        info!("Waiting for CroxyProxy frame to render...");
        client
          .find(Locator::Css("#__cpsHeaderTab")).await
          .map_err(|e| AppError::FantocciniError(format!("{e:?}")))?;
        info!("CroxyProxy frame rendered.");

        html_content = client
          .source().await
          .map_err(|e| AppError::FantocciniError(format!("{e:?}")))?;
        info!("Retrieved page content.");
        // Success, break out of retry loop
      }
      Err(e) => {
        error!("Attempt {} failed: {:?}", attempt, e);
        if attempt == MAX_RETRIES {
          return Err(AppError::FantocciniError(format!("{e:?}")));
        }
      }
    }
  } // Closing brace for the for loop, correctly indented.

  if html_content.is_empty() {
    return Err(AppError::Other("Failed to retrieve HTML content after all retries.".to_string()));
  }

  let duration = start_time.elapsed().as_millis();
  info!("Total execution time: {} ms", duration);

  Ok(html_content)
}

pub async fn scrape_croxy_proxy_cached(
  client: &Client,
  target_url: &str
) -> Result<String, AppError> {
  let mut conn = get_redis_connection()?;
  let cache_key = format!("scrapeCroxyProxy:{target_url}");

  let cached: Option<String> = redis::cmd("GET").arg(&cache_key).query(&mut conn)?;
  if let Some(html) = cached {
    info!("[scrapeCroxyProxyCached] Returning cached result for {}", target_url);
    return Ok(html);
  }

  let html = scrape_croxy_proxy(client, target_url).await?;
  redis::cmd("SET").arg(&cache_key).arg(&html).arg("EX").arg(3600).query::<()>(&mut conn)?;
  info!("[scrapeCroxyProxyCached] Cached result for {} (1 hour)", target_url);

  Ok(html)
}
