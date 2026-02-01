use rustexpress::infra::db_setup;
use sea_orm::{Database, ConnectOptions};
use std::env;
use dotenvy::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    
    // Default to localhost if not set, or read from .env
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");

    println!("Migrating database at: {}", database_url);

    let mut opt = ConnectOptions::new(database_url);
    opt.max_connections(100)
       .min_connections(5)
       .sqlx_logging(true);

    let db = Database::connect(opt).await?;
    println!("✓ Connected to database");

    db_setup::init(&db).await?;
    println!("✓ Database migration completed successfully.");

    Ok(())
}
