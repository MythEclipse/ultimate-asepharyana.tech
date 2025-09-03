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

  // Buat path file lengkap, contoh: "src/routes/api/products/list.rs"
  let mut file_path = PathBuf::from("src/routes/api");
  file_path.push(route_path);
  file_path.set_extension("rs");

  // Cek apakah file sudah ada
  if file_path.exists() {
    println!("⚠️ File already exists at {:?}. No changes were made.", file_path);
    process::exit(0);
  }

  // Buat direktori parent jika belum ada
  if let Some(parent_dir) = file_path.parent() {
    fs::create_dir_all(parent_dir)?;
  } else {
    eprintln!("❌ Invalid file path specified.");
    process::exit(1);
  }

  // Buat file kosong. `build.rs` akan mengisinya nanti.
  fs::write(&file_path, "")?;

  println!("✅ Empty file created successfully at: {:?}", file_path);
  println!("   Run `cargo build` to auto-populate the file with the handler template.");

  Ok(())
}
