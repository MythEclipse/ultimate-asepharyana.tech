use reqwest::Client;
use scraper::{Html, Selector};
use std::error::Error;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    // Test with a known manga ID
    let komik_id = "boku-to-kimi-gyaru-ga-fufu-ni-naru-made";
    let base_url = "https://api.komiku.org";
    let url = format!("{}/manga/{}/", base_url, komik_id);

    println!("Fetching URL: {}", url);

    // Fetch the response
    let response = client.get(&url).send().await?;

    // Check response status
    println!("Response status: {}", response.status());

    // Print response headers
    println!("\n=== Response Headers ===");
    for (name, value) in response.headers() {
        println!("{}: {}", name.to_string(), value.to_str().unwrap_or(""));
    }

    // Get the HTML content
    let html = response.text().await?;
    println!("\nResponse body length: {} characters", html.len());

    if html.is_empty() {
        println!("Response body is empty!");
        return Ok(());
    }

    // Parse the HTML
    let document = Html::parse_document(&html);

    // Test various selectors to see what's available
    println!("\n=== Testing selectors ===");

    // Test basic structure
    let body_selector = Selector::parse("body").unwrap();
    let has_body = document.select(&body_selector).next().is_some();
    println!("Has body: {}", has_body);

    // Test if the page has the expected structure
    let judul_selector = Selector::parse("#Judul").unwrap();
    let has_judul = document.select(&judul_selector).next().is_some();
    println!("Has #Judul: {}", has_judul);

    let inftable_selector = Selector::parse(".inftable").unwrap();
    let has_inftable = document.select(&inftable_selector).next().is_some();
    println!("Has .inftable: {}", has_inftable);

    let daftar_chapter_selector = Selector::parse("#Daftar_Chapter").unwrap();
    let has_daftar_chapter = document.select(&daftar_chapter_selector).next().is_some();
    println!("Has #Daftar_Chapter: {}", has_daftar_chapter);

    // Print more of the HTML to analyze structure
    println!("\n=== HTML Structure Analysis ===");

    // Check for common HTML patterns
    let document = Html::parse_document(&html);

    // Look for any headings
    let heading_selectors = ["h1", "h2", "h3", "h4", "h5", "h6"];
    for &selector in &heading_selectors {
        let sel = Selector::parse(selector).unwrap();
        let count = document.select(&sel).count();
        println!("Number of {} elements: {}", selector, count);
        if count > 0 {
            let first = document.select(&sel).next().unwrap();
            println!("First {} text: {:?}", selector, first.text().collect::<String>());
        }
    }

    // Look for any tables
    let table_selector = Selector::parse("table").unwrap();
    let table_count = document.select(&table_selector).count();
    println!("Number of table elements: {}", table_count);

    // Look for any divs with class attributes
    let div_selector = Selector::parse("div[class]").unwrap();
    let div_count = document.select(&div_selector).count();
    println!("Number of divs with classes: {}", div_count);

    // Print the full HTML to analyze structure
    println!("\n=== Full HTML ===");
    println!("{}", html);

    Ok(())
}
