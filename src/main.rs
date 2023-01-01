use clap::Parser;
use log::info;
use std::str::FromStr;
use stderrlog::Timestamp;

/// Simple program to reboot a server if it is unreachable over the network
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct ServermonArgs {
    /// Hostname of server to monitor
    #[arg()]
    server: String,

    /// Hostname of Tasmota smart plug to monitor
    #[arg()]
    smartplug: String,

    /// Verbosity of output
    #[arg(short, action = clap::ArgAction::Count)]
    verbosity: usize,
}

fn main() {
    let args = ServermonArgs::parse();
    stderrlog::new()
        .module(module_path!())
        .verbosity(args.verbosity)
        .timestamp(Timestamp::from_str("sec").unwrap())
        .init()
        .unwrap();

    info!(
        "monitoring server {} and smart plug {}",
        args.server, args.smartplug
    );
}
