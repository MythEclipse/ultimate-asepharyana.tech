#!/usr/bin/env rust-script

// Test script to validate scraping selectors against real komiku.org pages
// Usage: cargo run --bin test_selector_validity -- batsu-hare the-girl-who-see-it-chapter-01

use reqwest::Client;
use scraper::{Html, Selector};
use std::env;
use std::error::Error;
use std::time::Duration;

const KOMIKU_BASE_URL: &str = "https://komiku.org";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <komik_id> <chapter_url>", args[0]);
        eprintln!("Example: {} batsu-hare the-girl-who-see-it-chapter-01", args[0]);
        std::process::exit(1);
    }

    let komik_id = &args[1];
    let chapter_url = &args[2];

    println!("=== Testing Komik Detail Selectors ===");
    test_detail_selectors(komik_id).await?;

    println!("\n=== Testing Chapter Selectors ===");
    test_chapter_selectors(chapter_url).await?;

    println!("\n✅ All selector tests passed!");
    Ok(())
}

async fn test_detail_selectors(komik_id: &str) -> Result<(), Box<dyn Error>> {
    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .build()?;

    let url = format!("{}/manga/{}/", KOMIKU_BASE_URL, komik_id);
    println!("Testing detail URL: {}", url);

    let response = client.get(&url).send().await?;
    let html = response.text().await?;
    let document = Html::parse_document(&html);

    // Test title selector (using meta tag now)
    let title_selector = Selector::parse("meta[name='thumbnailUrl']").unwrap();
    let poster = document.select(&title_selector)
        .next()
        .and_then(|el| el.value().attr("content"))
        .unwrap_or("");

    if poster.is_empty() {
        return Err("❌ Failed to find poster (meta[name='thumbnailUrl'])".into());
    }
    println!("✅ Poster found: {}", poster);

    // Test description selector
    let desc_selector = Selector::parse("meta[name='description']").unwrap();
    let description = document.select(&desc_selector)
        .next()
        .and_then(|el| el.value().attr("content"))
        .unwrap_or("");

    if description.is_empty() {
        return Err("❌ Failed to find description (meta[name='description'])".into());
    }
    println!("✅ Description found (length: {} chars)", description.len());

    // Test chapter list selector
    let chapter_selector = Selector::parse("table#Daftar_Chapter tbody tr").unwrap();
    let chapter_count = document.select(&chapter_selector).count();

    if chapter_count == 0 {
        return Err("❌ Failed to find any chapters (table#Daftar_Chapter tbody tr)".into());
    }
    println!("✅ Found {} chapters", chapter_count);

    Ok(())
}

async fn test_chapter_selectors(chapter_url: &str) -> Result<(), Box<dyn Error>> {
    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .build()?;

    let url = format!("{}/{}", KOMIKU_BASE_URL, chapter_url);
    println!("Testing chapter URL: {}", url);

    let response = client.get(&url).send().await?;
    let html = response.text().await?;
    let document = Html::parse_document(&html);

    // Test title selector
    let title_selector = Selector::parse("title").unwrap();
    let title = document.select(&title_selector)
        .next()
        .map(|el| el.text().collect::<String>())
        .unwrap_or_default();

    if title.is_empty() {
        return Err("❌ Failed to find title (title tag)".into());
    }
    println!("✅ Title found: {}", title);

    // Test image selector
    let image_selector = Selector::parse("img[data-src], img[src]").unwrap();
    let image_count = document.select(&image_selector).count();

    if image_count == 0 {
        return Err("❌ Failed to find any images (img[data-src], img[src])".into());
    }
    println!("✅ Found {} images", image_count);

    Ok(())
}
