// CroxyProxy scraping and caching logic using shared chromiumoxide library and Redis.
// Updated to use shared browser instance.

use crate::chromiumoxide::{ BrowserConfig, BrowserPool };
use std::time::Instant;
use tracing::{ info, warn, error };
use crate::redis_client::get_redis_connection;
use crate::utils::error::AppError;

const CROXY_PROXY_URL: &str = "https://www.croxyproxy.com/";
const URL_INPUT_SELECTOR: &str = "input#url";
const SUBMIT_BUTTON_SELECTOR: &str = "#requestSubmit";
const MAX_RETRIES: u8 = 1;

pub async fn scrape_croxy_proxy(target_url: &str) -> Result<String, AppError> {
  let start_time = Instant::now();
  info!("Scraping {} with CroxyProxy", target_url);

  let config = BrowserConfig::default();
  let chrome = BrowserPool::new(config).await.map_err(|e|
    AppError::ChromiumoxideError(format!("{e:?}"))
  )?;
  let tab = chrome
    .get_tab_manager().await
    .map_err(|e| AppError::ChromiumoxideError(format!("{e:?}")))?;

  let mut html_content = String::new();

  for attempt in 1..=MAX_RETRIES {
    info!("Attempt {}/{}", attempt, MAX_RETRIES);
    match tab.navigate(CROXY_PROXY_URL).await {
      Ok(_) => {
        tab
          .wait_for_element(URL_INPUT_SELECTOR).await
          .map_err(|e| AppError::ChromiumoxideError(format!("{e:?}")))?;

        // There is no direct equivalent of find_element and type_into in the new API
        // We will use evaluate_script to achieve the same
        tab
          .evaluate_script(
            &format!(
              r#"document.querySelector('{}').value = '{}';"#,
              URL_INPUT_SELECTOR,
              target_url
            )
          ).await
          .map_err(|e| AppError::ChromiumoxideError(format!("{e:?}")))?;

        tab
          .evaluate_script(
            &format!(r#"document.querySelector('{}').click();"#, SUBMIT_BUTTON_SELECTOR)
          ).await
          .map_err(|e| AppError::ChromiumoxideError(format!("{e:?}")))?;

        // Wait for navigation after form submission
        // The new API's navigate function already handles waiting for navigation
        // We just need to get the content after the click
        let page_content = tab
          .get_content().await
          .map_err(|e| AppError::ChromiumoxideError(format!("{e:?}")))?;
        let page_text = page_content.to_lowercase();

        let current_url = tab.page().url().await; // This returns Result<Option<String>, BrowserError>
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
          continue; // Retry
        }

        if page_text.contains("proxy is launching") {
          info!("Proxy launching page detected. Waiting for final redirect...");
          // The new API's navigate function already handles waiting for navigation, so
          // we just log the current URL.
          info!("Redirected successfully to: {:?}", tab.page().url().await);
        } else {
          info!("Mapped directly to: {:?}", tab.page().url().await);
        }

        info!("Waiting for CroxyProxy frame to render...");
        tab
          .wait_for_element("#__cpsHeaderTab").await
          .map_err(|e| AppError::ChromiumoxideError(format!("{e:?}")))?;
        info!("CroxyProxy frame rendered.");

        html_content = tab
          .get_content().await
          .map_err(|e| AppError::ChromiumoxideError(format!("{e:?}")))?;
        info!("Retrieved page content.");
        break; // Success, break out of retry loop
      }
      Err(e) => {
        error!("Attempt {} failed: {:?}", attempt, e);
        if attempt == MAX_RETRIES {
          return Err(AppError::ChromiumoxideError(format!("{e:?}")));
        }
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

pub async fn scrape_croxy_proxy_cached(target_url: &str) -> Result<String, AppError> {
  let mut conn = get_redis_connection()?;
  let cache_key = format!("scrapeCroxyProxy:{target_url}");

  let cached: Option<String> = redis::cmd("GET").arg(&cache_key).query(&mut conn)?;
  if let Some(html) = cached {
    info!("[scrapeCroxyProxyCached] Returning cached result for {}", target_url);
    return Ok(html);
  }

  let html = scrape_croxy_proxy(target_url).await?;
  redis::cmd("SET").arg(&cache_key).arg(&html).arg("EX").arg(3600).query::<()>(&mut conn)?;
  info!("[scrapeCroxyProxyCached] Cached result for {} (1 hour)", target_url);

  Ok(html)
}
