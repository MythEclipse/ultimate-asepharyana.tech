//! Job worker for processing background jobs.
//!
//! The worker runs as a separate process and continuously polls
//! the job queue for work.

use super::queue::{JobMeta, JobStatus};
use deadpool_redis::Pool;
use redis::AsyncCommands;
use std::time::Duration;
use tokio::time::sleep;

/// Configuration for the job worker.
#[derive(Debug, Clone)]
pub struct WorkerConfig {
    /// Queues to process (in priority order)
    pub queues: Vec<String>,
    /// Number of concurrent jobs to process
    pub concurrency: usize,
    /// Sleep duration when no jobs are available
    pub sleep_duration: Duration,
}

impl Default for WorkerConfig {
    fn default() -> Self {
        Self {
            queues: vec!["default".to_string()],
            concurrency: 4,
            sleep_duration: Duration::from_secs(1),
        }
    }
}

/// Job worker that processes queued jobs.
pub struct Worker {
    redis_pool: Pool,
    config: WorkerConfig,
    /// Registry of job handlers by name
    handlers: std::collections::HashMap<&'static str, Box<dyn JobHandler>>,
}

/// Trait for job handlers (type-erased).
#[async_trait::async_trait]
pub trait JobHandler: Send + Sync {
    async fn process(&self, payload: &str) -> anyhow::Result<()>;
}

impl Worker {
    /// Create a new worker.
    pub fn new(redis_pool: Pool, config: WorkerConfig) -> Self {
        Self {
            redis_pool,
            config,
            handlers: std::collections::HashMap::new(),
        }
    }

    /// Register a job handler.
    pub fn register<H: JobHandler + 'static>(&mut self, name: &'static str, handler: H) {
        self.handlers.insert(name, Box::new(handler));
    }

    /// Run the worker loop.
    pub async fn run(&self) -> anyhow::Result<()> {
        tracing::info!(
            "🔧 Worker started - processing queues: {:?}",
            self.config.queues
        );

        loop {
            let mut processed = false;

            for queue in &self.config.queues {
                if let Some(job_id) = self.pop_job(queue).await? {
                    self.process_job(queue, &job_id).await?;
                    processed = true;
                }
            }

            if !processed {
                // No jobs available, sleep
                sleep(self.config.sleep_duration).await;
            }
        }
    }

    /// Pop a job from the queue.
    async fn pop_job(&self, queue: &str) -> anyhow::Result<Option<String>> {
        let queue_key = format!("jobs:queue:{}", queue);
        let mut conn = self.redis_pool.get().await?;

        let job_id: Option<String> = conn.lpop(&queue_key, None).await?;
        Ok(job_id)
    }

    /// Process a single job.
    async fn process_job(&self, queue: &str, job_id: &str) -> anyhow::Result<()> {
        let job_key = format!("jobs:data:{}", job_id);
        let meta_key = format!("{}:meta", job_key);

        let mut conn = self.redis_pool.get().await?;

        // Get job metadata
        let meta_json: Option<String> = conn.get(&meta_key).await?;
        let mut meta: JobMeta = match meta_json {
            Some(json) => serde_json::from_str(&json)?,
            None => {
                tracing::warn!("Job {} not found", job_id);
                return Ok(());
            }
        };

        // Get job payload
        let payload: Option<String> = conn.get(&job_key).await?;
        let payload = match payload {
            Some(p) => p,
            None => {
                tracing::warn!("Job {} payload not found", job_id);
                return Ok(());
            }
        };

        // Update status to processing
        meta.status = JobStatus::Processing;
        meta.started_at = Some(chrono::Utc::now());
        meta.attempts += 1;
        let _: () = conn.set(&meta_key, serde_json::to_string(&meta)?).await?;

        tracing::info!(
            "Processing job {} ({}) - attempt {}/{}",
            meta.job_type,
            job_id,
            meta.attempts,
            meta.max_attempts
        );

        // Find handler
        let handler = match self.handlers.get(meta.job_type.as_str()) {
            Some(h) => h,
            None => {
                tracing::error!("No handler registered for job type: {}", meta.job_type);
                meta.status = JobStatus::Failed;
                meta.error = Some(format!("No handler for job type: {}", meta.job_type));
                meta.completed_at = Some(chrono::Utc::now());
                let _: () = conn.set(&meta_key, serde_json::to_string(&meta)?).await?;
                return Ok(());
            }
        };

        // Execute job
        match handler.process(&payload).await {
            Ok(()) => {
                meta.status = JobStatus::Completed;
                meta.completed_at = Some(chrono::Utc::now());
                tracing::info!("Job {} completed successfully", job_id);
            }
            Err(e) => {
                let error_msg = e.to_string();
                tracing::error!("Job {} failed: {}", job_id, error_msg);

                if meta.attempts >= meta.max_attempts {
                    meta.status = JobStatus::Failed;
                    meta.error = Some(error_msg);
                    meta.completed_at = Some(chrono::Utc::now());
                } else {
                    // Retry - push back to queue
                    meta.status = JobStatus::Pending;
                    meta.error = Some(format!("Attempt {} failed: {}", meta.attempts, error_msg));
                    let queue_key = format!("jobs:queue:{}", queue);
                    let _: () = conn.rpush(&queue_key, job_id).await?;
                    tracing::info!("Job {} queued for retry", job_id);
                }
            }
        }

        // Save final status
        let _: () = conn.set(&meta_key, serde_json::to_string(&meta)?).await?;

        Ok(())
    }
}
