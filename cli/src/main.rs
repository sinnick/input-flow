//! Input Flow CLI
//!
//! Command-line interface for Input Flow

use clap::{Parser, Subcommand};
use anyhow::Result;

#[derive(Parser)]
#[command(name = "input-flow")]
#[command(about = "Cross-platform input sharing", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the Input Flow daemon
    Start {
        /// Start minimized
        #[arg(long)]
        minimized: bool,
    },
    /// Pair with a device
    Pair {
        /// Scan for devices
        #[arg(long)]
        scan: bool,
        /// Connect to specific IP
        #[arg(long)]
        ip: Option<String>,
    },
    /// Show connection status
    Status,
    /// List discovered devices
    Devices,
    /// Test connection to a device
    Test {
        /// Device name to test
        #[arg(long)]
        device: String,
    },
    /// Show debug information
    Debug {
        /// Follow logs in real-time
        #[arg(long)]
        follow: bool,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Start { minimized } => {
            println!("Starting Input Flow daemon{}", if minimized { " (minimized)" } else { "" });
            // TODO: Start daemon
        }
        Commands::Pair { scan, ip } => {
            if scan {
                println!("Scanning for devices...");
                // TODO: Scan for devices
            } else if let Some(ip) = ip {
                println!("Pairing with {}...", ip);
                // TODO: Manual pairing
            } else {
                println!("Please specify either --scan or --ip");
            }
        }
        Commands::Status => {
            println!("Status: Not implemented yet");
            // TODO: Show status
        }
        Commands::Devices => {
            println!("Devices: Not implemented yet");
            // TODO: List devices
        }
        Commands::Test { device } => {
            println!("Testing connection to {}...", device);
            // TODO: Test connection
        }
        Commands::Debug { follow } => {
            println!("Debug mode{}", if follow { " (following)" } else { "" });
            // TODO: Show debug info
        }
    }

    Ok(())
}
