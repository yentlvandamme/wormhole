use std::{collections::HashMap, path::PathBuf};
use clap::{Parser, Args, Subcommand};

#[derive(Parser)]
struct Cli {
    // The subcommand name which should be executed (add, remove, edit, ...)
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    // Add alias to library
    Add(DefaultArgs)
}

#[derive(Args)]
struct DefaultArgs {
    alias_name: String,
    path: Option<String> // To be transformed to a PathBuf
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add(args) => println!("ADDING, {}", args.alias_name), // Handle the logic to add
                                                                        // a command
    }
    let mut aliases: HashMap<String, PathBuf> = HashMap::new();
}
