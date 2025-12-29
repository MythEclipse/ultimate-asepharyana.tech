//! Broadcaster implementation for real-time events.

use axum::response::sse::{Event as SseEvent, Sse};
use futures::stream::Stream;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};

/// Broadcast event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    /// Event type/name.
    pub event: String,
    /// Event data (JSON).
    pub data: serde_json::Value,
    /// Optional event ID.
    pub id: Option<String>,
}

impl Event {
    /// Create a new event.
    pub fn new(event: &str, data: serde_json::Value) -> Self {
        Self {
            event: event.to_string(),
            data,
            id: None,
        }
    }

    /// Create with ID.
    pub fn with_id(event: &str, data: serde_json::Value, id: &str) -> Self {
        Self {
            event: event.to_string(),
            data,
            id: Some(id.to_string()),
        }
    }

    /// Convert to SSE event.
    pub fn to_sse(&self) -> SseEvent {
        let mut sse = SseEvent::default()
            .event(&self.event)
            .data(self.data.to_string());

        if let Some(id) = &self.id {
            sse = sse.id(id);
        }

        sse
    }
}

/// Broadcast channel.
pub struct Channel {
    tx: broadcast::Sender<Event>,
}

impl Channel {
    /// Create a new channel.
    pub fn new(capacity: usize) -> Self {
        let (tx, _) = broadcast::channel(capacity);
        Self { tx }
    }

    /// Subscribe to channel.
    pub fn subscribe(&self) -> broadcast::Receiver<Event> {
        self.tx.subscribe()
    }

    /// Broadcast event.
    pub fn broadcast(&self, event: Event) -> usize {
        self.tx.send(event).unwrap_or(0)
    }
}

/// Multi-channel broadcaster.
#[derive(Clone)]
pub struct Broadcaster {
    channels: Arc<RwLock<HashMap<String, Arc<Channel>>>>,
    default_capacity: usize,
}

impl Default for Broadcaster {
    fn default() -> Self {
        Self::new()
    }
}

impl Broadcaster {
    /// Create a new broadcaster.
    pub fn new() -> Self {
        Self {
            channels: Arc::new(RwLock::new(HashMap::new())),
            default_capacity: 100,
        }
    }

    /// Create with custom channel capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            channels: Arc::new(RwLock::new(HashMap::new())),
            default_capacity: capacity,
        }
    }

    /// Get or create a channel.
    pub async fn channel(&self, name: &str) -> Arc<Channel> {
        let channels = self.channels.read().await;
        if let Some(ch) = channels.get(name) {
            return ch.clone();
        }
        drop(channels);

        let mut channels = self.channels.write().await;
        if let Some(ch) = channels.get(name) {
            return ch.clone();
        }

        let channel = Arc::new(Channel::new(self.default_capacity));
        channels.insert(name.to_string(), channel.clone());
        channel
    }

    /// Subscribe to a channel.
    pub async fn subscribe(&self, channel: &str) -> broadcast::Receiver<Event> {
        self.channel(channel).await.subscribe()
    }

    /// Broadcast to a channel.
    pub async fn broadcast(&self, channel: &str, event: Event) -> usize {
        self.channel(channel).await.broadcast(event)
    }

    /// Broadcast to multiple channels.
    pub async fn broadcast_many(&self, channels: &[&str], event: Event) -> usize {
        let mut total = 0;
        for ch in channels {
            total += self.broadcast(ch, event.clone()).await;
        }
        total
    }

    /// Broadcast to a specific user.
    pub async fn broadcast_to_user(&self, user_id: &str, event: Event) -> usize {
        self.broadcast(&format!("user:{}", user_id), event).await
    }

    /// List active channels.
    pub async fn channels_list(&self) -> Vec<String> {
        self.channels.read().await.keys().cloned().collect()
    }
}

/// SSE stream wrapper for broadcast receiver.
pub struct SseStream {
    rx: broadcast::Receiver<Event>,
}

impl SseStream {
    pub fn new(rx: broadcast::Receiver<Event>) -> Self {
        Self { rx }
    }
}

impl Stream for SseStream {
    type Item = Result<SseEvent, Infallible>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        use std::task::Poll;

        match self.rx.try_recv() {
            Ok(event) => Poll::Ready(Some(Ok(event.to_sse()))),
            Err(broadcast::error::TryRecvError::Empty) => {
                cx.waker().wake_by_ref();
                Poll::Pending
            }
            Err(broadcast::error::TryRecvError::Lagged(_)) => {
                cx.waker().wake_by_ref();
                Poll::Pending
            }
            Err(broadcast::error::TryRecvError::Closed) => Poll::Ready(None),
        }
    }
}

/// Create SSE response from broadcaster.
pub async fn sse_handler(broadcaster: &Broadcaster, channel: &str) -> Sse<SseStream> {
    let rx = broadcaster.subscribe(channel).await;
    Sse::new(SseStream::new(rx))
}

/// Broadcaster extension for Axum.
pub type BroadcasterExtension = axum::Extension<Broadcaster>;
