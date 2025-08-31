use reqwest::Client;

pub async fn fetch_with_proxy(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    let response = client.get(url).send().await?.text().await?;
    Ok(response)
}
