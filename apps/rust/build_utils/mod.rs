pub mod constants;
pub mod handler_template;
pub mod handler_updater;
pub mod mod_generator;
pub mod openapi_generator;
pub mod path_utils;

use std::fs;
use std::path::{ Path, PathBuf };
use anyhow::{ Context, Result };

/// Tracks file backups for rollback purposes
#[derive(Debug)]
pub struct FileBackup {
  pub path: PathBuf,
  pub original_content: String,
}

impl FileBackup {
  pub fn new(path: &Path) -> Result<Self> {
    let original_content = fs
      ::read_to_string(path)
      .with_context(|| format!("Failed to read file for backup: {:?}", path))?;

    Ok(Self {
      path: path.to_path_buf(),
      original_content,
    })
  }

  pub fn restore(&self) -> Result<()> {
    fs
      ::write(&self.path, &self.original_content)
      .with_context(|| format!("Failed to restore file: {:?}", self.path))?;
    log::info!("Restored file: {:?}", self.path);
    Ok(())
  }
}

/// Build operation result with rollback capability
#[derive(Debug)]
pub struct BuildOperation {
  pub backups: Vec<FileBackup>,
  pub errors: Vec<String>,
}

#[allow(dead_code)]
impl BuildOperation {
  pub fn new() -> Self {
    Self {
      backups: Vec::new(),
      errors: Vec::new(),
    }
  }

  pub fn add_backup(&mut self, backup: FileBackup) {
    self.backups.push(backup);
  }

  pub fn add_error(&mut self, error: String) {
    self.errors.push(error);
  }

  pub fn has_errors(&self) -> bool {
    !self.errors.is_empty()
  }

  pub fn rollback(&self) -> Result<()> {
    log::warn!("Rolling back {} file changes due to errors", self.backups.len());
    for backup in &self.backups {
      backup.restore()?;
    }
    Ok(())
  }

  pub fn clear_backups(&mut self) {
    self.backups.clear();
  }
}
