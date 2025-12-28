# Input Flow

> Seamless cross-platform input sharing with Wayland support

Input Flow is a modern solution for sharing mouse and keyboard across multiple computers, designed from the ground up to work with both X11 and Wayland display servers.

## Features

- ✅ **Seamless Cursor Flow**: Move your cursor between screens like they're one display
- ✅ **Wayland Native**: First-class support for modern Linux desktops (GNOME, KDE, Sway)
- ✅ **X11 Compatible**: Works on traditional X11 setups
- ✅ **Zero-Config Discovery**: Automatic device discovery via mDNS
- ✅ **Visual Configuration**: Drag-and-drop monitor layout editor
- ✅ **Secure**: TLS 1.3 encryption with Trust-On-First-Use pairing
- ✅ **Workspace Independent**: Each PC can switch workspaces without losing control

## Quick Start

### Installation

```bash
# From source
cargo build --release

# The daemon
./target/release/input-flow-daemon

# Or use the CLI
./target/release/input-flow start
```

### Usage

1. **Start on both machines**:
   ```bash
   input-flow start
   ```

2. **Pair devices** (automatic discovery):
   ```bash
   input-flow pair --scan
   ```

3. **Move your cursor** to the edge of the screen - it will flow to the other machine!

## Project Status

**Current Phase**: MVP - X11 Support (In Development)

- [x] Project structure
- [ ] Core protocol (QUIC)
- [ ] Device discovery (mDNS)
- [ ] X11 backend
- [ ] CLI interface
- [ ] GUI (Tauri + Svelte)

See [TASKS.md](../TASKS.md) for detailed progress tracking.

## Architecture

```
┌─────────────────────────────────────────┐
│     Application Layer (Tauri GUI)       │
├─────────────────────────────────────────┤
│    Protocol Layer (QUIC/Discovery)      │
├─────────────────────────────────────────┤
│    Platform Abstraction (trait)         │
│  ┌──────────┬──────────┬──────────┐    │
│  │ Wayland  │   X11    │  macOS   │    │
│  │ Backend  │ Backend  │ Backend  │    │
│  └──────────┴──────────┴──────────┘    │
└─────────────────────────────────────────┘
```

## Roadmap

### Phase 1: MVP (6-8 weeks) - Current
- X11 support
- Basic cursor flow
- Device discovery & pairing
- CLI interface

### Phase 2: Production (4-6 weeks)
- Wayland support (libei/libeis)
- Clipboard sharing
- System tray
- Auto-start

### Phase 3: Advanced (6-8 weeks)
- macOS support
- File transfer
- Multi-device (3+)
- Advanced features

## How It's Different

Compared to Barrier/Input Leap:
- **Works with Wayland**: Native support via libei/libeis
- **Better UX**: Auto-discovery, visual layout editor, clear pairing
- **Modern Protocol**: QUIC with TLS 1.3, optimized for low latency
- **Workspace Freedom**: Switch workspaces independently

Compared to KDE Connect:
- **Seamless Flow**: Automatic cursor transition at screen edges
- **Designed for KVM**: Built specifically for keyboard/mouse sharing

## Technology Stack

- **Language**: Rust (safety, performance, async)
- **Protocol**: QUIC (low latency, encrypted)
- **Discovery**: mDNS/Bonjour (zero-config)
- **UI**: Tauri + Svelte (lightweight, native)
- **Backends**:
  - X11: x11rb (XTest + XRecord)
  - Wayland: libei/libeis + portals
  - macOS (future): Core Graphics

## Contributing

This project is in early development. Contributions welcome!

## License

MIT OR Apache-2.0

---

**Status**: 🚧 In Development - MVP Phase
**Target**: Q1 2026 for stable release
# input-flow
