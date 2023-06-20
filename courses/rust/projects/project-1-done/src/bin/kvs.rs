use clap::{Parser, Subcommand};
use kvs::KvStore;

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

fn main() {
    let cli = Cli::parse();

    let mut store = KvStore::new();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Get { key } => match store.get(key.to_string()) {
            Some(value) => println!("{}", value),
            None => {
                eprintln!("Key not found");
                std::process::exit(1);
            }
        },
        Commands::Set { key, value } => {
            store.set(key.to_string(), value.to_string());
        }
        Commands::Rm { key } => {
            store.remove(key.to_string());
        }
    }
}
