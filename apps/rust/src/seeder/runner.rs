//! Seeder runner implementation.

use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use std::sync::Arc;
use tracing::{error, info};
use uuid::Uuid;

/// Trait for database seeders.
#[async_trait]
pub trait Seeder: Send + Sync {
    /// Seeder name.
    fn name(&self) -> &'static str;

    /// Run the seeder.
    async fn run(&self, db: &DatabaseConnection) -> anyhow::Result<()>;
}

/// Runner for executing seeders.
pub struct SeederRunner {
    seeders: Vec<Arc<dyn Seeder>>,
    db: Arc<DatabaseConnection>,
}

impl SeederRunner {
    /// Create a new seeder runner.
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self {
            seeders: Vec::new(),
            db,
        }
    }

    /// Add a seeder.
    pub fn add<S: Seeder + 'static>(&mut self, seeder: S) -> &mut Self {
        self.seeders.push(Arc::new(seeder));
        self
    }

    /// Run all seeders.
    pub async fn run_all(&self) -> anyhow::Result<()> {
        info!("🌱 Running {} seeders...", self.seeders.len());

        for seeder in &self.seeders {
            info!("  Running seeder: {}", seeder.name());
            match seeder.run(&self.db).await {
                Ok(_) => info!("  ✅ {} completed", seeder.name()),
                Err(e) => {
                    error!("  ❌ {} failed: {}", seeder.name(), e);
                    return Err(e);
                }
            }
        }

        info!("🌱 All seeders completed!");
        Ok(())
    }

    /// Run a specific seeder by name.
    pub async fn run_one(&self, name: &str) -> anyhow::Result<()> {
        for seeder in &self.seeders {
            if seeder.name() == name {
                info!("Running seeder: {}", name);
                return seeder.run(&self.db).await;
            }
        }
        Err(anyhow::anyhow!("Seeder '{}' not found", name))
    }

    /// List all registered seeders.
    pub fn list(&self) -> Vec<&'static str> {
        self.seeders.iter().map(|s| s.name()).collect()
    }
}

// Real seeders with actual implementations

/// Users seeder - creates test users.
pub struct UsersSeeder {
    pub count: usize,
}

impl Default for UsersSeeder {
    fn default() -> Self {
        Self { count: 10 }
    }
}

#[async_trait]
impl Seeder for UsersSeeder {
    fn name(&self) -> &'static str {
        "users"
    }

    async fn run(&self, db: &DatabaseConnection) -> anyhow::Result<()> {
        use crate::entities::user;

        info!("Creating {} test users...", self.count);

        for i in 1..=self.count {
            let user_id = Uuid::new_v4().to_string();
            let email = format!("testuser{}@example.com", i);
            let name = format!("Test User {}", i);

            // Hash password with bcrypt
            let password_hash = bcrypt::hash("password123", bcrypt::DEFAULT_COST)?;

            // Match the actual User entity schema
            let new_user = user::ActiveModel {
                id: Set(user_id),
                email: Set(Some(email.clone())),
                name: Set(Some(name.clone())),
                password: Set(Some(password_hash)),
                role: Set("user".to_string()),
                email_verified: Set(None),
                image: Set(None),
                refresh_token: Set(None),
            };

            match new_user.insert(db).await {
                Ok(_) => info!("  Created user: {}", email),
                Err(e) => {
                    if e.to_string().contains("Duplicate entry") {
                        info!("  User {} already exists, skipping", email);
                    } else {
                        return Err(anyhow::anyhow!("Failed to create user {}: {}", email, e));
                    }
                }
            }
        }

        info!("Created {} test users", self.count);
        Ok(())
    }
}

/// Admin seeder - creates admin users.
pub struct AdminSeeder;

#[async_trait]
impl Seeder for AdminSeeder {
    fn name(&self) -> &'static str {
        "admin"
    }

    async fn run(&self, db: &DatabaseConnection) -> anyhow::Result<()> {
        use crate::entities::user;

        info!("Creating admin user...");

        let admin_id = Uuid::new_v4().to_string();
        let password_hash = bcrypt::hash("admin123", bcrypt::DEFAULT_COST)?;

        let admin = user::ActiveModel {
            id: Set(admin_id),
            email: Set(Some("admin@example.com".to_string())),
            name: Set(Some("Admin".to_string())),
            password: Set(Some(password_hash)),
            role: Set("admin".to_string()),
            email_verified: Set(Some(chrono::Utc::now().into())),
            image: Set(None),
            refresh_token: Set(None),
        };

        match admin.insert(db).await {
            Ok(_) => info!("  Created admin user: admin@example.com"),
            Err(e) => {
                if e.to_string().contains("Duplicate entry") {
                    info!("  Admin user already exists, skipping");
                } else {
                    return Err(anyhow::anyhow!("Failed to create admin: {}", e));
                }
            }
        }

        Ok(())
    }
}
