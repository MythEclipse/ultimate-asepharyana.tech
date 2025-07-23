use axum::{routing::{get, post}, Router, response::Redirect};
use sqlx::MySqlPool;
use std::sync::{Arc, Mutex};
use axum::extract::{State, WebSocketUpgrade, Multipart};
use axum::response::{IntoResponse, Response};
use axum::extract::ws::{Message, WebSocket};
use crate::models::ChatMessage;
use crate::chat_service;
use crate::pdf_service;
use axum::http::StatusCode;
use axum::body::Body;
use axum::Json;
use serde_json::json;
use futures::{sink::SinkExt, stream::StreamExt};
use tokio::sync::mpsc;

// Define a shared state struct
pub struct ChatState {
    pub pool: Arc<MySqlPool>,
    pub clients: Mutex<Vec<mpsc::UnboundedSender<Message>>>,
}

pub fn create_routes() -> Router<Arc<ChatState>> {
    Router::new()
        .route("/", get(root_handler))
        .route("/ws", get(ws_handler))
        .route("/merge-pdfs", post(merge_pdfs_handler))
        .route("/api/health", get(health_check))
        .route("/api/status", get(status_handler))
}

// Root handler - compatible with Express.js version
async fn root_handler() -> impl IntoResponse {
    // Redirect to the same URL as Express version for compatibility
    Redirect::permanent("https://asepharyana.cloud/chat")
}

// Health check endpoint
async fn health_check() -> impl IntoResponse {
    Json(json!({
        "status": "healthy",
        "service": "RustExpress",
        "version": "0.1.0",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

// Status endpoint for monitoring
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
        "features": ["websocket_chat", "pdf_merge", "mysql_database"]
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
    match chat_service::load_messages(&state.pool, 50).await {
        Ok(messages) => {
            let history_message = serde_json::json!({
                "type": "history",
                "messages": messages,
            });
            if tx.send(Message::Text(history_message.to_string())).is_err() {
                tracing::error!("Failed to send history to client channel");
            }
        }
        Err(e) => {
            tracing::error!("Failed to load history: {}", e);
        }
    }

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

                match serde_json::from_str::<ChatMessage>(&text) {
                    Ok(mut chat_message) => {
                        if chat_message.id.is_empty() {
                            chat_message.id = uuid::Uuid::new_v4().to_string();
                        }
                        if chat_message.timestamp.is_empty() {
                            chat_message.timestamp = chrono::Utc::now().to_rfc3339();
                        }

                        match chat_service::save_message(&recv_task_state.pool, &chat_message).await {
                            Ok(saved_message) => {
                                let broadcast_message = serde_json::json!({
                                    "type": "new_message",
                                    "message": saved_message,
                                });
                                let broadcast_text = broadcast_message.to_string();

                                let clients = recv_task_state.clients.lock().unwrap();
                                for client_tx in clients.iter() {
                                    if client_tx.send(Message::Text(broadcast_text.clone())).is_err() {
                                        tracing::warn!("Failed to send message to a client (channel closed)");
                                    }
                                }
                            }
                            Err(e) => {
                                tracing::error!("Failed to save message: {}", e);
                                // Optionally send an error back to the original client
                            }
                        }
                    }
                    Err(e) => {
                        tracing::error!("Failed to parse chat message: {}", e);
                    }
                }
            } else if let Message::Close(_) = msg {
                break;
            }
        }
    });

    // Wait for either task to finisha
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };

    // Client disconnected, remove it from the list
    let mut clients = state.clients.lock().unwrap();
    clients.retain(|client_tx| !client_tx.same_channel(&tx));
    tracing::info!("Client disconnected, {} clients remaining.", clients.len());
}

async fn merge_pdfs_handler(mut multipart: Multipart) -> Response {
    let mut files = Vec::new();
    let mut file_count = 0;

    // Extract files from multipart data - matching Express.js structure
    while let Some(field) = multipart.next_field().await.unwrap_or(None) {
        if field.name() == Some("files") {
            let data = field.bytes().await.unwrap_or_default();
            if !data.is_empty() {
                files.push(data);
                file_count += 1;
            }
        }
    }

    // Validate file count - same validation as Express version
    if file_count < 2 {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .header("Content-Type", "application/json")
            .body(Body::from(r#"{"error":"Please upload at least 2 PDF files"}"#))
            .unwrap();
    }

    // Process PDF merging
    match pdf_service::merge_pdfs(files).await {
        Ok(merged_pdf_bytes) => {
            Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/pdf")
                .header("Content-Disposition", "attachment; filename=\"merged.pdf\"")
                .body(Body::from(merged_pdf_bytes))
                .unwrap()
        }
        Err(e) => {
            tracing::error!("Failed to merge PDFs: {}", e);
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .header("Content-Type", "application/json")
                .body(Body::from(format!(r#"{{"error":"{}"}}"#, e)))
                .unwrap()
        }
    }
}