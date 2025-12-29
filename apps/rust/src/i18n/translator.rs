//! Translation implementation.

use serde_json::Value;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Locale identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Locale(pub String);

impl Locale {
    /// Create a new locale.
    pub fn new(code: &str) -> Self {
        Self(code.to_lowercase())
    }

    /// Get the locale code.
    pub fn code(&self) -> &str {
        &self.0
    }

    /// Parse from Accept-Language header.
    pub fn from_accept_language(header: &str) -> Self {
        // Parse "en-US,en;q=0.9,id;q=0.8" format
        header
            .split(',')
            .next()
            .and_then(|s| s.split(';').next())
            .map(|s| s.trim())
            .map(|s| {
                // Convert en-US to en
                s.split('-').next().unwrap_or(s).to_lowercase()
            })
            .map(|s| Self(s))
            .unwrap_or_else(|| Self::default())
    }
}

impl Default for Locale {
    fn default() -> Self {
        Self("en".to_string())
    }
}

impl From<&str> for Locale {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl std::fmt::Display for Locale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Translation error types.
#[derive(Debug, thiserror::Error)]
pub enum TranslationError {
    #[error("Failed to load translations: {0}")]
    LoadError(String),
    #[error("Translation not found: {0}")]
    NotFound(String),
    #[error("IO error: {0}")]
    IoError(String),
}

type TranslationMap = HashMap<String, Value>;

/// Internationalization service.
#[derive(Clone)]
pub struct I18n {
    translations: Arc<RwLock<HashMap<Locale, TranslationMap>>>,
    default_locale: Locale,
    fallback_locale: Locale,
}

impl I18n {
    /// Create a new empty i18n instance.
    pub fn new() -> Self {
        Self {
            translations: Arc::new(RwLock::new(HashMap::new())),
            default_locale: Locale::default(),
            fallback_locale: Locale::default(),
        }
    }

    /// Create with default and fallback locales.
    pub fn with_locales(default: &str, fallback: &str) -> Self {
        Self {
            translations: Arc::new(RwLock::new(HashMap::new())),
            default_locale: Locale::new(default),
            fallback_locale: Locale::new(fallback),
        }
    }

    /// Load translations from a directory.
    ///
    /// Directory structure:
    /// ```text
    /// locales/
    ///   en.json
    ///   id.json
    ///   es.json
    /// ```
    pub async fn load(dir: &str) -> Result<Self, TranslationError> {
        let mut i18n = Self::new();
        i18n.load_directory(dir).await?;
        Ok(i18n)
    }

    /// Load translations from a directory.
    pub async fn load_directory(&mut self, dir: &str) -> Result<(), TranslationError> {
        let path = Path::new(dir);

        if !path.exists() {
            tracing::warn!("Translations directory not found: {}", dir);
            return Ok(());
        }

        let mut entries = tokio::fs::read_dir(path)
            .await
            .map_err(|e| TranslationError::IoError(e.to_string()))?;

        while let Some(entry) = entries
            .next_entry()
            .await
            .map_err(|e| TranslationError::IoError(e.to_string()))?
        {
            let file_path = entry.path();

            if file_path.extension().and_then(|s| s.to_str()) != Some("json") {
                continue;
            }

            let locale_code = file_path
                .file_stem()
                .and_then(|s| s.to_str())
                .ok_or_else(|| TranslationError::LoadError("Invalid filename".to_string()))?;

            let content = tokio::fs::read_to_string(&file_path)
                .await
                .map_err(|e| TranslationError::IoError(e.to_string()))?;

            let translations: TranslationMap = serde_json::from_str(&content)
                .map_err(|e| TranslationError::LoadError(e.to_string()))?;

            self.translations
                .write()
                .await
                .insert(Locale::new(locale_code), translations);

            tracing::info!("Loaded translations for locale: {}", locale_code);
        }

        Ok(())
    }

    /// Add translations for a locale.
    pub async fn add_translations(&self, locale: &str, translations: TranslationMap) {
        self.translations
            .write()
            .await
            .insert(Locale::new(locale), translations);
    }

    /// Get translation for key using default locale.
    pub async fn t(&self, key: &str) -> String {
        self.t_locale(key, &self.default_locale).await
    }

    /// Get translation for key using specific locale.
    pub async fn t_locale(&self, key: &str, locale: &Locale) -> String {
        let translations = self.translations.read().await;

        // Try requested locale
        if let Some(value) = self.get_nested(&translations, locale, key) {
            return value;
        }

        // Try fallback locale
        if locale != &self.fallback_locale {
            if let Some(value) = self.get_nested(&translations, &self.fallback_locale, key) {
                return value;
            }
        }

        // Return key as fallback
        key.to_string()
    }

    /// Get translation with placeholder substitution.
    pub async fn t_with(&self, key: &str, params: &[(&str, &str)]) -> String {
        self.t_locale_with(key, &self.default_locale, params).await
    }

    /// Get translation with locale and placeholder substitution.
    pub async fn t_locale_with(
        &self,
        key: &str,
        locale: &Locale,
        params: &[(&str, &str)],
    ) -> String {
        let mut result = self.t_locale(key, locale).await;

        for (placeholder, value) in params {
            result = result.replace(&format!("{{{}}}", placeholder), value);
            result = result.replace(&format!(":{}", placeholder), value);
        }

        result
    }

    /// Get nested value from translations.
    fn get_nested(
        &self,
        translations: &HashMap<Locale, TranslationMap>,
        locale: &Locale,
        key: &str,
    ) -> Option<String> {
        let locale_translations = translations.get(locale)?;

        // Support nested keys like "auth.login.success"
        let parts: Vec<&str> = key.split('.').collect();

        let mut current: &Value = locale_translations.get(parts[0])?;

        for part in &parts[1..] {
            current = current.get(*part)?;
        }

        match current {
            Value::String(s) => Some(s.clone()),
            _ => None,
        }
    }

    /// Set the default locale.
    pub fn set_default_locale(&mut self, locale: &str) {
        self.default_locale = Locale::new(locale);
    }

    /// Get available locales.
    pub async fn available_locales(&self) -> Vec<String> {
        self.translations
            .read()
            .await
            .keys()
            .map(|l| l.code().to_string())
            .collect()
    }

    /// Check if a locale is available.
    pub async fn has_locale(&self, locale: &str) -> bool {
        self.translations
            .read()
            .await
            .contains_key(&Locale::new(locale))
    }
}

impl Default for I18n {
    fn default() -> Self {
        Self::new()
    }
}

/// Extract locale from request (Axum extractor).
impl<S> axum::extract::FromRequestParts<S> for Locale
where
    S: Send + Sync,
{
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        let locale = parts
            .headers
            .get("Accept-Language")
            .and_then(|h| h.to_str().ok())
            .map(Locale::from_accept_language)
            .unwrap_or_default();

        Ok(locale)
    }
}
