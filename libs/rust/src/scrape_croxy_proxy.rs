// CroxyProxy scraping and caching logic using shared chromiumoxide library and Redis.
// Updated to use shared browser instance.

use crate::headless_chrome::BrowserPool;
use std::time::Instant;
use tracing::{ info, warn, error };
use crate::redis_client::get_redis_connection;
use crate::utils::error::AppError;
use rand::Rng;
use tokio::time;
use url::Url;
use serde_json;

const CROXY_PROXY_URL: &str = "https://www.croxyproxy.com/";
const URL_INPUT_SELECTOR: &str = "input[name='url']";
const SUBMIT_BUTTON_SELECTOR: &str = "button[type='submit'], input[type='submit'], #requestSubmit";
const MAX_RETRIES: u8 = 1;

fn get_random_user_agent() -> String {
  let versions = ["115.0.0.0", "116.0.0.0", "117.0.0.0", "118.0.0.0"];
  let os_options = ["Windows NT 10.0; Win64; x64", "Macintosh; Intel Mac OS X 10_15_7"];
  let mut rng = rand::thread_rng();
  let random_os = os_options[rng.gen_range(0..os_options.len())];
  let random_version = versions[rng.gen_range(0..versions.len())];
  format!(
    "Mozilla/5.0 ({}) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/{} Safari/537.36",
    random_os,
    random_version
  )
}

fn is_ipv4(s: &str) -> bool {
  let parts: Vec<&str> = s.split('.').collect();
  if parts.len() != 4 {
    return false;
  }
  for part in parts {
    if part.parse::<u8>().is_err() {
      return false;
    }
  }
  true
}

fn random_delay() -> u64 {
  rand::thread_rng().gen_range(50..=1000)
}

