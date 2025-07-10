use anyhow::{Context, Result, anyhow};
use flate2::Compression;
use flate2::write::GzEncoder;
use std::io::{BufReader, BufWriter, Read};
use std::{fmt::format, fs::File, path::Path};

pub fn compress_directory(dir_path: &Path) -> Result<()> {
    if !dir_path.exists() {
        let err_msg = format!(
            "Directory {} does not exist",
            dir_path.to_str().unwrap_or("<invalid UTF-8 path>")
        );
        return Err(anyhow!(err_msg));
    }
    std::fs::read_dir(dir_path)?
        .into_iter()
        .try_for_each::<_, Result<_, anyhow::Error>>(|dir_entry_result| {
            let dir_entry =
                dir_entry_result.with_context(|| format!("Failed to read entry {:?}", dir_path))?;

            let metadata = dir_entry
                .metadata()
                .with_context(|| format!("Failed to get metadata for {:?}", dir_path))?;

            if metadata.is_dir() {
                compress_directory(dir_entry.path().as_path())?;
            } else {
                compress_file(dir_entry.path().as_path())?;
            };
            Ok(())
        })?;
    Ok(())
}

pub fn compress_file(file_path: &Path) -> Result<()> {
    let source = File::open(file_path)?;
    let sink = File::create_new(file_path.with_extension("gz"))?;
    let mut reader = BufReader::new(source);
    let mut writer = GzEncoder::new(BufWriter::new(sink), Compression::default());
    std::io::copy(&mut reader, &mut writer);
    writer.finish();
    Ok(())
}
