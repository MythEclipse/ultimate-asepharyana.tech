use std::fs;
use std::path::Path;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use anyhow::{Result, Context};
use rayon::prelude::*;

/// Compute hash of all files in the API routes directory for conditional regeneration
pub fn compute_directory_hash(dir: &Path) -> Result<u64> {
    fn collect_rs_files(dir: &Path) -> Result<Vec<std::path::PathBuf>> {
        Ok(fs::read_dir(dir)
            .with_context(|| format!("Failed to read directory for hashing: {:?}", dir))?
            .par_bridge()
            .map(|entry| {
                let entry = entry.with_context(|| format!("Failed to read entry in directory: {:?}", dir))?;
                let path = entry.path();
                if path.is_dir() {
                    collect_rs_files(&path)
                } else if path.extension().map_or(false, |e| e == "rs") {
                    Ok(vec![path])
                } else {
                    Ok(vec![])
                }
            })
            .collect::<Result<Vec<Vec<std::path::PathBuf>>>>()?
            .into_iter()
            .flatten()
            .collect::<Vec<std::path::PathBuf>>())
    }

    let mut hasher = DefaultHasher::new();
    let mut paths = collect_rs_files(dir)?;

    // Sort for consistent hashing
    paths.sort();

    for path in paths {
        let content = fs::read(&path).with_context(|| format!("Failed to read file for hashing: {:?}", path))?;
        content.hash(&mut hasher);
        // Also hash modification time
        if let Ok(metadata) = fs::metadata(&path) {
            if let Ok(modified) = metadata.modified() {
                modified.hash(&mut hasher);
            }
        }
    }

    Ok(hasher.finish())
}
