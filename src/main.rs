use clap::{Parser, Args, Subcommand};
use crate::alias::Aliases;

pub mod alias;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add(DefaultArgs),
    Print
}

#[derive(Args)]
struct DefaultArgs {
    alias_name: String,
    path: Option<String>
}

fn main() {
    let cli = Cli::parse();
    let mut aliases = Aliases::init();

    match cli.command {
        Commands::Add(args) => aliases.add(args.alias_name, args.path),
        Commands::Print => aliases.all_aliases(),
    }
}

