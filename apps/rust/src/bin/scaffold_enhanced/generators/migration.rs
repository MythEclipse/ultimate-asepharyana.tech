//! Database migration generator

use anyhow::{Context, Result};
use chrono::Local;
use std::fs;
use std::path::Path;

pub fn generate_migration(name: &str, table: Option<&str>) -> Result<()> {
    let timestamp = Local::now().format("%Y%m%d%H%M%S");
    let file_name = format!("m{}_{}.rs", timestamp, name);
    
    let migrations_dir = Path::new("migrations");
    fs::create_dir_all(migrations_dir)?;
    
    let content = if let Some(table_name) = table {
        generate_create_table_migration(name, table_name)
    } else {
        generate_empty_migration(name)
    };
    
    let migration_path = migrations_dir.join(&file_name);
    fs::write(&migration_path, content)
        .with_context(|| format!("Failed to write migration: {:?}", migration_path))?;
    
    update_migrations_mod(&file_name)?;
    
    Ok(())
}

pub fn generate_model_migration(
    model: &str,
    table: &str,
    timestamps: bool,
    soft_delete: bool,
) -> Result<()> {
    let timestamp = Local::now().format("%Y%m%d%H%M%S");
    let name = format!("create_{}_table", table);
    let file_name = format!("m{}_{}.rs", timestamp, name);
    
    let migrations_dir = Path::new("migrations");
    fs::create_dir_all(migrations_dir)?;
    
    let content = generate_model_table_migration(table, timestamps, soft_delete);
    
    let migration_path = migrations_dir.join(&file_name);
    fs::write(&migration_path, content)?;
    
    update_migrations_mod(&file_name)?;
    
    Ok(())
}

fn generate_create_table_migration(name: &str, table: &str) -> String {
    let struct_name = name.split('_').map(|s| {
        let mut c = s.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().chain(c).collect(),
        }
    }).collect::<String>();
    
    format!(r#"use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {{
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {{
        manager
            .create_table(
                Table::create()
                    .table({table}::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new({table}::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new({table}::Name).string().not_null())
                    .to_owned(),
            )
            .await
    }}

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {{
        manager
            .drop_table(Table::drop().table({table}::Table).to_owned())
            .await
    }}
}}

#[derive(DeriveIden)]
enum {table} {{
    Table,
    Id,
    Name,
}}
"#, table = struct_name)
}

fn generate_model_table_migration(table: &str, timestamps: bool, soft_delete: bool) -> String {
    let table_pascal = table.split('_').map(|s| {
        let mut c = s.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().chain(c).collect(),
        }
    }).collect::<String>();
    
    let timestamp_cols = if timestamps {
        format!(r#"
                    .col(ColumnDef::new({}::CreatedAt).timestamp().null())
                    .col(ColumnDef::new({}::UpdatedAt).timestamp().null())"#, 
                    table_pascal, table_pascal)
    } else {
        String::new()
    };
    
    let soft_delete_col = if soft_delete {
        format!(r#"
                    .col(ColumnDef::new({}::DeletedAt).timestamp().null())"#, table_pascal)
    } else {
        String::new()
    };
    
    let enum_fields = if timestamps && soft_delete {
        format!("    CreatedAt,\n    UpdatedAt,\n    DeletedAt,")
    } else if timestamps {
        format!("    CreatedAt,\n    UpdatedAt,")
    } else if soft_delete {
        format!("    DeletedAt,")
    } else {
        String::new()
    };
    
    format!(r#"use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {{
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {{
        manager
            .create_table(
                Table::create()
                    .table({table}::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new({table}::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new({table}::Name).string().not_null()){timestamps}{soft_delete}
                    .to_owned(),
            )
            .await
    }}

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {{
        manager
            .drop_table(Table::drop().table({table}::Table).to_owned())
            .await
    }}
}}

#[derive(DeriveIden)]
enum {table} {{
    Table,
    Id,
    Name,
{enum_fields}
}}
"#, table = table_pascal, timestamps = timestamp_cols, soft_delete = soft_delete_col, enum_fields = enum_fields)
}

fn generate_empty_migration(name: &str) -> String {
    format!(r#"use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {{
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {{
        // Add your migration logic here
        Ok(())
    }}

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {{
        // Add your rollback logic here
        Ok(())
    }}
}}
"#)
}

fn update_migrations_mod(file_name: &str) -> Result<()> {
    let mod_path = Path::new("migrations/mod.rs");
    let module_name = file_name.trim_end_matches(".rs");
    let module_line = format!("mod {};", module_name);
    
    if mod_path.exists() {
        let content = fs::read_to_string(mod_path)?;
        if !content.contains(&module_line) {
            // Find the vec![] and add migration
            let new_content = if content.contains("vec![") {
                content.replace(
                    "vec![",
                    &format!("vec![\n        Box::new({}::Migration),", module_name)
                )
            } else {
                format!("{}\n{}", content.trim(), module_line)
            };
            fs::write(mod_path, new_content)?;
        }
    } else {
        let initial_content = format!(r#"pub use sea_orm_migration::prelude::*;

{}

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {{
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {{
        vec![
            Box::new({}::Migration),
        ]
    }}
}}
"#, module_line, module_name);
        fs::write(mod_path, initial_content)?;
    }
    
    Ok(())
}
