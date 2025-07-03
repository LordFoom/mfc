use args::CliArgs;
use clap::Parser;

mod args;

fn main() {
    let args = CliArgs::parse();
    println!("I'mma I'mma Compressorizer, Compressorizer");
}
