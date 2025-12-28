//! Network protocol implementation
//!
//! Defines the message format and QUIC protocol for Input Flow

use serde::{Deserialize, Serialize};

/// Protocol magic number
pub const PROTOCOL_MAGIC: u8 = 0x42; // 'B' for Barrier-compatible

/// Current protocol version
pub const PROTOCOL_VERSION: u8 = 0x01;

/// Message types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum MessageType {
    // Handshake
    HandshakeInit = 0xF0,
    HandshakeResponse = 0xF1,
    HandshakeComplete = 0xF2,

    // Input events
    MouseMove = 0x01,
    MouseButton = 0x02,
    MouseWheel = 0x03,
    KeyDown = 0x10,
    KeyUp = 0x11,

    // Control
    Heartbeat = 0x40,
    Ack = 0x41,
}

/// Protocol message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Message {
    HandshakeInit {
        version: u8,
        device_id: String,
        hostname: String,
    },
    HandshakeResponse {
        accepted: bool,
        reason: Option<String>,
    },
    MouseMove {
        x: i32,
        y: i32,
        timestamp: u64,
    },
    KeyDown {
        keycode: u32,
        modifiers: u8,
    },
    KeyUp {
        keycode: u32,
        modifiers: u8,
    },
    Heartbeat,
}

// TODO: Implement message encoding/decoding
// TODO: Implement QUIC connection management
