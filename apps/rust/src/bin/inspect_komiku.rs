use reqwest::Client;
use scraper::{Html, Selector};
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
    let document = Html::parse_document(&html);

    // Save the HTML to a file for inspection
    write("komiku_full.html", &html)?;
    println!("Full HTML saved to komiku_full.html");

    // Extract and print specific information using meta tags
    let title_selector = Selector::parse("title").unwrap();
    let title = document.select(&title_selector).next().map(|n| n.text().collect::<String>()).unwrap_or_default();
    println!("\nPage Title: {}", title);

    // Extract meta description
    let desc_selector = Selector::parse("meta[name='description']").unwrap();
    let description = document.select(&desc_selector).next()
        .and_then(|n| n.value().attr("content"))
        .unwrap_or_default();
    println!("Description: {}", description);

    // Extract thumbnail URL
    let thumbnail_selector = Selector::parse("meta[name='thumbnailUrl']").unwrap();
    let thumbnail = document.select(&thumbnail_selector).next()
        .and_then(|n| n.value().attr("content"))
        .unwrap_or_default();
    println!("Thumbnail URL: {}", thumbnail);

    // Try to find chapter list
    let chapter_selectors = [
        "#chapter-list li",
        ".chapter-item",
        ".eplister ul li",
        "table#Daftar_Chapter tbody tr"
    ];

    println!("\n=== Chapter List Detection ===");
    for &selector in &chapter_selectors {
        let sel = Selector::parse(selector).unwrap();
        let count = document.select(&sel).count();
        println!("Found {} chapters with selector: {}", count, selector);

        if count > 0 {
            let first = document.select(&sel).next().unwrap();
            println!("First chapter HTML: {:?}", first.html());
        }
    }

    // Try to find info sections
    let info_selectors = [
        ".info-row",
        ".inftable tr",
        ".detail-info",
        ".manga-info"
    ];

    println!("\n=== Info Section Detection ===");
    for &selector in &info_selectors {
        let sel = Selector::parse(selector).unwrap();
        let count = document.select(&sel).count();
        println!("Found {} info rows with selector: {}", count, selector);

        if count > 0 {
            let first = document.select(&sel).next().unwrap();
            println!("First info row HTML: {:?}", first.html());
        }
    }

    Ok(())
}
