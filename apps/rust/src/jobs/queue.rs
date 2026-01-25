//! Job queue implementation using Redis.
//!
//! Provides a simple but robust job queue system for background processing.

use async_trait::async_trait;
use deadpool_redis::Pool;
use redis::AsyncCommands;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use uuid::Uuid;

/// Status of a queued job.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum JobStatus {
    /// Job is waiting in the queue
    Pending,
    /// Job is currently being processed
    Processing,
    /// Job completed successfully
    Completed,
    /// Job failed with an error
    Failed,
    /// Job was manually cancelled
    Cancelled,
}

/// Metadata for a queued job.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobMeta {
    pub id: String,
    pub job_type: String,
    pub status: JobStatus,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub attempts: u32,
    pub max_attempts: u32,
    pub error: Option<String>,
}

/// Trait for background jobs.
///
/// Implement this trait to define a job that can be queued and processed.
///
/// # Example
///
/// ```ignore
/// use rustexpress::jobs::{Job, JobDispatcher};
/// use serde::{Serialize, Deserialize};
/// use async_trait::async_trait;
///
/// #[derive(Serialize, Deserialize)]
/// struct SendWelcomeEmail {
///     user_id: String,
///     email: String,
/// }
///
/// #[async_trait]
/// impl Job for SendWelcomeEmail {
///     const NAME: &'static str = "send_welcome_email";
///     const MAX_ATTEMPTS: u32 = 3;
///
///     async fn handle(&self) -> anyhow::Result<()> {
///         // Send the email...
///         println!("Sending welcome email to {}", self.email);
///         Ok(())
///     }
/// }
///
/// // Dispatch the job
/// let job = SendWelcomeEmail {
///     user_id: "123".to_string(),
///     email: "user@example.com".to_string(),
/// };
/// dispatcher.dispatch(job).await?;
/// ```
#[async_trait]
pub trait Job: Serialize + DeserializeOwned + Send + Sync {
    /// Unique name for this job type.
    const NAME: &'static str;

    /// Maximum number of retry attempts.
    const MAX_ATTEMPTS: u32 = 3;

    /// Queue name to use (default: "default")
    const QUEUE: &'static str = "default";

    /// Execute the job.
    async fn handle(&self) -> anyhow::Result<()>;

    /// Called when the job fails after all retries.
    async fn failed(&self, error: &str) {
        tracing::error!("Job {} failed: {}", Self::NAME, error);
    }
}

/// Job dispatcher for queuing jobs.
#[derive(Clone)]
pub struct JobDispatcher {
    redis_pool: Pool,
}

impl JobDispatcher {
    /// Create a new job dispatcher.
    pub fn new(redis_pool: Pool) -> Self {
        Self { redis_pool }
    }

    /// Dispatch a job to be processed.
    pub async fn dispatch<J: Job>(&self, job: J) -> anyhow::Result<String> {
        let job_id = Uuid::new_v4().to_string();
        let queue_key = format!("jobs:queue:{}", J::QUEUE);
        let job_key = format!("jobs:data:{}", job_id);

        let payload = serde_json::to_string(&job)?;

        let meta = JobMeta {
            id: job_id.clone(),
            job_type: J::NAME.to_string(),
            status: JobStatus::Pending,
            created_at: chrono::Utc::now(),
            started_at: None,
            completed_at: None,
            attempts: 0,
            max_attempts: J::MAX_ATTEMPTS,
            error: None,
        };

        let meta_json = serde_json::to_string(&meta)?;

        let mut conn = self.redis_pool.get().await?;

        // Store job data
        let _: () = conn.set(&job_key, payload).await?;
        let _: () = conn.set(format!("{}:meta", job_key), meta_json).await?;

        // Push to queue
        let _: () = conn.rpush(&queue_key, &job_id).await?;

        tracing::info!("Dispatched job {} ({})", J::NAME, job_id);

        Ok(job_id)
    }

    /// Dispatch a job with a delay (in seconds).
    pub async fn dispatch_delayed<J: Job>(
        &self,
        job: J,
        delay_seconds: u64,
    ) -> anyhow::Result<String> {
        let job_id = Uuid::new_v4().to_string();
        let delayed_key = "jobs:delayed";
        let job_key = format!("jobs:data:{}", job_id);

        let payload = serde_json::to_string(&job)?;
        let execute_at = chrono::Utc::now().timestamp() + delay_seconds as i64;

        let meta = JobMeta {
            id: job_id.clone(),
            job_type: J::NAME.to_string(),
            status: JobStatus::Pending,
            created_at: chrono::Utc::now(),
            started_at: None,
            completed_at: None,
            attempts: 0,
            max_attempts: J::MAX_ATTEMPTS,
            error: None,
        };

        let meta_json = serde_json::to_string(&meta)?;

        let mut conn = self.redis_pool.get().await?;

        // Store job data
        let _: () = conn.set(&job_key, payload).await?;
        let _: () = conn.set(format!("{}:meta", job_key), meta_json).await?;

        // Add to delayed sorted set (score = execution timestamp)
        let delayed_entry = format!("{}:{}", J::QUEUE, job_id);
        let _: () = conn.zadd(delayed_key, delayed_entry, execute_at).await?;

        tracing::info!(
            "Dispatched delayed job {} ({}) - executes in {}s",
            J::NAME,
            job_id,
            delay_seconds
        );

        Ok(job_id)
    }

    /// Get the status of a job.
    pub async fn status(&self, job_id: &str) -> anyhow::Result<Option<JobMeta>> {
        let meta_key = format!("jobs:data:{}:meta", job_id);
        let mut conn = self.redis_pool.get().await?;

        let meta_json: Option<String> = conn.get(&meta_key).await?;

        match meta_json {
            Some(json) => Ok(Some(serde_json::from_str(&json)?)),
            None => Ok(None),
        }
    }
}
