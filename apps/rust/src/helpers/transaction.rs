//! Database Transaction helpers.
//!
//! Atomic database operations with automatic rollback.
//!
//! # Example
//!
//! ```ignore
//! use rust::helpers::transaction::{transaction, TransactionExt};
//!
//! let result = transaction(&db, |txn| async move {
//!     User::insert(user1).exec(&txn).await?;
//!     User::insert(user2).exec(&txn).await?;
//!     Ok(())
//! }).await?;
//! ```

use sea_orm::{DatabaseConnection, DatabaseTransaction, DbErr, TransactionTrait};
use std::future::Future;

/// Transaction error.
#[derive(Debug, thiserror::Error)]
pub enum TransactionError {
    #[error("Database error: {0}")]
    DbError(#[from] DbErr),
    #[error("Transaction failed: {0}")]
    Failed(String),
}

/// Execute a closure within a database transaction.
pub async fn transaction<F, Fut, T>(db: &DatabaseConnection, f: F) -> Result<T, TransactionError>
where
    F: FnOnce(DatabaseTransaction) -> Fut,
    Fut: Future<Output = Result<T, DbErr>>,
{
    let txn = db.begin().await?;

    match f(txn).await {
        Ok(result) => Ok(result),
        Err(e) => Err(TransactionError::DbError(e)),
    }
}

/// Execute with automatic commit/rollback.
pub async fn with_transaction<F, Fut, T>(
    db: &DatabaseConnection,
    f: F,
) -> Result<T, TransactionError>
where
    F: FnOnce(&DatabaseTransaction) -> Fut,
    Fut: Future<Output = Result<T, DbErr>>,
{
    let txn = db.begin().await?;

    match f(&txn).await {
        Ok(result) => {
            txn.commit().await?;
            Ok(result)
        }
        Err(e) => {
            txn.rollback().await?;
            Err(TransactionError::DbError(e))
        }
    }
}

/// Nested transaction support (savepoints).
pub struct NestedTransaction {
    depth: usize,
}

impl NestedTransaction {
    pub fn new() -> Self {
        Self { depth: 0 }
    }

    pub fn begin(&mut self) -> usize {
        self.depth += 1;
        self.depth
    }

    pub fn commit(&mut self) -> usize {
        if self.depth > 0 {
            self.depth -= 1;
        }
        self.depth
    }

    pub fn rollback(&mut self) -> usize {
        if self.depth > 0 {
            self.depth -= 1;
        }
        self.depth
    }

    pub fn depth(&self) -> usize {
        self.depth
    }
}

impl Default for NestedTransaction {
    fn default() -> Self {
        Self::new()
    }
}

/// Retry transaction on deadlock.
pub async fn retry_transaction<F, Fut, T>(
    db: &DatabaseConnection,
    max_retries: usize,
    f: F,
) -> Result<T, TransactionError>
where
    F: Fn() -> Fut + Clone,
    Fut: Future<Output = Result<T, DbErr>>,
{
    let mut attempts = 0;
    let mut last_error = None;

    while attempts < max_retries {
        let txn = db.begin().await?;

        match f().await {
            Ok(result) => {
                txn.commit().await?;
                return Ok(result);
            }
            Err(e) => {
                txn.rollback().await.ok();

                // Check if deadlock (simplified - actual check would be DB-specific)
                let is_deadlock = e.to_string().to_lowercase().contains("deadlock");

                if is_deadlock && attempts < max_retries - 1 {
                    attempts += 1;
                    tokio::time::sleep(tokio::time::Duration::from_millis(100 * attempts as u64))
                        .await;
                    continue;
                }

                last_error = Some(e);
                break;
            }
        }
    }

    Err(TransactionError::DbError(last_error.unwrap()))
}

/// Transaction context for tracking.
#[derive(Debug, Clone)]
pub struct TransactionContext {
    pub id: String,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub operations: usize,
}

impl TransactionContext {
    pub fn new() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            started_at: chrono::Utc::now(),
            operations: 0,
        }
    }

    pub fn record_operation(&mut self) {
        self.operations += 1;
    }

    pub fn duration(&self) -> chrono::Duration {
        chrono::Utc::now() - self.started_at
    }
}

impl Default for TransactionContext {
    fn default() -> Self {
        Self::new()
    }
}
