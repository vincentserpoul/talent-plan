use clap::{Parser, Subcommand};
use kvs::{KvStore, Result};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Retrieves value from kv store
    Get { key: String },
    /// Adds value to kv store
    Set { key: String, value: String },
    /// Removes value from kv store
    Rm { key: String },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut store = KvStore::open(std::path::Path::new("./"))?; // TODO: make this configurable

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Get { key } => match store.get(key.to_string()) {
            Ok(Some(value)) => {
                println!("{}", value);
                Ok(())
            }
            Ok(None) => {
                println!("Key not found");
                Ok(())
            }
            Err(e) => {
                eprintln!("{:?}", e);
                std::process::exit(1);
            }
        },
        Commands::Set { key, value } => store.set(key.to_string(), value.to_string()),
        Commands::Rm { key } => match store.remove(key.to_string()) {
            Ok(_) => Ok(()),
            _ => {
                println!("Key not found");
                std::process::exit(1);
            }
        },
    }
}
