//! OAuth2 Social Login support.
//!
//! Login with Google, GitHub, Discord, and other OAuth2 providers.
//!
//! # Example
//!
//! ```ignore
//! use rust::auth::oauth::{OAuthProvider, OAuthClient, GoogleProvider};
//!
//! let google = GoogleProvider::new(
//!     "client_id",
//!     "client_secret",
//!     "https://myapp.com/auth/google/callback"
//! );
//!
//! // Get authorization URL
//! let (url, state) = google.authorize_url();
//!
//! // Exchange code for user info
//! let user = google.get_user(&code).await?;
//! ```

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// OAuth user info.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthUser {
    /// Provider user ID.
    pub id: String,
    /// User email (if available).
    pub email: Option<String>,
    /// Display name.
    pub name: Option<String>,
    /// Avatar URL.
    pub avatar: Option<String>,
    /// Raw provider data.
    pub raw: serde_json::Value,
}

/// OAuth error.
#[derive(Debug, thiserror::Error)]
pub enum OAuthError {
    #[error("HTTP error: {0}")]
    HttpError(String),
    #[error("Invalid response: {0}")]
    InvalidResponse(String),
    #[error("Token exchange failed: {0}")]
    TokenError(String),
    #[error("State mismatch")]
    StateMismatch,
}

/// OAuth token response.
#[derive(Debug, Clone, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: Option<u64>,
    pub refresh_token: Option<String>,
    pub scope: Option<String>,
}

/// OAuth provider trait.
#[async_trait]
pub trait OAuthProvider: Send + Sync {
    /// Provider name (e.g., "google", "github").
    fn name(&self) -> &str;

    /// Generate authorization URL and state.
    fn authorize_url(&self) -> (String, String);

    /// Exchange authorization code for tokens.
    async fn exchange_code(&self, code: &str) -> Result<TokenResponse, OAuthError>;

    /// Get user info from access token.
    async fn get_user(&self, access_token: &str) -> Result<OAuthUser, OAuthError>;

    /// Get user info from authorization code (convenience method).
    async fn get_user_from_code(&self, code: &str) -> Result<OAuthUser, OAuthError> {
        let tokens = self.exchange_code(code).await?;
        self.get_user(&tokens.access_token).await
    }
}

/// Google OAuth provider.
pub struct GoogleProvider {
    client_id: String,
    client_secret: String,
    redirect_uri: String,
    scopes: Vec<String>,
    http: reqwest::Client,
}

impl GoogleProvider {
    pub fn new(client_id: &str, client_secret: &str, redirect_uri: &str) -> Self {
        Self {
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            redirect_uri: redirect_uri.to_string(),
            scopes: vec![
                "openid".to_string(),
                "email".to_string(),
                "profile".to_string(),
            ],
            http: reqwest::Client::new(),
        }
    }

    pub fn with_scopes(mut self, scopes: Vec<String>) -> Self {
        self.scopes = scopes;
        self
    }
}

#[async_trait]
impl OAuthProvider for GoogleProvider {
    fn name(&self) -> &str {
        "google"
    }

    fn authorize_url(&self) -> (String, String) {
        let state = generate_state();
        let url = format!(
            "https://accounts.google.com/o/oauth2/v2/auth?\
            client_id={}&\
            redirect_uri={}&\
            response_type=code&\
            scope={}&\
            state={}&\
            access_type=offline",
            urlencoding::encode(&self.client_id),
            urlencoding::encode(&self.redirect_uri),
            urlencoding::encode(&self.scopes.join(" ")),
            state
        );
        (url, state)
    }

    async fn exchange_code(&self, code: &str) -> Result<TokenResponse, OAuthError> {
        let response = self
            .http
            .post("https://oauth2.googleapis.com/token")
            .form(&[
                ("client_id", &self.client_id),
                ("client_secret", &self.client_secret),
                ("code", &code.to_string()),
                ("grant_type", &"authorization_code".to_string()),
                ("redirect_uri", &self.redirect_uri),
            ])
            .send()
            .await
            .map_err(|e| OAuthError::HttpError(e.to_string()))?;

        response
            .json()
            .await
            .map_err(|e| OAuthError::TokenError(e.to_string()))
    }

    async fn get_user(&self, access_token: &str) -> Result<OAuthUser, OAuthError> {
        let response = self
            .http
            .get("https://www.googleapis.com/oauth2/v2/userinfo")
            .bearer_auth(access_token)
            .send()
            .await
            .map_err(|e| OAuthError::HttpError(e.to_string()))?;

        let data: serde_json::Value = response
            .json()
            .await
            .map_err(|e| OAuthError::InvalidResponse(e.to_string()))?;

        Ok(OAuthUser {
            id: data["id"].as_str().unwrap_or_default().to_string(),
            email: data["email"].as_str().map(String::from),
            name: data["name"].as_str().map(String::from),
            avatar: data["picture"].as_str().map(String::from),
            raw: data,
        })
    }
}

/// GitHub OAuth provider.
pub struct GitHubProvider {
    client_id: String,
    client_secret: String,
    redirect_uri: String,
    scopes: Vec<String>,
    http: reqwest::Client,
}

