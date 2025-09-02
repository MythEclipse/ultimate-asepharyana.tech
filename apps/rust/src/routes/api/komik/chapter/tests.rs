#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;
    use http::StatusCode;

    #[test]
    async fn test_handler_missing_chapter_url() {
        let params = ChapterParams { chapter_url: None };
        let response = handler(Query(params)).await.into_response();
        assert_eq!(response.status(), StatusCode::OK); // Should return 200 OK with error JSON
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["status"], "false");
        assert_eq!(json["message"], "Missing chapter_url parameter");
    }

    #[test]
    async fn test_parse_manga_chapter_success() {
        let html_content = r#"
            <html><body>
                <h1 class="entry-title">Chapter 1: Test Title</h1>
                <div class="nextprev">
                    <a href="https://komikcast.site/chapter/prev-chapter-id/" rel="prev">Prev</a>
                    <a href="https://komikcast.site/komik/manga-title/"><i class="icol daftarch"></i></a>
                    <a href="https://komikcast.site/chapter/next-chapter-id/" rel="next">Next</a>
                </div>
                <div id="chimg-auh">
                    <img src="https://komikcast.site/image1.jpg">
                    <img src="https://komikcast.site/image2.png">
                </div>
            </body></html>
        "#;
        let document = Html::parse_document(html_content);

        let title = select_text(&document, ".entry-title");
        assert_eq!(title, "Chapter 1: Test Title");

        let prev_chapter_id = {
            let sel = Selector::parse(".nextprev a[rel=\"prev\"]").expect("Failed to parse selector for previous chapter");
            document.select(&sel).next()
                .and_then(|e| e.value().attr("href"))
                .and_then(|href| href.split('/').nth(3))
                .unwrap_or("")
                .to_string()
        };
        assert_eq!(prev_chapter_id, "prev-chapter-id");

        let next_chapter_id = {
            let sel = Selector::parse(".nextprev a[rel=\"next\"]").expect("Failed to parse selector for next chapter");
            document.select(&sel).next()
                .and_then(|e| e.value().attr("href"))
                .and_then(|href| href.split('/').nth(3))
                .unwrap_or("")
                .to_string()
        };
        assert_eq!(next_chapter_id, "next-chapter-id");

        let list_chapter = {
            let sel = Selector::parse(".nextprev a:has(.icol.daftarch)").expect("Failed to parse selector for list chapter");
            document.select(&sel).next()
                .and_then(|e| e.value().attr("href"))
                .and_then(|href| href.split('/').nth(4))
                .unwrap_or("")
                .to_string()
        };
        assert_eq!(list_chapter, "manga-title");

        let mut images = Vec::new();
        if let Ok(sel) = Selector::parse("#chimg-auh img") {
            for el in document.select(&sel) {
                if let Some(src) = el.value().attr("src") {
                    images.push(src.to_string());
                }
            }
        }
        assert_eq!(images, vec!["https://komikcast.site/image1.jpg", "https://komikcast.site/image2.png"]);
    }

    #[test]
    async fn test_parse_manga_chapter_no_nav_links() {
        let html_content = r#"
            <html><body>
                <h1 class="entry-title">Chapter 1: Test Title</h1>
                <div class="nextprev">
                </div>
                <div id="chimg-auh">
                    <img src="https://komikcast.site/image1.jpg">
                </div>
            </body></html>
        "#;
        let document = Html::parse_document(html_content);

        let prev_chapter_id = {
            let sel = Selector::parse(".nextprev a[rel=\"prev\"]").expect("Failed to parse selector for previous chapter");
            document.select(&sel).next()
                .and_then(|e| e.value().attr("href"))
                .and_then(|href| href.split('/').nth(3))
                .unwrap_or("")
                .to_string()
        };
        assert_eq!(prev_chapter_id, "");

        let next_chapter_id = {
            let sel = Selector::parse(".nextprev a[rel=\"next\"]").expect("Failed to parse selector for next chapter");
            document.select(&sel).next()
                .and_then(|e| e.value().attr("href"))
                .and_then(|href| href.split('/').nth(3))
                .unwrap_or("")
                .to_string()
        };
        assert_eq!(next_chapter_id, "");

        let list_chapter = {
            let sel = Selector::parse(".nextprev a:has(.icol.daftarch)").expect("Failed to parse selector for list chapter");
            document.select(&sel).next()
                .and_then(|e| e.value().attr("href"))
                .and_then(|href| href.split('/').nth(4))
                .unwrap_or("")
                .to_string()
        };
        assert_eq!(list_chapter, "");
    }
}
