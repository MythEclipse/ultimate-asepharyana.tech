use std::env;
use std::fs;
use std::process::Command;

fn main() -> std::io::Result<()> {
    println!("🧪 Starting Scaffold System Test");
    println!("=================================");

    // Initial cleanup to ensure a clean slate for the test
    println!("\n🗑️  Initial cleanup of potential test routes...");
    cleanup_files(&[
        "src/routes/api/test/v1/list.rs",
        "src/routes/api/test/v1/search.rs",
        "src/routes/api/test/v1/users/list.rs",
        "src/routes/api/test/v2/products/detail/id.rs",
        "src/routes/api/test/v2/users/profile/slug.rs",
        "src/routes/api/test/v2/orders/detail/uuid.rs",
        "src/routes/api/test/v2/posts/detail/key.rs",
    ])?;

    println!("\n📝 Test 1: Creating static routes...");
    run_scaffold_batch(&["test/v1/list", "test/v1/search", "test/v1/users/list"])?;

    println!("\n📝 Test 2: Creating dynamic routes... (using new single-file dynamic routing)");
    run_scaffold_batch(&[
        "test/v2/products/detail/id",
        "test/v2/users/profile/slug",
        "test/v2/orders/detail/uuid",
        "test/v2/posts/detail/key",
    ])?;

    println!("\n📝 Test 3: Creating protected static routes...");
    run_scaffold_batch_protected(&[
        "test/v3/protected/list",
        "test/v3/protected/search",
        "test/v3/protected/users/list",
    ])?;

    println!("\n📝 Test 4: Creating protected dynamic routes...");
    run_scaffold_batch_protected(&[
        "test/v3/protected/products/detail/id",
        "test/v3/protected/users/profile/slug",
        "test/v3/protected/orders/detail/uuid",
        "test/v3/protected/posts/detail/key",
    ])?;

    println!("\n✅ Test 5: Verifying generated files...");
    verify_file_exists("src/routes/api/test/v1/list.rs")?;
    verify_file_exists("src/routes/api/test/v1/search.rs")?;
    verify_file_exists("src/routes/api/test/v1/users/list.rs")?;
    verify_file_exists("src/routes/api/test/v2/products/detail/id.rs")?;
    verify_file_exists("src/routes/api/test/v2/users/profile/slug.rs")?;
    verify_file_exists("src/routes/api/test/v2/orders/detail/uuid.rs")?;
    verify_file_exists("src/routes/api/test/v2/posts/detail/key.rs")?;
    verify_file_exists("src/routes/api/test/v3/protected/list.rs")?;
    verify_file_exists("src/routes/api/test/v3/protected/search.rs")?;
    verify_file_exists("src/routes/api/test/v3/protected/users/list.rs")?;
    verify_file_exists("src/routes/api/test/v3/protected/products/detail/id.rs")?;
    verify_file_exists("src/routes/api/test/v3/protected/users/profile/slug.rs")?;
    verify_file_exists("src/routes/api/test/v3/protected/orders/detail/uuid.rs")?;
    verify_file_exists("src/routes/api/test/v3/protected/posts/detail/key.rs")?;

    println!("\n🔐 Test 7: Verifying protected routes contain authentication boilerplate...");
    verify_protected_content("src/routes/api/test/v3/protected/list.rs")?;
    verify_protected_content("src/routes/api/test/v3/protected/search.rs")?;
    verify_protected_content("src/routes/api/test/v3/protected/users/list.rs")?;
    verify_protected_content("src/routes/api/test/v3/protected/products/detail/id.rs")?;
    verify_protected_content("src/routes/api/test/v3/protected/users/profile/slug.rs")?;
    verify_protected_content("src/routes/api/test/v3/protected/orders/detail/uuid.rs")?;
    verify_protected_content("src/routes/api/test/v3/protected/posts/detail/key.rs")?;

    println!("\n📋 Test 8: Verifying OpenAPI documentation includes security schemes...");
    verify_openapi_security_schemes()?;

    println!("\n🗑️  Test 6: Cleaning up test routes...");
    cleanup_files(&[
        "src/routes/api/test/v1/list.rs",
        "src/routes/api/test/v1/search.rs",
        "src/routes/api/test/v1/users/list.rs",
        "src/routes/api/test/v2/products/detail/id.rs",
        "src/routes/api/test/v2/users/profile/slug.rs",
        "src/routes/api/test/v2/orders/detail/uuid.rs",
        "src/routes/api/test/v2/posts/detail/key.rs",
        "src/routes/api/test/v3/protected/list.rs",
        "src/routes/api/test/v3/protected/search.rs",
        "src/routes/api/test/v3/protected/users/list.rs",
        "src/routes/api/test/v3/protected/products/detail/id.rs",
        "src/routes/api/test/v3/protected/users/profile/slug.rs",
        "src/routes/api/test/v3/protected/orders/detail/uuid.rs",
        "src/routes/api/test/v3/protected/posts/detail/key.rs",
    ])?;

    println!("\n🎉 All tests passed! Scaffold system is working correctly.");
    println!("=================================");
    println!("✅ Static routes: Created and deleted successfully");
    println!("✅ Dynamic routes: Created and deleted successfully");
    println!("✅ Protected static routes: Created and deleted successfully");
    println!("✅ Protected dynamic routes: Created and deleted successfully");
    println!("✅ Authentication boilerplate: Verified in protected routes");
    println!("✅ File generation: Working correctly");
    println!("✅ Cleanup: Working correctly");

    Ok(())
}

