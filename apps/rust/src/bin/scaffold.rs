//! Scaffold tool for generating API handler templates.
//!
//! This binary provides a CLI for quickly generating new API handler files
//! with proper structure, types, and OpenAPI annotations.

use anyhow::{Context, Result};
use clap::Parser;
use rust::build_utils::template_generator;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process;

#[derive(Parser)]
#[command(name = "scaffold")]
#[command(about = "Generate API handler templates", version, long_about = None)]
struct Args {
    /// The route path for the API handler (e.g., "users/profile" or "posts/[id]")
    route_path: String,

    /// Generate a protected API handler that requires authentication
    #[arg(long, short = 'p')]
    protected: bool,

    /// Show what would be generated without creating files
    #[arg(long, short = 'd')]
    dry_run: bool,

    /// Enable verbose output for debugging
    #[arg(long, short = 'v')]
    verbose: bool,

    /// Skip automatic formatting with rustfmt
    #[arg(long)]
    skip_format: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.verbose {
        println!("🔍 Verbose mode enabled");
        println!("Route path: {}", args.route_path);
        println!("Protected: {}", args.protected);
        println!("Dry run: {}", args.dry_run);
    }

    // Validate route path
    validate_route_path(&args.route_path)?;

    let base_api_dir = get_base_api_dir(&args)?;
    let mut file_path = base_api_dir.join(&args.route_path);
    file_path.set_extension("rs");

    if args.verbose {
        println!("📂 Base API directory: {}", base_api_dir.display());
        println!("📄 Target file: {}", file_path.display());
    }

    if file_path.exists() {
        eprintln!(
            "⚠️  File already exists at {}\n   No changes were made.",
            file_path.display()
        );
        process::exit(0);
    }

    let template_content =
        template_generator::generate_template_content(&file_path, &base_api_dir, args.protected)
            .context("Failed to generate template content")?;

    if args.dry_run {
        println!("🔍 Dry run mode - would create file at: {}", file_path.display());
        println!("\n--- Template Content ---\n");
        println!("{}", template_content);
        println!("\n--- End of Template ---\n");
        return Ok(());
    }

    let parent_dir = file_path
        .parent()
        .context("Could not determine parent directory")?;
    fs::create_dir_all(parent_dir)
        .with_context(|| format!("Failed to create directory: {}", parent_dir.display()))?;

    fs::write(&file_path, &template_content)
        .with_context(|| format!("Failed to write file: {}", file_path.display()))?;

    println!(
        "✅ Handler template generated successfully\n   📄 {}",
        file_path.display()
    );

    // Auto-format the generated file unless skipped
    if !args.skip_format {
        format_file(&file_path, args.verbose)?;
    }

    if args.verbose {
        println!("✨ Done!");
    }

    Ok(())
}

/// Validate the route path format
fn validate_route_path(path: &str) -> Result<()> {
    if path.is_empty() {
        anyhow::bail!("Route path cannot be empty");
    }

    if path.starts_with('/') {
        anyhow::bail!(
            "Route path should not start with '/' (got: '{}')",
            path
        );
    }

    if path.contains("..") {
        anyhow::bail!("Route path cannot contain '..' (got: '{}')", path);
    }

    Ok(())
}

/// Get the base API directory, with improved error messages
fn get_base_api_dir(args: &Args) -> Result<PathBuf> {
    let cwd = env::current_dir().context("Failed to get current directory")?;

    let candidates = vec![
        cwd.join("src").join("routes").join("api"),
        cwd.join("apps")
            .join("rust")
            .join("src")
            .join("routes")
            .join("api"),
    ];

    if args.verbose {
        println!("🔍 Searching for API directory...");
        for candidate in &candidates {
            println!("   - Checking: {}", candidate.display());
        }
    }

    for candidate in &candidates {
        if candidate.exists() {
            if args.verbose {
                println!("   ✓ Found: {}", candidate.display());
            }
            return Ok(candidate.clone());
        }
    }

    // If not found, create the default structure
    let default_path = if cwd.join("apps").join("rust").exists() {
        cwd.join("apps")
            .join("rust")
            .join("src")
            .join("routes")
            .join("api")
    } else {
        cwd.join("src").join("routes").join("api")
    };

    if args.verbose {
        println!("   ℹ️  Will use: {}", default_path.display());
    }

    Ok(default_path)
}

/// Format the generated file using rustfmt
fn format_file(file_path: &PathBuf, verbose: bool) -> Result<()> {
    if verbose {
        println!("🎨 Formatting file with rustfmt...");
    }

    let output = process::Command::new("rustfmt")
        .arg(file_path)
        .output()
        .context("Failed to run rustfmt. Is it installed?")?;

    if !output.status.success() {
        if verbose {
            eprintln!("⚠️  rustfmt failed, but file was created");
            if !output.stderr.is_empty() {
                eprintln!("   Error: {}", String::from_utf8_lossy(&output.stderr));
            }
        }
    } else if verbose {
        println!("   ✓ Formatted successfully");
    }

    Ok(())
}
