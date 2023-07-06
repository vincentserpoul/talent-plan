use clap::{Parser, ValueEnum};
use kvs::{KvCommand, KvsEngine, Result};
use log::LevelFilter;
use log::{info, trace};
use std::fmt::Display;
use std::io::prelude::*;
use std::io::BufRead;
use std::net::{TcpListener, TcpStream};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(next_line_help = true)]
struct Cli {
    #[arg(long)]
    addr: std::net::SocketAddr,
    #[arg(long, value_enum, default_value_t = Engine::Kvs)]
    engine: Engine,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Engine {
    Kvs,
    Sled,
}

impl Display for Engine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Engine::Kvs => write!(f, "kvs"),
            Engine::Sled => write!(f, "sled"),
        }
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    env_logger::builder().filter_level(LevelFilter::Info).init();

    info!("kvs-server {}", env!("CARGO_PKG_VERSION"));
    info!("Storage engine: {}", cli.engine);
    info!("Listening on {}", cli.addr);

    let mut store: Box<dyn KvsEngine> = match cli.engine {
        Engine::Kvs => {
            let s = kvs::KvStore::open(std::path::Path::new("./")).expect("failed to open");
            Box::new(s)
        }
        Engine::Sled => {
            let s = sled::open("my_db").expect("failed to open sled");
            Box::new(s)
        }
    };

    let listener = TcpListener::bind(cli.addr)?;
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(&mut store, stream)?;
            }
            Err(e) => {
                eprintln!("{:?}", e);
            }
        }
    }

    Ok(())
}

fn handle_connection(store: &mut Box<dyn KvsEngine>, stream: TcpStream) -> Result<()> {
    let mut reader = std::io::BufReader::new(stream.try_clone().unwrap());
    let mut writer = std::io::BufWriter::new(stream);

    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    trace!("received: {}", line.trim());

    let command = serde_json::from_str(&line)?;
    trace!("parsed: {:?}", command);

    match command {
        KvCommand::Get(key) => match store.get(key) {
            Ok(Some(value)) => {
                writeln!(writer, "{}", value)?;
                Ok(())
            }
            Ok(None) => {
                writeln!(writer, "Key not found")?;
                Ok(())
            }
            Err(e) => {
                writeln!(writer, "error: {:?}", e)?;
                Ok(())
            }
        },
        KvCommand::Set(key, value) => store.set(key, value),
        KvCommand::Remove(key) => match store.remove(key) {
            Ok(_) => Ok(()),
            _ => {
                writeln!(writer, "error: Key not found")?;
                Ok(())
            }
        },
    }
}
