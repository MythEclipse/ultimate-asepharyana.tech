use chromiumoxide::{Browser, BrowserConfig};
use tracing::info;

pub async fn launch_browser(
  headless: bool,
  proxy_addr: Option<String>
) -> Result<Browser, Box<dyn std::error::Error + Send + Sync>> {
  let mut config = BrowserConfig::builder();

  if headless {
    config = config.with_head();
  }

  // Set Chrome arguments for better stability
  let mut chrome_args = vec![
    "--no-sandbox".to_string(),
    "--disable-gpu".to_string(),
    "--disable-dev-shm-usage".to_string(),
    "--disable-background-timer-throttling".to_string(),
    "--disable-backgrounding-occluded-windows".to_string(),
    "--disable-renderer-backgrounding".to_string(),
  ];

  // Set proxy if provided
  if let Some(proxy) = proxy_addr {
    chrome_args.push(format!("--proxy-server={}", proxy));
  }

  config = config.args(chrome_args);

  let (browser, _) = Browser::launch(config.build()?).await
    .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("{:?}", e))))?;
  info!("Browser (Chrome) launched successfully.");
  Ok(browser)
}
