use rust_lib::fetch_with_proxy::fetch_with_proxy;
use rust_lib::urls::get_komik2_url;
use scraper::{Html, Selector};
use tokio;
use std::time::Instant;

#[derive(Debug, Clone)]
struct TestResult {
    title: String,
    alternative_title: String,
    score: String,
    updated_on: String,
    chapters: Vec<String>,
    has_chapter_data: bool
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Test with various manga, manhua, and manhwa titles
    let test_titles = vec![
        "naruto",           // Manga
        "infinite-evolution-starting-from-zero", // Manhua
        "number-one-star-instructor-master-baek" // Manhwa
    ];

    for title in &test_titles {
        println!("\n\n=====================================");
        println!("Testing: {}", title);
        println!("=====================================");

        let result = test_komik_detail(title.to_string()).await?;

        println!("\n✅ Results for {}:", title);
        println!("Title: {}", result.title);
        println!("Alternative Title: {}", result.alternative_title);
        println!("Score: {}", result.score);
        println!("Updated On: {}", result.updated_on);
        println!("Chapters Found: {}", result.chapters.len());
        println!("Has Chapter Data: {}", result.has_chapter_data);

        // Print first few chapters if available
        if !result.chapters.is_empty() {
            println!("First 3 chapters: {:?}", &result.chapters[0..3.min(result.chapters.len())]);
        }
    }

    Ok(())
}

async fn test_komik_detail(komik_id: String) -> Result<TestResult, Box<dyn std::error::Error>> {
    let start_time = Instant::now();
    let base_url = get_komik2_url();
    let url = if komik_id == "naruto" {
        format!("{}/?post_type=manga&s={}", base_url, komik_id)
    } else if komik_id == "infinite-evolution-starting-from-zero" {
        format!("{}/manga/page/1/?tipe=manhua", base_url)
    } else if komik_id == "number-one-star-instructor-master-baek" {
        format!("{}/manga/page/1/?tipe=manhwa", base_url)
    } else {
        format!("{}/?post_type=manga&s={}", base_url, komik_id)
    };

    println!("Fetching: {}", url);

    let html = fetch_with_proxy(&url).await?.data;
    let document = Html::parse_document(&html);

    let duration = start_time.elapsed();
    println!("Fetched and parsed in: {:?}", duration);

    // Test title extraction
    let title = extract_title(&document);
    println!("Title extraction: {}", if !title.is_empty() { "✅" } else { "❌" });

    // Test alternative title extraction
    let alternative_title = extract_alternative_title(&document);
    println!("Alternative title extraction: {}", if !alternative_title.is_empty() { "✅" } else { "❌" });

    // Test score extraction
    let score = extract_score(&document);
    println!("Score extraction: {}", if !score.is_empty() { "✅" } else { "❌" });

    // Test updated_on extraction
    let updated_on = extract_updated_on(&document);
    println!("Updated on extraction: {}", if !updated_on.is_empty() { "✅" } else { "❌" });

    // Test chapter extraction
    let chapters = extract_chapters(&document);
    let has_chapter_data = chapters.iter().any(|c| !c.chapter_id.is_empty());
    println!("Chapter extraction: Found {} chapters, with data: {}", chapters.len(), if has_chapter_data { "✅" } else { "❌" });

    Ok(TestResult {
        title,
        alternative_title,
        score,
        updated_on,
        chapters: chapters.iter().map(|c| format!("{} ({})", c.chapter, c.chapter_id)).collect(),
        has_chapter_data
    })
}

fn extract_title(document: &Html) -> String {
    // First try: Look for title in .bge .kan h3 (API response format)
    document.select(&Selector::parse(".bge .kan h3").unwrap())
        .next()
        .map(|e| {
            let text = e.text().collect::<String>().trim().to_string();
            text.replace("Komik ", "")
                .replace("Manga ", "")
                .replace("Manhua ", "")
                .replace("Manhwa ", "")
                .trim()
                .to_string()
        })
        // Second try: Look for title in main content areas
        .or_else(|| {
            document
                .select(&Selector::parse("h1#Judul, h1.entry-title, .entry-title, .title-series, .post-title").unwrap())
                .next()
                .map(|e| {
                    let text = e.text().collect::<String>().trim().to_string();
                    text.replace("Komik ", "")
                        .replace("Manga ", "")
                        .replace("Manhua ", "")
                        .replace("Manhwa ", "")
                        .trim()
                        .to_string()
                })
        })
        // Fallback: Try any h3 that might contain the title
        .or_else(|| {
            document.select(&Selector::parse("h3").unwrap())
                .next()
                .map(|e| {
                    let text = e.text().collect::<String>().trim().to_string();
                    text.replace("Komik ", "")
                        .replace("Manga ", "")
                        .replace("Manhua ", "")
                        .replace("Manhwa ", "")
                        .trim()
                        .to_string()
                })
        })
        .unwrap_or_default()
}

fn extract_alternative_title(document: &Html) -> String {
    // First try: Look for alternative title in .spe section using text filtering
    document.select(&Selector::parse(".spe span").unwrap())
        .find(|e| {
            let text = e.text().collect::<String>().to_lowercase();
            text.contains("alternatif") || text.contains("alternative")
        })
        .map(|e| {
            let text = e.text().collect::<String>().trim().to_string();
            text.replace("Judul Alternatif:", "")
                .replace("Alternative:", "")
                .replace(":", "")
                .trim()
                .to_string()
        })
        // Fallback: Check for any span with "alternatif" in API response format
        .or_else(|| {
            document.select(&Selector::parse(".bge .kan span").unwrap())
                .find(|e| {
                    let text = e.text().collect::<String>().to_lowercase();
                    text.contains("alternatif") || text.contains("alternative")
                })
                .map(|e| {
                    let text = e.text().collect::<String>().trim().to_string();
                    text.replace("Judul Alternatif:", "")
                        .replace("Alternative:", "")
                        .replace(":", "")
                        .trim()
                        .to_string()
                })
        })
        .unwrap_or_default()
}

