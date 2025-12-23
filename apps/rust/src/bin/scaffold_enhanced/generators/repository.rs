//! Repository pattern generator

use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

pub fn generate_repository(name: &str, model: &str) -> Result<()> {
    let repos_dir = Path::new("src/repositories");
    fs::create_dir_all(repos_dir)?;
    
    let repo_content = generate_repository_content(name, model);
    
    let repo_path = repos_dir.join(format!("{}_repository.rs", name.to_lowercase()));
    fs::write(&repo_path, repo_content)
        .with_context(|| format!("Failed to write repository: {:?}", repo_path))?;
    
    update_repositories_mod(name)?;
    
    Ok(())
}

fn generate_repository_content(name: &str, model: &str) -> String {
    format!(r#"//! {} repository

use sea_orm::*;
use crate::entities::{}::{{Entity as {}, Model, ActiveModel, Column}};

#[derive(Clone)]
pub struct {}Repository {{
    db: DatabaseConnection,
}}

impl {}Repository {{
    pub fn new(db: DatabaseConnection) -> Self {{
        Self {{ db }}
    }}
    
    /// Find all records
    pub async fn find_all(&self) -> Result<Vec<Model>, DbErr> {{
        {}.find().all(&self.db).await
    }}
    
    /// Find by ID
    pub async fn find_by_id(&self, id: i32) -> Result<Option<Model>, DbErr> {{
        {}.find_by_id(id).one(&self.db).await
    }}
    
    /// Find with pagination
    pub async fn paginate(&self, page: u64, per_page: u64) -> Result<(Vec<Model>, u64), DbErr> {{
        let paginator = {}.find()
            .paginate(&self.db, per_page);
        
        let total = paginator.num_items().await?;
        let items = paginator.fetch_page(page).await?;
        
        Ok((items, total))
    }}
    
    /// Create new record
    pub async fn create(&self, data: ActiveModel) -> Result<Model, DbErr> {{
        data.insert(&self.db).await
    }}
    
    /// Update existing record
    pub async fn update(&self, data: ActiveModel) -> Result<Model, DbErr> {{
        data.update(&self.db).await
    }}
    
    /// Delete by ID
    pub async fn delete(&self, id: i32) -> Result<DeleteResult, DbErr> {{
        {}.delete_by_id(id).exec(&self.db).await
    }}
    
    /// Find by custom condition
    pub async fn find_by_name(&self, name: &str) -> Result<Vec<Model>, DbErr> {{
        {}.find()
            .filter(Column::Name.contains(name))
            .all(&self.db)
            .await
    }}
}}
"#, name, model.to_lowercase(), model, model, model, model, model, model, model, model)
}

fn update_repositories_mod(name: &str) -> Result<()> {
    let mod_path = Path::new("src/repositories/mod.rs");
    let module_line = format!("pub mod {}_repository;", name.to_lowercase());
    
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
