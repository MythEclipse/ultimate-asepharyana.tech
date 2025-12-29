//! Notification sender implementation.

use super::channels::{DiscordConfig, NotificationChannel, SlackConfig};
use chrono::{DateTime, Utc};
use deadpool_redis::{redis::AsyncCommands, Pool};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::Arc;

/// Notification error.
#[derive(Debug, thiserror::Error)]
pub enum NotificationError {
    #[error("Redis error: {0}")]
    RedisError(String),
    #[error("HTTP error: {0}")]
    HttpError(String),
    #[error("Serialization error: {0}")]
    SerializationError(String),
}

/// Notification data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    /// Notification ID.
    pub id: String,
    /// Notification type (e.g., "message", "alert").
    pub notification_type: String,
    /// Title.
    pub title: String,
    /// Body/message.
    pub body: Option<String>,
    /// Data payload.
    pub data: Option<serde_json::Value>,
    /// Channels to send via.
    #[serde(skip)]
    pub channels: HashSet<NotificationChannel>,
    /// Created timestamp.
    pub created_at: DateTime<Utc>,
    /// Read timestamp.
    pub read_at: Option<DateTime<Utc>>,
}

impl Notification {
    /// Create a new notification.
    pub fn new(title: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            notification_type: "default".to_string(),
            title: title.to_string(),
            body: None,
            data: None,
            channels: HashSet::new(),
            created_at: Utc::now(),
            read_at: None,
        }
    }

    /// Set notification type.
    pub fn of_type(mut self, t: &str) -> Self {
        self.notification_type = t.to_string();
        self
    }

    /// Set body.
    pub fn body(mut self, body: &str) -> Self {
        self.body = Some(body.to_string());
        self
    }

    /// Set data payload.
    pub fn with_data(mut self, data: serde_json::Value) -> Self {
        self.data = Some(data);
        self
    }

    /// Add a channel.
    pub fn via(mut self, channel: NotificationChannel) -> Self {
        self.channels.insert(channel);
        self
    }

    /// Add multiple channels.
    pub fn via_many(mut self, channels: &[NotificationChannel]) -> Self {
        for c in channels {
            self.channels.insert(*c);
        }
        self
    }
}

/// Stored notification entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredNotification {
    pub id: String,
    pub notification_type: String,
    pub title: String,
    pub body: Option<String>,
    pub data: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub read_at: Option<DateTime<Utc>>,
}

impl From<&Notification> for StoredNotification {
    fn from(n: &Notification) -> Self {
        Self {
            id: n.id.clone(),
            notification_type: n.notification_type.clone(),
            title: n.title.clone(),
            body: n.body.clone(),
            data: n.data.clone(),
            created_at: n.created_at,
            read_at: n.read_at,
        }
    }
}

/// Notification sender.
#[derive(Clone)]
pub struct Notifier {
    pool: Arc<Pool>,
    http_client: reqwest::Client,
    prefix: String,
    slack_config: Option<SlackConfig>,
    discord_config: Option<DiscordConfig>,
}

impl Notifier {
    /// Create a new notifier.
    pub fn new(pool: Arc<Pool>) -> Self {
        Self {
            pool,
            http_client: reqwest::Client::new(),
            prefix: "notifications:".to_string(),
            slack_config: None,
            discord_config: None,
        }
    }

    /// Configure Slack.
    pub fn with_slack(mut self, config: SlackConfig) -> Self {
        self.slack_config = Some(config);
        self
    }

    /// Configure Discord.
    pub fn with_discord(mut self, config: DiscordConfig) -> Self {
        self.discord_config = Some(config);
        self
    }

    fn key(&self, user_id: &str) -> String {
        format!("{}{}", self.prefix, user_id)
    }

    /// Send a notification.
    pub async fn send(
        &self,
        user_id: &str,
        notification: Notification,
    ) -> Result<(), NotificationError> {
        for channel in &notification.channels {
            match channel {
                NotificationChannel::Database => {
                    self.send_to_database(user_id, &notification).await?;
                }
                NotificationChannel::Slack => {
                    self.send_to_slack(&notification).await?;
                }
                NotificationChannel::Discord => {
                    self.send_to_discord(&notification).await?;
                }
                _ => {
                    tracing::warn!("Channel {:?} not implemented", channel);
                }
            }
        }

        Ok(())
    }

    /// Store notification in Redis.
    async fn send_to_database(
        &self,
        user_id: &str,
        notification: &Notification,
    ) -> Result<(), NotificationError> {
        let mut conn = self.pool.get().await.map_err(|e| {
            tracing::error!("Redis connection error: {}", e);
            NotificationError::RedisError(e.to_string())
        })?;

        let stored = StoredNotification::from(notification);
        let json = serde_json::to_string(&stored)
            .map_err(|e| NotificationError::SerializationError(e.to_string()))?;

        let key = self.key(user_id);
        conn.lpush::<_, _, ()>(&key, &json).await.map_err(|e| {
            tracing::error!("Redis lpush error: {}", e);
            NotificationError::RedisError(e.to_string())
        })?;

        // Keep only last 100 notifications
        let _: () = conn.ltrim(&key, 0, 99).await.unwrap_or(());

        tracing::debug!("Stored notification for user {}", user_id);
        Ok(())
    }

