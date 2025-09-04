use std::env;
use std::fs;
use std::path::PathBuf;
use std::process;

fn main() -> std::io::Result<()> {
  let args: Vec<String> = env::args().collect();

  if args.len() != 2 {
    eprintln!("❌ Usage: cargo run --bin scaffold -- <route_path>");
    eprintln!("   Example (static):  cargo run --bin scaffold -- products/list");
    eprintln!("   Example (index):   cargo run --bin scaffold -- products/index");
    eprintln!("   Example (dynamic): cargo run --bin scaffold -- products/[id]");
    process::exit(1);
  }

  let route_path = &args[1];

  // Determine base API directory so this can be run from workspace root or from apps/rust
  let cwd = env::current_dir()?;
  let candidate_a = cwd.join("src").join("routes").join("api"); // when running from apps/rust
  let candidate_b = cwd.join("apps").join("rust").join("src").join("routes").join("api"); // when running from workspace root

  let base_api_dir = if candidate_a.exists() {
    candidate_a
  } else if candidate_b.exists() {
    candidate_b
  } else if cwd.join("apps").join("rust").exists() {
    // workspace root but api dir not present yet -> prefer apps/rust path
    cwd.join("apps").join("rust").join("src").join("routes").join("api")
  } else {
    // fallback to the local src path
    cwd.join("src").join("routes").join("api")
  };

  // Build full file path, e.g. "<base_api_dir>/products/list.rs"
  let mut file_path = base_api_dir.clone();
  file_path.push(route_path);
  file_path.set_extension("rs");

  // Check if file already exists
  if file_path.exists() {
    println!("⚠️ File already exists at {}. No changes were made.", file_path.display());
    process::exit(0);
  }

  // Create parent directory if missing
  if let Some(parent_dir) = file_path.parent() {
    fs::create_dir_all(parent_dir)?;
  } else {
    eprintln!("❌ Invalid file path specified: {}", file_path.display());
    process::exit(1);
  }

  // Create empty file. `build.rs` will populate it on cargo build.
  fs::write(&file_path, "")?;

  println!("✅ Empty file created successfully at: {}", file_path.display());
  println!("   Run `cargo build` (in the crate) to auto-populate the file with the handler template.");

  Ok(())
}
