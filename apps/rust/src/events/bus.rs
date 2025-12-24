//! Event bus implementation.

use async_trait::async_trait;

use std::{any::TypeId, collections::HashMap, sync::Arc};
use tokio::sync::{broadcast, RwLock};
use tracing::{debug, info};

/// Trait for events that can be published.
pub trait Event: Clone + Send + Sync + 'static {
    /// Event name for logging/debugging.
    const NAME: &'static str;
}

/// Trait for event handlers.
#[async_trait]
pub trait EventHandler<E: Event>: Send + Sync {
    async fn handle(&self, event: E);
}

/// The event bus for publishing and subscribing to events.
pub struct EventBus {
    channels: RwLock<HashMap<TypeId, Box<dyn std::any::Any + Send + Sync>>>,
}

impl EventBus {
    /// Create a new event bus.
    pub fn new() -> Self {
        Self {
            channels: RwLock::new(HashMap::new()),
        }
    }

    /// Publish an event to all subscribers.
    pub async fn publish<E: Event>(&self, event: E) {
        let type_id = TypeId::of::<E>();
        let channels = self.channels.read().await;

        if let Some(sender) = channels.get(&type_id) {
            if let Some(tx) = sender.downcast_ref::<broadcast::Sender<E>>() {
                let _ = tx.send(event);
                debug!("Published event: {}", E::NAME);
            }
        }
    }

    /// Subscribe to events of a specific type.
    /// Returns a receiver that can be used to receive events.
    pub async fn subscribe<E: Event>(&self) -> broadcast::Receiver<E> {
        let type_id = TypeId::of::<E>();

        // Check if channel exists
        {
            let channels = self.channels.read().await;
            if let Some(sender) = channels.get(&type_id) {
                if let Some(tx) = sender.downcast_ref::<broadcast::Sender<E>>() {
                    return tx.subscribe();
                }
            }
        }

        // Create new channel
        let (tx, rx) = broadcast::channel::<E>(100);
        {
            let mut channels = self.channels.write().await;
            channels.insert(type_id, Box::new(tx));
        }

        // Re-get the receiver from the stored sender
        let channels = self.channels.read().await;
        if let Some(sender) = channels.get(&type_id) {
            if let Some(tx) = sender.downcast_ref::<broadcast::Sender<E>>() {
                return tx.subscribe();
            }
        }

        rx
    }

    /// Register a handler for a specific event type.
    /// The handler will be called whenever an event of that type is published.
    pub async fn on<E: Event, H: EventHandler<E> + 'static>(&self, handler: H) {
        let mut rx = self.subscribe::<E>().await;
        let handler = Arc::new(handler);

        tokio::spawn(async move {
            loop {
                match rx.recv().await {
                    Ok(event) => {
                        handler.handle(event).await;
                    }
                    Err(broadcast::error::RecvError::Closed) => break,
                    Err(broadcast::error::RecvError::Lagged(n)) => {
                        tracing::warn!("Event handler lagged by {} events", n);
                    }
                }
            }
        });

        info!("Registered handler for event: {}", E::NAME);
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

// Common events
/// User registered event.
#[derive(Clone, Debug)]
pub struct UserRegistered {
    pub user_id: String,
    pub email: String,
    pub name: String,
}

impl Event for UserRegistered {
    const NAME: &'static str = "user.registered";
}

/// User logged in event.
#[derive(Clone, Debug)]
pub struct UserLoggedIn {
    pub user_id: String,
    pub ip_address: Option<String>,
}

impl Event for UserLoggedIn {
    const NAME: &'static str = "user.logged_in";
}

/// Order created event.
#[derive(Clone, Debug)]
pub struct OrderCreated {
    pub order_id: String,
    pub user_id: String,
    pub total: f64,
}

impl Event for OrderCreated {
    const NAME: &'static str = "order.created";
}
