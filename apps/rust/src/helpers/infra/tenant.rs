//! Multi-tenancy support.
//!
//! Tenant isolation for SaaS applications.
//!
//! # Example
//!
//! ```ignore
//! use rustexpress::helpers::tenant::{TenantManager, Tenant, TenantContext};
//!
//! let manager = TenantManager::new(redis_pool);
//! let tenant = manager.get_by_domain("acme.example.com").await?;
//!
//! // Use in middleware
//! let ctx = TenantContext::new(tenant);
//! ```

use deadpool_redis::{redis::AsyncCommands, Pool};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Tenant error.
#[derive(Debug, thiserror::Error)]
pub enum TenantError {
    #[error("Tenant not found")]
    NotFound,
    #[error("Tenant inactive")]
    Inactive,
    #[error("Redis error: {0}")]
    RedisError(String),
}

/// Tenant data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tenant {
    pub id: String,
    pub name: String,
    pub domain: String,
    pub subdomain: Option<String>,
    pub active: bool,
    pub plan: String,
    pub settings: serde_json::Value,
}

impl Tenant {
    pub fn new(id: &str, name: &str, domain: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            domain: domain.to_string(),
            subdomain: None,
            active: true,
            plan: "free".to_string(),
            settings: serde_json::json!({}),
        }
    }

    pub fn is_premium(&self) -> bool {
        self.plan != "free"
    }
}

/// Tenant manager.
#[derive(Clone)]
pub struct TenantManager {
    pool: Arc<Pool>,
    prefix: String,
}

impl TenantManager {
    /// Create a new tenant manager.
    pub fn new(pool: Arc<Pool>) -> Self {
        Self {
            pool,
            prefix: "tenant:".to_string(),
        }
    }

    fn id_key(&self, id: &str) -> String {
        format!("{}id:{}", self.prefix, id)
    }

    fn domain_key(&self, domain: &str) -> String {
        format!("{}domain:{}", self.prefix, domain)
    }

    /// Create a tenant.
    pub async fn create(&self, tenant: &Tenant) -> Result<(), TenantError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| TenantError::RedisError(e.to_string()))?;

        let json =
            serde_json::to_string(tenant).map_err(|e| TenantError::RedisError(e.to_string()))?;

        // Store by ID
        conn.set::<_, _, ()>(&self.id_key(&tenant.id), &json)
            .await
            .map_err(|e| TenantError::RedisError(e.to_string()))?;

        // Index by domain
        conn.set::<_, _, ()>(&self.domain_key(&tenant.domain), &tenant.id)
            .await
            .map_err(|e| TenantError::RedisError(e.to_string()))?;

        // Index by subdomain if present
        if let Some(subdomain) = &tenant.subdomain {
            conn.set::<_, _, ()>(&self.domain_key(subdomain), &tenant.id)
                .await
                .map_err(|e| TenantError::RedisError(e.to_string()))?;
        }

        Ok(())
    }

    /// Get tenant by ID.
    pub async fn get(&self, id: &str) -> Result<Tenant, TenantError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| TenantError::RedisError(e.to_string()))?;

        let json: Option<String> = conn
            .get(&self.id_key(id))
            .await
            .map_err(|e| TenantError::RedisError(e.to_string()))?;

        match json {
            Some(j) => {
                let tenant: Tenant =
                    serde_json::from_str(&j).map_err(|e| TenantError::RedisError(e.to_string()))?;
                if !tenant.active {
                    return Err(TenantError::Inactive);
                }
                Ok(tenant)
            }
            None => Err(TenantError::NotFound),
        }
    }

    /// Get tenant by domain.
    pub async fn get_by_domain(&self, domain: &str) -> Result<Tenant, TenantError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| TenantError::RedisError(e.to_string()))?;

        let id: Option<String> = conn
            .get(&self.domain_key(domain))
            .await
            .map_err(|e| TenantError::RedisError(e.to_string()))?;

        match id {
            Some(i) => self.get(&i).await,
            None => Err(TenantError::NotFound),
        }
    }

    /// Update tenant.
    pub async fn update(&self, tenant: &Tenant) -> Result<(), TenantError> {
        self.create(tenant).await
    }

    /// Deactivate tenant.
    pub async fn deactivate(&self, id: &str) -> Result<(), TenantError> {
        let mut tenant = self.get(id).await?;
        tenant.active = false;
        self.update(&tenant).await
    }
}

/// Tenant context for request handling.
#[derive(Clone)]
pub struct TenantContext {
    pub tenant: Tenant,
}

impl TenantContext {
    pub fn new(tenant: Tenant) -> Self {
        Self { tenant }
    }

    pub fn id(&self) -> &str {
        &self.tenant.id
    }

    pub fn name(&self) -> &str {
        &self.tenant.name
    }

    pub fn setting<T: serde::de::DeserializeOwned>(&self, key: &str) -> Option<T> {
        self.tenant
            .settings
            .get(key)
            .and_then(|v| serde_json::from_value(v.clone()).ok())
    }
}

/// Extract tenant from request (for middleware).
pub fn extract_tenant_from_host(host: &str) -> String {
    // Extract subdomain or use full domain
    host.split('.').next().unwrap_or(host).to_string()
}
