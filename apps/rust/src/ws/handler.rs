//! WebSocket handler for Axum.

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
};
use futures::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::sync::broadcast;
use tracing::{debug, error, info};

use super::{RoomManager, WsEvent, WsMessage};

/// WebSocket state shared across handlers.
#[derive(Clone)]
pub struct WsState {
    pub rooms: Arc<RoomManager>,
}

impl WsState {
    /// Create new WebSocket state.
    pub fn new() -> Self {
        Self {
            rooms: Arc::new(RoomManager::new()),
        }
    }
}

impl Default for WsState {
    fn default() -> Self {
        Self::new()
    }
}

/// WebSocket upgrade handler.
///
/// # Example
///
/// ```ignore
/// use rust::ws::{ws_handler, WsState};
///
/// let ws_state = WsState::new();
/// router.route("/ws", get(ws_handler)).with_state(ws_state);
/// ```
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<WsState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: WsState) {
    let (mut sender, mut receiver) = socket.split();

    // Create broadcast channel for this connection
    let (tx, mut rx) = broadcast::channel::<String>(100);
    let user_id = uuid::Uuid::new_v4().to_string();

    info!("WebSocket connected: {}", user_id);

    // Send connected message
    let connected = WsMessage::connected().from(&user_id).to_json();
    if sender.send(Message::Text(connected.into())).await.is_err() {
        return;
    }

    // Spawn task to forward broadcast messages to client
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender.send(Message::Text(msg.into())).await.is_err() {
                break;
            }
        }
    });

    // Handle incoming messages
    let tx_clone = tx.clone();
    let user_id_clone = user_id.clone();
    let state_clone = state.clone();

    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                Message::Text(text) => {
                    handle_message(&text, &user_id_clone, &tx_clone, &state_clone).await;
                }
                Message::Binary(data) => {
                    if let Ok(text) = String::from_utf8(data.to_vec()) {
                        handle_message(&text, &user_id_clone, &tx_clone, &state_clone).await;
                    }
                }
                Message::Ping(_) => {
                    debug!("Ping from {}", user_id_clone);
                }
                Message::Pong(_) => {
                    debug!("Pong from {}", user_id_clone);
                }
                Message::Close(_) => {
                    info!("WebSocket closed: {}", user_id_clone);
                    break;
                }
            }
        }
    });

    // Wait for either task to complete
    tokio::select! {
        _ = &mut send_task => recv_task.abort(),
        _ = &mut recv_task => send_task.abort(),
    }

    // Cleanup: remove from all rooms
    for room_name in state.rooms.list_rooms() {
        if let Some(room) = state.rooms.get(&room_name) {
            room.leave(&user_id);
            state.rooms.remove_if_empty(&room_name);
        }
    }

    info!("WebSocket disconnected: {}", user_id);
}

async fn handle_message(
    text: &str,
    user_id: &str,
    tx: &broadcast::Sender<String>,
    state: &WsState,
) {
    let msg = match WsMessage::from_json(text) {
        Ok(m) => m,
        Err(e) => {
            error!("Invalid message format: {}", e);
            let _ = tx.send(WsMessage::error("Invalid message format").to_json());
            return;
        }
    };

    match msg.event {
        WsEvent::Ping => {
            let _ = tx.send(WsMessage::pong().to_json());
        }
        WsEvent::Join => {
            if let Some(room_name) = msg.room {
                let room = state.rooms.get_or_create(&room_name);
                room.join(user_id, tx.clone());
                
                let response = WsMessage::new(WsEvent::Join, serde_json::json!({
                    "room": room_name,
                    "user_id": user_id,
                    "members": room.member_count()
                }));
                room.broadcast(&response.to_json());
            }
        }
        WsEvent::Leave => {
            if let Some(room_name) = msg.room {
                if let Some(room) = state.rooms.get(&room_name) {
                    room.leave(user_id);
                    
                    let response = WsMessage::new(WsEvent::Leave, serde_json::json!({
                        "room": room_name,
                        "user_id": user_id
                    }));
                    room.broadcast(&response.to_json());
                    
                    state.rooms.remove_if_empty(&room_name);
                }
            }
        }
        WsEvent::Message | WsEvent::Broadcast => {
            if let Some(room_name) = msg.room {
                if let Some(room) = state.rooms.get(&room_name) {
                    let response = WsMessage::new(WsEvent::Message, msg.data)
                        .room(&room_name)
                        .from(user_id);
                    room.broadcast(&response.to_json());
                }
            } else {
                // Echo back
                let _ = tx.send(WsMessage::new(WsEvent::Message, msg.data).from(user_id).to_json());
            }
        }
        WsEvent::Private => {
            if let Some(target_user) = msg.to {
                // Find target user in any room
                for room_name in state.rooms.list_rooms() {
                    if let Some(room) = state.rooms.get(&room_name) {
                        if room.has_member(&target_user) {
                            if let Some(entry) = room.members.get(&target_user) {
                                let response = WsMessage::new(WsEvent::Private, msg.data)
                                    .from(user_id)
                                    .to(&target_user);
                                let _ = entry.value().send(response.to_json());
                            }
                            break;
                        }
                    }
                }
            }
        }
        _ => {}
    }
}
