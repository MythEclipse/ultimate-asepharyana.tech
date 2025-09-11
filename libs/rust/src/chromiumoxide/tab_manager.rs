//! Tab management with retry and timeout mechanisms

use crate::chromiumoxide::{
  config::BrowserConfig,
  error::{ BrowserError, BrowserResult },
  stealth::StealthManager,
};
use chromiumoxide::{Browser, Page};
use std::sync::Arc;
use tokio::time::timeout;
use serde_json; // Added for evaluate_script return type

/// Tab manager for performing operations on a browser tab
#[allow(dead_code)]
pub struct TabManager {
  page: Arc<Page>,
  stealth_manager: Option<StealthManager>,
  config: BrowserConfig,
}

impl TabManager {
  /// Create a new tab manager
  pub async fn new(
    browser: Arc<Browser>,
    stealth_manager: Option<StealthManager>,
    config: BrowserConfig
  ) -> BrowserResult<Self> {
    let page = browser
      .new_page("about:blank").await
      .map_err(|e| BrowserError::TabCreationError(e.to_string()))?;

    let page = Arc::new(page);

    // Apply stealth features if enabled
    if let Some(ref stealth) = stealth_manager {
      stealth.apply_stealth(&page).await?;
    }

    Ok(Self {
      page,
      stealth_manager,
      config,
    })
  }

  /// Navigate to a URL with retry and timeout
  pub async fn navigate(&self, url: &str) -> BrowserResult<()> {
    self.retry_operation(|| Box::pin(self.navigate_once(url)), "navigation").await
  }

  /// Navigate to URL once (internal method)
  async fn navigate_once(&self, url: &str) -> BrowserResult<()> {
    let navigation_future = self.page.goto(url);

    match timeout(self.config.default_timeout, navigation_future).await {
      Ok(result) => {
        result.map_err(|e| BrowserError::NavigationError(e.to_string()))?;
        tracing::debug!("Navigated to: {}", url);
        Ok(())
      }
      Err(_) => Err(BrowserError::TimeoutError(format!("Navigation timeout for: {}", url))),
    }
  }

  /// Wait for an element with retry and timeout
  pub async fn wait_for_element(&self, selector: &str) -> BrowserResult<()> {
    self.retry_operation(
      || Box::pin(self.wait_for_element_once(selector)),
      &format!("wait for element: {}", selector)
    ).await
  }

  /// Wait for element once (internal method)
  async fn wait_for_element_once(&self, selector: &str) -> BrowserResult<()> {
    let wait_future = self.page.find_element(selector);

    match timeout(self.config.default_timeout, wait_future).await {
      Ok(result) => {
        result.map_err(|_| BrowserError::ElementNotFound(selector.to_string()))?;
        tracing::debug!("Element found: {}", selector);
        Ok(())
      }
      Err(_) => Err(BrowserError::TimeoutError(format!("Element wait timeout: {}", selector))),
    }
  }

  /// Get page content with retry
  pub async fn get_content(&self) -> BrowserResult<String> {
    self.retry_operation(|| Box::pin(self.get_content_once()), "get content").await
  }

  /// Get page content once (internal method)
  async fn get_content_once(&self) -> BrowserResult<String> {
    let content_future = self.page.content();

    match timeout(self.config.default_timeout, content_future).await {
      Ok(result) => {
        let content = result.map_err(|e| BrowserError::GenericError(e.to_string()))?;
        tracing::debug!("Retrieved page content ({} chars)", content.len());
        Ok(content)
      }
      Err(_) => Err(BrowserError::TimeoutError("Content retrieval timeout".to_string())),
    }
  }

  /// Execute JavaScript with retry
  pub async fn evaluate_script(
      &self,
      script: &str
  ) -> BrowserResult<serde_json::Value> {
      self.retry_operation(|| Box::pin(self.evaluate_script_once(script)), "script evaluation").await
  }

