use std::path::Path;

use crate::error::Result;
use crate::error::Error::DirectoryExistence;

pub mod date_range;
pub mod deserialize;

pub fn check_path(path: &str) -> Result<()> {
    if !Path::new(path).is_dir() {
        return Err(DirectoryExistence(path.to_string()));
    }
    Ok(())
}