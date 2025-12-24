//! Browser pool for managing a single browser with multiple tabs.
//!
//! This pool maintains one headless Chrome instance and provides tabs
//! on-demand for scraping. Tabs are returned to the pool after use.

use chromiumoxide::{Browser, BrowserConfig, Page};
use futures::StreamExt;
use std::sync::Arc;
use tokio::sync::{Mutex, Semaphore};
use tracing::{debug, info, warn};

/// Configuration for the browser pool.
#[derive(Debug, Clone)]
pub struct BrowserPoolConfig {
    /// Maximum number of concurrent tabs
    pub max_tabs: usize,
    /// Chrome/Chromium executable path (None = auto-detect)
    pub chrome_path: Option<String>,
    /// Whether to run headless
    pub headless: bool,
    /// Enable sandbox (disable for Docker)
    pub sandbox: bool,
    /// User agent string
    pub user_agent: Option<String>,
    /// Window dimensions
    pub window_size: Option<(u32, u32)>,
}

impl Default for BrowserPoolConfig {
    fn default() -> Self {
        Self {
            max_tabs: 10,
            chrome_path: None,
            headless: true,
            sandbox: false, // Usually disabled for servers
            user_agent: None,
            window_size: Some((1920, 1080)),
        }
    }
}

/// A pool of browser tabs backed by a single browser instance.
///
/// # Example
///
/// ```ignore
/// use rust::browser::{BrowserPool, BrowserPoolConfig};
///
/// let pool = BrowserPool::new(BrowserPoolConfig::default()).await?;
///
/// // Get a tab from the pool
/// let tab = pool.get_tab().await?;
///
/// // Navigate and scrape
/// tab.goto("https://example.com").await?;
/// let html = tab.content().await?;
///
/// // Tab is automatically returned to the pool when dropped
/// ```
pub struct BrowserPool {
    /// The browser instance (single process)
    browser: Arc<Browser>,
    /// Available (idle) tabs
    available_tabs: Mutex<Vec<Arc<Page>>>,
    /// Semaphore to limit concurrent tabs
    semaphore: Arc<Semaphore>,
    /// Configuration
    config: BrowserPoolConfig,
}

impl BrowserPool {
    /// Create a new browser pool and launch the browser.
    pub async fn new(config: BrowserPoolConfig) -> anyhow::Result<Arc<Self>> {
        info!("🌐 Launching browser pool with max {} tabs", config.max_tabs);

        // Build browser configuration
        let mut browser_config = BrowserConfig::builder();

        if config.headless {
            browser_config = browser_config.with_head();
        }

        if !config.sandbox {
            browser_config = browser_config.no_sandbox();
        }

        if let Some(ref path) = config.chrome_path {
            browser_config = browser_config.chrome_executable(path);
        }

        if let Some((width, height)) = config.window_size {
            browser_config = browser_config.window_size(width, height);
        }

        // User agent set via command line arg if provided
        if let Some(ref ua) = config.user_agent {
            browser_config = browser_config.arg(format!("--user-agent={}", ua));
        }

        // Add common args for better performance
        browser_config = browser_config
            .arg("--disable-gpu")
            .arg("--disable-dev-shm-usage")
            .arg("--disable-setuid-sandbox")
            .arg("--no-first-run")
            .arg("--no-zygote")
            .arg("--disable-extensions")
            .arg("--disable-background-networking")
            .arg("--disable-background-timer-throttling")
            .arg("--disable-backgrounding-occluded-windows")
            .arg("--disable-breakpad")
            .arg("--disable-component-extensions-with-background-pages")
            .arg("--disable-ipc-flooding-protection")
            .arg("--disable-renderer-backgrounding")
            .arg("--enable-features=NetworkService,NetworkServiceInProcess")
            .arg("--force-color-profile=srgb");

        let browser_config = browser_config.build().map_err(|e| {
            anyhow::anyhow!("Failed to build browser config: {}", e)
        })?;

        // Launch browser
        let (browser, mut handler) = Browser::launch(browser_config).await.map_err(|e| {
            anyhow::anyhow!("Failed to launch browser: {}", e)
        })?;

        // Spawn browser event handler
        tokio::spawn(async move {
            while let Some(event) = handler.next().await {
                debug!("Browser event: {:?}", event);
            }
        });

        info!("✅ Browser launched successfully");

        let pool = Arc::new(Self {
            browser: Arc::new(browser),
            available_tabs: Mutex::new(Vec::new()),
            semaphore: Arc::new(Semaphore::new(config.max_tabs)),
            config,
        });

        Ok(pool)
    }

    /// Get a tab from the pool.
    ///
    /// This will reuse an existing idle tab or create a new one.
    /// The returned `PooledTab` automatically returns to the pool when dropped.
    pub async fn get_tab(self: &Arc<Self>) -> anyhow::Result<PooledTab> {
        // Acquire semaphore permit (limits concurrent tabs)
        let permit = self.semaphore.clone().acquire_owned().await?;

        // Try to get an existing tab
        let page = {
            let mut tabs = self.available_tabs.lock().await;
            tabs.pop()
        };

        let page = match page {
            Some(page) => {
                debug!("Reusing existing tab");
                // Navigate to blank page to reset state
                if let Err(e) = page.goto("about:blank").await {
                    warn!("Failed to reset tab, creating new one: {}", e);
                    self.create_new_tab().await?
                } else {
                    page
                }
            }
            None => {
                debug!("Creating new tab");
                self.create_new_tab().await?
            }
        };

        Ok(PooledTab {
            page,
            pool: Arc::clone(self),
            _permit: permit,
        })
    }

