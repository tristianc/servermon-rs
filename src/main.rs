use clap::Parser;
use log::info;
use std::net::Ipv4Addr;
use std::str::FromStr;
use std::{thread, time};
use stderrlog::Timestamp;

/// Interval in which servermon will ping the server
const PERIOD: u64 = 6;

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
    verbosity: u8,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = ServermonArgs::parse();
    stderrlog::new()
        .module(module_path!())
        .verbosity(usize::from(args.verbosity))
        .timestamp(Timestamp::from_str("sec").unwrap())
        .init()
        .unwrap();

    info!(
        "monitoring server {} and smart plug {}",
        args.server, args.smartplug
    );

    let period = time::Duration::from_secs(PERIOD);
    loop {
        thread::sleep(period);

        let payload = [0; 8];
        let (_packet, ping_duration) = surge_ping::ping("192.0.0.1".parse()?, &payload).await?;
        println!("Ping took {:.3?}", ping_duration);

        info!("server {} is available", args.server);
    }

    Ok(())
}
