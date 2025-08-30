use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::Value;
use std::fs;
use std::path::PathBuf;
use serde_yaml;

use axum::{routing::get, Router};

pub fn create_routes() -> Router<Arc<ChatState>> {
    Router::new()
        .route("/", get(docs_handler))
}

pub async fn docs_handler() -> Response {
    let mut current_dir = std::env::current_dir().expect("Failed to get current directory");
    current_dir.push("public");
    current_dir.push("OpenApi.yaml");

    let file_path = current_dir;

    match fs::read_to_string(&file_path) {
        Ok(yaml_content) => {
            match serde_yaml::from_str::<Value>(&yaml_content) {
                Ok(json_data) => (StatusCode::OK, Json(json_data)).into_response(),
                Err(e) => {
                    eprintln!("Error parsing YAML: {:?}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json("Failed to parse OpenAPI YAML".into()),
                    )
                        .into_response()
                }
            }
        }
        Err(e) => {
            eprintln!("Error reading OpenAPI.yaml: {:?}", e);
            (
                StatusCode::NOT_FOUND,
                Json("OpenAPI.yaml not found".into()),
            )
                .into_response()
        }
    }
}
