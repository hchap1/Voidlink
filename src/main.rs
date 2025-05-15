mod send;
mod receive;
use clap::{Parser, Subcommand};

#[derive(Clone, Copy, Debug, Subcommand)]
enum Mode {
    Send,
    Receive
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    mode: Mode,

    #[arg(short, long)]
    ip: String,

    #[arg(short, long, default_value_t = 7070)]
    port: u16,
}

fn main() {
    let args = Args::parse();
}
