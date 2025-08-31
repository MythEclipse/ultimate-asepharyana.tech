use headless_chrome::{Browser, LaunchOptions};
use std::error::Error;
use std::time::{Duration, Instant};
use tracing::{info, warn, error};
use crate::redis_client::get_redis_connection;
use crate::utils::http::is_internet_baik_block_page;

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

pub async fn scrape_croxy_proxy(target_url: &str) -> Result<String, Box<dyn Error>> {
    let start_time = Instant::now();
    info!("Scraping {} with CroxyProxy", target_url);

    let browser = Browser::new(LaunchOptions::default().with_headless(true))?;
    let tab = browser.new_tab()?;

    let mut html_content = String::new();

    for attempt in 1..=MAX_RETRIES {
        info!("Attempt {}/{}", attempt, MAX_RETRIES);
        match tab.navigate_to(CROXY_PROXY_URL) {
            Ok(_) => {
                tab.wait_for_element_with_custom_timeout(URL_INPUT_SELECTOR, Duration::from_secs(30))?;
                tab.type_str(URL_INPUT_SELECTOR, target_url)?;
                tab.click(SUBMIT_BUTTON_SELECTOR)?;

                // Wait for navigation after form submission
                tab.wait_for_navigation()?;

                let current_url = tab.get_url();
                let page_content = tab.get_content()?;
                let page_text = page_content.to_lowercase();

                let is_error_url = current_url.contains("/requests?fso=");
                let has_error_text = page_text.contains("your session has outdated") || page_text.contains("something went wrong");

                if is_error_url || has_error_text {
                    warn!("Error detected (URL: {}, Text: {}). Retrying...", is_error_url, has_error_text);
                    continue; // Retry
                }

                if page_text.contains("proxy is launching") {
                    info!("Proxy launching page detected. Waiting for final redirect...");
                    tab.wait_for_navigation_with_timeout(Duration::from_secs(120))?;
                    info!("Redirected successfully to: {}", tab.get_url());
                } else {
                    info!("Mapped directly to: {}", tab.get_url());
                }

                info!("Waiting for CroxyProxy frame to render...");
                tab.wait_for_element_with_custom_timeout("#__cpsHeaderTab", Duration::from_secs(30))?;
                info!("CroxyProxy frame rendered.");

                html_content = tab.get_content()?;
                info!("Retrieved page content.");
                break; // Success, break out of retry loop
            },
            Err(e) => {
                error!("Attempt {} failed: {:?}", attempt, e);
                if attempt == MAX_RETRIES {
                    return Err(Box::new(e)); // Return the last error
                }
            }
        }
    }

    if html_content.is_empty() {
        return Err("Failed to retrieve HTML content after all retries.".into());
    }

    let duration = start_time.elapsed().as_millis();
    info!("Total execution time: {} ms", duration);

    Ok(html_content)
}

pub async fn scrape_croxy_proxy_cached(target_url: &str) -> Result<String, Box<dyn Error>> {
    let mut conn = get_redis_connection()?;
    let cache_key = format!("scrapeCroxyProxy:{}", target_url);

    let cached: Option<String> = redis::cmd("GET").arg(&cache_key).query(&mut conn)?;
    if let Some(html) = cached {
        info!("[scrapeCroxyProxyCached] Returning cached result for {}", target_url);
        return Ok(html);
    }

    let html = scrape_croxy_proxy(target_url).await?;
    redis::cmd("SET").arg(&cache_key).arg(&html).arg("EX").arg(3600).query(&mut conn)?;
    info!("[scrapeCroxyProxyCached] Cached result for {} (1 hour)", target_url);

    Ok(html)
}