impl GitHubProvider {
    pub fn new(client_id: &str, client_secret: &str, redirect_uri: &str) -> Self {
        Self {
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            redirect_uri: redirect_uri.to_string(),
            scopes: vec!["read:user".to_string(), "user:email".to_string()],
            http: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl OAuthProvider for GitHubProvider {
    fn name(&self) -> &str {
        "github"
    }

    fn authorize_url(&self) -> (String, String) {
        let state = generate_state();
        let url = format!(
            "https://github.com/login/oauth/authorize?\
            client_id={}&\
            redirect_uri={}&\
            scope={}&\
            state={}",
            urlencoding::encode(&self.client_id),
            urlencoding::encode(&self.redirect_uri),
            urlencoding::encode(&self.scopes.join(" ")),
            state
        );
        (url, state)
    }

    async fn exchange_code(&self, code: &str) -> Result<TokenResponse, OAuthError> {
        let response = self
            .http
            .post("https://github.com/login/oauth/access_token")
            .header("Accept", "application/json")
            .form(&[
                ("client_id", &self.client_id),
                ("client_secret", &self.client_secret),
                ("code", &code.to_string()),
                ("redirect_uri", &self.redirect_uri),
            ])
            .send()
            .await
            .map_err(|e| OAuthError::HttpError(e.to_string()))?;

        response
            .json()
            .await
            .map_err(|e| OAuthError::TokenError(e.to_string()))
    }

    async fn get_user(&self, access_token: &str) -> Result<OAuthUser, OAuthError> {
        let response = self
            .http
            .get("https://api.github.com/user")
            .header("User-Agent", "rust-oauth")
            .bearer_auth(access_token)
            .send()
            .await
            .map_err(|e| OAuthError::HttpError(e.to_string()))?;

        let data: serde_json::Value = response
            .json()
            .await
            .map_err(|e| OAuthError::InvalidResponse(e.to_string()))?;

        Ok(OAuthUser {
            id: data["id"].to_string(),
            email: data["email"].as_str().map(String::from),
            name: data["name"]
                .as_str()
                .or(data["login"].as_str())
                .map(String::from),
            avatar: data["avatar_url"].as_str().map(String::from),
            raw: data,
        })
    }
}

/// Discord OAuth provider.
pub struct DiscordProvider {
    client_id: String,
    client_secret: String,
    redirect_uri: String,
    scopes: Vec<String>,
    http: reqwest::Client,
}

impl DiscordProvider {
    pub fn new(client_id: &str, client_secret: &str, redirect_uri: &str) -> Self {
        Self {
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            redirect_uri: redirect_uri.to_string(),
            scopes: vec!["identify".to_string(), "email".to_string()],
            http: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl OAuthProvider for DiscordProvider {
    fn name(&self) -> &str {
        "discord"
    }

    fn authorize_url(&self) -> (String, String) {
        let state = generate_state();
        let url = format!(
            "https://discord.com/api/oauth2/authorize?\
            client_id={}&\
            redirect_uri={}&\
            response_type=code&\
            scope={}&\
            state={}",
            urlencoding::encode(&self.client_id),
            urlencoding::encode(&self.redirect_uri),
            urlencoding::encode(&self.scopes.join(" ")),
            state
        );
        (url, state)
    }

    async fn exchange_code(&self, code: &str) -> Result<TokenResponse, OAuthError> {
        let response = self
            .http
            .post("https://discord.com/api/oauth2/token")
            .form(&[
                ("client_id", &self.client_id),
                ("client_secret", &self.client_secret),
                ("code", &code.to_string()),
                ("grant_type", &"authorization_code".to_string()),
                ("redirect_uri", &self.redirect_uri),
            ])
            .send()
            .await
            .map_err(|e| OAuthError::HttpError(e.to_string()))?;

        response
            .json()
            .await
            .map_err(|e| OAuthError::TokenError(e.to_string()))
    }

    async fn get_user(&self, access_token: &str) -> Result<OAuthUser, OAuthError> {
        let response = self
            .http
            .get("https://discord.com/api/users/@me")
            .bearer_auth(access_token)
            .send()
            .await
            .map_err(|e| OAuthError::HttpError(e.to_string()))?;

        let data: serde_json::Value = response
            .json()
            .await
            .map_err(|e| OAuthError::InvalidResponse(e.to_string()))?;

        let avatar = data["avatar"].as_str().map(|a| {
            format!(
                "https://cdn.discordapp.com/avatars/{}/{}.png",
                data["id"].as_str().unwrap_or(""),
                a
            )
        });

        Ok(OAuthUser {
            id: data["id"].as_str().unwrap_or_default().to_string(),
            email: data["email"].as_str().map(String::from),
            name: data["username"].as_str().map(String::from),
            avatar,
            raw: data,
        })
    }
}

/// Generate random state for OAuth.
fn generate_state() -> String {
    use rand::Rng;
    let mut rng = rand::rng();
    let bytes: Vec<u8> = (0..32).map(|_| rng.random()).collect();
    hex::encode(bytes)
}
