//! Browser pool management with round-robin load balancing

use crate::headless_chrome::{
  config::{ BrowserConfig, ProxyConfig },
  error::{ BrowserError, BrowserResult },
  stealth::StealthManager,
  tab_manager::TabManager,
};
use chromiumoxide::{ Browser, BrowserConfig as ChromiumConfig };
use chromiumoxide::browser::BrowserConfigBuilder;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Browser pool that manages multiple browser instances
pub struct BrowserPool {
  browsers: Vec<Arc<BrowserInstance>>,
  current_index: Mutex<usize>,
  config: BrowserConfig,
}

#[allow(dead_code)]
struct BrowserInstance {
  browser: Arc<Browser>,
  tab_count: Mutex<usize>,
  max_tabs: usize,
  proxy_config: Option<ProxyConfig>,
  stealth_manager: Option<StealthManager>,
}

impl BrowserPool {
  /// Create a new browser pool
  pub async fn new(config: BrowserConfig) -> BrowserResult<Self> {
    let mut browsers = Vec::with_capacity(config.browser_instances);

    for i in 0..config.browser_instances {
      let browser_instance = Self::create_browser_instance(&config, i).await?;
      browsers.push(Arc::new(browser_instance));
    }

    Ok(Self {
      browsers,
      current_index: Mutex::new(0),
      config,
    })
  }

  /// Create a single browser instance
  async fn create_browser_instance(
    config: &BrowserConfig,
    index: usize
  ) -> BrowserResult<BrowserInstance> {
    let mut chrome_config_builder = ChromiumConfig::builder();

    // Apply chrome arguments
    for arg in &config.chrome_args {
        chrome_config_builder = chrome_config_builder.arg(arg);
    }

    // Apply proxy settings if configured
    if let Some(proxy) = &config.proxy {
        chrome_config_builder = Self::apply_proxy_config(chrome_config_builder, proxy);
    }

    let (browser, _handler) = Browser::launch(chrome_config_builder.build().unwrap()).await.map_err(|e|
        BrowserError::BrowserStartupError(format!("Browser {}: {}", index, e))
    )?;

    let stealth_manager = if config.stealth_enabled {
      Some(StealthManager::new(Default::default()))
    } else {
      None
    };

    tracing::info!("Created browser instance {}", index);

    Ok(BrowserInstance {
      browser: Arc::new(browser),
      max_tabs: config.max_tabs_per_browser,
      tab_count: Mutex::new(0),
      proxy_config: config.proxy.clone(),
      stealth_manager,
    })
  }

  /// Apply proxy configuration to browser config
  fn apply_proxy_config(
      mut config_builder: BrowserConfigBuilder,
      proxy: &ProxyConfig
  ) -> BrowserConfigBuilder {
      match proxy.proxy_type {
          crate::headless_chrome::config::ProxyType::Http => {
              config_builder = config_builder.arg(&format!("--proxy-server={}", proxy.server));
          }
          crate::headless_chrome::config::ProxyType::Https => {
              config_builder = config_builder.arg(&format!("--proxy-server={}", proxy.server));
          }
          crate::headless_chrome::config::ProxyType::Socks4 => {
              config_builder = config_builder.arg(&format!("--proxy-server=socks4://{}", proxy.server));
          }
          crate::headless_chrome::config::ProxyType::Socks5 => {
              config_builder = config_builder.arg(&format!("--proxy-server=socks5://{}", proxy.server));
          }
      }

      if let (Some(username), Some(password)) = (&proxy.username, &proxy.password) {
          config_builder = config_builder.arg(&format!("--proxy-auth={}:{}", username, password));
      }

      config_builder
  }

  /// Get a tab manager using round-robin selection
  pub async fn get_tab_manager(&self) -> BrowserResult<TabManager> {
    let mut current = self.current_index.lock().await;
    let start_index = *current;

    // Find an available browser instance
    loop {
      let browser_instance = &self.browsers[*current];
      let tab_count = *browser_instance.tab_count.lock().await;

      if tab_count < browser_instance.max_tabs {
        *browser_instance.tab_count.lock().await += 1;
        let tab_manager = TabManager::new(
          browser_instance.browser.clone(),
          browser_instance.stealth_manager.clone(),
          self.config.clone()
        ).await?;

        tracing::debug!("Assigned tab to browser instance {}", *current);

        *current = (*current + 1) % self.browsers.len();
        return Ok(tab_manager);
      }

      *current = (*current + 1) % self.browsers.len();

      // If we've checked all browsers and none are available
      if *current == start_index {
        return Err(BrowserError::GenericError("No available browser instances".to_string()));
      }
    }
  }

  /// Get pool statistics
  pub async fn get_stats(&self) -> PoolStats {
    let mut total_tabs = 0;
    let mut browser_stats = Vec::new();

    for (i, browser) in self.browsers.iter().enumerate() {
      let tab_count = *browser.tab_count.lock().await;
      total_tabs += tab_count;
      browser_stats.push(BrowserStat {
        index: i,
        active_tabs: tab_count,
        max_tabs: browser.max_tabs,
      });
    }

    PoolStats {
      total_browsers: self.browsers.len(),
      total_active_tabs: total_tabs,
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
