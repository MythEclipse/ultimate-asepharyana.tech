//! Notification channels configuration.

use serde::{Deserialize, Serialize};

/// Notification delivery channel.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum NotificationChannel {
    /// Store in database/Redis for in-app notifications.
    Database,
    /// Send via email.
    Email,
    /// Send to Slack webhook.
    Slack,
    /// Send to Discord webhook.
    Discord,
    /// Send push notification.
    Push,
    /// Send SMS.
    Sms,
}

/// Slack webhook configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlackConfig {
    pub webhook_url: String,
    pub channel: Option<String>,
    pub username: Option<String>,
    pub icon_emoji: Option<String>,
}

impl SlackConfig {
    pub fn new(webhook_url: &str) -> Self {
        Self {
            webhook_url: webhook_url.to_string(),
            channel: None,
            username: None,
            icon_emoji: None,
        }
    }

    pub fn with_channel(mut self, channel: &str) -> Self {
        self.channel = Some(channel.to_string());
        self
    }

    pub fn with_username(mut self, username: &str) -> Self {
        self.username = Some(username.to_string());
        self
    }
}

/// Discord webhook configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscordConfig {
    pub webhook_url: String,
    pub username: Option<String>,
    pub avatar_url: Option<String>,
}

impl DiscordConfig {
    pub fn new(webhook_url: &str) -> Self {
        Self {
            webhook_url: webhook_url.to_string(),
            username: None,
            avatar_url: None,
        }
    }

    pub fn with_username(mut self, username: &str) -> Self {
        self.username = Some(username.to_string());
        self
    }
}

/// Push notification configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushConfig {
    pub title: String,
    pub body: String,
    pub icon: Option<String>,
    pub url: Option<String>,
}
