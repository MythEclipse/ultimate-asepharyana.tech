//! Enhanced scaffold tool with SeaORM integration
//! Laravel Artisan-inspired code generation

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "scaffold")]
#[command(about = "Laravel-inspired scaffolding for Rust APIs with SeaORM", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate SeaORM entity model
    #[command(name = "make:model")]
    MakeModel {
        /// Name of the model (e.g., User, Post)
        name: String,
        
        /// Generate migration file
        #[arg(short, long)]
        migration: bool,
        
        /// Add timestamps (created_at, updated_at)
        #[arg(short, long, default_value = "true")]
        timestamps: bool,
        
        /// Add soft delete (deleted_at)
        #[arg(long)]
        soft_delete: bool,
    },
    
    /// Generate database migration
    #[command(name = "make:migration")]
    MakeMigration {
        /// Migration name (e.g., create_users_table)
        name: String,
        
        /// Table name for create migrations
        #[arg(short, long)]
        table: Option<String>,
    },
    
    /// Generate API controller with CRUD operations
    #[command(name = "make:controller")]
    MakeController {
        /// Resource name (e.g., users, posts)
        name: String,
        
        /// Generate full CRUD methods
        #[arg(long)]
        crud: bool,
        
        /// Associated model name
        #[arg(short, long)]
        model: Option<String>,
    },
    
    /// Generate service layer
    #[command(name = "make:service")]
    MakeService {
        /// Service name (e.g., UserService)
        name: String,
        
        /// Associated model
        #[arg(short, long)]
        model: Option<String>,
    },
    
    /// Generate repository pattern
    #[command(name = "make:repository")]
    MakeRepository {
        /// Repository name (e.g., UserRepository)
        name: String,
        
        /// Associated model
        #[arg(short, long)]
        model: String,
    },
    
    /// Generate complete CRUD API (model + migration + controller + service + repository)
    #[command(name = "make:api")]
    MakeApi {
        /// Resource name (pluralized, e.g., users, posts)
        name: String,
        
        /// Generate complete stack
        #[arg(long, default_value = "true")]
        full: bool,
        
        /// Add authentication
        #[arg(long)]
        auth: bool,
    },
    
    /// List all models and routes
    List {
        /// What to list: models, routes, all
        #[arg(default_value = "all")]
        what: String,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::MakeModel { name, migration, timestamps, soft_delete } => {
            println!("🔨 Generating SeaORM model: {}", name);
            generators::model::generate_model(&name, migration, timestamps, soft_delete)?;
            println!("✅ Model {} created successfully!", name);
        }
        
        Commands::MakeMigration { name, table } => {
            println!("🔨 Generating migration: {}", name);
            generators::migration::generate_migration(&name, table.as_deref())?;
            println!("✅ Migration created successfully!");
        }
        
        Commands::MakeController { name, crud, model } => {
            println!("🔨 Generating controller: {}", name);
            generators::controller::generate_controller(&name, crud, model.as_deref())?;
            println!("✅ Controller {} created successfully!", name);
        }
        
        Commands::MakeService { name, model } => {
            println!("🔨 Generating service: {}", name);
            generators::service::generate_service(&name, model.as_deref())?;
            println!("✅ Service {} created successfully!", name);
        }
        
        Commands::MakeRepository { name, model } => {
            println!("🔨 Generating repository: {}", name);
            generators::repository::generate_repository(&name, &model)?;
            println!("✅ Repository {} created successfully!", name);
        }
        
        Commands::MakeApi { name, full, auth } => {
            println!("🚀 Generating complete API for: {}", name);
            generators::api::generate_full_api(&name, full, auth)?;
            println!("✅ Complete API created successfully!");
            println!("📝 Run 'cargo build' to compile and register routes");
        }
        
        Commands::List { what } => {
            println!("📋 Listing {}...", what);
            utils::list_resources(&what)?;
        }
    }

    Ok(())
}

mod generators {
    pub mod model;
    pub mod migration;
    pub mod controller;
    pub mod service;
    pub mod repository;
    pub mod api;
}

mod utils {
    use anyhow::Result;
    use std::fs;
    use std::path::Path;
    
    pub fn list_resources(what: &str) -> Result<()> {
        match what {
            "models" => list_models(),
            "routes" => list_routes(),
            _ => {
                list_models()?;
                list_routes()
            }
        }
    }
    
    fn list_models() -> Result<()> {
        println!("\n📦 Models:");
        
        let entities_dir = Path::new("src/entities");
        if !entities_dir.exists() {
            println!("  No models found (src/entities doesn't exist)");
            return Ok(());
        }
        
        let entries = fs::read_dir(entities_dir)?;
        let mut count = 0;
        
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() && path.extension().map(|e| e == "rs").unwrap_or(false) {
                if let Some(name) = path.file_stem() {
                    if name != "mod" {
                        println!("  - {}", name.to_string_lossy());
                        count += 1;
                    }
                }
            }
        }
        
        if count == 0 {
            println!("  No models found");
        } else {
            println!("\n  Total: {} models", count);
        }
        
        Ok(())
    }
    
    fn list_routes() -> Result<()> {
        println!("\n🔗 Routes:");
        
        let api_dir = Path::new("src/routes/api");
        if !api_dir.exists() {
            println!("  No routes found (src/routes/api doesn't exist)");
            return Ok(());
        }
        
        let mut routes = Vec::new();
        scan_routes_recursive(api_dir, api_dir, &mut routes)?;
        
        routes.sort();
        
        if routes.is_empty() {
            println!("  No routes found");
        } else {
            for route in &routes {
                println!("  {}", route);
            }
            println!("\n  Total: {} route files", routes.len());
        }
        
        Ok(())
    }
    
    fn scan_routes_recursive(dir: &Path, root: &Path, routes: &mut Vec<String>) -> Result<()> {
        let entries = fs::read_dir(dir)?;
        
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                scan_routes_recursive(&path, root, routes)?;
            } else if path.is_file() && path.extension().map(|e| e == "rs").unwrap_or(false) {
                if let Some(name) = path.file_stem() {
                    if name != "mod" {
                        let rel_path = path.strip_prefix(root).unwrap_or(&path);
                        routes.push(format!("📄 {}", rel_path.display()));
                    }
                }
            }
        }
        
        Ok(())
    }
}
