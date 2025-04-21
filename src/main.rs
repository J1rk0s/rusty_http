use clap::Parser;
use server::HttpServer;

mod server;
mod request;

#[macro_use]
mod macros;

/// Simple http server made in Rust
/// Made by: J1rk0s
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the website root directory
    #[arg(short, long, default_value = ".")]
    root: String,

    /// Port to listen on
    #[arg(short, long, default_value_t = 80)]
    port: u16,

    #[arg(short, long)]
    ip: String
}

fn main() {
    let args = Args::parse();
    let server = HttpServer::new(args.ip, args.port, args.root);
    server.listen();
}
