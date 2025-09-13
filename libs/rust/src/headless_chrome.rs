use headless_chrome::{ Browser, LaunchOptionsBuilder, Tab };
use std::path::{ Path, PathBuf };
use tracing::{ info, warn, error };
use std::fs;
use std::sync::Arc;
use tokio::sync::Mutex as TokioMutex;
use tokio::time;

use std::process::Command;
use tempfile::TempDir;
use crate::utils::error::AppError; // Import AppError
use serde_json::Value;

fn find_puppeteer_chrome() -> Option<String> {
  // Get user home directory (cross-platform)
  let home = if cfg!(target_os = "windows") {
    std::env::var("USERPROFILE").ok()?
  } else {
    std::env::var("HOME").ok()?
  };

  let puppeteer_cache = Path::new(&home).join(".cache").join("puppeteer").join("chrome");

  if !puppeteer_cache.exists() {
    return None;
  }

  // Find the latest version directory
  let entries = fs::read_dir(&puppeteer_cache).ok()?;
  let mut versions = Vec::new();

  // Platform-specific directory patterns
  let platform_prefix = if cfg!(target_os = "windows") {
    "win64-"
  } else if cfg!(target_os = "linux") {
    "linux64-"
  } else if cfg!(target_os = "macos") {
    "mac-"
  } else {
    return None; // Unsupported platform
  };

  for entry in entries {
    if let Ok(entry) = entry {
      if let Some(name) = entry.file_name().to_str() {
        if name.starts_with(platform_prefix) {
          versions.push(entry.path());
        }
      }
    }
  }

  // Sort by version (assuming semantic versioning)
  versions.sort_by(|a, b| {
    let a_name = a.file_name().unwrap().to_str().unwrap();
    let b_name = b.file_name().unwrap().to_str().unwrap();
    b_name.cmp(a_name) // Reverse sort to get latest first
  });

  if let Some(latest_version) = versions.first() {
    // Platform-specific executable path
    let chrome_exe = if cfg!(target_os = "windows") {
      latest_version.join("chrome-win64").join("chrome.exe")
    } else if cfg!(target_os = "linux") {
      latest_version.join("chrome-linux64").join("chrome")
    } else if cfg!(target_os = "macos") {
      latest_version
        .join("chrome-mac")
        .join("Chromium.app")
        .join("Contents")
        .join("MacOS")
        .join("Chromium")
    } else {
      return None;
    };

    if chrome_exe.exists() {
      return chrome_exe.to_str().map(|s| s.to_string());
    }
  }

  None
}

/// Kill existing Chrome processes before launching a new browser
fn kill_existing_chrome_processes() {
  info!("Checking for existing Chrome processes to terminate...");

  let result = if cfg!(target_os = "windows") {
    Command::new("taskkill").args(&["/F", "/IM", "chrome.exe"]).output()
  } else {
    Command::new("pkill").args(&["-f", "chrome"]).output()
  };

  match result {
    Ok(output) => {
      if output.status.success() {
        info!("Successfully terminated existing Chrome processes");
      } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        warn!("Failed to terminate Chrome processes: {}", stderr);
      }
    }
    Err(e) => {
      warn!("Error executing process termination command: {:?}", e);
    }
  }

  // Small delay to ensure processes are fully terminated
  std::thread::sleep(std::time::Duration::from_millis(500));
}

