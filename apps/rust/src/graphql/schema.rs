//! GraphQL schema definition.

use async_graphql::{Context, EmptySubscription, InputObject, Object, Schema, SimpleObject, ID};
use sea_orm::{DatabaseConnection, QuerySelect};
use std::sync::Arc;

/// The GraphQL schema type.
pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

/// Create the GraphQL schema.
pub fn create_schema(db: Arc<DatabaseConnection>) -> AppSchema {
    Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(db)
        .finish()
}

// ============================================================================
// Query Types
// ============================================================================

/// Root query object.
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Get API version.
    async fn version(&self) -> &str {
        env!("CARGO_PKG_VERSION")
    }

    /// Health check.
    async fn health(&self) -> HealthResponse {
        HealthResponse {
            status: "ok".to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Get user by ID.
    async fn user(&self, ctx: &Context<'_>, id: ID) -> async_graphql::Result<Option<User>> {
        use crate::entities::user;
        use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

        let db = ctx.data::<Arc<DatabaseConnection>>()?;

        let user_model = user::Entity::find()
            .filter(user::Column::Id.eq(id.to_string()))
            .one(db.as_ref())
            .await?;

        Ok(user_model.map(|u| User {
            id: ID(u.id),
            name: u.name,
            email: u.email,
            role: u.role,
        }))
    }

    /// List users with pagination.
    async fn users(
        &self,
        ctx: &Context<'_>,
        #[graphql(default = 0)] offset: i32,
        #[graphql(default = 10)] limit: i32,
    ) -> async_graphql::Result<Vec<User>> {
        use crate::entities::user;
        use sea_orm::{EntityTrait, QueryOrder};

        let db = ctx.data::<Arc<DatabaseConnection>>()?;

        let users = user::Entity::find()
            .order_by_asc(user::Column::Id)
            .offset(offset as u64)
            .limit(limit as u64)
            .all(db.as_ref())
            .await?;

        Ok(users
            .into_iter()
            .map(|u| User {
                id: ID(u.id),
                name: u.name,
                email: u.email,
                role: u.role,
            })
            .collect())
    }
}

// ============================================================================
// Mutation Types
// ============================================================================

/// Root mutation object.
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    /// Create a new user.
    async fn create_user(
        &self,
        ctx: &Context<'_>,
        input: CreateUserInput,
    ) -> async_graphql::Result<User> {
        use crate::entities::user;
        use sea_orm::{ActiveModelTrait, Set};

        let db = ctx.data::<Arc<DatabaseConnection>>()?;

        let password_hash = bcrypt::hash(&input.password, bcrypt::DEFAULT_COST)
            .map_err(|e| async_graphql::Error::new(format!("Password hash error: {}", e)))?;

        let new_user = user::ActiveModel {
            id: Set(uuid::Uuid::new_v4().to_string()),
            email: Set(Some(input.email.clone())),
            name: Set(Some(input.name.clone())),
            password: Set(Some(password_hash)),
            role: Set("user".to_string()),
            email_verified: Set(None),
            image: Set(None),
            refresh_token: Set(None),
        };

        let created = new_user.insert(db.as_ref()).await?;

        Ok(User {
            id: ID(created.id),
            name: created.name,
            email: created.email,
            role: created.role,
        })
    }

    /// Update user role.
    async fn update_user_role(
        &self,
        ctx: &Context<'_>,
        id: ID,
        role: String,
    ) -> async_graphql::Result<User> {
        use crate::entities::user;
        use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, Set};

        let db = ctx.data::<Arc<DatabaseConnection>>()?;

        let user_model = user::Entity::find_by_id(id.to_string())
            .one(db.as_ref())
            .await?
            .ok_or_else(|| async_graphql::Error::new("User not found"))?;

        let mut user_active: user::ActiveModel = user_model.into_active_model();
        user_active.role = Set(role);

        let updated = user_active.update(db.as_ref()).await?;

        Ok(User {
            id: ID(updated.id),
            name: updated.name,
            email: updated.email,
            role: updated.role,
        })
    }

    /// Delete a user.
    async fn delete_user(&self, ctx: &Context<'_>, id: ID) -> async_graphql::Result<bool> {
        use crate::entities::user;
        use sea_orm::EntityTrait;

        let db = ctx.data::<Arc<DatabaseConnection>>()?;

        let result = user::Entity::delete_by_id(id.to_string())
            .exec(db.as_ref())
            .await?;

        Ok(result.rows_affected > 0)
    }
}

// ============================================================================
// GraphQL Types
// ============================================================================

/// User type for GraphQL.
#[derive(SimpleObject)]
pub struct User {
    pub id: ID,
    pub name: Option<String>,
    pub email: Option<String>,
    pub role: String,
}

/// Input for creating a user.
#[derive(InputObject)]
pub struct CreateUserInput {
    pub name: String,
    pub email: String,
    pub password: String,
}

/// Health response.
#[derive(SimpleObject)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: String,
}
