//! Input Flow Daemon
//!
//! Main daemon process that handles connections and input routing

use anyhow::Result;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<()> {
    // Setup logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    info!("Input Flow Daemon starting...");

    // TODO: Initialize backend (X11/Wayland detection)
    // TODO: Start mDNS discovery
    // TODO: Start QUIC server
    // TODO: Handle connections

    info!("Daemon initialized successfully");

    // Keep running
    tokio::signal::ctrl_c().await?;
    info!("Shutting down...");

    Ok(())
}
