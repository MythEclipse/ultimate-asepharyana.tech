//! Browser pool management with shared global browser instance

use crate::chromiumoxide::{
  config::{ BrowserConfig, ProxyConfig },
  error::{ BrowserError, BrowserResult },
  stealth::StealthManager,
  tab_manager::TabManager,
};
use chromiumoxide::{ Browser, BrowserConfig as ChromiumConfig };
use chromiumoxide::browser::BrowserConfigBuilder;
use once_cell::sync::Lazy;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Global shared browser instance
static GLOBAL_BROWSER: Lazy<Arc<Mutex<Option<Arc<Browser>>>>> = Lazy::new(|| Arc::new(Mutex::new(None)));
static GLOBAL_CONFIG: Lazy<Arc<Mutex<Option<BrowserConfig>>>> = Lazy::new(|| Arc::new(Mutex::new(None)));
static GLOBAL_STEALTH: Lazy<Arc<Mutex<Option<StealthManager>>>> = Lazy::new(|| Arc::new(Mutex::new(None)));

/// Browser pool that manages access to the global shared browser instance
pub struct BrowserPool {
  config: BrowserConfig,
}

impl BrowserPool {
  /// Create a new browser pool that uses the global shared browser instance
  pub async fn new(config: BrowserConfig) -> BrowserResult<Self> {
    // Initialize the global browser if not already done
    Self::ensure_global_browser_initialized(&config).await?;

    Ok(Self { config })
  }

  /// Ensure the global browser is initialized with the given config
  async fn ensure_global_browser_initialized(config: &BrowserConfig) -> BrowserResult<()> {
    let mut global_browser = GLOBAL_BROWSER.lock().await;
    let mut global_config = GLOBAL_CONFIG.lock().await;
    let mut global_stealth = GLOBAL_STEALTH.lock().await;

    // If browser is not initialized, create it
    if global_browser.is_none() {
      let browser = Self::create_global_browser(config).await?;
      *global_browser = Some(Arc::new(browser));
      *global_config = Some(config.clone());

      let stealth_manager = if config.stealth_enabled {
        Some(StealthManager::new(Default::default()))
      } else {
        None
      };
      *global_stealth = stealth_manager;

      tracing::info!("Global shared browser instance initialized");
    } else {
      tracing::debug!("Global shared browser instance already initialized");
    }

    Ok(())
  }

  /// Create the global browser instance
  async fn create_global_browser(config: &BrowserConfig) -> BrowserResult<Browser> {
    let mut chrome_config_builder = ChromiumConfig::builder();

    // Apply chrome arguments
    for arg in &config.chrome_args {
        chrome_config_builder = chrome_config_builder.arg(arg);
    }

    // Apply proxy settings if configured
    if let Some(proxy) = &config.proxy {
        chrome_config_builder = Self::apply_proxy_config(chrome_config_builder, proxy);
    }

    let (browser, _handler) = Browser::launch(chrome_config_builder.build().unwrap()).await
        .map_err(|e| BrowserError::BrowserStartupError(format!("Global browser: {}", e)))?;

    Ok(browser)
  }

  /// Apply proxy configuration to browser config
  fn apply_proxy_config(
      mut config_builder: BrowserConfigBuilder,
      proxy: &ProxyConfig
  ) -> BrowserConfigBuilder {
      match proxy.proxy_type {
          crate::chromiumoxide::config::ProxyType::Http => {
              config_builder = config_builder.arg(&format!("--proxy-server={}", proxy.server));
          }
          crate::chromiumoxide::config::ProxyType::Https => {
              config_builder = config_builder.arg(&format!("--proxy-server={}", proxy.server));
          }
          crate::chromiumoxide::config::ProxyType::Socks4 => {
              config_builder = config_builder.arg(&format!("--proxy-server=socks4://{}", proxy.server));
          }
          crate::chromiumoxide::config::ProxyType::Socks5 => {
              config_builder = config_builder.arg(&format!("--proxy-server=socks5://{}", proxy.server));
          }
      }

      if let (Some(username), Some(password)) = (&proxy.username, &proxy.password) {
          config_builder = config_builder.arg(&format!("--proxy-auth={}:{}", username, password));
      }

      config_builder
  }

  /// Get a tab manager from the global shared browser instance
  pub async fn get_tab_manager(&self) -> BrowserResult<TabManager> {
    let global_browser = GLOBAL_BROWSER.lock().await;
    let global_stealth = GLOBAL_STEALTH.lock().await;

    if let Some(browser) = global_browser.as_ref() {
      let tab_manager = TabManager::new(
        browser.clone(),
        global_stealth.clone(),
        self.config.clone()
      ).await?;

      tracing::debug!("Assigned tab to global shared browser instance");
      Ok(tab_manager)
    } else {
      Err(BrowserError::GenericError("Global browser not initialized".to_string()))
    }
  }

  /// Get pool statistics for the global shared browser
  pub async fn get_stats(&self) -> PoolStats {
    // For global browser, we don't track tab count per browser instance
    // since tabs are created on-demand and managed by the browser itself
    let browser_stats = vec![BrowserStat {
      index: 0,
      active_tabs: 0, // We don't track this for global browser
      max_tabs: self.config.max_tabs_per_browser,
    }];

    PoolStats {
      total_browsers: 1,
      total_active_tabs: 0, // We don't track this for global browser
      max_concurrent_tabs: self.config.max_concurrent_tabs,
      browser_stats,
    }
  }
}

/// Statistics for the browser pool
#[derive(Debug, Clone)]
pub struct PoolStats {
  pub total_browsers: usize,
  pub total_active_tabs: usize,
  pub max_concurrent_tabs: usize,
  pub browser_stats: Vec<BrowserStat>,
}

/// Statistics for a single browser instance
#[derive(Debug, Clone)]
pub struct BrowserStat {
  pub index: usize,
  pub active_tabs: usize,
  pub max_tabs: usize,
}
