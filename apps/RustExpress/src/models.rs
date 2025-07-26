use serde::{Deserialize, Serialize, Deserializer};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::NaiveDateTime;

fn parse_timestamp<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;
    let s = String::deserialize(deserializer)?;
    NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%S%.f")
        .or_else(|_| NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%S%.fZ"))
        .map_err(D::Error::custom)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRow)]
pub struct ChatMessage {
    pub id: String,
    #[serde(rename = "user_id")]
    #[sqlx(rename = "userId")]
    pub user_id: String,
    pub text: String,
    pub email: Option<String>,
    #[serde(rename = "image_profile")]
    #[sqlx(rename = "imageProfile")]
    pub image_profile: Option<String>,
    #[serde(rename = "image_message")]
    #[sqlx(rename = "imageMessage")]
    pub image_message: Option<String>,
    pub role: Option<String>,
    #[serde(deserialize_with = "parse_timestamp")]
    pub timestamp: NaiveDateTime,
}

#[allow(dead_code)]
impl ChatMessage {
    pub fn new(
        user_id: String,
        text: String,
        email: Option<String>,
        image_profile: Option<String>,
        image_message: Option<String>,
        role: Option<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            user_id,
            text,
            email,
            image_profile,
            image_message,
            role,
            timestamp: chrono::Utc::now().naive_utc(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_chat_message_new() {
        // Arrange
        let user_id = "test_user_123".to_string();
        let text = "Hello, World!".to_string();
        let email = Some("test@example.com".to_string());
        let image_profile = Some("https://example.com/avatar.jpg".to_string());
        let image_message = None;
        let role = Some("user".to_string());

        // Act
        let message = ChatMessage::new(
            user_id.clone(),
            text.clone(),
            email.clone(),
            image_profile.clone(),
            image_message.clone(),
            role.clone(),
        );

        // Assert
        assert!(!message.id.is_empty(), "ID should not be empty");
        assert!(Uuid::parse_str(&message.id).is_ok(), "ID should be a valid UUID");
        assert_eq!(message.user_id, user_id);
        assert_eq!(message.text, text);
        assert_eq!(message.email, email);
        assert_eq!(message.image_profile, image_profile);
        assert_eq!(message.image_message, image_message);
        assert_eq!(message.role, role);
    }
}