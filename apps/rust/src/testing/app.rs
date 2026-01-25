//! Test application utilities.
//!
//! Provides a `TestApp` struct for integration testing that boots
//! the application in-memory with a test configuration.

use axum::{
    body::Body,
    http::{Method, Request, StatusCode},
    response::Response,
    Router,
};
use serde::{de::DeserializeOwned, Serialize};
use tower::ServiceExt;

/// A test application instance for integration testing.
///
/// # Example
///
/// ```ignore
/// use rustexpress::testing::TestApp;
///
/// #[tokio::test]
/// async fn test_health_endpoint() {
///     let app = TestApp::new().await;
///     
///     let response = app.get("/health").await;
///     assert_eq!(response.status(), 200);
///     
///     let body = response.json::<HealthResponse>().await;
///     assert_eq!(body.status, "ok");
/// }
/// ```
pub struct TestApp {
    router: Router,
}

impl TestApp {
    /// Create a new test application with default configuration.
    ///
    /// This sets up the router without starting a server.
    pub fn with_router(router: Router) -> Self {
        Self { router }
    }

    /// Make a GET request.
    pub async fn get(&self, path: &str) -> TestResponse {
        self.request(Method::GET, path, Body::empty()).await
    }

    /// Make a POST request with JSON body.
    pub async fn post<T: Serialize>(&self, path: &str, body: &T) -> TestResponse {
        let body = serde_json::to_string(body).unwrap_or_default();
        self.request_with_json(Method::POST, path, body).await
    }

    /// Make a PUT request with JSON body.
    pub async fn put<T: Serialize>(&self, path: &str, body: &T) -> TestResponse {
        let body = serde_json::to_string(body).unwrap_or_default();
        self.request_with_json(Method::PUT, path, body).await
    }

    /// Make a DELETE request.
    pub async fn delete(&self, path: &str) -> TestResponse {
        self.request(Method::DELETE, path, Body::empty()).await
    }

    /// Make a request with custom method and body.
    pub async fn request(&self, method: Method, path: &str, body: Body) -> TestResponse {
        let request = Request::builder()
            .method(method)
            .uri(path)
            .body(body)
            .unwrap_or_else(|_| Request::new(Body::empty()));

        let response = self
            .router
            .clone()
            .oneshot(request)
            .await
            .unwrap_or_else(|_| Response::builder().status(500).body(Body::empty()).unwrap());

        TestResponse::new(response)
    }

    /// Make a request with JSON content type.
    async fn request_with_json(&self, method: Method, path: &str, body: String) -> TestResponse {
        let request = Request::builder()
            .method(method)
            .uri(path)
            .header("Content-Type", "application/json")
            .body(Body::from(body))
            .unwrap_or_else(|_| Request::new(Body::empty()));

        let response = self
            .router
            .clone()
            .oneshot(request)
            .await
            .unwrap_or_else(|_| Response::builder().status(500).body(Body::empty()).unwrap());

        TestResponse::new(response)
    }
}

/// A test response with assertion helpers.
pub struct TestResponse {
    response: Response<Body>,
    body: Option<bytes::Bytes>,
}

impl TestResponse {
    fn new(response: Response<Body>) -> Self {
        Self {
            response,
            body: None,
        }
    }

    /// Get the response status code.
    pub fn status(&self) -> StatusCode {
        self.response.status()
    }

    /// Assert the status code.
    pub fn assert_status(self, expected: u16) -> Self {
        assert_eq!(
            self.response.status().as_u16(),
            expected,
            "Expected status {} but got {}",
            expected,
            self.response.status()
        );
        self
    }

    /// Assert the status is 2xx (success).
    pub fn assert_success(self) -> Self {
        assert!(
            self.response.status().is_success(),
            "Expected success status but got {}",
            self.response.status()
        );
        self
    }

    /// Assert the status is 4xx (client error).
    pub fn assert_client_error(self) -> Self {
        assert!(
            self.response.status().is_client_error(),
            "Expected client error status but got {}",
            self.response.status()
        );
        self
    }

    /// Get a header value.
    pub fn header(&self, name: &str) -> Option<&str> {
        self.response
            .headers()
            .get(name)
            .and_then(|v| v.to_str().ok())
    }

    /// Get the response body as bytes.
    pub async fn bytes(mut self) -> bytes::Bytes {
        if let Some(body) = self.body.take() {
            return body;
        }

        let body = std::mem::replace(self.response.body_mut(), Body::empty());
        axum::body::to_bytes(body, usize::MAX)
            .await
            .unwrap_or_default()
    }

    /// Get the response body as a string.
    pub async fn text(self) -> String {
        let bytes = self.bytes().await;
        String::from_utf8_lossy(&bytes).to_string()
    }

    /// Get the response body as JSON.
    pub async fn json<T: DeserializeOwned>(self) -> T {
        let bytes = self.bytes().await;
        serde_json::from_slice(&bytes).expect("Failed to parse response as JSON")
    }

    /// Assert the response body contains a string.
    pub async fn assert_body_contains(self, expected: &str) -> Self {
        let body = self.text().await;
        assert!(
            body.contains(expected),
            "Expected body to contain '{}' but got: {}",
            expected,
            body
        );
        // Recreate self with cached body (simplified)
        Self {
            response: Response::builder().status(200).body(Body::empty()).unwrap(),
            body: Some(bytes::Bytes::from(body)),
        }
    }
}

/// Builder for TestApp with custom configuration.
pub struct TestAppBuilder {
    // Future: Add configuration options
}

impl TestAppBuilder {
    /// Create a new test app builder.
    pub fn new() -> Self {
        Self {}
    }

    /// Build the test app with a router.
    pub fn build(self, router: Router) -> TestApp {
        TestApp::with_router(router)
    }
}

impl Default for TestAppBuilder {
    fn default() -> Self {
        Self::new()
    }
}
