//! Routing module for chat application
//! All environment variables (including JWT secret) must be loaded via rust_lib::config::CONFIG_MAP

use axum::{
    routing::{get},
    Router,
    response::Redirect,
    extract::{State, WebSocketUpgrade},
    response::{IntoResponse},
};
use sqlx::MySqlPool;
use std::sync::{Arc, Mutex};
use axum::extract::ws::{Message, WebSocket};
use tokio::sync::mpsc;
use axum::Json;
use serde_json::json;
use futures::StreamExt;
use futures::SinkExt;
pub mod api; // Declare the new top-level api module

pub struct ChatState {
    pub pool: Arc<MySqlPool>,
    pub clients: Mutex<Vec<mpsc::UnboundedSender<Message>>>,
    /// JWT secret loaded from CONFIG_MAP
    #[allow(dead_code)]
    pub jwt_secret: String,
}

pub fn create_routes() -> Router<Arc<ChatState>> {
    Router::new()
        .route("/", get(root_handler))
        .route("/ws", get(ws_handler))
        .route("/api/health", get(health_check))
        .route("/api/status", get(status_handler))
        // .nest("/api", api::create_api_routes())
}

async fn root_handler() -> impl IntoResponse {
    Redirect::permanent("https://asepharyana.tech/chat")
}

// Health check endpoint
#[utoipa::path(
    get,
    path = "/api/health",
    responses(
        (status = 200, description = "Health check OK")
    )
)]
async fn health_check() -> impl IntoResponse {
    Json(json!({
        "status": "healthy",
        "service": "RustExpress",
        "version": "0.1.0",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

// Status endpoint for monitoring
#[utoipa::path(
    get,
    path = "/api/status",
    responses(
        (status = 200, description = "Status OK")
    )
)]
async fn status_handler(State(state): State<Arc<ChatState>>) -> impl IntoResponse {
    // Check database connection
    let db_status = match sqlx::query("SELECT 1").execute(state.pool.as_ref()).await {
        Ok(_) => "connected",
        Err(_) => "disconnected",
    };

    Json(json!({
        "status": "running",
        "database": db_status,
        "service": "RustExpress (Rust migration of Express.js)",
        "features": ["websocket_chat", "mysql_database"]
    }))
}

async fn ws_handler(ws: WebSocketUpgrade, State(state): State<Arc<ChatState>>) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: Arc<ChatState>) {
    let (mut sender, mut receiver) = socket.split();

    // Create a channel for this client
    let (tx, mut rx) = mpsc::unbounded_channel::<Message>();

    // Add the client's sender to the shared state
    state.clients.lock().unwrap().push(tx.clone());

    // Send historical messages
    // chat module is missing; skipping loading messages

    // This task forwards messages from the client's channel to the WebSocket sender
    let mut send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if sender.send(msg).await.is_err() {
                break;
            }
        }
    });

    // This task receives messages from the WebSocket and broadcasts them
    let recv_task_state = Arc::clone(&state);
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if let Message::Text(text) = msg {
                tracing::info!("Received message: {}", text);

                // ChatMessage type and chat module missing; skipping message handling
            } else if let Message::Close(_) = msg {
                break;
            }
        }
    });

    // Wait for either task to finish
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };

    // Client disconnected, remove it from the list
    let mut clients = state.clients.lock().unwrap();
    clients.retain(|client_tx| !client_tx.same_channel(&tx));
    tracing::info!("Client disconnected, {} clients remaining.", clients.len());
}
