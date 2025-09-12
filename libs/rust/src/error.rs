// Error module for crate-wide error handling

use thiserror::Error;

#[derive(Error, Debug)]
pub enum LibError {
  #[error("An unknown error occurred")]
  Unknown,
  #[error("Fantoccini error: {0}")] FantocciniError(String),
}
