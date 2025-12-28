//! Common types used across all backends

use serde::{Deserialize, Serialize};

/// Screen edge
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Edge {
    Left,
    Right,
    Top,
    Bottom,
}

impl Edge {
    pub fn is_vertical(&self) -> bool {
        matches!(self, Edge::Left | Edge::Right)
    }

    pub fn is_horizontal(&self) -> bool {
        matches!(self, Edge::Top | Edge::Bottom)
    }
}

/// Mouse button
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MouseButton {
    Left = 1,
    Middle = 2,
    Right = 3,
    ScrollUp = 4,
    ScrollDown = 5,
}

/// Keyboard key code (platform-agnostic)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyCode(pub u32);

/// Keyboard modifiers
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct Modifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub meta: bool, // Windows key / Command key
}

impl Modifiers {
    pub fn to_u8(&self) -> u8 {
        let mut value = 0u8;
        if self.shift { value |= 1; }
        if self.ctrl { value |= 2; }
        if self.alt { value |= 4; }
        if self.meta { value |= 8; }
        value
    }

    pub fn from_u8(value: u8) -> Self {
        Self {
            shift: (value & 1) != 0,
            ctrl: (value & 2) != 0,
            alt: (value & 4) != 0,
            meta: (value & 8) != 0,
        }
    }
}

/// Screen information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenInfo {
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub is_primary: bool,
}
