use std::collections::HashMap;
use tracing::{info, error};
use serde_json::{json, Value};

// This struct would represent the incoming request properties, similar to NextRequest and props.params
// For a generic library, we'll use basic types.
pub struct RequestInfo {
    pub ip: Option<String>,
    pub url: String,
    pub method: String,
    pub request_id: Option<String>,
}

// This struct represents the response from the handler, similar to NextResponse.
// In a real web framework, this would be a framework-specific response object.
pub struct HandlerResponse {
    pub status: u16,
    pub body: Value,
    pub headers: HashMap<String, String>,
}

fn format_log_context(context: &HashMap<&str, &dyn std::fmt::Display>) -> String {
    context.iter()
        .filter_map(|(&k, v)| {
            if format!("{}", v) != "undefined" {
                Some(format!("{}={}", k, v))
            } else {
                None
            }
        })
        .collect::<Vec<String>>()
        .join(" | ")
}

// A generic API handler trait that can be implemented by specific API logic.
// The `Output` type would typically be a web framework's Response type.
pub trait ApiHandler {
    type Error: Into<Box<dyn Error>>;
    async fn handle(&self, req_info: RequestInfo) -> Result<HandlerResponse, Self::Error>;
}

// This function wraps an ApiHandler with logging and error handling.
pub async fn with_logging<T: ApiHandler>(handler: &T, req_info: RequestInfo) -> HandlerResponse {
    let start = std::time::Instant::now();
    let ip = req_info.ip.clone();
    let url = req_info.url.clone();
    let method = req_info.method.clone();
    let request_id = req_info.request_id.clone();

    match handler.handle(req_info).await {
        Ok(response) => {
            let duration = start.elapsed().as_millis();
            let mut log_context = HashMap::new();
            log_context.insert("ip", &ip);
            log_context.insert("url", &url);
            log_context.insert("method", &method);
            log_context.insert("status", &response.status);
            log_context.insert("durationMs", &duration);
            if let Some(id) = &request_id {
                log_context.insert("requestId", id);
            }
            info!("[Request processed] {}", format_log_context(&log_context));

            let mut final_headers = response.headers;
            if let Some(id) = request_id {
                final_headers.insert("x-request-id".to_string(), id);
            }
            HandlerResponse {
                status: response.status,
                body: response.body,
                headers: final_headers,
            }
        },
        Err(error) => {
            let error_message = error.into().to_string();
            let duration = start.elapsed().as_millis();
            let mut log_context = HashMap::new();
            log_context.insert("ip", &ip);
            log_context.insert("url", &url);
            log_context.insert("method", &method);
            log_context.insert("error", &error_message);
            log_context.insert("durationMs", &duration);
            if let Some(id) = &request_id {
                log_context.insert("requestId", id);
            }
            error!("[Error processing request] {}", format_log_context(&log_context));

            let mut headers = HashMap::new();
            headers.insert("Content-Type".to_string(), "application/json".to_string());
            // Assuming corsHeaders are handled by the web framework or added here if needed
            // For now, just basic CORS
            headers.insert("Access-Control-Allow-Origin".to_string(), "*".to_string());
            headers.insert("Access-Control-Allow-Methods".to_string(), "GET, POST, PUT, DELETE, OPTIONS".to_string());
            headers.insert("Access-Control-Allow-Headers".to_string(), "Content-Type, Authorization".to_string());

            if let Some(id) = request_id {
                headers.insert("x-request-id".to_string(), id);
            }

            HandlerResponse {
                status: 500,
                body: json!({
                    "message": "Failed to process request",
                    "error": error_message,
                }),
                headers,
            }
        }
    }
}