  /// Execute JavaScript once (internal method)
  async fn evaluate_script_once(
      &self,
      script: &str
  ) -> BrowserResult<serde_json::Value> {
      let eval_future = self.page.evaluate(script);

      match timeout(self.config.default_timeout, eval_future).await {
          Ok(result) => {
              let value = result.map_err(|e| BrowserError::GenericError(e.to_string()))?.into_value().map_err(|e| BrowserError::GenericError(e.to_string()))?;
              tracing::debug!("Executed script: {}", script);
              Ok(value)
          }
          Err(_) => Err(BrowserError::TimeoutError(format!("Script execution timeout: {}", script))),
      }
  }

  /// Take screenshot with retry
  pub async fn screenshot(&self) -> BrowserResult<Vec<u8>> {
    self.retry_operation(|| Box::pin(self.screenshot_once()), "screenshot").await
  }

  /// Take screenshot once (internal method)
  async fn screenshot_once(&self) -> BrowserResult<Vec<u8>> {
    let screenshot_future = self.page.screenshot(chromiumoxide::page::ScreenshotParams::default());

    match timeout(self.config.default_timeout, screenshot_future).await {
      Ok(result) => {
        let screenshot = result.map_err(|e| BrowserError::GenericError(e.to_string()))?;
        tracing::debug!("Captured screenshot ({} bytes)", screenshot.len());
        Ok(screenshot)
      }
      Err(_) => Err(BrowserError::TimeoutError("Screenshot timeout".to_string())),
    }
  }

  /// Check if Cloudflare challenge is present
  pub async fn detect_cloudflare(&self) -> BrowserResult<bool> {
    let script =
      r#"
            document.querySelector('div[class*="challenge"]') !== null ||
            document.querySelector('div[class*="cf-browser-verification"]') !== null ||
            document.querySelector('div[class*="cf-challenge"]') !== null ||
            document.title.includes('Just a moment') ||
            document.title.includes('Checking your browser')
        "#;

    let result = self.evaluate_script_once(script).await?;
    Ok(result.as_bool().unwrap_or(false))
  }

  /// Get the underlying page (for advanced operations)
  pub fn page(&self) -> &Arc<Page> {
    &self.page
  }

  async fn retry_operation<F, Fut, T>(&self, operation: F, operation_name: &str) -> BrowserResult<T>
    where
      F: Fn() -> Fut + Send + Sync,
      Fut: std::future::Future<Output = BrowserResult<T>> + Send,
      T: Send + 'static,
  {
    let mut last_error = None;

    for attempt in 0..self.config.max_retries {
      match operation().await {
        Ok(result) => {
          if attempt > 0 {
            tracing::info!("Operation '{}' succeeded on attempt {}", operation_name, attempt + 1);
          }
          return Ok(result);
        }
        Err(e) => {
          last_error = Some(e);

          // Check for Cloudflare challenge
          if let Ok(true) = self.detect_cloudflare().await {
            tracing::warn!("Cloudflare challenge detected during '{}'", operation_name);
            return Err(
              BrowserError::CloudflareChallenge(
                format!("Cloudflare challenge detected during {}", operation_name)
              )
            );
          }

          if attempt < self.config.max_retries - 1 {
            let delay = self.config.retry_delay * (2_u32).pow(attempt);
            tracing::warn!(
              "Operation '{}' failed (attempt {}/{}), retrying in {:?}: {}",
              operation_name,
              attempt + 1,
              self.config.max_retries,
              delay,
              last_error.as_ref().unwrap()
            );
            tokio::time::sleep(delay).await;
          }
        }
      }
    }

    Err(
      BrowserError::RetryLimitExceeded(
        format!(
          "Operation '{}' failed after {} attempts: {}",
          operation_name,
          self.config.max_retries,
          last_error.unwrap()
        )
      )
    )
  }
}

impl Drop for TabManager {
  fn drop(&mut self) {
    // Note: In a real implementation, you might want to close the page here
    // But chromiumoxide handles page lifecycle automatically
    tracing::debug!("TabManager dropped");
  }
}
