use std::collections::HashMap;

pub fn get_cors_headers() -> HashMap<&'static str, &'static str> {
    let mut headers = HashMap::new();
    headers.insert("Access-Control-Allow-Origin", "*");
    headers.insert("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS");
    headers.insert("Access-Control-Allow-Headers", "Content-Type, Authorization");
    headers
}

// The `cors` function from TypeScript would typically be handled by a web framework in Rust.
// For example, using `actix-web` or `warp`, you would configure CORS middleware.
// This library will only provide the header values.
