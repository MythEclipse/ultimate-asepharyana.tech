// CroxyProxy scraping and caching logic using shared chromiumoxide library and Redis.
// Updated to use shared browser instance.

use std::time::Instant;
use tracing::{ info, warn, error };
use crate::redis_client::get_redis_connection;
use crate::utils::error::AppError;
use chromiumoxide::Browser;
use std::sync::Arc;
use tokio::sync::Mutex;

const CROXY_PROXY_URL: &str = "https://www.croxyproxy.com/";
const URL_INPUT_SELECTOR: &str = "input#url";
const SUBMIT_BUTTON_SELECTOR: &str = "#requestSubmit";
const MAX_RETRIES: u8 = 1;

pub async fn scrape_croxy_proxy(
  browser: &Arc<Mutex<Browser>>,
  target_url: &str
) -> Result<String, AppError> {
  let start_time = Instant::now();
  info!("Scraping {} with CroxyProxy", target_url);

  let mut html_content = String::new();

  for attempt in 1..=MAX_RETRIES {
    info!("Attempt {}/{}", attempt, MAX_RETRIES);

    // Create a new page for this attempt
    let page = browser
      .lock().await
      .new_page("about:blank").await
      .map_err(|e| AppError::Other(format!("Failed to create page: {:?}", e)))?;

    let result = match page.goto(CROXY_PROXY_URL).await {
      Ok(_) => {
        // Wait for page to load
        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;

        // Set URL input value using JavaScript
        page
          .evaluate(
            format!(
              "document.querySelector('{}').value = '{}'",
              URL_INPUT_SELECTOR,
              target_url.replace("'", "\\'")
            )
          ).await
          .map_err(|e| AppError::Other(format!("Failed to set URL input value: {:?}", e)))?;

        // Find submit button and click
        let submit_button = page
          .find_element(SUBMIT_BUTTON_SELECTOR).await
          .map_err(|e| AppError::Other(format!("Failed to find submit button: {:?}", e)))?;

        submit_button
          .click().await
          .map_err(|e| AppError::Other(format!("Failed to click submit: {:?}", e)))?;

        // Wait for navigation
        tokio::time::sleep(tokio::time::Duration::from_millis(2000)).await;

        let page_content = page
          .content().await
          .map_err(|e| AppError::Other(format!("Failed to get page content: {:?}", e)))?;
        let page_text = page_content.to_lowercase();

        let current_url = page
          .url().await
          .map_err(|e| AppError::Other(format!("Failed to get current URL: {:?}", e)))?;
        let is_error_url = current_url.as_ref().map_or(false, |url| url.contains("/requests?fso="));
        let has_error_text =
          page_text.contains("your session has outdated") ||
          page_text.contains("something went wrong");

        if is_error_url || has_error_text {
          warn!("Error detected (URL: {}, Text: {}). Retrying...", is_error_url, has_error_text);
          Err(AppError::Other("Error detected, retrying.".to_string())) // Indicate failure for retry
        } else {
          if page_text.contains("proxy is launching") {
            info!("Proxy launching page detected. Waiting for final redirect...");
            info!("Redirected successfully to: {:?}", page.url().await);
          } else {
            info!("Mapped directly to: {:?}", page.url().await);
          }

          info!("Waiting for CroxyProxy frame to render...");
          // Wait for the frame to appear
          tokio::time::sleep(tokio::time::Duration::from_millis(3000)).await;
          info!("CroxyProxy frame rendered.");

          html_content = page
            .content().await
            .map_err(|e| AppError::Other(format!("Failed to get final content: {:?}", e)))?;
          info!("Retrieved page content.");
          Ok(()) // Success
        }
      }
      Err(e) => {
        error!("Attempt {} failed: {:?}", attempt, e);
        Err(AppError::Other(format!("Attempt failed: {:?}", e))) // Indicate failure for retry
      }
    }; // This closes the match statement, aligned with 'let result = match'

    // Close the page after each attempt to prevent resource leaks
    let _ = page.close().await;

    if result.is_ok() {
      break; // Break out of retry loop on success
    }
  } // This closes the for loop

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
