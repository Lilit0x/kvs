use std::process::exit;

use clap::{Parser, Subcommand};
use kvs::Result;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Set the value of a string key to a string
    Set {
        /// key
        key: String,

        /// value
        value: String,
    },
    /// Get the string value of a given string key
    Get {
        /// key to get
        key: String,
    },
    /// Remove a given key
    Rm {
        /// key to remove
        key: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Set { key, value } => {
            eprintln!("unimplemented");
            exit(1);
        }
        Commands::Get { key } => {
            eprintln!("unimplemented");
            exit(1);
        }
        Commands::Rm { key } => {
            eprintln!("unimplemented");
            exit(1);
        }
    }
}
