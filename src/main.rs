use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    mode: String,

    #[arg(short, long)]
    ip: String,

    #[arg(short, long, default_value_t = 7070)]
    port: u16,
}

fn main() {
    let args = Args::parse();
    println!("You said {} {} {}", args.mode, args.ip, args.port);
}