pub async fn scrape_croxy_proxy(
  chrome: &BrowserPool,
  target_url: &str
) -> Result<String, AppError> {
  let start_time = Instant::now();
  info!("Scraping {} with CroxyProxy", target_url);

  let page = chrome
    .new_page("about:blank").await
    .map_err(|e| AppError::ChromiumoxideError(format!("{e:?}")))?;

  // Set random user agent
  let user_agent = get_random_user_agent();
  page
    .evaluate(
      &format!("Object.defineProperty(navigator, 'userAgent', {{value: '{}'}});", user_agent)
    ).await
    .map_err(|e| AppError::ChromiumoxideError(format!("{e:?}")))?;

  // Set Accept-Language header
  page
    .evaluate("Object.defineProperty(navigator, 'languages', {value: ['en-US', 'en']});").await
    .map_err(|e| AppError::ChromiumoxideError(format!("{e:?}")))?;

  // Small delay to ensure user agent is set
  time::sleep(std::time::Duration::from_millis(random_delay())).await;

  let mut html_content = String::new();

  for attempt in 1..=MAX_RETRIES {
    info!("Attempt {}/{}", attempt, MAX_RETRIES);

    match
      (async {
        // Navigate to croxyproxy first
        info!("Navigating to CroxyProxy...");
        page.tab
          .navigate_to("https://www.croxyproxy.com/")
          .map_err(|e| AppError::ChromiumoxideError(format!("{e:?}")))?;

        // Wait for the page to load completely
        info!("Waiting for page to load completely...");

        // Simple wait for elements to be available
        page
          .find_element(URL_INPUT_SELECTOR).await
          .map_err(|e| AppError::ChromiumoxideError(format!("{e:?}")))?;

        page
          .find_element(SUBMIT_BUTTON_SELECTOR).await
          .map_err(|e| AppError::ChromiumoxideError(format!("{e:?}")))?;

        // Additional wait to ensure page is stable and interactive
        time::sleep(std::time::Duration::from_millis(random_delay())).await;
        info!("Page should now be ready for interaction");
        // Debug: Check if elements exist
        let debug_result = page
          .evaluate(
            "({input: document.querySelector('input[name=\"url\"]') ? 'found' : 'not found', button: document.querySelector('button[type=\"submit\"], input[type=\"submit\"], #requestSubmit') ? 'found' : 'not found', url: window.location.href})"
          ).await
          .map_err(|e| AppError::ChromiumoxideError(format!("{e:?}")))?;
        info!("Debug elements: {:?}", debug_result);

        // Type into the input using chromiumoxide's type helper (more reliable)
        page
          .type_text(URL_INPUT_SELECTOR, target_url)
          .await
          .map_err(|e| AppError::ChromiumoxideError(format!("Typing into input failed: {:?}", e)))?;

        // Small delay to let page process input events triggered by typing
        time::sleep(std::time::Duration::from_millis(100)).await;

        // Wait for the input to be processed
        time::sleep(std::time::Duration::from_millis(200)).await;

        // Small delay to let page process input events triggered by typing
        time::sleep(std::time::Duration::from_millis(100)).await;

        // Wait for the input to be processed
        time::sleep(std::time::Duration::from_millis(200)).await;

        // Verify that input field has the correct value
        let input_value_v = page
          .evaluate(
            &format!(r#"
            const input = document.querySelector('{}');
            input ? input.value : '';
            "#, URL_INPUT_SELECTOR)
          ).await
          .map_err(|e| AppError::ChromiumoxideError(format!("{e:?}")))?;
        let input_value = match input_value_v {
          Some(v) =>
            v
              .as_str()
              .map(|s| s.to_string())
              .unwrap_or_default(),
          None => "".to_string(),
        };
        info!("Input field value: {:?}", input_value);

        // Wait for the submit button to appear
        page
          .find_element(SUBMIT_BUTTON_SELECTOR).await
          .map_err(|e|
            AppError::ChromiumoxideError(format!("Wait for submit button failed: {e:?}"))
          )?;

        // Additional delay to ensure everything is ready
        time::sleep(std::time::Duration::from_millis(random_delay())).await;

        info!("Clicking submit button (using element.click fallback to JS dispatch if needed)...");
        // Try high-level click first
        let click_res = page.click(SUBMIT_BUTTON_SELECTOR).await;
        if let Err(e) = click_res {
          warn!("High-level click failed: {:?}, falling back to dispatching click event via JS", e);
          // Fallback: dispatch click via JS
          let _ = page
            .evaluate(
              r#"
(function () {
  try {
    const sel = 'button[type=\"submit\"], input[type=\"submit\"], #requestSubmit';
    const button = document.querySelector(sel);
    if (!button) return { success: false, message: 'Button not found' };
    button.scrollIntoView({ behavior: 'instant', block: 'center' });
    const evt = new MouseEvent('click', { bubbles: true, cancelable: true, view: window });
    button.dispatchEvent(evt);
    return { success: true, message: 'Dispatched click' };
  } catch (error) {
    return { success: false, message: String(error) };
  }
})();
              "#
            ).await
            .map_err(|e| AppError::ChromiumoxideError(format!("{e:?}")))?;
        } else {
          info!("High-level click succeeded");
        }

        // Wait briefly for the submit to process
        time::sleep(std::time::Duration::from_millis(random_delay())).await;

        // Wait for "Proxy is launching..." page to redirect
        info!("Waiting for proxy launching page to redirect...");
        let proxy_launch_wait = tokio::time::timeout(std::time::Duration::from_secs(10), async {
          loop {
            let title_result = page.evaluate("document.title").await;

            let title = match title_result {
              Ok(Some(v)) =>
                v
                  .as_str()
                  .map(|s| s.to_string())
                  .unwrap_or_default(),
              _ => "".to_string(),
            };

            info!("Current page title: '{}'", title);

            if !title.to_lowercase().contains("proxy is launching") && !title.is_empty() {
              info!("Proxy launching page redirected, title changed to: '{}'", title);
              break;
            }

            time::sleep(std::time::Duration::from_millis(random_delay())).await;
          }
        }).await;

        match proxy_launch_wait {
          Ok(_) => info!("Successfully waited for proxy launch redirect"),
          Err(_) => warn!("Timeout waiting for proxy launch redirect - continuing anyway"),
        }

        // Use wait_for_navigation with timeout instead of manual polling
        info!("Waiting for navigation...");
        let navigation_result = tokio::time::timeout(std::time::Duration::from_secs(15), async {
          // Try to wait for navigation
          page.wait_for_navigation().await
        }).await;

        match navigation_result {
          Ok(Ok(_)) => {
            info!("Navigation completed successfully");
          }
          Ok(Err(e)) => {
            warn!("Navigation wait failed: {:?}, checking page manually", e);

            // Check if URL changed despite navigation error
            if let Ok(Some(current_url)) = page.url().await {
              if current_url != CROXY_PROXY_URL && current_url != "https://www.croxyproxy.com/" {
                info!("URL did change to: {}", current_url);
              } else {
                return Err(AppError::Other("Submit failed - still on homepage".to_string()));
              }
            }
          }
          Err(_) => {
            warn!("Navigation timeout - checking final state");

            // Check if we're on a different page despite timeout
            if let Ok(Some(current_url)) = page.url().await {
              if current_url != CROXY_PROXY_URL && current_url != "https://www.croxyproxy.com/" {
                info!("Navigation completed despite timeout, URL: {}", current_url);
              } else {
                return Err(AppError::Other("Submit timeout - navigation failed".to_string()));
              }
            } else {
              return Err(AppError::Other("Submit timeout - cannot check URL".to_string()));
            }
          }
        }
        let page_content = page
          .content().await
          .map_err(|e| AppError::ChromiumoxideError(format!("{e:?}")))?;
        let page_text = page_content.to_lowercase();

        let current_url = page.url().await;
        let is_error_url = if let Ok(Some(url_string)) = current_url {
          url_string.contains("/requests?fso=")
        } else {
          false
        };
        let has_error_text =
          page_text.contains("your session has outdated") ||
          page_text.contains("something went wrong");

        if is_error_url || has_error_text {
          warn!("Error detected (URL: {}, Text: {}). Retrying...", is_error_url, has_error_text);
          return Err(AppError::Other("Error detected, retrying".to_string()));
        }

        if page_text.contains("proxy is launching") {
          info!("Proxy launching page detected. Waiting for final redirect...");
          // Wait for navigation to complete with timeout
          let nav_result = tokio::time::timeout(
            std::time::Duration::from_secs(10),
            page.wait_for_navigation()
          ).await;

          match nav_result {
            Ok(Ok(_)) => info!("Redirected successfully to: {:?}", page.url().await),
            Ok(Err(e)) => warn!("Navigation failed: {:?}", e),
            Err(_) => warn!("Navigation timeout"),
          }
        } else {
          info!("Mapped directly to: {:?}", page.url().await);
        }

        // Wait for potential IP redirect with timeout
        info!("Waiting for URL to become IP address...");
        let ip_wait_result = tokio::time::timeout(std::time::Duration::from_secs(8), async {
          loop {
            if let Ok(current_url) = page.url().await {
              if let Some(url_str) = current_url {
                if let Ok(parsed) = Url::parse(&url_str) {
                  if let Some(host) = parsed.host_str() {
                    let host_part = host.split(':').next().unwrap_or(host);
                    if is_ipv4(host_part) {
                      info!("IP URL found: {}", url_str);
                      return;
                    }
                  }
                }
              }
            }
            time::sleep(std::time::Duration::from_millis(random_delay())).await;
          }
        }).await;

        match ip_wait_result {
          Ok(_) => {
            info!("IP redirect completed, waiting for network idle...");

            // Wait for network to be idle after IP redirect
            let network_idle_result = tokio::time::timeout(
              std::time::Duration::from_secs(10),
              async {
                // Simple approach: wait for a period with no new requests
                let mut idle_count = 0;
                let required_idle_cycles = 5; // 5 cycles of 500ms each = 2.5 seconds idle

                loop {
                  let start_time = std::time::Instant::now();
                  time::sleep(std::time::Duration::from_millis(random_delay())).await;

                  // Check if page is still loading or if there are pending requests
                  let is_loading = page.tab
                    .evaluate("document.readyState !== 'complete'", true)
                    .map(|result| {
                      if let Some(value) = result.value {
                        value.as_bool().unwrap_or(false)
                      } else {
                        false
                      }
                    })
                    .unwrap_or(false);

                  if !is_loading {
                    idle_count += 1;
                    if idle_count >= required_idle_cycles {
                      info!("Network appears idle (document ready for {} cycles)", required_idle_cycles);
                      break;
                    }
                  } else {
                    idle_count = 0; // Reset counter if still loading
                  }

                  // Additional check: ensure we're not waiting too long per cycle
                  if start_time.elapsed() > std::time::Duration::from_millis(600) {
                    warn!("Cycle took longer than expected, network may be busy");
                    idle_count = 0;
                  }
                }
              }
            ).await;

            match network_idle_result {
              Ok(_) => info!("Network idle achieved"),
              Err(_) => info!("Network idle timeout - continuing anyway"),
            }

            // Additional wait for page to fully load the target content
            info!("Waiting additional time for page content to fully load...");
            time::sleep(std::time::Duration::from_millis(random_delay())).await;

            // Verify we have the target page content, not CroxyProxy itself
            let current_url = page.url().await.unwrap_or_default().unwrap_or_default();
            info!("Final page URL: {}", current_url);

            // Check if we're still on CroxyProxy page instead of the target
            if current_url.contains("croxyproxy.com") && !current_url.contains("/go/") {
              warn!("Still on CroxyProxy main page, waiting longer for redirect...");
              time::sleep(std::time::Duration::from_millis(random_delay())).await;
            }
          }
          Err(_) => {
            info!("IP redirect timeout - continuing with current page");
            // Even without IP redirect, wait a bit for any content to load
            time::sleep(std::time::Duration::from_millis(random_delay())).await;
          }
        }
        html_content = page
          .content().await
          .map_err(|e| AppError::ChromiumoxideError(format!("{e:?}")))?;
        info!("Retrieved page content.");

        Ok(())
      }).await
    {
      Ok(_) => {
        break; // Success
      }
      Err(e) => {
        error!("Attempt {} failed: {:?}", attempt, e);
        if attempt == MAX_RETRIES {
          return Err(e);
        }
        // Continue to next attempt
      }
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
  browser_pool: &BrowserPool,
  target_url: &str
) -> Result<String, AppError> {
  let mut conn = get_redis_connection()?;
  let cache_key = format!("scrapeCroxyProxy:{target_url}");

  let cached: Option<String> = redis::cmd("GET").arg(&cache_key).query(&mut conn)?;
  if let Some(html) = cached {
    info!("[scrapeCroxyProxyCached] Returning cached result for {}", target_url);
    return Ok(html);
  }

  let html = scrape_croxy_proxy(browser_pool, target_url).await?;
  redis::cmd("SET").arg(&cache_key).arg(&html).arg("EX").arg(3600).query::<()>(&mut conn)?;
  info!("[scrapeCroxyProxyCached] Cached result for {} (1 hour)", target_url);

  Ok(html)
}