pub async fn launch_browser(
  headless: bool,
  proxy_addr: Option<String>
) -> Result<Browser, Box<dyn std::error::Error + Send + Sync>> {
  // Kill existing Chrome processes before launching new browser
  kill_existing_chrome_processes();

  let mut options = LaunchOptionsBuilder::default();

  options.headless(headless);

  // Try to find Chrome executable on Windows, Linux, and macOS
  #[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
  {
    let mut found = false;

    // First try to find Puppeteer Chromium
    if let Some(puppeteer_path) = find_puppeteer_chrome() {
      if Path::new(&puppeteer_path).exists() {
        options.path(Some(PathBuf::from(puppeteer_path.clone())));
        info!("Found Puppeteer Chrome at: {}", puppeteer_path);
        found = true;
      }
    }

    // Fallback to standard Chrome paths
    if !found {
      let chrome_paths = if cfg!(target_os = "windows") {
        vec![
          r"C:\Program Files\Google\Chrome\Application\chrome.exe",
          r"C:\Program Files (x86)\Google\Chrome\Application\chrome.exe"
        ]
      } else if cfg!(target_os = "linux") {
        vec![
          "/usr/bin/google-chrome",
          "/usr/bin/google-chrome-stable",
          "/usr/bin/chromium",
          "/usr/bin/chromium-browser"
        ]
      } else if cfg!(target_os = "macos") {
        vec![
          "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
          "/Applications/Chromium.app/Contents/MacOS/Chromium"
        ]
      } else {
        vec![]
      };

      for path in &chrome_paths {
        if Path::new(path).exists() {
          options.path(Some(PathBuf::from(path.to_string())));
          info!("Found Chrome at: {}", path);
          found = true;
          break;
        }
      }
    }

    if !found {
      info!("Chrome not found in standard paths, will try default detection");
    }
  }

  let temp_dir = TempDir::new()?;
  let user_data_dir = temp_dir.path().to_string_lossy().to_string();

  let mut chrome_args_strings = vec![
    "--no-sandbox".to_string(),
    format!("--user-data-dir={}", user_data_dir),
    "--safebrowsing-disable-auto-update".to_string(),
    "--user-agent=Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36".to_string(),
    "--remote-debugging-port=9222".to_string() // Added for debugging
  ];

  // Set proxy if provided
  if let Some(proxy) = proxy_addr {
    chrome_args_strings.push(format!("--proxy-server={}", proxy));
  }

  let chrome_args_os: Vec<std::ffi::OsString> = chrome_args_strings
    .into_iter()
    .map(std::ffi::OsString::from)
    .collect();
  let chrome_args_refs: Vec<&std::ffi::OsStr> = chrome_args_os
    .iter()
    .map(|s| s.as_os_str())
    .collect();
  options.args(chrome_args_refs);
  options.idle_browser_timeout(std::time::Duration::from_secs(180));

  let browser = Browser::new(options.build()?)?;
  info!("Browser (Chrome) launched successfully.");
  Ok(browser)
}

/// Check if the browser is still connected and healthy
pub async fn is_browser_healthy(browser_arc: &Arc<TokioMutex<Browser>>) -> Result<bool, AppError> {
  let tab_result = browser_arc.lock().await.new_tab();
  match tab_result {
    Ok(tab) => {
      let eval_result = tab.evaluate(r#"'healthy'"#, false);
      match eval_result {
        Ok(remote_object) => {
          // Check if the remote_object has a value and can be converted to a string
          if let Some(value) = remote_object.value {
            if let Some(s) = value.as_str() {
              if s == "healthy" {
                // Successfully evaluated, now close the tab
                tab
                  .close(true)
                  .map_err(|e| AppError::Other(format!("Failed to close tab: {:?}", e)))?;
                return Ok(true);
              }
            }
          }
          // If we reach here, evaluation failed or returned unexpected type
          warn!("Browser health check failed: Unexpected evaluation result.");
          tab.close(true).map_err(|e| AppError::Other(format!("Failed to close tab: {:?}", e)))?;
          Err(
            AppError::Other(
              "Browser health check failed: Unexpected evaluation result.".to_string()
            )
          )
        }
        Err(e) => {
          warn!("Browser health check failed during evaluation: {:?}", e);
          tab.close(true).map_err(|e| AppError::Other(format!("Failed to close tab: {:?}", e)))?;
          Err(AppError::Other(format!("Browser health check failed during evaluation: {:?}", e)))
        }
      }
    }
    Err(e) => {
      warn!("Browser health check failed during tab creation: {:?}", e);
      Err(AppError::Other(format!("Browser health check failed during tab creation: {:?}", e)))
    }
  }
}

/// Attempt to reconnect the browser if it's unhealthy
pub async fn reconnect_browser_if_needed(
  browser_arc: &Arc<TokioMutex<Browser>>,
  headless: bool,
  proxy_addr: Option<String>
) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
  if is_browser_healthy(browser_arc).await? {
    return Ok(false); // No reconnection needed
  }

  warn!("Browser is unhealthy, attempting to reconnect...");

  // Add a delay to give the system time to release resources, especially debugging ports.
  time::sleep(std::time::Duration::from_secs(2)).await;
  info!("Attempting to launch new browser instance after delay...");

  match launch_browser(headless, proxy_addr).await {
    Ok(new_browser) => {
      *browser_arc.lock().await = new_browser;
      info!("Browser reconnected successfully");
      Ok(true)
    }
    Err(e) => {
      error!("Failed to reconnect browser: {:?}", e);
      Err(e)
    }
  }
}