fn extract_score(document: &Html) -> String {
    // Always return empty string for score as per requirements
    "".to_string()
}

fn extract_updated_on(document: &Html) -> String {
    // First try: Extract from .judul2 class (API response format: "X pembaca • Y waktu lalu")
    document.select(&Selector::parse(".bge .kan .judul2").unwrap())
        .next()
        .map(|e| {
            let text = e.text().collect::<String>().trim().to_string();

            // Extract the time part after "• " (e.g., "7 jam lalu" from "18rb pembaca • 7 jam lalu")
            let time_part = text.split("• ").nth(1)
                .map(|s| s.trim().to_string())
                .unwrap_or_default();

            // Use regex to extract just the time information (e.g., "41 menit lalu" from "41 menit lalu Berwarna")
            time_part.split_whitespace()
                .take_while(|&word| !word.eq_ignore_ascii_case("berwarna"))
                .collect::<Vec<&str>>()
                .join(" ")
        })
        // Second try: Look for updated label in .spe section with specific text
        .or_else(|| {
            document.select(&Selector::parse(".spe span").unwrap())
                .find(|e| {
                    let text = e.text().collect::<String>().to_lowercase();
                    text.contains("diperbarui") || text.contains("updated")
                })
                .map(|e| {
                    let text = e.text().collect::<String>().trim().to_string();
                    let cleaned = text.replace("Diperbarui:", "")
                        .replace("Updated:", "")
                        .replace(":", "")
                        .trim()
                        .to_string();

                    // Remove "Berwarna" if it's present
                    if cleaned.ends_with("Berwarna") {
                        cleaned.split_whitespace()
                            .take_while(|&word| !word.eq_ignore_ascii_case("berwarna"))
                            .collect::<Vec<&str>>()
                            .join(" ")
                    } else {
                        cleaned
                    }
                })
        })
        // Third try: Look for any date-like text in the document
        .or_else(|| {
            document.select(&Selector::parse("td.tanggalseries, .rightarea .date, .epcontent .date, .udate, .chapter-date").unwrap())
                .next()
                .map(|e| e.text().collect::<String>().trim().to_string())
        })
        // Final fallback: Try to find any text that looks like a date/time
        .or_else(|| {
            document.select(&Selector::parse("span, div, p").unwrap())
                .find(|e| {
                    let text = e.text().collect::<String>().to_lowercase();
                    // Look for common time indicators
                    text.contains("menit") || text.contains("jam") || text.contains("hari") ||
                    text.contains("minggu") || text.contains("bulan") || text.contains("tahun")
                })
                .map(|e| e.text().collect::<String>().trim().to_string())
        })
        .unwrap_or_default()
}

#[derive(Debug, Clone)]
struct Chapter {
    chapter: String,
    date: String,
    chapter_id: String
}

fn extract_chapters(document: &Html) -> Vec<Chapter> {
    let mut chapters = Vec::new();

    for el in document.select(&Selector::parse("table#Daftar_Chapter tbody#daftarChapter tr, #chapter_list li, .eplister ul li, .chapter-list li, .bge").unwrap()) {
        // Extract chapter title/number
        let chapter = el
            .select(&Selector::parse("td.judulseries a, a.chapter, a, .chapter-item a, .new1 a").unwrap())
            .next()
            .map(|e| {
                let text = e.text().collect::<String>().trim().to_string();
                // Extract numeric chapter if available (e.g., "Chapter 123" -> "123")
                text.split_whitespace()
                    .find(|&s| s.chars().any(|c| c.is_digit(10)))
                    .map(|num_part| num_part.to_string())
                    .unwrap_or_else(|| {
                        // Try to extract chapter number from href if text doesn't have it
                        e.value().attr("href")
                            .and_then(|href| {
                                href.split('/')
                                    .find(|s| s.chars().any(|c| c.is_digit(10)))
                                    .map(|s| s.to_string())
                            })
                            .unwrap_or(text)
                    })
            })
            .unwrap_or_default();

        // Extract date
        let date = el
            .select(&Selector::parse("td.tanggalseries, .rightarea .date, .epcontent .date, .udate, .chapter-date, .judul2").unwrap())
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or_default();

        // Extract chapter ID from URL
        let chapter_id = el
            .select(&Selector::parse("td.judulseries a, a.chapter, a, .chapter-item a, .new1 a").unwrap())
            .next()
            .and_then(|e| e.value().attr("href"))
            .map(|href| {
                let parts: Vec<&str> = href
                    .split('/')
                    .filter(|s| !s.is_empty())
                    .collect();
                // Try to find segment after known category (manga|manhua|manhwa)
                if let Some(pos) = parts.iter().position(|s|
                    *s == "manga" || *s == "manhua" || *s == "manhwa" ||
                    *s == "chapter" || *s == "chapters"
                ) {
                    parts.get(pos + 1).cloned().unwrap_or("").to_string()
                } else {
                    // Fallback to last segment or full path if nothing else works
                    parts.last().cloned().unwrap_or("").to_string()
                }
            })
            .unwrap_or_default();

        // Only add chapter if it has at least a chapter ID or some content
        if !chapter_id.is_empty() || !chapter.is_empty() {
            chapters.push(Chapter { chapter, date, chapter_id });
        }
    }

    chapters
}
