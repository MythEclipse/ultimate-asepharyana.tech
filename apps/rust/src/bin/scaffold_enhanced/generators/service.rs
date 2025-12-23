//! Service layer generator

use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

pub fn generate_service(name: &str, model: Option<&str>) -> Result<()> {
    let services_dir = Path::new("src/services");
    fs::create_dir_all(services_dir)?;

    let model_name = model.unwrap_or(name);
    let service_content = generate_service_content(name, model_name);

    let service_path = services_dir.join(format!("{}_service.rs", name.to_lowercase()));
    fs::write(&service_path, service_content)
        .with_context(|| format!("Failed to write service: {:?}", service_path))?;

    update_services_mod(name)?;

    Ok(())
}

fn generate_service_content(name: &str, model: &str) -> String {
    format!(
        r#"//! {} service layer

use sea_orm::*;
use crate::entities::{}::{{Entity as {}, Model, ActiveModel}};

pub struct {}Service {{
    db: DatabaseConnection,
}}

impl {}Service {{
    pub fn new(db: DatabaseConnection) -> Self {{
        Self {{ db }}
    }}
    
    pub async fn find_all(&self) -> Result<Vec<Model>, DbErr> {{
        {}.find().all(&self.db).await
    }}
    
    pub async fn find_by_id(&self, id: i32) -> Result<Option<Model>, DbErr> {{
        {}.find_by_id(id).one(&self.db).await
    }}
    
    pub async fn create(&self, data: ActiveModel) -> Result<Model, DbErr> {{
        data.insert(&self.db).await
    }}
    
    pub async fn update(&self, id: i32, data: ActiveModel) -> Result<Model, DbErr> {{
        data.update(&self.db).await
    }}
    
    pub async fn delete(&self, id: i32) -> Result<DeleteResult, DbErr> {{
        {}.delete_by_id(id).exec(&self.db).await
    }}
}}
"#,
        name,
        model.to_lowercase(),
        model,
        model,
        model,
        model,
        model,
        model
    )
}

fn update_services_mod(name: &str) -> Result<()> {
    let mod_path = Path::new("src/services/mod.rs");
    let module_line = format!("pub mod {}_service;", name.to_lowercase());

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