fn run_scaffold_batch(routes: &[&str]) -> std::io::Result<()> {
    for route in routes {
        println!("📝 Creating scaffold route: {}", route);
        run_command("cargo", &["run", "--bin", "scaffold", "--", route])?;
    }
    Ok(())
}

fn run_scaffold_batch_protected(routes: &[&str]) -> std::io::Result<()> {
    for route in routes {
        println!("🔒 Creating protected scaffold route: {}", route);
        run_command(
            "cargo",
            &["run", "--bin", "scaffold", "--", "--protected", route],
        )?;
    }
    Ok(())
}

fn run_command(command: &str, args: &[&str]) -> std::io::Result<()> {
    println!("Running: {} {}", command, args.join(" "));
    let output = Command::new(command).args(args).output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(std::io::Error::other(format!("Command failed: {}", stderr)));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    if !stdout.trim().is_empty() {
        println!("{}", stdout);
    }

    Ok(())
}

fn verify_file_exists(path: &str) -> std::io::Result<()> {
    // Determine base API directory so this can be run from workspace root or from apps/rust
    let cwd = env::current_dir()?;
    let candidate_a = cwd.join("src").join("routes").join("api"); // when running from apps/rust
    let candidate_b = cwd
        .join("apps")
        .join("rust")
        .join("src")
        .join("routes")
        .join("api"); // when running from workspace root

    let base_api_dir = if candidate_a.exists() {
        candidate_a
    } else if candidate_b.exists() {
        candidate_b
    } else if cwd.join("apps").join("rust").exists() {
        // workspace root but api dir not present yet -> prefer apps/rust path
        cwd.join("apps")
            .join("rust")
            .join("src")
            .join("routes")
            .join("api")
    } else {
        // fallback to the local src path
        cwd.join("src").join("routes").join("api")
    };

    // Build full file path
    let mut file_path = base_api_dir.clone();
    file_path.push(path.strip_prefix("src/routes/api/").unwrap_or(path));

    // For dynamic routes, the path already includes the file name (e.g., id.rs), so no need to set extension
    // For static routes, set the extension
    if !path.ends_with(".rs") {
        file_path.set_extension("rs");
    }

    if fs::metadata(&file_path).is_ok() {
        println!("✅ File exists: {}", path);
        Ok(())
    } else {
        println!("❌ File missing: {}", path);
        Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "File not found",
        ))
    }
}

