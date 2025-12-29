//! Soft delete helpers for SeaORM entities.
//!
//! Provides soft delete filtering for entities with a `deleted_at` column.
//!
//! # Example
//!
//! ```ignore
//! use rust::helpers::soft_delete::{SoftDeletable, soft_delete_filter, SoftDeleteScope};
//! use sea_orm::*;
//!
//! // Query without deleted
//! let users = User::find()
//!     .filter(soft_delete_filter(user::Column::DeletedAt, SoftDeleteScope::WithoutDeleted))
//!     .all(db)
//!     .await?;
//!
//! // Query only deleted  
//! let deleted = User::find()
//!     .filter(soft_delete_filter(user::Column::DeletedAt, SoftDeleteScope::OnlyDeleted))
//!     .all(db)
//!     .await?;
//! ```

use chrono::{DateTime, Utc};
use sea_orm::{ColumnTrait, Condition};

/// Trait for entities that support soft deletes.
pub trait SoftDeletable {
    /// Get the deleted_at timestamp if soft deleted.
    fn deleted_at(&self) -> Option<DateTime<Utc>>;

    /// Check if the entity is soft deleted.
    fn is_deleted(&self) -> bool {
        self.deleted_at().is_some()
    }

    /// Check if the entity is not soft deleted.
    fn is_active(&self) -> bool {
        self.deleted_at().is_none()
    }
}

/// Query scope for filtering soft-deleted entities.
#[derive(Debug, Clone, Copy)]
pub enum SoftDeleteScope {
    /// Exclude soft-deleted entities (default).
    WithoutDeleted,
    /// Include soft-deleted entities.
    WithDeleted,
    /// Only include soft-deleted entities.
    OnlyDeleted,
}

/// Create a condition for soft delete filtering.
///
/// Use this in your entity queries:
/// ```ignore
/// User::find()
///     .filter(soft_delete_filter(user::Column::DeletedAt, SoftDeleteScope::WithoutDeleted))
///     .all(db)
///     .await
/// ```
pub fn soft_delete_filter<C: ColumnTrait>(column: C, scope: SoftDeleteScope) -> Condition {
    match scope {
        SoftDeleteScope::WithoutDeleted => Condition::all().add(column.is_null()),
        SoftDeleteScope::WithDeleted => Condition::all(),
        SoftDeleteScope::OnlyDeleted => Condition::all().add(column.is_not_null()),
    }
}

/// Macro to implement SoftDeletable for a model.
///
/// Usage:
/// ```ignore
/// impl_soft_deletable!(user::Model, deleted_at);
/// ```
#[macro_export]
macro_rules! impl_soft_deletable {
    ($model:ty, $deleted_at_field:ident) => {
        impl $crate::helpers::soft_delete::SoftDeletable for $model {
            fn deleted_at(&self) -> Option<chrono::DateTime<chrono::Utc>> {
                self.$deleted_at_field
            }
        }
    };
}
