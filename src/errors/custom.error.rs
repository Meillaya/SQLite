use anyhow::{Context, Result};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CustomError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] rusqlite::Error),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    // Add more specific error types as needed
    #[error("Validation error: {0}")]
    ValidationError(String),
}

// You can now use anyhow::Result in your function signatures
pub type CustomResult<T> = Result<T, anyhow::Error>;

// Example function using anyhow
pub fn some_fallible_function() -> CustomResult<()> {
    // Some operation that might fail
    std::fs::File::open("non_existent_file.txt")
        .context("Failed to open file")?;
    
    Ok(())
}
