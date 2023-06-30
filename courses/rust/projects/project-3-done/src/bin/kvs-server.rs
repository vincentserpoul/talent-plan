use clap::{Parser, ValueEnum};
use log::LevelFilter;
use log::{info, trace};
use std::fmt::Display;
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

fn main() {
    let cli = Cli::parse();
    env_logger::builder().filter_level(LevelFilter::Info).init();

    info!("kvs-server {}", env!("CARGO_PKG_VERSION"));
    info!("Storage engine: {}", cli.engine);
    info!("Listening on {}", cli.addr);

    let listener = TcpListener::bind(cli.addr).unwrap();
    loop {
        match listener.accept() {
            Ok((_, addr)) => {
                info!("New connection from {:?}", addr);
            }
            Err(e) => {
                eprintln!("Error accepting connection: {:?}", e);
            }
        }
    }
}
