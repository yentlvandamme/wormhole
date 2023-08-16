use clap::{Parser, Args, Subcommand};
use crate::aliases::{Aliases, AliasName};

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
    Navigate(NavigateArgs),
    Print
}

#[derive(Args)]
struct AddArgs {
    alias_name: AliasName,
    path: Option<String>
}

#[derive(Args)]
struct RemoveArgs {
    alias_name: AliasName
}

#[derive(Args)]
struct NavigateArgs {
    alias_name: AliasName
}

fn main() {
    let cli = Cli::parse();
    let mut aliases = Aliases::init();

    match cli.command {
        Commands::Add(args) => aliases.add(args.alias_name, args.path),
        Commands::Remove(args) => aliases.remove(args.alias_name),
        Commands::Navigate(args) => aliases.navigate_to_alias(args.alias_name),
        Commands::Print => aliases.all_aliases(),
    };
}

