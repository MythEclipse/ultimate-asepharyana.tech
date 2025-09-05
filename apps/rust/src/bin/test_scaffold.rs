use std::process::Command;
use std::fs;

fn main() -> std::io::Result<()> {
    println!("🧪 Starting Scaffold System Test");
    println!("=================================");

    // Initial cleanup to ensure a clean slate for the test
    println!("\n🗑️  Initial cleanup of potential test routes...");
    cleanup_files(&[
        "src/routes/api/test/static/list.rs",
        "src/routes/api/test/static/search.rs",
        "src/routes/api/test/static/users/list.rs",
        "src/routes/api/test/dynamic/products/detail/product_id.rs",
        "src/routes/api/test/dynamic/users/profile/user_slug.rs",
        "src/routes/api/test/dynamic/orders/detail/order_uuid.rs",
        "src/routes/api/test/dynamic/posts/detail/post_key.rs",
    ])?;

    println!("\n📝 Test 1: Creating static routes...");
    run_command("cargo", &["run", "--bin", "scaffold", "--", "test/static/list"])?;
    run_command("cargo", &["run", "--bin", "scaffold", "--", "test/static/search"])?;
    run_command("cargo", &["run", "--bin", "scaffold", "--", "test/static/users/list"])?;

    println!("\n📝 Test 2: Creating dynamic routes...");
    run_command("cargo", &["run", "--bin", "scaffold", "--", "test/dynamic/products/detail/product_id"])?;
    run_command("cargo", &["run", "--bin", "scaffold", "--", "test/dynamic/users/profile/user_slug"])?;
    run_command("cargo", &["run", "--bin", "scaffold", "--", "test/dynamic/orders/detail/order_uuid"])?;
    run_command("cargo", &["run", "--bin", "scaffold", "--", "test/dynamic/posts/detail/post_key"])?;

    println!("\n✅ Test 3: Verifying generated files...");
    verify_file_exists("src/routes/api/test/static/list.rs")?;
    verify_file_exists("src/routes/api/test/static/search.rs")?;
    verify_file_exists("src/routes/api/test/static/users/list.rs")?;
    verify_file_exists("src/routes/api/test/dynamic/products/detail/product_id.rs")?;
    verify_file_exists("src/routes/api/test/dynamic/users/profile/user_slug.rs")?;
    verify_file_exists("src/routes/api/test/dynamic/orders/detail/order_uuid.rs")?;
    verify_file_exists("src/routes/api/test/dynamic/posts/detail/post_key.rs")?;

    println!("\n🗑️  Test 4: Cleaning up test routes...");
    cleanup_files(&[
        "src/routes/api/test/static/list.rs",
        "src/routes/api/test/static/search.rs",
        "src/routes/api/test/static/users/list.rs",
        "src/routes/api/test/dynamic/products/detail/product_id.rs",
        "src/routes/api/test/dynamic/users/profile/user_slug.rs",
        "src/routes/api/test/dynamic/orders/detail/order_uuid.rs",
        "src/routes/api/test/dynamic/posts/detail/post_key.rs",
    ])?;

    println!("\n🎉 All tests passed! Scaffold system is working correctly.");
    println!("=================================");
    println!("✅ Static routes: Created and deleted successfully");
    println!("✅ Dynamic routes: Created and deleted successfully");
    println!("✅ File generation: Working correctly");
    println!("✅ Cleanup: Working correctly");

    Ok(())
}

fn run_command(command: &str, args: &[&str]) -> std::io::Result<()> {
    println!("Running: {} {}", command, args.join(" "));
    let output = Command::new(command)
        .args(args)
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
    let full_path = std::env::current_dir()?.join("apps").join("rust").join(path);
    if fs::metadata(&full_path).is_ok() {
        println!("✅ File exists: {}", path);
        Ok(())
    } else {
        println!("❌ File missing: {}", path);
        Err(std::io::Error::new(std::io::ErrorKind::NotFound, "File not found"))
    }
}

fn cleanup_files(paths: &[&str]) -> std::io::Result<()> {
    for path in paths {
        let full_path = std::env::current_dir()?.join("apps").join("rust").join(path);
        if fs::metadata(&full_path).is_ok() {
            fs::remove_file(&full_path)?;
            println!("🗑️  Deleted: {}", path);
        }
    }
    Ok(())
}
