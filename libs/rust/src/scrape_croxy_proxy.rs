// CroxyProxy scraping and caching logic using headless_chrome and Redis.
// Updated for latest headless_chrome API.

use headless_chrome::Browser;
use std::time::Instant;
use tracing::{info, warn, error};
use crate::redis_client::get_redis_connection;
use crate::utils::error::AppError;

const CROXY_PROXY_URL: &str = "https://www.croxyproxy.com/";
const URL_INPUT_SELECTOR: &str = "input#url";
const SUBMIT_BUTTON_SELECTOR: &str = "#requestSubmit";
const MAX_RETRIES: u8 = 1;

// Helper function to simulate getRandomUserAgent from TS
fn get_random_user_agent() -> String {
    let versions = ["115.0.0.0", "116.0.0.0", "117.0.0.0", "118.0.0.0"];
    let os = [
        "Windows NT 10.0; Win64; x64",
        "Macintosh; Intel Mac OS X 10_15_7",
    ];
    let random_os = os[rand::random::<usize>() % os.len()];
    let random_version = versions[rand::random::<usize>() % versions.len()];
    format!(
        "Mozilla/5.0 ({}) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/{} Safari/537.36",
        random_os, random_version
    )
}

pub async fn scrape_croxy_proxy(target_url: &str) -> Result<String, AppError> {
    let start_time = Instant::now();
    info!("Scraping {} with CroxyProxy", target_url);

    let browser = Browser::default().map_err(|e| AppError::HeadlessChromeError(format!("{:?}", e)))?;
    let tab = browser.new_tab().map_err(|e| AppError::HeadlessChromeError(format!("{:?}", e)))?;

    let mut html_content = String::new();

    for attempt in 1..=MAX_RETRIES {
        info!("Attempt {}/{}", attempt, MAX_RETRIES);
        match tab.navigate_to(CROXY_PROXY_URL) {
            Ok(_) => {
                tab.wait_for_element(URL_INPUT_SELECTOR)
                    .map_err(|e| AppError::HeadlessChromeError(format!("{:?}", e)))?;

                let input = tab.find_element(URL_INPUT_SELECTOR)
                    .map_err(|e| AppError::HeadlessChromeError(format!("{:?}", e)))?;
                input.type_into(target_url)
                    .map_err(|e| AppError::HeadlessChromeError(format!("{:?}", e)))?;

                let submit_btn = tab.find_element(SUBMIT_BUTTON_SELECTOR)
                    .map_err(|e| AppError::HeadlessChromeError(format!("{:?}", e)))?;
                submit_btn.click()
                    .map_err(|e| AppError::HeadlessChromeError(format!("{:?}", e)))?;

                // Wait for navigation after form submission
                tab.wait_until_navigated()
                    .map_err(|e| AppError::HeadlessChromeError(format!("{:?}", e)))?;

                let current_url = tab.get_url();
                // Use wait_for_element("body")?.get_inner_html()? to get HTML content
                let page_content = tab.wait_for_element("body")
                    .and_then(|el| {
                        let js_result = el.call_js_fn("function() { return this.innerHTML; }", false);
                        match js_result {
                            Ok(val) => {
                                // val.value is Option<serde_json::Value>, need to handle both Some(String) and fallback to to_string()
                                info!("Debug: val.value type: {:?}", val.value);
                                let html = match val.value {
                                    Some(serde_json::Value::String(ref s)) => {
                                        info!("Debug: Matched Some(String): {}", s);
                                        s.clone()
                                    },
                                    Some(ref v) => {
                                        info!("Debug: Matched Some(non-String): {:?}", v);
                                        v.to_string()
                                    },
                                    None => {
                                        info!("Debug: Matched None");
                                        String::new()
                                    }
                                };
                                info!("Debug: innerHTML extracted: {}", &html[..html.len().min(200)]);
                                Ok(html)
                            },
                            Err(e) => {
                                error!("Debug: call_js_fn failed: {:?}", e);
                                Err(e)
                            }
                        }
                    })
                    .map_err(|e| AppError::HeadlessChromeError(format!("{:?}", e)))?;
                let page_text = page_content.to_lowercase();

                let is_error_url = current_url.contains("/requests?fso=");
                let has_error_text = page_text.contains("your session has outdated") || page_text.contains("something went wrong");

                if is_error_url || has_error_text {
                    warn!("Error detected (URL: {}, Text: {}). Retrying...", is_error_url, has_error_text);
                    continue; // Retry
                }

                if page_text.contains("proxy is launching") {
                    info!("Proxy launching page detected. Waiting for final redirect...");
                    tab.wait_until_navigated()
                        .map_err(|e| AppError::HeadlessChromeError(format!("{:?}", e)))?;
                    info!("Redirected successfully to: {}", tab.get_url());
                } else {
                    info!("Mapped directly to: {}", tab.get_url());
                }

                info!("Waiting for CroxyProxy frame to render...");
                tab.wait_for_element("#__cpsHeaderTab")
                    .map_err(|e| AppError::HeadlessChromeError(format!("{:?}", e)))?;
                info!("CroxyProxy frame rendered.");

                html_content = tab.wait_for_element("body")
                    .and_then(|el| {
                        let js_result = el.call_js_fn("function() { return this.innerHTML; }", false);
                        match js_result {
                            Ok(val) => {
                                // val.value is Option<serde_json::Value>, need to handle both Some(String) and fallback to to_string()
                                info!("Debug: val.value type: {:?}", val.value);
                                let html = match val.value {
                                    Some(serde_json::Value::String(ref s)) => {
                                        info!("Debug: Matched Some(String): {}", s);
                                        s.clone()
                                    },
                                    Some(ref v) => {
                                        info!("Debug: Matched Some(non-String): {:?}", v);
                                        v.to_string()
                                    },
                                    None => {
                                        info!("Debug: Matched None");
                                        String::new()
                                    }
                                };
                                info!("Debug: innerHTML extracted: {}", &html[..html.len().min(200)]);
                                Ok(html)
                            },
                            Err(e) => {
                                error!("Debug: call_js_fn failed: {:?}", e);
                                Err(e)
                            }
                        }
                    })
                    .map_err(|e| AppError::HeadlessChromeError(format!("{:?}", e)))?;
                info!("Retrieved page content.");
                break; // Success, break out of retry loop
            },
            Err(e) => {
                error!("Attempt {} failed: {:?}", attempt, e);
                if attempt == MAX_RETRIES {
                    return Err(AppError::HeadlessChromeError(format!("{:?}", e)));
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
    let cache_key = format!("scrapeCroxyProxy:{}", target_url);

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
