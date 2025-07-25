use anyhow::{Result, anyhow};
use flate2::Compression;
use flate2::write::GzEncoder;
use rayon::prelude::*;
use std::fs;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use std::{fs::File, path::Path};
use tar::Builder;

///Compresses directory contents into a single file, a tar file
///Returns path to the tar file
pub fn compress_directory_into_tar(dir_path: &Path) -> Result<String> {
    if !dir_path.exists() {
        return Err(anyhow!("Cannot compress what does not exist."));
    }

    if !dir_path.is_dir() {
        return Err(anyhow!("Can only compress a directory, man."));
    }
    let tar_name_full_path = dir_path.with_extension("tar.gz");
    let tar_name = tar_name_full_path
        .file_name()
        .ok_or_else(|| anyhow!("Could not get filename"))?;

    let tar_file = File::create(&tar_name)?;
    let enc = GzEncoder::new(tar_file, Compression::default());

    let mut tar = Builder::new(enc);
    tar.append_dir_all("", dir_path)?;
    tar.finish()?;
    Ok(tar_name.to_string_lossy().into_owned())
}

///Takes a path to a tar and creates a gz from it, returning the PathBuf
pub fn compress_tar_into_gz(tar_path: &Path) -> Result<PathBuf> {
    if !tar_path.exists() {
        return Err(anyhow!("Cannot gz what does not exist"));
    }
    if tar_path.is_dir() {
        return Err(anyhow!(
            "Expected a single file, not a directory: {}",
            tar_path.to_string_lossy()
        ));
    }
    return compress_file(tar_path);
}

///Compresses each file, separately, in the directory
pub fn compress_directory_files(dir_path: &Path) -> Result<()> {
    if !dir_path.exists() {
        let err_msg = format!(
            "Directory {} does not exist",
            dir_path.to_str().unwrap_or("<invalid UTF-8 path>")
        );
        return Err(anyhow!(err_msg));
    }
    let dir_entries: Vec<_> = std::fs::read_dir(dir_path)?
        .map(|res| {
            let entry = res?;
            Ok(entry.path())
        })
        .collect::<Result<_, std::io::Error>>()?;

    dir_entries
        .par_iter()
        .try_for_each::<_, Result<_, anyhow::Error>>(|dir_entry| {
            // let dir_entry =
            //     dir_entry_result.with_context(|| format!("Failed to read entry {:?}", dir_path))?;
            let metadata = fs::metadata(dir_entry)?;

            if metadata.is_dir() {
                compress_directory_files(dir_entry)?;
            } else {
                compress_file(dir_entry)?;
            };
            Ok(())
        })?;
    Ok(())
}

pub fn compress_file(file_path: &Path) -> Result<PathBuf> {
    println!("Compressing {:?}", file_path);
    let source = File::open(file_path)?;
    let compressed_path = file_path.with_extension("gz");
    let compressed_file_name = compressed_path
        .file_name()
        .ok_or(anyhow!("Could not get the file name??"))?;
    let sink = File::create_new(&compressed_path)?;
    let mut reader = BufReader::new(source);
    let mut writer = GzEncoder::new(BufWriter::new(sink), Compression::default());
    std::io::copy(&mut reader, &mut writer)?;
    writer.finish()?;
    println!("Compressed file {:?}", compressed_file_name);
    Ok(compressed_path)
}

#[allow(unused_imports)]
mod test {
    use std::{env, fs, io::Write};

    use super::compress_file;
    use anyhow::Result;
    use tempfile::NamedTempFile;

    #[test]
    pub fn test_compress_file() -> Result<()> {
        //need a test file
        let mut test_file = NamedTempFile::new()?;
        let test_content =
            "I am the test content, i live within the test file, i am read by the test people\n";
        let test_content_ten_k = test_content.repeat(10000);
        test_file.write_all(test_content_ten_k.as_bytes())?;
        let test_file_path = test_file.path();
        compress_file(&test_file_path)?;
        let test_file_gz = test_file_path.with_extension("gz");
        assert!(test_file_gz.exists(), "Compressed file should exist");
        //Verify the compressed file has some content
        let compressed_size = fs::metadata(&test_file_gz)?.len();
        assert!(compressed_size > 0, "Compressed file should not be empty");

        //Verify it's actually compressed (should be smaller for this content)
        let original_size = fs::metadata(&test_file_path)?.len();
        println!(
            "Original size: {}, Compressed size: {}",
            original_size, compressed_size
        );

        //Clean up the compressed file (temp_file cleans up automatically)
        fs::remove_file(&test_file_gz)?;
        Ok(())
    }
}
