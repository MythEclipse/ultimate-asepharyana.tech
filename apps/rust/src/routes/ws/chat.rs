use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use futures::{sink::SinkExt, stream::StreamExt};
use std::sync::Arc;

use super::models::WsMessage;
use crate::routes::AppState;

pub async fn chat_websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| websocket_connection(socket, state))
}

async fn websocket_connection(socket: WebSocket, state: Arc<AppState>) {
    let (mut sender, mut receiver) = socket.split();

    // Subscribe to broadcast channel
    let mut rx = state.chat_tx.subscribe();

    // Task for receiving messages from broadcast and sending to client
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            let text = serde_json::to_string(&msg).unwrap_or_default();
            if sender.send(Message::Text(text.into())).await.is_err() {
                break;
            }
        }
    });

    // Task for receiving messages from client and broadcasting
    let tx = state.chat_tx.clone();
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if let Message::Text(text) = msg {
                if let Ok(ws_msg) = serde_json::from_str::<WsMessage>(&text) {
                    let _ = tx.send(ws_msg);
                }
            }
        }
    });

    // Wait for either task to finish and ensure proper cleanup
    tokio::select! {
        result = &mut send_task => {
            recv_task.abort();
            // Wait for recv_task to actually finish
            let _ = recv_task.await;
            if let Err(e) = result {
                tracing::warn!("Send task error: {:?}", e);
            }
        },
        result = &mut recv_task => {
            send_task.abort();
            // Wait for send_task to actually finish  
            let _ = send_task.await;
            if let Err(e) = result {
                tracing::warn!("Recv task error: {:?}", e);
            }
        }
    }
    
    tracing::info!("WebSocket connection closed and cleaned up");
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route("/ws/chat", get(chat_websocket_handler))
}

