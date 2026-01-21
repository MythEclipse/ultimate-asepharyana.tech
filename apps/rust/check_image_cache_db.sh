#!/bin/bash
# Simple script to check image_cache database content

DATABASE_URL="mysql://asephs:hunterz@127.0.0.1:3306/sosmed"

echo "=== Checking Image Cache Database ==="
echo ""
echo "Attempting to connect via Rust binary..."
echo ""

cd "$(dirname "$0")"

# Create temporary Rust script
cat > /tmp/check_img_cache.rs << 'EOF'
use sea_orm::{Database, ConnectionTrait, Statement};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = Database::connect("mysql://asephs:hunterz@127.0.0.1:3306/sosmed").await?;
    
    println!("✓ Connected to database\n");
    
    // Show all tables
    println!("=== ALL TABLES ===");
    let tables = db.query_all(Statement::from_string(
        db.get_database_backend(),
        "SHOW TABLES".to_owned()
    )).await?;
    for table in &tables {
        println!("  - {:?}", table);
    }
    
    // Count image_cache records
    println!("\n=== IMAGE_CACHE TABLE ===");
    let count = db.query_one(Statement::from_string(
        db.get_database_backend(),
        "SELECT COUNT(*) as total FROM ImageCache".to_owned()
    )).await?;
    println!("Total records: {:?}", count);
    
    // Show recent 10 records
    println!("\n=== RECENT RECORDS (Last 10) ===");
    let records = db.query_all(Statement::from_string(
        db.get_database_backend(),
        "SELECT id, originalUrl, cdnUrl, createdAt FROM ImageCache ORDER BY createdAt DESC LIMIT 10".to_owned()
    )).await?;
    
    for (i, record) in records.iter().enumerate() {
        println!("\n[{}]", i + 1);
        println!("  ID: {:?}", record.try_get::<String>("", "id"));
        println!("  Original: {:?}", record.try_get::<String>("", "originalUrl"));
        println!("  CDN URL: {:?}", record.try_get::<String>("", "cdnUrl"));
        println!("  Created: {:?}", record.try_get::<chrono::DateTime<chrono::Utc>>("", "createdAt"));
    }
    
    Ok(())
}
EOF

echo "Running database check..."
rustc --edition 2021 /tmp/check_img_cache.rs -o /tmp/check_img_cache \
  -L dependency=target/debug/deps \
  --extern sea_orm=target/debug/deps/libsea_orm.rlib \
  --extern tokio=target/debug/deps/libtokio.rlib \
  --extern chrono=target/debug/deps/libchrono.rlib \
  2>/dev/null

if [ $? -eq 0 ]; then
    /tmp/check_img_cache
else
    echo "Rust compilation failed, using cargo script instead..."
    cargo script /tmp/check_img_cache.rs
fi
