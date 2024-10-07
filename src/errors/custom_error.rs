use anyhow::{Context, Result};
use thiserror::Error;


// You can now use anyhow::Result in your function signatures
pub type CustomResult<T> = Result<T, anyhow::Error>;

// Example function using anyhow
pub fn some_fallible_function() -> CustomResult<()> {
    // Some operation that might fail
    std::fs::File::open("non_existent_file.txt")
        .context("Failed to open file")?;
    
    Ok(())
}
