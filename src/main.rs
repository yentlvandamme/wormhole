use clap::{Parser, Args, Subcommand};
use crate::aliases::Aliases;

pub mod aliases;
pub mod file_manager;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add(AddArgs),
    Remove(RemoveArgs),
    Print
}

#[derive(Args)]
struct AddArgs {
    alias_name: String,
    path: Option<String>
}

#[derive(Args)]
struct RemoveArgs {
    alias_name: String
}

fn main() {
    let cli = Cli::parse();

    let mut aliases = Aliases::init();

    match cli.command {
        Commands::Add(args) => aliases.add(args.alias_name, args.path),
        Commands::Remove(args) => aliases.remove(args.alias_name),
        Commands::Print => aliases.all_aliases(),
    };
}

