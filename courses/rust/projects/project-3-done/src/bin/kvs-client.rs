use clap::{Parser, Subcommand};
use kvs::{KvCommand, Result};
use log::info;
use serde_json::json;
use std::io::prelude::*;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(long, global = true)]
    addr: Option<SocketAddr>,
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

impl From<Commands> for KvCommand {
    fn from(cmd: Commands) -> Self {
        match cmd {
            Commands::Get { key } => KvCommand::Get(key),
            Commands::Set { key, value } => KvCommand::Set(key, value),
            Commands::Rm { key } => KvCommand::Remove(key),
        }
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    env_logger::init();

    let cli_addr = if let Some(addr) = cli.addr {
        addr
    } else {
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 4000)
    };

    let mut connection = TcpStream::connect(cli_addr)?;

    info!("connected");

    let cmd = KvCommand::from(cli.command);

    writeln!(connection, "{}", json!(cmd))?;

    let mut reader = std::io::BufReader::new(connection.try_clone().unwrap());
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();

    let line = line.trim();
    if !line.is_empty() {
        if line.contains("error: ") {
            eprint!("{}", &line[7..]);
            std::process::exit(1);
        }

        if line.contains("Key not found") {
            print!("{}", &line);
        } else {
            println!("{}", line);
        }
    }

    Ok(())
}
