use std::{path::PathBuf, process::exit};

use clap::{Parser, Subcommand};
use kvs::{KvStore, Result};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, default_value = "test.log")]
    log: PathBuf,
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
            let mut store = KvStore::open(&cli.log)?;
            if let Err(err) = store.set(key.to_string(), value.to_string()) {
                eprintln!("{err}");
                exit(1);
            }
            exit(0);
        }
        Commands::Get { key } => {
            eprintln!("unimplemented");
            exit(1);
        }
        Commands::Rm { key } => {
            let mut store = KvStore::open(&cli.log)?;
            exit(1);
        }
    }
}
