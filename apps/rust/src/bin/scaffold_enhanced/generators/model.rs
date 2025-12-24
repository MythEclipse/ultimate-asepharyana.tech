//! SeaORM model generator

use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

pub fn generate_model(
    name: &str,
    with_migration: bool,
    timestamps: bool,
    soft_delete: bool,
) -> Result<()> {
    let table_name = pluralize(name);

    // Create entities directory
    let entities_dir = Path::new("src/entities");
    fs::create_dir_all(entities_dir)?;

    // Generate model file
    let model_content = generate_model_content(name, &table_name, timestamps, soft_delete);
    let model_path = entities_dir.join(format!("{}.rs", name.to_lowercase()));
    fs::write(&model_path, model_content)
        .with_context(|| format!("Failed to write model file: {:?}", model_path))?;

    // Update entities/mod.rs
    update_entities_mod(name)?;

    // Generate migration if requested
    if with_migration {
        super::migration::generate_model_migration(&table_name, timestamps, soft_delete)?;
    }

    Ok(())
}

fn generate_model_content(
    name: &str,
    table_name: &str,
    timestamps: bool,
    soft_delete: bool,
) -> String {
    let timestamp_fields = if timestamps {
        r#"
    #[sea_orm(nullable)]
    pub created_at: Option<DateTimeUtc>,
    #[sea_orm(nullable)]
    pub updated_at: Option<DateTimeUtc>,"#
    } else {
        ""
    };

    let soft_delete_field = if soft_delete {
        r#"
    #[sea_orm(nullable)]
    pub deleted_at: Option<DateTimeUtc>,"#
    } else {
        ""
    };

    format!(
        r#"//! {} entity

use sea_orm::entity::prelude::*;
use serde::{{Deserialize, Serialize}};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "{}")]
pub struct Model {{
    #[sea_orm(primary_key)]
    pub id: i32,
    
    // Add your fields here
    pub name: String,{}{}
}}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {{}}

impl ActiveModelBehavior for ActiveModel {{}}
"#,
        name, table_name, timestamp_fields, soft_delete_field
    )
}

fn update_entities_mod(name: &str) -> Result<()> {
    let mod_path = Path::new("src/entities/mod.rs");
    let module_line = format!("pub mod {};", name.to_lowercase());

    if mod_path.exists() {
        let content = fs::read_to_string(mod_path)?;
        if !content.contains(&module_line) {
            let new_content = format!("{}\n{}", content.trim(), module_line);
            fs::write(mod_path, new_content)?;
        }
    } else {
        fs::write(mod_path, format!("{}\n", module_line))?;
    }

    Ok(())
}

pub fn pluralize(word: &str) -> String {
    let lower = word.to_lowercase();

    if lower.ends_with('y') {
        format!("{}ies", &lower[..lower.len() - 1])
    } else if lower.ends_with('s') || lower.ends_with("ch") || lower.ends_with("sh") {
        format!("{}es", lower)
    } else {
        format!("{}s", lower)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pluralize() {
        assert_eq!(pluralize("User"), "users");
        assert_eq!(pluralize("Category"), "categories");
        assert_eq!(pluralize("Post"), "posts");
        assert_eq!(pluralize("Box"), "boxes");
    }
}
