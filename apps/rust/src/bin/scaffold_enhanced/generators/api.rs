//! Complete API generator - combines model, migration, controller, service, repository

use anyhow::Result;

pub fn generate_full_api(name: &str, full: bool, auth: bool) -> Result<()> {
    println!("📦 Generating model...");
    let model_name = singularize(name);
    super::model::generate_model(&model_name, true, true, false)?;

    if full {
        println!("🔧 Generating service...");
        super::service::generate_service(name, Some(&model_name))?;

        println!("💾 Generating repository...");
        super::repository::generate_repository(name, &model_name)?;
    }

    println!("🎮 Generating CRUD controller...");
    super::controller::generate_controller(name, true, Some(&model_name))?;

    println!("\n✅ Complete API generated!");
    println!("\n📋 Generated files:");
    println!(
        "  - src/entities/{}.rs (SeaORM model)",
        model_name.to_lowercase()
    );
    println!(
        "  - migrations/m*_create_{}.rs (migration)",
        super::model::pluralize(&model_name)
    );

    if full {
        println!("  - src/services/{}_service.rs (service layer)", name);
        println!("  - src/repositories/{}_repository.rs (repository)", name);
    }

    println!("  - src/routes/api/{}/index.rs (list)", name);
    println!("  - src/routes/api/{}/[id].rs (get)", name);
    println!("  - src/routes/api/{}/create.rs (create)", name);
    println!("  - src/routes/api/{}/[id]/update.rs (update)", name);
    println!("  - src/routes/api/{}/[id]/delete.rs (delete)", name);

    println!("\n🚀 Next steps:");
    println!("  1. Run 'cargo build' to compile");
    println!("  2. Run migrations:  cargo run -- migration up");
    println!("  3. Start server:    cargo run");

    println!("\n📡 Available endpoints:");
    println!("  GET    /api/{} - List all", name);
    println!("  GET    /api/{}/{{id}} - Get one", name);
    println!("  POST   /api/{} - Create", name);
    println!("  PUT    /api/{}/{{id}} - Update", name);
    println!("  DELETE /api/{}/{{id}} - Delete", name);

    Ok(())
}

fn singularize(word: &str) -> String {
    let lower = word.to_lowercase();

    if lower.ends_with("ies") {
        format!("{}y", &lower[..lower.len() - 3])
    } else if lower.ends_with("es") {
        lower[..lower.len() - 2].to_string()
    } else if lower.ends_with('s') {
        lower[..lower.len() - 1].to_string()
    } else {
        lower
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singularize() {
        assert_eq!(singularize("users"), "user");
        assert_eq!(singularize("categories"), "category");
        assert_eq!(singularize("posts"), "post");
        assert_eq!(singularize("boxes"), "box");
    }
}