fn verify_protected_content(path: &str) -> std::io::Result<()> {
    // Determine base API directory so this can be run from workspace root or from apps/rust
    let cwd = env::current_dir()?;
    let candidate_a = cwd.join("src").join("routes").join("api"); // when running from apps/rust
    let candidate_b = cwd
        .join("apps")
        .join("rust")
        .join("src")
        .join("routes")
        .join("api"); // when running from workspace root

    let base_api_dir = if candidate_a.exists() {
        candidate_a
    } else if candidate_b.exists() {
        candidate_b
    } else if cwd.join("apps").join("rust").exists() {
        // workspace root but api dir not present yet -> prefer apps/rust path
        cwd.join("apps")
            .join("rust")
            .join("src")
            .join("routes")
            .join("api")
    } else {
        // fallback to the local src path
        cwd.join("src").join("routes").join("api")
    };

    // Build full file path
    let mut file_path = base_api_dir.clone();
    file_path.push(path.strip_prefix("src/routes/api/").unwrap_or(path));

    // For dynamic routes, the path already includes the file name (e.g., id.rs), so no need to set extension
    // For static routes, set the extension
    if !path.ends_with(".rs") {
        file_path.set_extension("rs");
    }

    let content = fs::read_to_string(&file_path).map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Could not read file {}: {}", path, e),
        )
    })?;

    // Check for required authentication boilerplate
    let checks = [
        (
            "AuthMiddleware import",
            "use crate::middleware::auth::AuthMiddleware;",
        ),
        ("Claims import", "use crate::utils::auth::Claims;"),
        ("Extension import", "use axum::Extension;"),
        (
            "Extension<Claims> parameter",
            "Extension(claims): Extension<Claims>",
        ),
        ("AuthMiddleware::layer()", "AuthMiddleware::layer()"),
        ("security scheme", "security("),
        ("ApiKeyAuth", "(\"ApiKeyAuth\" = [])"),
    ];

    for (description, expected) in &checks {
        if !content.contains(expected) {
            println!("❌ {} missing in {}: {}", description, path, expected);
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("{} not found in protected route", description),
            ));
        }
    }

    println!("✅ Protected authentication boilerplate verified: {}", path);
    Ok(())
}

fn verify_openapi_security_schemes() -> std::io::Result<()> {
    println!("🔍 Verifying OpenAPI documentation includes security schemes...");

    // Get the OpenAPI spec from OUT_DIR (generated during build)
    let out_dir = env::var("OUT_DIR").map_err(|_| {
        std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "OUT_DIR environment variable not set",
        )
    })?;
    let openapi_path = std::path::Path::new(&out_dir).join("openapi_spec.json");

    if fs::metadata(&openapi_path).is_err() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "OpenAPI spec file not found in OUT_DIR",
        ));
    }

    // Read and parse the OpenAPI JSON
    let openapi_content = fs::read_to_string(&openapi_path)?;
    let openapi: serde_json::Value = serde_json::from_str(&openapi_content).map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Failed to parse OpenAPI JSON: {}", e),
        )
    })?;

    // Check for security at the root level
    if let Some(security) = openapi.get("security") {
        println!("✅ Found security at OpenAPI root");

        // Check for ApiKeyAuth in security
        if let Some(security_array) = security.as_array() {
            if let Some(first_security) = security_array.first() {
                if let Some(api_key_auth) = first_security.get("ApiKeyAuth") {
                    if api_key_auth.is_array() && api_key_auth.as_array().unwrap().is_empty() {
                        println!("✅ Found ApiKeyAuth security requirement");
                    } else {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            "ApiKeyAuth security requirement is not an empty array",
                        ));
                    }
                } else {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::NotFound,
                        "ApiKeyAuth not found in security",
                    ));
                }
            } else {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "No security requirements found",
                ));
            }
        } else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "security is not an array",
            ));
        }
    } else {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "security not found at OpenAPI root",
        ));
    }

    println!("✅ OpenAPI security schemes verification completed successfully");
    Ok(())
}

fn cleanup_files(paths: &[&str]) -> std::io::Result<()> {
    // Determine base API directory so this can be run from workspace root or from apps/rust
    let cwd = env::current_dir()?;
    let candidate_a = cwd.join("src").join("routes").join("api"); // when running from apps/rust
    let candidate_b = cwd
        .join("apps")
        .join("rust")
        .join("src")
        .join("routes")
        .join("api"); // when running from workspace root

    let base_api_dir = if candidate_a.exists() {
        candidate_a
    } else if candidate_b.exists() {
        candidate_b
    } else if cwd.join("apps").join("rust").exists() {
        // workspace root but api dir not present yet -> prefer apps/rust path
        cwd.join("apps")
            .join("rust")
            .join("src")
            .join("routes")
            .join("api")
    } else {
        // fallback to the local src path
        cwd.join("src").join("routes").join("api")
    };

    for path in paths {
        // Build full file path
        let mut file_path = base_api_dir.clone();
        file_path.push(path.strip_prefix("src/routes/api/").unwrap_or(path));

        // For dynamic routes, the path already includes the file name (e.g., id.rs), so no need to set extension
        // For static routes, set the extension
        if !path.ends_with(".rs") {
            file_path.set_extension("rs");
        }

        if fs::metadata(&file_path).is_ok() {
            fs::remove_file(&file_path)?;
            println!("🗑️  Deleted: {}", path);
        }
    }
    Ok(())
}
