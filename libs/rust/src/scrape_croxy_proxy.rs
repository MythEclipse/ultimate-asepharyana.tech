// CroxyProxy scraping and caching logic using shared chromiumoxide library and Redis.
// Updated to use shared browser instance.

use std::time::Instant;
use tracing::{ info, warn, error };
use crate::redis_client::get_redis_connection;
use crate::utils::error::AppError;
use headless_chrome::Browser;
use std::sync::Arc;
use tokio::sync::Mutex;

const CROXY_PROXY_URL: &str = "https://www.croxyproxy.com/";
const URL_INPUT_SELECTOR: &str = "input#url";
const SUBMIT_BUTTON_SELECTOR: &str = "#requestSubmit";
const MAX_RETRIES: u8 = 3;

pub async fn scrape_croxy_proxy(
  browser: &Arc<Mutex<Browser>>,
  target_url: &str
) -> Result<String, AppError> {
  let start_time = Instant::now();
  info!("Scraping {} with CroxyProxy", target_url);

  let mut html_content = String::new();

  for attempt in 1..=MAX_RETRIES {
    info!("Attempt {}/{}", attempt, MAX_RETRIES);

    // Add a small delay before creating a new page
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

    let browser_lock = browser.lock().await;
    let tab = browser_lock.new_tab().map_err(|e| {
      error!("Failed to create new tab on attempt {}: {:?}", attempt, e);
      AppError::Other(format!("Failed to create new tab: {:?}", e))
    })?;

    let result = match tab.navigate_to(CROXY_PROXY_URL) {
      Ok(_) => {
        (&*tab).wait_until_navigated()?;

        // Set URL input value
        (&*tab).find_element(URL_INPUT_SELECTOR)?.type_into(target_url)?;

        // Find submit button and click
        (&*tab).find_element(SUBMIT_BUTTON_SELECTOR)?.click()?;

        // Wait for navigation
        (&*tab).wait_until_navigated()?;

        let page_content = (&*tab).get_content()?;
        let page_text = page_content.to_lowercase();

        let current_url = tab.get_url().to_string();
        let is_error_url = current_url.contains("/requests?fso=");
        let has_error_text =
          page_text.contains("your session has outdated") ||
          page_text.contains("something went wrong");

        let ok_result = if is_error_url || has_error_text {
          warn!("Error detected (URL: {}, Text: {}). Retrying...", is_error_url, has_error_text);
          Err(AppError::Other("Error detected, retrying.".to_string())) // Indicate failure for retry
        } else {
          if page_text.contains("proxy is launching") {
            info!("Proxy launching page detected. Waiting for final redirect...");
            info!("Redirected successfully to: {:?}", tab.get_url());
          } else {
            info!("Mapped directly to: {:?}", tab.get_url());
          }

          info!("Waiting for CroxyProxy frame to render...");
          // headless_chrome automatically waits for content to load, so a fixed sleep might not be necessary
          // but keeping a small one for complex pages if needed.
          tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
          info!("CroxyProxy frame rendered.");

          html_content = (&*tab).get_content()?;
          info!("Retrieved page content.");
          Ok(()) // Success
        };
        ok_result
      },
      Err(e) => {
        error!("Attempt {} failed: {:?}", attempt, e);
        Err(AppError::Other(format!("Attempt failed: {:?}", e))) // Use specific AppError variant
      },
    };
    // Close the tab after each attempt to prevent resource leaks
    (&*tab).close(true).map_err(|e| AppError::Other(format!("Failed to close tab: {:?}", e)))?;
    if result.is_ok() {
      break; // Break out of retry loop on success
    }
  }

  if html_content.is_empty() {
    return Err(AppError::Other("Failed to retrieve HTML content after all retries.".to_string()));
  }

  let duration = start_time.elapsed().as_millis();
  info!("Total execution time: {} ms", duration);

  Ok(html_content)
}

pub async fn scrape_croxy_proxy_cached(
  browser: &Arc<Mutex<Browser>>,
  target_url: &str
) -> Result<String, AppError> {
  let mut conn = get_redis_connection()?;
  let cache_key = format!("scrapeCroxyProxy:{target_url}");

  let cached: Option<String> = redis::cmd("GET").arg(&cache_key).query(&mut conn)?;
  if let Some(html) = cached {
    info!("[scrapeCroxyProxyCached] Returning cached result for {}", target_url);
    return Ok(html);
  }

  let html = scrape_croxy_proxy(browser, target_url).await?;
  redis::cmd("SET").arg(&cache_key).arg(&html).arg("EX").arg(3600).query::<()>(&mut conn)?;
  info!("[scrapeCroxyProxyCached] Cached result for {} (1 hour)", target_url);

  Ok(html)
}
