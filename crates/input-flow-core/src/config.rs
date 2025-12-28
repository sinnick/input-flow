//! Configuration management
//!
//! Handles loading and saving of TOML configuration

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Main configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub device: DeviceConfig,
    pub network: NetworkConfig,
    pub behavior: BehaviorConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceConfig {
    pub name: String,
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub port: u16,
    pub auto_discover: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorConfig {
    pub clipboard_sharing: bool,
    pub edge_delay_ms: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            device: DeviceConfig {
                name: hostname::get()
                    .ok()
                    .and_then(|h| h.into_string().ok())
                    .unwrap_or_else(|| "unknown".to_string()),
                id: uuid::Uuid::new_v4().to_string(),
            },
            network: NetworkConfig {
                port: 24842,
                auto_discover: true,
            },
            behavior: BehaviorConfig {
                clipboard_sharing: true,
                edge_delay_ms: 150,
            },
        }
    }
}

/// Get config directory path
pub fn config_dir() -> PathBuf {
    dirs::config_dir()
        .expect("Failed to get config directory")
        .join("input-flow")
}

// TODO: Implement load/save functions
