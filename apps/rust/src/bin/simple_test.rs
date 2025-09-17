use reqwest::Client;
use std::error::Error;
use tokio;
use std::fs::write;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    // Test with a known manga URL from komiku.org
    let url = "https://komiku.org/manga/boku-to-kimi-gyaru-ga-fufu-ni-naru-made/";

    println!("Fetching URL: {}", url);

    // Fetch the response with headers to mimic a browser
    let response = client
        .get(url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
        .send()
        .await?;

    println!("Response status: {}", response.status());

    // Get the HTML content
    let html = response.text().await?;
    println!("Response body length: {} characters", html.len());

    // Save the HTML to a file for inspection
    write("komiku_test.html", &html)?;
    println!("HTML saved to komiku_test.html");

    // Print the first 1000 characters to get a sense of the structure
    let html_preview = html.chars().take(1000).collect::<String>();
    println!("\nFirst 1000 characters of HTML:\n{}", html_preview);

    Ok(())
}
