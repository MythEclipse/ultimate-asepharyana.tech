use std::env;
use std::fs;
use std::process;

fn main() -> std::io::Result<()> {
  let args: Vec<String> = env::args().collect();

  if args.len() != 2 {
    eprintln!("❌ Usage: cargo run --bin scaffold -- <route_path>");
    eprintln!("   Example (static):  cargo run --bin scaffold -- products/list");
    eprintln!("   Example (dynamic): cargo run --bin scaffold -- products/detail/id");
    process::exit(1);
  }

  let full_route_path = args[1].clone();
  let parts: Vec<&str> = full_route_path.split('/').collect();
  let last_part = parts.last().unwrap_or(&"");

  let is_dynamic_route =
    *last_part == "id" ||
    *last_part == "slug" ||
    *last_part == "uuid" ||
    *last_part == "key";

  let base_api_dir = get_base_api_dir()?;
  let mut file_path = base_api_dir.join(&full_route_path);
  file_path.set_extension("rs");

  if file_path.exists() {
    println!("⚠️ File already exists at {}. No changes were made.", file_path.display());
    process::exit(0);
  }

  let parent_dir = file_path.parent().unwrap();
  fs::create_dir_all(parent_dir)?;

  let initial_content = if is_dynamic_route {
    "//! DYNAMIC_ROUTE\n".to_string()
  } else {
    "".to_string()
  };

  fs::write(&file_path, initial_content)?;

  println!("✅ Empty file created successfully at: {}", file_path.display());
  println!(
    "   Run `cargo build` (in the crate) to auto-populate the file with the handler template."
  );

  Ok(())
}

fn get_base_api_dir() -> std::io::Result<std::path::PathBuf> {
  let cwd = env::current_dir()?;
  let candidate_a = cwd.join("src").join("routes").join("api");
  let candidate_b = cwd.join("apps").join("rust").join("src").join("routes").join("api");

  if candidate_a.exists() {
    Ok(candidate_a)
  } else if candidate_b.exists() {
    Ok(candidate_b)
  } else if cwd.join("apps").join("rust").exists() {
    Ok(cwd.join("apps").join("rust").join("src").join("routes").join("api"))
  } else {
    Ok(cwd.join("src").join("routes").join("api"))
  }
}
