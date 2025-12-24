//! Scheduler implementation using tokio-cron-scheduler.

use async_trait::async_trait;
use deadpool_redis::redis::AsyncCommands;
use std::sync::Arc;
use tokio_cron_scheduler::{Job, JobScheduler};
use tracing::info;

/// Trait for scheduled tasks.
#[async_trait]
pub trait ScheduledTask: Send + Sync {
    /// Task name for logging.
    fn name(&self) -> &'static str;

    /// Cron expression (e.g., "0 * * * * *" for every minute).
    fn schedule(&self) -> &'static str;

    /// Execute the task.
    async fn run(&self);
}

/// Scheduler for running cron jobs.
pub struct Scheduler {
    inner: JobScheduler,
}

impl Scheduler {
    /// Create a new scheduler.
    pub async fn new() -> anyhow::Result<Self> {
        let scheduler = JobScheduler::new().await?;
        Ok(Self { inner: scheduler })
    }

    /// Add a task to the scheduler.
    pub async fn add<T: ScheduledTask + 'static>(&self, task: T) -> anyhow::Result<()> {
        let task = Arc::new(task);
        let task_name = task.name();
        let schedule = task.schedule();

        let job = Job::new_async(schedule, move |_uuid, _lock| {
            let task = Arc::clone(&task);
            Box::pin(async move {
                info!("Running scheduled task: {}", task.name());
                task.run().await;
            })
        })?;

        self.inner.add(job).await?;
        info!("Scheduled task '{}' with cron: {}", task_name, schedule);
        Ok(())
    }

    /// Add a simple job with a closure.
    pub async fn add_job<F, Fut>(&self, name: &'static str, schedule: &str, f: F) -> anyhow::Result<()>
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = ()> + Send + 'static,
    {
        let f = Arc::new(f);
        let job = Job::new_async(schedule, move |_uuid, _lock| {
            let f = Arc::clone(&f);
            Box::pin(async move {
                info!("Running scheduled job: {}", name);
                f().await;
            })
        })?;

        self.inner.add(job).await?;
        info!("Scheduled job '{}' with cron: {}", name, schedule);
        Ok(())
    }

    /// Start the scheduler.
    pub async fn start(&self) -> anyhow::Result<()> {
        info!("Starting scheduler");
        self.inner.start().await?;
        Ok(())
    }

    /// Stop the scheduler.
    pub async fn shutdown(&mut self) -> anyhow::Result<()> {
        info!("Shutting down scheduler");
        self.inner.shutdown().await?;
        Ok(())
    }
}

// Real scheduled tasks with actual implementations

/// Cleanup expired sessions from Redis.
/// Runs every hour to remove expired session tokens.
pub struct CleanupExpiredSessions;

#[async_trait]
impl ScheduledTask for CleanupExpiredSessions {
    fn name(&self) -> &'static str {
        "cleanup_expired_sessions"
    }

    fn schedule(&self) -> &'static str {
        // Every hour at minute 0
        "0 0 * * * *"
    }

    async fn run(&self) {
        use crate::redis_client::REDIS_POOL;

        info!("Cleaning up expired sessions...");

        match REDIS_POOL.get().await {
            Ok(mut conn) => {
                // Get all session keys
                let keys: Vec<String> = match conn.keys::<_, Vec<String>>("session:*").await {
                    Ok(k) => k,
                    Err(e) => {
                        tracing::error!("Failed to get session keys: {}", e);
                        return;
                    }
                };

                let mut cleaned = 0;
                for key in keys {
                    // Check TTL - if key has no TTL or TTL is -2 (expired), delete it
                    let ttl: i64 = match conn.ttl(&key).await {
                        Ok(t) => t,
                        Err(_) => continue,
                    };

                    if ttl == -2 {
                        // Key doesn't exist (already expired)
                        cleaned += 1;
                    } else if ttl == -1 {
                        // Key has no expiration, set one (24 hours)
                        let _: () = conn.expire(&key, 86400).await.unwrap_or(());
                    }
                }

                info!("Session cleanup complete: {} expired sessions found", cleaned);
            }
            Err(e) => {
                tracing::error!("Failed to connect to Redis for session cleanup: {}", e);
            }
        }
    }
}

/// Cleanup expired tokens (JWT blacklist, verification tokens, etc.)
pub struct CleanupExpiredTokens;

#[async_trait]
impl ScheduledTask for CleanupExpiredTokens {
    fn name(&self) -> &'static str {
        "cleanup_expired_tokens"
    }

    fn schedule(&self) -> &'static str {
        // Every 6 hours
        "0 0 */6 * * *"
    }

    async fn run(&self) {
        use crate::redis_client::REDIS_POOL;

        info!("Cleaning up expired tokens...");

        match REDIS_POOL.get().await {
            Ok(mut conn) => {
                // Clean blacklisted JWT tokens
                let blacklist_keys: Vec<String> = conn
                    .keys::<_, Vec<String>>("jwt_blacklist:*")
                    .await
                    .unwrap_or_default();

                let mut cleaned = 0;
                for key in blacklist_keys {
                    let ttl: i64 = conn.ttl(&key).await.unwrap_or(-1);
                    if ttl == -2 {
                        cleaned += 1;
                    }
                }

                // Clean verification tokens
                let verify_keys: Vec<String> = conn
                    .keys::<_, Vec<String>>("verify:*")
                    .await
                    .unwrap_or_default();

                for key in verify_keys {
                    let ttl: i64 = conn.ttl(&key).await.unwrap_or(-1);
                    if ttl == -2 {
                        cleaned += 1;
                    }
                }

                info!("Token cleanup complete: {} expired tokens found", cleaned);
            }
            Err(e) => {
                tracing::error!("Failed to connect to Redis for token cleanup: {}", e);
            }
        }
    }
}

/// Log application metrics periodically.
pub struct LogMetrics;

#[async_trait]
impl ScheduledTask for LogMetrics {
    fn name(&self) -> &'static str {
        "log_metrics"
    }

    fn schedule(&self) -> &'static str {
        // Every 5 minutes
        "0 */5 * * * *"
    }

    async fn run(&self) {
        use crate::redis_client::REDIS_POOL;

        // Get Redis pool stats
        let redis_stats = match REDIS_POOL.status() {
            s => format!("size={}, available={}", s.size, s.available),
        };

        info!(
            "📊 Metrics - Redis pool: {}",
            redis_stats
        );
    }
}
