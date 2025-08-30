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
