use fantoccini::{ Client, ClientBuilder };
use fantoccini::wd::Capabilities;
use serde_json::json;
use tracing::{info, warn};

pub async fn launch_browser(
  headless: bool,
  proxy_addr: Option<String>
) -> Result<Client, fantoccini::error::NewSessionError> {
  let mut caps = Capabilities::new();
  let chrome_args = if headless {
    vec![
      "--headless=new".to_string(),
      "--no-sandbox".to_string(),
      "--disable-gpu".to_string(),
      "--disable-dev-shm-usage".to_string(),
    ]
  } else {
    vec![]
  };

  caps.insert("goog:chromeOptions".to_string(), json!({ "args": chrome_args }));

  let mut client_builder = ClientBuilder::native();
  if let Some(proxy) = proxy_addr {
    let mut proxy_caps = Capabilities::new();
    proxy_caps.insert("proxy".to_string(), json!({"proxyType": "manual", "httpProxy": proxy, "sslProxy": proxy}).into());
    client_builder.capabilities(proxy_caps);
  }

  match client_builder
    .capabilities(caps)
    .connect("http://localhost:4444").await {
      Ok(client) => {
        info!("Browser (Chrome) launched successfully.");
        Ok(client)
      }
      Err(e) => {
        warn!("Failed to launch Chrome: {:?}. Make sure Chrome and ChromeDriver are installed.", e);
        Err(e)
      }
    }
}
