use std::process::Command;
use std::fs;

fn main() -> std::io::Result<()> {
    println!("🧪 Starting Scaffold System Test");
    println!("=================================");

    // Test 1: Create static routes
    println!("\n📝 Test 1: Creating static routes...");
    run_command("cargo", &["run", "--bin", "scaffold", "--", "test/items/list"])?;
    run_command("cargo", &["run", "--bin", "scaffold", "--", "test/items/search"])?;
    run_command("cargo", &["run", "--bin", "scaffold", "--", "users/list"])?;

    // Test 2: Create dynamic routes
    println!("\n📝 Test 2: Creating dynamic routes...");
    run_command("cargo", &["run", "--bin", "scaffold", "--", "products/detail/product_id"])?;
    run_command("cargo", &["run", "--bin", "scaffold", "--", "users/profile/user_slug"])?;
    run_command("cargo", &["run", "--bin", "scaffold", "--", "orders/detail/order_uuid"])?;
    run_command("cargo", &["run", "--bin", "scaffold", "--", "posts/detail/post_key"])?;

    // Test 3: Build the project
    println!("\n🔨 Test 3: Building project...");
    run_command("cargo", &["build"])?;

    // Test 4: Verify generated files
    println!("\n✅ Test 4: Verifying generated files...");
    verify_file_exists("src/routes/api/test/items/list.rs")?;
    verify_file_exists("src/routes/api/test/items/search.rs")?;
    verify_file_exists("src/routes/api/users/list.rs")?;
    verify_file_exists("src/routes/api/products/detail/product_id.rs")?;
    verify_file_exists("src/routes/api/users/profile/user_slug.rs")?;
    verify_file_exists("src/routes/api/orders/detail/order_uuid.rs")?;
    verify_file_exists("src/routes/api/posts/detail/post_key.rs")?;

    // Test 5: Check if handlers compile correctly
    println!("\n🔍 Test 5: Checking handler compilation...");
    check_handler_compilation("src/routes/api/test/items/list.rs")?;
    check_handler_compilation("src/routes/api/products/detail/product_id.rs")?;

    // Test 6: Clean up - delete created routes
    println!("\n🗑️  Test 6: Cleaning up test routes...");
    cleanup_files(&[
        "src/routes/api/test/items/list.rs",
        "src/routes/api/test/items/search.rs",
        "src/routes/api/users/list.rs",
        "src/routes/api/products/detail/product_id.rs",
        "src/routes/api/users/profile/user_slug.rs",
        "src/routes/api/orders/detail/order_uuid.rs",
        "src/routes/api/posts/detail/post_key.rs",
    ])?;

    // Final build to ensure cleanup didn't break anything
    println!("\n🔨 Final build after cleanup...");
    run_command("cargo", &["build"])?;

    println!("\n🎉 All tests passed! Scaffold system is working correctly.");
    println!("=================================");
    println!("✅ Static routes: Created and deleted successfully");
    println!("✅ Dynamic routes: Created and deleted successfully");
    println!("✅ Build system: Working correctly");
    println!("✅ File generation: Working correctly");
    println!("✅ Cleanup: Working correctly");

    Ok(())
}

fn run_command(command: &str, args: &[&str]) -> std::io::Result<()> {
    println!("Running: {} {}", command, args.join(" "));
    let output = Command::new(command)
        .args(args)
        .current_dir("apps/rust")
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("❌ Command failed: {}", stderr);
        std::process::exit(1);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    if !stdout.trim().is_empty() {
        println!("{}", stdout);
    }

    Ok(())
}

fn verify_file_exists(path: &str) -> std::io::Result<()> {
    let full_path = format!("apps/rust/{}", path);
    if fs::metadata(&full_path).is_ok() {
        println!("✅ File exists: {}", path);
        Ok(())
    } else {
        println!("❌ File missing: {}", path);
        Err(std::io::Error::new(std::io::ErrorKind::NotFound, "File not found"))
    }
}

fn check_handler_compilation(path: &str) -> std::io::Result<()> {
    let full_path = format!("apps/rust/{}", path);
    let content = fs::read_to_string(&full_path)?;

    // Check for basic handler structure
    if content.contains("pub async fn") && content.contains("Json(") {
        println!("✅ Handler structure OK: {}", path);
        Ok(())
    } else {
        println!("❌ Handler structure invalid: {}", path);
        Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid handler structure"))
    }
}

fn cleanup_files(paths: &[&str]) -> std::io::Result<()> {
    for path in paths {
        let full_path = format!("apps/rust/{}", path);
        if fs::metadata(&full_path).is_ok() {
            fs::remove_file(&full_path)?;
            println!("🗑️  Deleted: {}", path);
        }
    }
    Ok(())
}
