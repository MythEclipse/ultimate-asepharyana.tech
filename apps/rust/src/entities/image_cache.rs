//! `SeaORM` Entity for ImageCache - stores URL mappings for CDN caching

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "ImageCache"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Eq, Serialize, Deserialize)]
pub struct Model {
    pub id: String,
    pub original_url: String,
    pub cdn_url: String,
    pub created_at: DateTimeUtc,
    pub expires_at: Option<DateTimeUtc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    #[sea_orm(column_name = "originalUrl")]
    OriginalUrl,
    #[sea_orm(column_name = "cdnUrl")]
    CdnUrl,
    #[sea_orm(column_name = "createdAt")]
    CreatedAt,
    #[sea_orm(column_name = "expiresAt")]
    ExpiresAt,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Id,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = String;
    fn auto_increment() -> bool {
        false
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::Id => ColumnType::String(StringLen::N(36u32)).def(),
            Self::OriginalUrl => ColumnType::Text.def().unique(),
            Self::CdnUrl => ColumnType::Text.def(),
            Self::CreatedAt => ColumnType::Timestamp.def(),
            Self::ExpiresAt => ColumnType::Timestamp.def().null(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No relations defined for ImageCache")
    }
}

impl ActiveModelBehavior for ActiveModel {}