pub struct BrowserPool {
  browser: Arc<TokioMutex<Browser>>,
}

impl BrowserPool {
  pub fn new(browser: Browser) -> Self {
    Self {
      browser: Arc::new(TokioMutex::new(browser)),
    }
  }

  pub fn from_arc(browser: Arc<TokioMutex<Browser>>) -> Self {
    Self {
      browser,
    }
  }

  pub async fn new_page(&self, url: &str) -> Result<PageWrapper, AppError> {
    let tab = self.browser
      .lock().await
      .new_tab()
      .map_err(|e| AppError::ChromiumoxideError(format!("Failed to create new tab: {:?}", e)))?;

    if url != "about:blank" {
      tab
        .navigate_to(url)
        .map_err(|e| AppError::ChromiumoxideError(format!("Failed to navigate: {:?}", e)))?;
    }

    Ok(PageWrapper { tab })
  }
}

pub struct PageWrapper {
  pub tab: Arc<Tab>,
}

impl PageWrapper {
  pub async fn goto(&self, url: &str) -> Result<(), AppError> {
    self.tab
      .navigate_to(url)
      .map_err(|e| AppError::ChromiumoxideError(format!("Failed to navigate: {:?}", e)))?;
    Ok(())
  }

  pub async fn find_element(&self, selector: &str) -> Result<(), AppError> {
    // Add a small delay to ensure page is ready
    time::sleep(std::time::Duration::from_millis(500)).await;

    match self.tab.wait_for_element(selector) {
      Ok(_) => Ok(()),
      Err(e) => {
        // On timeout, get the current page content for debugging
        let content = self.tab
          .get_content()
          .unwrap_or_else(|_| "Failed to get content".to_string());
        Err(
          AppError::ChromiumoxideError(
            format!(
              "Failed to find element '{}': {:?}\nCurrent page HTML:\n{}",
              selector,
              e,
              content
            )
          )
        )
      }
    }
  }

  pub async fn evaluate(&self, js: &str) -> Result<Option<Value>, AppError> {
    // Use await_promise = true so async IIFEs return their resolved values
    match self.tab.evaluate(js, true) {
      Ok(remote_object) => Ok(remote_object.value),
      Err(e) => Err(AppError::ChromiumoxideError(format!("Failed to evaluate JS: {:?}", e))),
    }
  }

  pub async fn content(&self) -> Result<String, AppError> {
    self.tab
      .get_content()
      .map_err(|e| AppError::ChromiumoxideError(format!("Failed to get content: {:?}", e)))
  }

  pub async fn url(&self) -> Result<Option<String>, AppError> {
    let url = self.tab.get_url();
    Ok(Some(url))
  }

  pub async fn wait_for_navigation(&self) -> Result<(), AppError> {
    // Wait for navigation to complete using the tab's wait_until_navigated method
    match self.tab.wait_until_navigated() {
      Ok(_) => Ok(()),
      Err(e) =>
        Err(AppError::ChromiumoxideError(format!("Failed to wait for navigation: {:?}", e))),
    }
  }

  pub async fn type_text(&self, selector: &str, text: &str) -> Result<(), AppError> {
    let element = self.tab
      .wait_for_element(selector)
      .map_err(|e| AppError::ChromiumoxideError(format!("Failed to find element: {:?}", e)))?;
    element
      .type_into(text)
      .map_err(|e| AppError::ChromiumoxideError(format!("Failed to type: {:?}", e)))?;
    Ok(())
  }

  pub async fn click(&self, selector: &str) -> Result<(), AppError> {
    // Wait for the element to appear
    let element = self.tab
      .wait_for_element(selector)
      .map_err(|e|
        AppError::ChromiumoxideError(format!("Failed to find element for click: {:?}", e))
      )?;
    element
      .click()
      .map_err(|e| AppError::ChromiumoxideError(format!("Failed to click element: {:?}", e)))?;
    Ok(())
  }
}
