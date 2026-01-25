//! Scheduled task for cleaning up empty WebSocket rooms.

use async_trait::async_trait;
use std::sync::Arc;
use tracing::info;

use crate::ws::room::RoomManager;

use super::ScheduledTask;

/// Cleanup empty WebSocket rooms to prevent memory leaks.
/// Runs every 5 minutes to clean abandoned rooms.
pub struct CleanupEmptyRooms {
    room_manager: Arc<RoomManager>,
}

impl CleanupEmptyRooms {
    pub fn new(room_manager: Arc<RoomManager>) -> Self {
        Self { room_manager }
    }
}

#[async_trait]
impl ScheduledTask for CleanupEmptyRooms {
    fn name(&self) -> &'static str {
        "cleanup_empty_rooms"
    }

    fn schedule(&self) -> &'static str {
        // Every 5 minutes
        "0 */5 * * * *"
    }

    async fn run(&self) {
        let removed = self.room_manager.cleanup_empty_rooms();

        if removed > 0 {
            info!("🧹 Cleaned up {} empty WebSocket rooms", removed);
        }

        // Log metrics
        let total_rooms = self.room_manager.list_rooms().len();
        let total_users = self.room_manager.total_users();

        info!(
            "📊 Room stats: {} active rooms, {} total users",
            total_rooms, total_users
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cleanup_task_creation() {
        let manager = Arc::new(RoomManager::new());
        let task = CleanupEmptyRooms::new(manager);

        assert_eq!(task.name(), "cleanup_empty_rooms");
        assert_eq!(task.schedule(), "0 */5 * * * *");
    }

    #[tokio::test]
    async fn test_cleanup_removes_empty_rooms() {
        let manager = Arc::new(RoomManager::new());

        // Create empty room
        let _room1 = manager.get_or_create("test-room-1");
        let _room2 = manager.get_or_create("test-room-2");

        assert_eq!(manager.list_rooms().len(), 2);

        let task = CleanupEmptyRooms::new(manager.clone());
        task.run().await;

        // Both rooms should be removed as they're empty
        assert_eq!(manager.list_rooms().len(), 0);
    }
}
