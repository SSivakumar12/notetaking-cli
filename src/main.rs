use clap::{Parser, Subcommand};
use std::io::{self};
mod utils;

const FILE_PATH: &'static str = "notes.json";

#[derive(Parser)]
//#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        title: String,
        body: String,
    },
    Remove {
        id: u32,
    },
    Modify {
        id: u32,
        title: Option<String>,
        body: Option<String>,
    },
    List,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { title, body } => utils::add_note(title, body, FILE_PATH)?,
        Commands::Remove { id } => utils::remove_note(id, FILE_PATH)?,
        Commands::Modify { id, title, body } => utils::modify_note(id, title, body, FILE_PATH)?,
        Commands::List => utils::list_notes(FILE_PATH),
    }
    Ok(())
}
