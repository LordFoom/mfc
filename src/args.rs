use clap::Parser;
#[derive(Parser)]
#[command(
    author = "foom",
    version = "1.1",
    about = "Flashcards in rust",
    long_about = "Flashcard to make knowledge stick like rust to metal"
)]
pub struct CliArgs {
    ///path to directory or files - by default will compress all files IN a directory
    pub path: String,
    ///If this is set to true, and path is a directory, it will compress directory into single
    ///file, not compress each file separately
    #[arg(short, long)]
    pub single_file: bool,
}
