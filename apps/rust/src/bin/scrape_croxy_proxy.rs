use rust_lib::scrape_croxy_proxy::scrape_croxy_proxy;
use rust_lib::headless_chrome::{launch_browser, BrowserPool};
use std::env;
use tracing::{info, error};

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    let args: Vec<String> = env::args().collect();
    let mut headless = false; // Default to non-headless for debugging
    let mut target_url = String::new();

    let mut i = 1;
    while i < args.len() {
        if args[i] == "--headless" {
            if i + 1 < args.len() {
                headless = args[i + 1].parse().unwrap_or(true);
                i += 2;
            } else {
                headless = false;
                i += 1;
            }
        } else if args[i] == "--no-headless" {
            headless = false;
            i += 1;
        } else if target_url.is_empty() {
            target_url = args[i].clone();
            i += 1;
        } else {
            error!("Usage: {} [--headless] [--no-headless] <url>\n  Default: non-headless (visible browser window)", args[0]);
            std::process::exit(1);
        }
    }

    if target_url.is_empty() {
        error!("Usage: {} [--headless] [--no-headless] <url>\n  Default: non-headless (visible browser window)", args[0]);
        std::process::exit(1);
    }
    info!("CLI execution started for URL: {} (headless: {})", target_url, headless);

    let start_time = std::time::Instant::now();

    match launch_browser(headless, None).await {
        Ok(browser) => {
            let browser_pool = BrowserPool::new(browser);
            match scrape_croxy_proxy(&browser_pool, &target_url).await {
                Ok(html) => {
                    println!("{}", html);
                    info!("CLI execution finished successfully.");
                    let duration = start_time.elapsed().as_millis();
                    info!("Total execution time: {} ms", duration);
                }
                Err(e) => {
                    error!("Scraping failed: {:?}", e);
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            error!("Failed to launch browser: {:?}", e);
            std::process::exit(1);
        }
    }
}
