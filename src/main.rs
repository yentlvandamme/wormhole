use std::{collections::HashMap, path::PathBuf};
use clap::{Parser, Args, Subcommand};

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
        Commands::Add(args) => aliases.add(args.alias_name.clone()),
        Commands::Print => aliases.all_aliases(),
    }
}

pub struct Aliases {
    aliases: HashMap<String, PathBuf>
}

impl Aliases {
    pub fn init () -> Self {
        Aliases{aliases: HashMap::new()}
    }

    pub fn add (&mut self, alias_name: String) {
        self.aliases.insert(alias_name, PathBuf::new());
    }

    pub fn all_aliases (&self) {
        let all_keys = self.aliases.keys();

        for key in all_keys {
            println!("{:?}", key);
        }
    }
}

// The idea is to build a struct which handles the aliases (storing, retrieving, updating,
// removing)
//
// Down the line we'll implement additional commands such as moving, copying, removing, navigating
// to, ... the file/folder the alias path is pointing to. For now we just want to make sure that we
// can do CRUD operations
