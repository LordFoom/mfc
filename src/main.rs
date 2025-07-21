use args::CliArgs;
use clap::Parser;

mod args;
mod compressor;
use anyhow::Result;
use compressor::{
    compress_directory_files, compress_directory_into_tar, compress_file, compress_tar_into_gz,
};

///TODO want directory to compress into ONE thing not MANY things
///TODO add an approve maybe? I almost burned myself :sweaty:
fn main() -> Result<()> {
    let args = CliArgs::parse();

    let path = std::path::Path::new(&args.path);
    if !path.exists() {
        panic!("Cannot compress non-existent path: {:?}", path)
    }
    if path.is_dir() {
        if args.single_file {
            let tar_file = compress_directory_into_tar(&path)?;
        } else {
            compress_directory_files(&path)?
        }
    } else {
        let _ = compress_file(&path)?;
    };

    println!("I'mma I'mma Compressorizer, Compressorizer");
    Ok(())
}
