use std::fs;

use args::CliArgs;
use clap::Parser;

mod args;
mod compressor;
use anyhow::Result;

fn main() -> Result<()> {
    let args = CliArgs::parse();

    if let Some(dir_name) = args.directory {
        let dir_path = std::path::Path::new(&dir_name);
        if dir_path.is_dir() {
            //compress this guy
        } else {
            //throw an error
        }
    }

    println!("I'mma I'mma Compressorizer, Compressorizer");
    Ok(())
}
