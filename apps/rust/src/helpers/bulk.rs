//! Bulk Operations for batch database operations.
//!
//! # Example
//!
//! ```ignore
//! use rust::helpers::bulk::{BulkResult, batch_insert};
//!
//! // Bulk insert
//! let result = batch_insert::<User, _>(&db, users, 100).await?;
//! ```

use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

/// Bulk operation result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkResult {
    pub total: usize,
    pub inserted: usize,
    pub updated: usize,
    pub failed: usize,
    pub errors: Vec<BulkError>,
}

/// Bulk operation error.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkError {
    pub index: usize,
    pub message: String,
}

impl BulkResult {
    pub fn new(total: usize) -> Self {
        Self {
            total,
            inserted: 0,
            updated: 0,
            failed: 0,
            errors: Vec::new(),
        }
    }

    pub fn success(&self) -> bool {
        self.failed == 0
    }
}

/// Batch insert helper.
pub async fn batch_insert<E, A>(
    db: &DatabaseConnection,
    models: Vec<A>,
    chunk_size: usize,
) -> Result<BulkResult, DbErr>
where
    E: EntityTrait,
    A: ActiveModelTrait<Entity = E> + Send,
{
    let total = models.len();
    let mut result = BulkResult::new(total);

    // Process in chunks without requiring Clone
    let mut iter = models.into_iter().peekable();
    let mut chunk_idx = 0;

    while iter.peek().is_some() {
        let chunk: Vec<A> = iter.by_ref().take(chunk_size).collect();
        let chunk_len = chunk.len();

        match E::insert_many(chunk).exec(db).await {
            Ok(_) => {
                result.inserted += chunk_len;
            }
            Err(e) => {
                result.failed += chunk_len;
                result.errors.push(BulkError {
                    index: chunk_idx * chunk_size,
                    message: e.to_string(),
                });
            }
        }
        chunk_idx += 1;
    }

    Ok(result)
}

/// Batch delete helper.
pub async fn batch_delete<E, C>(
    db: &DatabaseConnection,
    column: C,
    values: Vec<impl Into<sea_orm::Value> + Clone>,
    chunk_size: usize,
) -> Result<BulkResult, DbErr>
where
    E: EntityTrait,
    C: ColumnTrait,
{
    let total = values.len();
    let mut result = BulkResult::new(total);

    for chunk in values.chunks(chunk_size) {
        let chunk_values: Vec<_> = chunk.iter().cloned().map(|v| v.into()).collect();
        match E::delete_many()
            .filter(column.is_in(chunk_values))
            .exec(db)
            .await
        {
            Ok(delete_result) => {
                result.updated += delete_result.rows_affected as usize;
            }
            Err(e) => {
                result.failed += chunk.len();
                result.errors.push(BulkError {
                    index: 0,
                    message: e.to_string(),
                });
            }
        }
    }

    Ok(result)
}

/// Progress callback for long operations.
pub type ProgressCallback = Box<dyn Fn(usize, usize) + Send + Sync>;

/// Batch process with progress reporting.
pub async fn batch_with_progress<T, F, Fut>(
    items: Vec<T>,
    chunk_size: usize,
    process_fn: F,
    on_progress: Option<ProgressCallback>,
) -> BulkResult
where
    T: Send,
    F: Fn(T) -> Fut + Send + Sync,
    Fut: std::future::Future<Output = Result<(), String>> + Send,
{
    let total = items.len();
    let mut result = BulkResult::new(total);
    let mut processed = 0;

    for (idx, item) in items.into_iter().enumerate() {
        match process_fn(item).await {
            Ok(_) => {
                result.inserted += 1;
            }
            Err(e) => {
                result.failed += 1;
                result.errors.push(BulkError {
                    index: idx,
                    message: e,
                });
            }
        }

        processed += 1;
        if let Some(ref callback) = on_progress {
            if processed % chunk_size == 0 || processed == total {
                callback(processed, total);
            }
        }
    }

    result
}
