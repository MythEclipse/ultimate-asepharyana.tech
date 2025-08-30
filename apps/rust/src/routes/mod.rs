use axum::{routing::{get}, Router, response::Redirect};
use sqlx::MySqlPool;
use std::sync::{Arc, Mutex};
use axum::extract::{State, WebSocketUpgrade};
use axum::response::{IntoResponse};
use axum::extract::ws::{Message, WebSocket};
use rust_lib::models::chat_message::ChatMessage;
use rust_lib::services::chat;
use tokio::sync::mpsc;
use axum::Json;
use serde_json::json;
use futures::StreamExt;
use futures::SinkExt;
pub mod api; // Declare the new top-level api module


pub struct ChatState {
    pub pool: Arc<MySqlPool>,
    pub clients: Mutex<Vec<mpsc::UnboundedSender<Message>>>,
    #[allow(dead_code)]
    pub jwt_secret: String,
}

pub fn create_routes() -> Router<Arc<ChatState>> {
    Router::new()
        .route("/", get(root_handler))
        .route("/ws", get(ws_handler))
        .route("/api/health", get(health_check))
        .route("/api/status", get(status_handler))
        .nest("/api", api::create_api_routes()) // Nest all API routes under /api
}

// Root handler - compatible with Express.js version
async fn root_handler() -> impl IntoResponse {
    Redirect::permanent("https://asepharyana.tech/chat")
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
    match chat::load_messages(&state.pool, 50).await {
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
                        // Remove is_empty check for timestamp; always set to now if needed
                        // if chat_message.timestamp.is_empty() {
                        //     chat_message.timestamp = chrono::Utc::now().to_rfc3339();
                        // }
                        // Instead, set to now if timestamp is the default (1970-01-01)
                        if chat_message.timestamp == chrono::DateTime::from_timestamp(0, 0).unwrap().naive_utc() {
                            chat_message.timestamp = chrono::Utc::now().naive_utc();
                        }

                        match chat::save_message(&recv_task_state.pool, &chat_message).await {
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


