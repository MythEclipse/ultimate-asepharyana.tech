use rust_lib::fetch_with_proxy::fetch_with_proxy;
use tokio; // Add tokio for async main

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://api.komiku.org/manga/";
    println!("Fetching URL: {}", url);
    let response = fetch_with_proxy(url).await?;
    println!("{}", response.data);
    Ok(())
}
