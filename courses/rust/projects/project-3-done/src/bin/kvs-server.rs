use clap::{Parser, ValueEnum};
use log::{info, trace};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(next_line_help = true)]
struct Cli {
    #[arg(long)]
    addr: std::net::IpAddr,
    #[arg(long, value_enum, default_value_t = Engine::Kvs)]
    engine: Engine,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Engine {
    Kvs,
    Sled,
}

fn main() {
    let cli = Cli::parse();
    env_logger::init();

    info!(
        "Starting server with engine '{:?}' on address '{:?}'",
        cli.engine, cli.addr
    );

    trace!("Commencing yak shaving");
}
