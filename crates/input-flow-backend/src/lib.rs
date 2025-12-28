//! Input Flow - Backend abstraction
//!
//! This crate defines the platform-independent interface for input capture
//! and injection. Platform-specific implementations (X11, Wayland, macOS)
//! implement the `InputBackend` trait.

pub mod types;

use std::io::Result;
use types::*;

/// Platform-agnostic input backend trait
///
/// Each platform (X11, Wayland, macOS) implements this trait to provide
/// input capture and injection capabilities.
pub trait InputBackend {
    /// Setup edge detection for cursor flow
    fn setup_edge_detection(&mut self, edges: EdgeConfig) -> Result<()>;

    /// Get current cursor position
    fn get_cursor_position(&self) -> Result<(i32, i32)>;

    /// Start capturing input events
    fn start_capture(&mut self) -> Result<()>;

    /// Stop capturing input events
    fn stop_capture(&mut self) -> Result<()>;

    /// Inject mouse move event
    fn inject_mouse_move(&mut self, x: i32, y: i32) -> Result<()>;

    /// Inject mouse button event
    fn inject_mouse_button(&mut self, button: MouseButton, pressed: bool) -> Result<()>;

    /// Inject keyboard event
    fn inject_key(&mut self, key: KeyCode, pressed: bool, modifiers: Modifiers) -> Result<()>;

    /// Get screen information (resolution, layout)
    fn get_screen_info(&self) -> Result<Vec<ScreenInfo>>;

    /// Check if this backend supports absolute positioning
    fn supports_absolute_positioning(&self) -> bool {
        true
    }
}

/// Edge configuration for cursor flow
#[derive(Debug, Clone)]
pub struct EdgeConfig {
    pub enabled_edges: Vec<Edge>,
    pub dwell_time_ms: u64,
}

impl Default for EdgeConfig {
    fn default() -> Self {
        Self {
            enabled_edges: vec![Edge::Left, Edge::Right, Edge::Top, Edge::Bottom],
            dwell_time_ms: 150,
        }
    }
}
