#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
        routing::get,
        Router,
    };
    use tower::ServiceExt; // for `oneshot` and `ready`
    use serde_json::json;

    // A mock compress_service for testing
    mod mock_compress_service {
        pub async fn compress_image_from_url(_url: &str, _size: &super::CompressionSize) -> anyhow::Result<String> {
            Ok("http://mock.com/compressed_image.jpg".to_string())
        }

        pub async fn compress_video_from_url(_url: &str, _size: &super::CompressionSize) -> anyhow::Result<String> {
            Ok("http://mock.com/compressed_video.mp4".to_string())
        }
    }

    // Override the compress_service module with our mock for testing
    // This requires some trickery with Rust's module system and is typically done with
    // a feature flag or conditional compilation if you want to swap out implementations.
    // For a simple test, we can just define a mock within the test module.
    // In a real project, consider using dependency injection or traits for easier mocking.

    async fn create_test_app() -> Router {
        // We're not using ChatState for this handler, but the Router expects it.
        // Provide a dummy Arc<ChatState> for testing purposes.
        let pool = sqlx::mysql::MySqlPoolOptions::new()
            .connect("mysql://user:password@localhost:3306/test_db") // Dummy URL
            .await
            .unwrap();
        let chat_state = std::sync::Arc::new(crate::routes::ChatState {
            pool: std::sync::Arc::new(pool),
            clients: Default::default(),
            jwt_secret: "test_secret".to_string(),
        });

        Router::new().route("/api/compress", get(handler)).with_state(chat_state)
    }

    #[tokio::test]
    async fn test_compress_handler_image_success() {
        let app = create_test_app().await;

        let request = Request::builder()
            .uri("/api/compress?url=http://example.com/image.jpg&size=100kb")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["compressed_url"], "http://mock.com/compressed_image.jpg");
    }

    #[tokio::test]
    async fn test_compress_handler_video_success() {
        let app = create_test_app().await;

        let request = Request::builder()
            .uri("/api/compress?url=http://example.com/video.mp4&size=50%")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["compressed_url"], "http://mock.com/compressed_video.mp4");
    }

    #[tokio::test]
    async fn test_compress_handler_missing_url() {
        let app = create_test_app().await;

        let request = Request::builder()
            .uri("/api/compress?size=100kb")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["message"], "URL parameter is required");
    }

    #[tokio::test]
    async fn test_compress_handler_invalid_size_format() {
        let app = create_test_app().await;

        let request = Request::builder()
            .uri("/api/compress?url=http://example.com/image.jpg&size=invalid")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert!(json["message"].as_str().unwrap().contains("Invalid size format"));
    }

    #[tokio::test]
    async fn test_compression_size_from_str_kilobytes() {
        let size = "123kb".parse::<CompressionSize>().unwrap();
        match size {
            CompressionSize::Kilobytes(val) => assert_eq!(val, 123),
            _ => panic!("Expected Kilobytes"),
        }
    }

    #[tokio::test]
    async fn test_compression_size_from_str_percentage() {
        let size = "75%".parse::<CompressionSize>().unwrap();
        match size {
            CompressionSize::Percentage(val) => assert_eq!(val, 75),
            _ => panic!("Expected Percentage"),
        }
    }

    #[tokio::test]
    async fn test_compression_size_from_str_invalid_kb() {
        let err = "abc_kb".parse::<CompressionSize>().unwrap_err();
        assert!(err.contains("Invalid kilobytes value"));
    }

    #[tokio::test]
    async fn test_compression_size_from_str_invalid_percentage() {
        let err = "120%".parse::<CompressionSize>().unwrap_err();
        assert!(err.contains("Percentage cannot be greater than 100"));
    }

    #[tokio::test]
    async fn test_compression_size_from_str_unknown_format() {
        let err = "123".parse::<CompressionSize>().unwrap_err();
        assert!(err.contains("Invalid size format"));
    }
}


pub fn register_routes(router: Router<Arc<ChatState>>) -> Router<Arc<ChatState>> {
    router.route(ENDPOINT_PATH, axum::routing::get(compress_image_from_url))
}