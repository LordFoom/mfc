use anyhow::{Context, Error, Result};
use std::{
    fmt::{Result, format},
    fs::File,
    path::Path,
};

pub fn compress_directory(dir_path: &Path) -> Result<()> {
    if !dir_path.exists() {
        let err_msg = format!("Directory {} does not exist", dir_path.to_str());
        return Err(anyhow::Error::new(err_msg));
    }
    std::fs::read_dir(dir_path)?
        .into_iter()
        .try_for_each(|dir_entry_result| {
            let dir_entry =
                dir_entry_result.with_context(|| format!("Failed to read entry {:?}", dir_path))?;

            let metadata = dir_entry
                .metadata()
                .with_context(|| format!("Failed to get metadata for {:?}", dir_path))?;

            if metadata.is_dir() {
                compress_directory(dir_entry.path().as_path());
            } else {
                compress_file(dir_entry.path().as_path());
            };
            Ok(())
        });
    Ok(())
}

pub fn compress_file(file_path: &Path) -> Result<()> {}
