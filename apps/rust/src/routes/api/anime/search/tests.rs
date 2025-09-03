#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;
    use http::StatusCode;

    // Helper function to create a mock HTML response
    fn create_mock_html(body: &str) -> String {
        format!("<html><body>{}</body></html>", body)
    }

    #[test]
    async fn test_handler_success() {
        // This test would require mocking reqwest client, which is complex.
        // For now, we'll focus on parsing logic.
        // A more complete test would use a mock HTTP server.
    }

    #[test]
    async fn test_parse_anime_data_single_item() {
        let html_content = r##"
            <div id="venkonten">
                <ul class="chivsrc">
                    <li>
                        <h2><a href="https://otakudesu.cloud/anime/example-anime/">Example Anime (Episode 12)</a></h2>
                        <img src="https://otakudesu.cloud/poster.jpg" alt="Poster">
                        <div class="set">
                            <b>Genres :</b> <a href="#">Action</a>, <a href="#">Adventure</a>
                            <b>Status :</b> Completed
                            <b>Rating :</b> 8.5
                        </div>
                    </li>
                </ul>
            </div>
        "##;
        let q = "example";
        let (anime_list, pagination) = parse_anime_data(html_content, q);

        assert_eq!(anime_list.len(), 1);
        let item = &anime_list[0];
        assert_eq!(item.title, "Example Anime");
        assert_eq!(item.slug, "example-anime");
        assert_eq!(item.poster, "https://otakudesu.cloud/poster.jpg");
        assert_eq!(item.episode, "Episode 12");
        assert_eq!(item.anime_url, "https://otakudesu.cloud/anime/example-anime/");
        assert_eq!(item.genres, vec!["Action".to_string(), "Adventure".to_string()]);
        assert_eq!(item.status, "Completed");
        assert_eq!(item.rating, "8.5");

        assert_eq!(pagination.current_page, 1);
        assert!(!pagination.has_next_page);
        assert_eq!(pagination.next_page, None);
        assert!(!pagination.has_previous_page);
        assert_eq!(pagination.previous_page, None);
    }

    #[test]
    async fn test_parse_anime_data_multiple_items() {
        let html_content = r##"
            <div id="venkonten">
                <ul class="chivsrc">
                    <li>
                        <h2><a href="https://otakudesu.cloud/anime/anime-1/">Anime One (Episode 1)</a></h2>
                        <img src="https://otakudesu.cloud/poster1.jpg" alt="Poster">
                        <div class="set">
                            <b>Genres :</b> <a href="#">Action</a>
                            <b>Status :</b> Ongoing
                            <b>Rating :</b> 7.0
                        </div>
                    </li>
                    <li>
                        <h2><a href="https://otakudesu.cloud/anime/anime-2/">Anime Two (Episode 2)</a></h2>
                        <img src="https://otakudesu.cloud/poster2.jpg" alt="Poster">
                        <div class="set">
                            <b>Genres :</b> <a href="#">Comedy</a>
                            <b>Status :</b> Completed
                            <b>Rating :</b> 9.0
                        </div>
                    </li>
                </ul>
                <div class="hpage"><span class="r">Next Page</span></div>
            </div>
        "##;
        let q = "1";
        let (anime_list, pagination) = parse_anime_data(html_content, q);

        assert_eq!(anime_list.len(), 2);
        assert_eq!(anime_list[0].title, "Anime One");
        assert_eq!(anime_list[1].title, "Anime Two");

        assert_eq!(pagination.current_page, 1);
        assert!(pagination.has_next_page);
        assert_eq!(pagination.next_page, Some(2));
        assert!(!pagination.has_previous_page);
        assert_eq!(pagination.previous_page, None);
    }

    #[test]
    async fn test_parse_anime_data_no_items() {
        let html_content = r##"
            <div id="venkonten">
                <ul class="chivsrc"></ul>
            </div>
        "##;
        let q = "noresult";
        let (anime_list, pagination) = parse_anime_data(html_content, q);

        assert!(anime_list.is_empty());
        assert_eq!(pagination.current_page, 1);
        assert!(!pagination.has_next_page);
        assert_eq!(pagination.next_page, None);
        assert!(!pagination.has_previous_page);
        assert_eq!(pagination.previous_page, None);
    }
}
