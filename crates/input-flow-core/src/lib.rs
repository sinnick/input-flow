//! Input Flow - Core library
//!
//! This crate contains the core functionality for Input Flow, including:
//! - Network protocol (QUIC)
//! - Device discovery (mDNS)
//! - Pairing and authentication
//! - Configuration management

pub mod protocol;
pub mod discovery;
pub mod pairing;
pub mod config;
pub mod crypto;

pub use protocol::*;
pub use discovery::*;
pub use pairing::*;
pub use config::*;