    /// Send to Slack webhook.
    async fn send_to_slack(&self, notification: &Notification) -> Result<(), NotificationError> {
        let config = match &self.slack_config {
            Some(c) => c,
            None => {
                tracing::warn!("Slack not configured");
                return Ok(());
            }
        };

        let mut payload = serde_json::json!({
            "text": format!("*{}*\n{}", notification.title, notification.body.as_deref().unwrap_or("")),
        });

        if let Some(channel) = &config.channel {
            payload["channel"] = serde_json::Value::String(channel.clone());
        }
        if let Some(username) = &config.username {
            payload["username"] = serde_json::Value::String(username.clone());
        }
        if let Some(emoji) = &config.icon_emoji {
            payload["icon_emoji"] = serde_json::Value::String(emoji.clone());
        }

        self.http_client
            .post(&config.webhook_url)
            .json(&payload)
            .send()
            .await
            .map_err(|e| NotificationError::HttpError(e.to_string()))?;

        tracing::debug!("Sent notification to Slack");
        Ok(())
    }

    /// Send to Discord webhook.
    async fn send_to_discord(&self, notification: &Notification) -> Result<(), NotificationError> {
        let config = match &self.discord_config {
            Some(c) => c,
            None => {
                tracing::warn!("Discord not configured");
                return Ok(());
            }
        };

        let mut payload = serde_json::json!({
            "content": format!("**{}**\n{}", notification.title, notification.body.as_deref().unwrap_or("")),
        });

        if let Some(username) = &config.username {
            payload["username"] = serde_json::Value::String(username.clone());
        }
        if let Some(avatar) = &config.avatar_url {
            payload["avatar_url"] = serde_json::Value::String(avatar.clone());
        }

        self.http_client
            .post(&config.webhook_url)
            .json(&payload)
            .send()
            .await
            .map_err(|e| NotificationError::HttpError(e.to_string()))?;

        tracing::debug!("Sent notification to Discord");
        Ok(())
    }

    /// Get unread notifications for a user.
    pub async fn unread(
        &self,
        user_id: &str,
        limit: usize,
    ) -> Result<Vec<StoredNotification>, NotificationError> {
        let mut conn = self.pool.get().await.map_err(|e| {
            tracing::error!("Redis connection error: {}", e);
            NotificationError::RedisError(e.to_string())
        })?;

        let key = self.key(user_id);
        let entries: Vec<String> =
            conn.lrange(&key, 0, (limit - 1) as isize)
                .await
                .map_err(|e| {
                    tracing::error!("Redis lrange error: {}", e);
                    NotificationError::RedisError(e.to_string())
                })?;

        let mut result = Vec::new();
        for json in entries {
            if let Ok(n) = serde_json::from_str::<StoredNotification>(&json) {
                if n.read_at.is_none() {
                    result.push(n);
                }
            }
        }

        Ok(result)
    }

    /// Mark notification as read.
    pub async fn mark_read(
        &self,
        user_id: &str,
        notification_id: &str,
    ) -> Result<(), NotificationError> {
        let mut conn = self.pool.get().await.map_err(|e| {
            tracing::error!("Redis connection error: {}", e);
            NotificationError::RedisError(e.to_string())
        })?;

        let key = self.key(user_id);
        let entries: Vec<String> = conn.lrange(&key, 0, -1).await.map_err(|e| {
            tracing::error!("Redis lrange error: {}", e);
            NotificationError::RedisError(e.to_string())
        })?;

        for (idx, json) in entries.iter().enumerate() {
            if let Ok(mut n) = serde_json::from_str::<StoredNotification>(json) {
                if n.id == notification_id {
                    n.read_at = Some(Utc::now());
                    let new_json = serde_json::to_string(&n)
                        .map_err(|e| NotificationError::SerializationError(e.to_string()))?;
                    let _: () = conn.lset(&key, idx as isize, &new_json).await.unwrap_or(());
                    break;
                }
            }
        }

        Ok(())
    }

    /// Clear all notifications for a user.
    pub async fn clear(&self, user_id: &str) -> Result<(), NotificationError> {
        let mut conn = self.pool.get().await.map_err(|e| {
            tracing::error!("Redis connection error: {}", e);
            NotificationError::RedisError(e.to_string())
        })?;

        conn.del::<_, ()>(&self.key(user_id)).await.map_err(|e| {
            tracing::error!("Redis del error: {}", e);
            NotificationError::RedisError(e.to_string())
        })?;

        Ok(())
    }
}
