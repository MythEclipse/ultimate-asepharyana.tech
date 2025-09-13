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
    info!("Successfully created new tab on attempt {}", attempt);

    let result = match tab.navigate_to(CROXY_PROXY_URL) {
      Ok(_) => {
        info!("Successfully navigated to CroxyProxy URL.");
        info!("Waiting for initial navigation to complete.");
        (&*tab).wait_until_navigated()?;
        info!("Initial navigation completed.");

        // Set URL input value
        info!("Typing target URL '{}' into input field with selector '{}'", target_url, URL_INPUT_SELECTOR);
        (&*tab).find_element(URL_INPUT_SELECTOR)?.type_into(target_url)?;
        info!("Successfully typed target URL.");

        // Find submit button and click
        info!("Clicking submit button with selector '{}'", SUBMIT_BUTTON_SELECTOR);
        (&*tab).find_element(SUBMIT_BUTTON_SELECTOR)?.click()?;
        info!("Successfully clicked submit button.");

        // Wait for navigation
        info!("Waiting for navigation after submit button click.");
        (&*tab).wait_until_navigated()?;
        info!("Navigation after submit button click completed.");

        info!("Getting page content.");
        let page_content = (&*tab).get_content()?;
        info!("Successfully retrieved page content.");
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

            let start_wait_time = Instant::now();
            let max_wait_duration = std::time::Duration::from_secs(30); // 30 seconds
            let poll_interval = std::time::Duration::from_millis(500); // 500ms

            loop {
                tokio::time::sleep(poll_interval).await;
                let current_page_content = (&*tab).get_content()?;
                let current_page_text = current_page_content.to_lowercase();

                if !current_page_text.contains("proxy is launching") {
                    info!("Proxy launching page disappeared. Content loaded.");
                    html_content = current_page_content;
                    break;
                }

                if start_wait_time.elapsed() > max_wait_duration {
                    warn!("Timeout waiting for CroxyProxy to load target page after {} seconds.", max_wait_duration.as_secs());
                    html_content = current_page_content; // Get whatever content is available
                    break;
                }
                info!("Still waiting for CroxyProxy to load target page...");
            }
          } else {
            info!("Mapped directly to: {:?}", tab.get_url());
            html_content = (&*tab).get_content()?;
          }

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
    info!("Closing tab.");
    (&*tab).close(true).map_err(|e| AppError::Other(format!("Failed to close tab: {:?}", e)))?;
    info!("Tab closed.");
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