    /// Create a new tab.
    async fn create_new_tab(&self) -> anyhow::Result<Arc<Page>> {
        let page = self.browser.new_page("about:blank").await.map_err(|e| {
            anyhow::anyhow!("Failed to create new tab: {}", e)
        })?;

        // Set default timeout
        // page.set_default_timeout(Duration::from_secs(30));

        Ok(Arc::new(page))
    }

    /// Return a tab to the pool for reuse.
    async fn return_tab(&self, page: Arc<Page>) {
        let mut tabs = self.available_tabs.lock().await;

        // Only keep up to max_tabs in the pool
        if tabs.len() < self.config.max_tabs {
            tabs.push(page);
            debug!("Tab returned to pool (available: {})", tabs.len());
        } else {
            debug!("Pool full, discarding tab");
            // Tab will be dropped and closed
        }
    }

    /// Get the number of available (idle) tabs in the pool.
    pub async fn available_count(&self) -> usize {
        self.available_tabs.lock().await.len()
    }

    /// Close the browser and all tabs.
    pub async fn close(&self) -> anyhow::Result<()> {
        info!("Closing browser pool");
        // Clear available tabs
        self.available_tabs.lock().await.clear();
        // Browser will be closed when dropped
        Ok(())
    }
}

/// A tab borrowed from the pool.
///
/// When dropped, the tab is automatically returned to the pool.
pub struct PooledTab {
    page: Arc<Page>,
    pool: Arc<BrowserPool>,
    _permit: tokio::sync::OwnedSemaphorePermit,
}

impl PooledTab {
    /// Navigate to a URL.
    pub async fn goto(&self, url: &str) -> anyhow::Result<()> {
        self.page.goto(url).await.map_err(|e| {
            anyhow::anyhow!("Failed to navigate to {}: {}", url, e)
        })?;
        Ok(())
    }

    /// Wait for navigation to complete.
    pub async fn wait_for_navigation(&self) -> anyhow::Result<()> {
        self.page.wait_for_navigation().await.map_err(|e| {
            anyhow::anyhow!("Navigation timeout: {}", e)
        })?;
        Ok(())
    }

    /// Get the page content (HTML).
    pub async fn content(&self) -> anyhow::Result<String> {
        self.page.content().await.map_err(|e| {
            anyhow::anyhow!("Failed to get page content: {}", e)
        })
    }

    /// Execute JavaScript and return the result.
    pub async fn evaluate<T: serde::de::DeserializeOwned>(&self, expression: &str) -> anyhow::Result<T> {
        self.page.evaluate(expression).await.map_err(|e| {
            anyhow::anyhow!("Failed to evaluate JS: {}", e)
        })?.into_value().map_err(|e| {
            anyhow::anyhow!("Failed to convert JS result: {}", e)
        })
    }

    /// Wait for a selector to appear.
    pub async fn wait_for_selector(&self, selector: &str) -> anyhow::Result<()> {
        self.page.find_element(selector).await.map_err(|e| {
            anyhow::anyhow!("Selector '{}' not found: {}", selector, e)
        })?;
        Ok(())
    }

    /// Click an element by selector.
    pub async fn click(&self, selector: &str) -> anyhow::Result<()> {
        let element = self.page.find_element(selector).await.map_err(|e| {
            anyhow::anyhow!("Element '{}' not found: {}", selector, e)
        })?;
        element.click().await.map_err(|e| {
            anyhow::anyhow!("Failed to click '{}': {}", selector, e)
        })?;
        Ok(())
    }

    /// Type text into an element.
    pub async fn type_text(&self, selector: &str, text: &str) -> anyhow::Result<()> {
        let element = self.page.find_element(selector).await.map_err(|e| {
            anyhow::anyhow!("Element '{}' not found: {}", selector, e)
        })?;
        element.type_str(text).await.map_err(|e| {
            anyhow::anyhow!("Failed to type into '{}': {}", selector, e)
        })?;
        Ok(())
    }

    /// Take a screenshot as PNG bytes.
    pub async fn screenshot(&self) -> anyhow::Result<Vec<u8>> {
        self.page.screenshot(
            chromiumoxide::page::ScreenshotParams::builder()
                .full_page(true)
                .build()
        ).await.map_err(|e| {
            anyhow::anyhow!("Failed to take screenshot: {}", e)
        })
    }

    /// Get the current URL.
    pub async fn url(&self) -> anyhow::Result<String> {
        self.page.url().await.map_err(|e| {
            anyhow::anyhow!("Failed to get URL: {}", e)
        }).map(|u| u.map(|url| url.to_string()).unwrap_or_default())
    }

    /// Get access to the underlying Page for advanced operations.
    pub fn page(&self) -> &Page {
        &self.page
    }
}

impl Drop for PooledTab {
    fn drop(&mut self) {
        // Return the tab to the pool
        let page = Arc::clone(&self.page);
        let pool = Arc::clone(&self.pool);

        tokio::spawn(async move {
            pool.return_tab(page).await;
        });
    }
}

// Global browser pool instance
use once_cell::sync::OnceCell;

static BROWSER_POOL: OnceCell<Arc<BrowserPool>> = OnceCell::new();

/// Initialize the global browser pool.
/// Call this once at application startup.
pub async fn init_browser_pool(config: BrowserPoolConfig) -> anyhow::Result<()> {
    let pool = BrowserPool::new(config).await?;
    BROWSER_POOL.set(pool).map_err(|_| anyhow::anyhow!("Browser pool already initialized"))?;
    Ok(())
}

/// Get the global browser pool.
/// Returns None if not initialized.
pub fn get_browser_pool() -> Option<Arc<BrowserPool>> {
    BROWSER_POOL.get().cloned()
}
