//! WebSocket room management.

use dashmap::DashMap;
use std::sync::Arc;
use tokio::sync::broadcast;
use tracing::{debug, info};

/// A WebSocket room that users can join/leave.
pub struct Room {
    pub name: String,
    pub members: DashMap<String, broadcast::Sender<String>>,
}

impl Room {
    /// Create a new room.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            members: DashMap::new(),
        }
    }

    /// Add a member to the room.
    pub fn join(&self, user_id: &str, sender: broadcast::Sender<String>) {
        self.members.insert(user_id.to_string(), sender);
        debug!("User {} joined room {}", user_id, self.name);
    }

    /// Remove a member from the room.
    pub fn leave(&self, user_id: &str) {
        self.members.remove(user_id);
        debug!("User {} left room {}", user_id, self.name);
    }

    /// Broadcast a message to all members.
    pub fn broadcast(&self, message: &str) {
        for entry in self.members.iter() {
            let _ = entry.value().send(message.to_string());
        }
    }

    /// Broadcast to all except sender.
    pub fn broadcast_except(&self, message: &str, exclude: &str) {
        for entry in self.members.iter() {
            if entry.key() != exclude {
                let _ = entry.value().send(message.to_string());
            }
        }
    }

    /// Get member count.
    pub fn member_count(&self) -> usize {
        self.members.len()
    }

    /// Check if user is in room.
    pub fn has_member(&self, user_id: &str) -> bool {
        self.members.contains_key(user_id)
    }
}

/// Manages multiple WebSocket rooms.
pub struct RoomManager {
    rooms: DashMap<String, Arc<Room>>,
}

impl RoomManager {
    /// Create a new room manager.
    pub fn new() -> Self {
        Self {
            rooms: DashMap::new(),
        }
    }

    /// Get or create a room.
    pub fn get_or_create(&self, name: &str) -> Arc<Room> {
        self.rooms
            .entry(name.to_string())
            .or_insert_with(|| Arc::new(Room::new(name)))
            .clone()
    }

    /// Get a room if it exists.
    pub fn get(&self, name: &str) -> Option<Arc<Room>> {
        self.rooms.get(name).map(|r| r.clone())
    }

    /// Remove a room if empty.
    pub fn remove_if_empty(&self, name: &str) {
        if let Some(room) = self.rooms.get(name) {
            if room.member_count() == 0 {
                self.rooms.remove(name);
                info!("Removed empty room: {}", name);
            }
        }
    }

    /// List all room names.
    pub fn list_rooms(&self) -> Vec<String> {
        self.rooms.iter().map(|r| r.key().clone()).collect()
    }

    /// Get total user count across all rooms.
    pub fn total_users(&self) -> usize {
        self.rooms.iter().map(|r| r.member_count()).sum()
    }
}

impl Default for RoomManager {
    fn default() -> Self {
        Self::new()
    }
}
