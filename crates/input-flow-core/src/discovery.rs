//! Device discovery via mDNS
//!
//! Handles automatic discovery of Input Flow devices on the local network

/// Service type for mDNS
pub const SERVICE_TYPE: &str = "_kvm._udp.local.";

/// Default port
pub const DEFAULT_PORT: u16 = 24842;

// TODO: Implement mDNS service announcement
// TODO: Implement mDNS service browsing
// TODO: Device list management
