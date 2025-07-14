use args::CliArgs;
use clap::Parser;

mod args;
mod compressor;
use anyhow::Result;
use compressor::{compress_directory, compress_file};

fn main() -> Result<()> {
    let args = CliArgs::parse();

    let path = std::path::Path::new(&args.path);
    if !path.exists() {
        panic!("Cannot compress non-existent path: {:?}", path)
    }
    if path.is_dir() {
        compress_directory(&path)?
    } else {
        compress_file(&path)?
    };

    println!("I'mma I'mma Compressorizer, Compressorizer");
    Ok(())
}
