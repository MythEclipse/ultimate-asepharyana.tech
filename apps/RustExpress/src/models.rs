use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRow)]
pub struct ChatMessage {
    pub id: String,
    pub user_id: String,
    pub text: String,
    pub email: Option<String>,
    pub image_profile: Option<String>,
    pub image_message: Option<String>,
    pub role: String,
    pub timestamp: String,
}

#[allow(dead_code)]
impl ChatMessage {
    pub fn new(
        user_id: String,
        text: String,
        email: Option<String>,
        image_profile: Option<String>,
        image_message: Option<String>,
        role: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            user_id,
            text,
            email,
            image_profile,
            image_message,
            role,
            timestamp: chrono::Utc::now().to_rfc3339(),
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
        let role = "user".to_string();

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
        assert!(!message.timestamp.is_empty(), "Timestamp should not be empty");

        // Verify timestamp is valid RFC3339
        assert!(chrono::DateTime::parse_from_rfc3339(&message.timestamp).is_ok());
    }

    #[test]
    fn test_chat_message_new_minimal() {
        // Arrange
        let user_id = "test_user".to_string();
        let text = "Test message".to_string();
        let role = "assistant".to_string();

        // Act
        let message = ChatMessage::new(
            user_id.clone(),
            text.clone(),
            None,
            None,
            None,
            role.clone(),
        );

        // Assert
        assert_eq!(message.user_id, user_id);
        assert_eq!(message.text, text);
        assert_eq!(message.role, role);
        assert!(message.email.is_none());
        assert!(message.image_profile.is_none());
        assert!(message.image_message.is_none());
    }

    #[test]
    fn test_chat_message_serialization() {
        // Arrange
        let message = ChatMessage::new(
            "test_user".to_string(),
            "Test message".to_string(),
            Some("test@example.com".to_string()),
            None,
            None,
            "user".to_string(),
        );

        // Act
        let json_result = serde_json::to_string(&message);

        // Assert
        assert!(json_result.is_ok(), "Should serialize to JSON successfully");
        let json = json_result.unwrap();
        assert!(json.contains("test_user"));
        assert!(json.contains("Test message"));
        assert!(json.contains("test@example.com"));
    }

    #[test]
    fn test_chat_message_deserialization() {
        // Arrange
        let json = r#"{
            "id": "123e4567-e89b-12d3-a456-426614174000",
            "user_id": "test_user",
            "text": "Test message",
            "email": "test@example.com",
            "image_profile": null,
            "image_message": null,
            "role": "user",
            "timestamp": "2024-01-01T12:00:00Z"
        }"#;

        // Act
        let result: Result<ChatMessage, _> = serde_json::from_str(json);

        // Assert
        assert!(result.is_ok(), "Should deserialize from JSON successfully");
        let message = result.unwrap();
        assert_eq!(message.id, "123e4567-e89b-12d3-a456-426614174000");
        assert_eq!(message.user_id, "test_user");
        assert_eq!(message.text, "Test message");
        assert_eq!(message.email, Some("test@example.com".to_string()));
        assert_eq!(message.role, "user");
        assert_eq!(message.timestamp, "2024-01-01T12:00:00Z");
    }

    #[test]
    fn test_chat_message_clone() {
        // Arrange
        let original = ChatMessage::new(
            "test_user".to_string(),
            "Test message".to_string(),
            Some("test@example.com".to_string()),
            None,
            None,
            "user".to_string(),
        );

        // Act
        let cloned = original.clone();

        // Assert
        assert_eq!(original.id, cloned.id);
        assert_eq!(original.user_id, cloned.user_id);
        assert_eq!(original.text, cloned.text);
        assert_eq!(original.email, cloned.email);
        assert_eq!(original.role, cloned.role);
        assert_eq!(original.timestamp, cloned.timestamp);
    }
}