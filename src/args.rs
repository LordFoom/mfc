use clap::Parser;
#[derive(Parser)]
#[command(
    author = "foom",
    version = "1.1",
    about = "Flashcards in rust",
    long_about = "Flashcard to make knowledge stick like rust to metal"
)]
pub struct CliArgs {
    #[arg(short, long, conflicts_with = "directory")]
    pub files: Vec<String>,
    #[arg(short, long, conflicts_with = "files")]
    pub directory: Option<String>,
}
