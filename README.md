# 🎮 MHFZ-Launcher

**Cross-platform launcher for Monster Hunter Frontier Z**  
✅ **Windows Native** • ✅ **Linux (Wine + mhf-iel)** • 🎬 **Demo Available**

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)  
[![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20Linux-success)](#)  
[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)  
[![Tauri](https://img.shields.io/badge/Tauri-1.6-24C8DB.svg)](https://tauri.app/)  
[![Status](https://img.shields.io/badge/Status-Production%20Ready-brightgreen)](#)

---

## 🎉 Major Milestone: Linux Support Now Working!

**Date**: December 12, 2025  
**Status**: ✅ **FULLY FUNCTIONAL** on Linux via Wine + mhf-iel integration

The launcher successfully launches Monster Hunter Frontier Z on Linux using:
- `mhf-iel-cli.exe` for game injection
- Wine 8.0+ for Windows compatibility
- DXVK for Vulkan-based rendering
- Automatic config.json generation (25+ fields)

**Tested Configuration**:
- ✅ Arch Linux + Wine 9.21 + DXVK 2.4
- ✅ Avalanche server (avalanchemhfz.ddns.net:53310)
- ✅ Character selection, login, full gameplay
- ✅ HR 999, GR 110 character tested

---

## 🎬 Video Demo

> **Full launch sequence** available in `docs/videos/`  
> (Shows launcher startup → character selection → game launch on Linux)

---

## 📋 Overview

MHFZ-Launcher is a modern, cross-platform game launcher for **Monster Hunter Frontier Z**, designed to work with private servers (primarily [Erupe](https://github.com/mrsasy89/Erupe) and [Avalanche](http://avalanchemhfz.ddns.net)). Built with Rust (Tauri backend) + Vue.js frontend.

### 🌟 Key Features

- ✅ **Cross-platform**: Windows native + Linux (Wine/mhf-iel)
- 🎨 **Vanilla UI**: CAPCOM-style interface (no custom branding)
- 🔧 **Modular**: Easy server switching and configuration
- 🚀 **Lightweight**: ~10MB binary size
- 🔐 **Secure**: Token-based authentication (16-char tokens)
- 📦 **Auto-patcher**: Server-side patch management
- 🐧 **Linux Ready**: mhf-iel integration for seamless game launch

---

## 🛠️ Current Development Status

### ✅ Completed (Phase 1-2)

- [x] Backend refactoring (cross-platform)
- [x] INI parsing (`configparser` instead of Win32 API)
- [x] Wine/Proton integration architecture
- [x] Branding cleanup (vanilla CAPCOM style)
- [x] Server configuration system
- [x] Character selection UI
- [x] **Linux game launch** (mhf-iel integration) ⭐ **NEW**
- [x] **config.json generation** (25+ fields) ⭐ **NEW**
- [x] **Wine prefix auto-detection** ⭐ **NEW**
- [x] **Thread-safe config storage** (OnceLock) ⭐ **NEW**

### 🚧 In Progress (Phase 3)

- [ ] Friends list UI display (data received, mapping pending)
- [ ] MezFes stall details rendering
- [ ] Advanced patching system
- [ ] Multi-server switching

### 📅 Roadmap (Phase 4)

- [ ] Auto-update system
- [ ] Multi-language support (EN/JP/IT)
- [ ] AppImage/Flatpak packaging (Linux)
- [ ] Steam Deck optimization
- [ ] In-launcher news feed

---

## 🚀 Quick Start

### Prerequisites

#### All Platforms
- [Rust](https://rustup.rs/) (1.70+ recommended, 1.91+ for OnceLock)
- [Node.js](https://nodejs.org/) (16+)
- [npm](https://www.npmjs.com/) (8+)

#### Linux Additional
- **Wine 8.0+** (Wine 9.x recommended)
- **DXVK** (optional but highly recommended for performance)
- **mhf-iel-cli.exe** ([build instructions](https://github.com/rockisch/mhf-iel))
- WebKitGTK development libraries

```bash
# Arch Linux / Manjaro
sudo pacman -S wine-staging dxvk webkit2gtk base-devel

# Ubuntu / Debian
sudo apt install wine-staging dxvk libwebkit2gtk-4.0-dev build-essential

# Fedora
sudo dnf install wine-core dxvk webkit2gtk4.0-devel gcc
```

### Build Steps

```bash
# 1. Clone the repository
git clone https://github.com/mrsasy89/MHFZ-Launcher.git
cd MHFZ-Launcher

# 2. Install dependencies
npm install

# 3. Development mode (hot-reload enabled)
npm run tauri:dev

# 4. Production build
npm run tauri:build
```

**Output location**: 
- Linux: `src-tauri/target/release/app`
- Windows: `src-tauri/target/release/app.exe`

---

## 🐧 Linux Setup Guide

### Step 1: Prepare mhf-iel

```bash
# Clone and build mhf-iel (i686 Windows binary)
git clone https://github.com/rockisch/mhf-iel.git
cd mhf-iel

# Build for 32-bit Windows (Wine compatible)
cargo build --release --target=i686-pc-windows-gnu

# Copy to MHFZ game folder
cp target/i686-pc-windows-gnu/release/mhf-iel-cli.exe ~/Games/MHFZ/
```

### Step 2: Configure Wine Prefix

```bash
# Create isolated prefix
mkdir -p ~/Games/MHFZ/pfx
export WINEPREFIX=~/Games/MHFZ/pfx

# Initialize Wine (32-bit)
WINEARCH=win32 wineboot

# Install dependencies (optional but recommended)
winetricks dxvk  # For Vulkan rendering
```

### Step 3: Setup Game Files

```
~/Games/MHFZ/
├── mhf-iel-cli.exe      # Launcher bypass (from Step 1)
├── mhfo-hd.dll          # HD client (ZZ version)
├── mhf.exe              # Game executable
├── mhf.ini              # Game configuration
├── dat/                 # Game data (auto-downloaded)
├── pfx/                 # Wine prefix (auto-created)
└── config.json          # Auto-generated by launcher
```

### Step 4: Launch!

```bash
cd MHFZ-Launcher
npm run tauri:dev

# OR use production build
./src-tauri/target/release/app
```

The launcher will:
1. Connect to configured server
2. Authenticate user
3. Show character selection
4. Generate `config.json` with auth tokens
5. Launch `mhf-iel-cli.exe` via Wine
6. Inject configuration into game process
7. Start MHFZ!

---

## ⚙️ Configuration

### Server Setup

The launcher auto-connects to Avalanche by default. To add custom servers:

**Option 1**: In-launcher Settings panel  
**Option 2**: Edit `src-tauri/tauri.conf.json`:

```json
{
  "tauri": {
    "bundle": {
      "resources": ["config/servers.json"]
    }
  }
}
```

Then create `config/servers.json`:

```json
{
  "servers": [
    {
      "name": "Avalanche (Italy)",
      "host": "avalanchemhfz.ddns.net",
      "game_port": 53310,
      "patch_port": 8094,
      "launcher_port": 9010,
      "version": "ZZ"
    },
    {
      "name": "Custom Server",
      "host": "your.server.com",
      "game_port": 54001,
      "patch_port": 8094,
      "launcher_port": 8080,
      "version": "ZZ"
    }
  ]
}
```

### Advanced: Wine Configuration

For optimal performance on Linux:

```bash
# Enable DXVK HUD (shows FPS)
export DXVK_HUD=fps

# Force Vulkan backend
export DXVK_ASYNC=1

# Disable debug output (improves performance)
export WINEDEBUG=-all

# Launch launcher
npm run tauri:dev
```

---

## 🔧 Technical Architecture

### Backend (Rust/Tauri)

```
src-tauri/src/
├── main.rs              # Entry point + state management
│                         # Linux launch logic (lines 1130-1200)
├── lib_linux.rs         # ⭐ NEW: mhf-iel integration
│                         # - MhfIelConfig struct (20+ fields)
│                         # - config.json generation
│                         # - Wine prefix detection
│                         # - OnceLock thread-safe storage
├── config.rs            # Server endpoints configuration
├── settings.rs          # INI parser (cross-platform)
├── endpoint.rs          # Server connection logic
├── patcher.rs           # Update system
└── server.rs            # HTTP client for auth/API
```

### Key Implementation: lib_linux.rs

```rust
pub struct MhfIelConfig {
    pub char_id: u32,
    pub char_name: String,
    pub char_hr: u32,
    pub char_gr: u32,
    pub user_token: String,      // 16-char auth token
    pub server_host: String,
    pub server_port: u32,
    pub current_ts: u64,
    pub expiry_ts: u64,
    pub mez_event_id: u32,       // MezFes data
    pub mez_start: u64,
    pub mez_end: u64,
    // ... +15 more fields
}

// Thread-safe global storage (Rust 1.70+)
static MHF_IEL_CONFIG_GLOBAL: OnceLock<MhfIelConfig> = OnceLock::new();

pub fn run_linux(config: MhfConfigLinux) -> std::io::Result<()> {
    // 1. Generate config.json
    generate_mhf_iel_config(&config.game_folder, iel_config)?;
    
    // 2. Detect Wine prefix
    let wine_prefix = config.game_folder.join("pfx");
    
    // 3. Launch mhf-iel-cli.exe
    Command::new("wine")
        .arg("mhf-iel-cli.exe")
        .env("WINEPREFIX", wine_prefix)
        .env("DXVK_HUD", "fps")
        .spawn()?
        .wait()?;
}
```

### Frontend (Vue.js)

```
src/
├── Classic.vue          # Classic UI (default)
├── Modern.vue           # Modern UI (alternative)
├── Settings.vue         # Configuration panel
├── CharacterSelect.vue  # Character picker
└── store.js             # Vuex state management
```

---

## 🐛 Known Issues

| Issue | Status | Workaround |
|-------|--------|------------|
| Friends list display | ⚠️ Data received, UI pending | None needed (functional) |
| MezFes stall names | ⚠️ Array format instead of struct | Use numeric IDs |
| Wine prefix detection | ✅ Fixed | Auto-creates in `<game>/pfx` |
| DXVK initialization | ⚠️ First launch slow | Wait 10-15s on first run |

---

## 🤝 Contributing

Contributions are welcome! Priority areas:

1. **Friends list UI**: Complete frontend rendering of friends data
2. **MezFes display**: Parse stall array into readable format
3. **Testing**: More Linux distros (Ubuntu, Fedora, Debian, etc.)
4. **Localization**: Japanese/Italian translations
5. **Documentation**: Wiki pages, troubleshooting guides

### Development Workflow

```bash
# Create feature branch
git checkout -b feature/friends-ui

# Make changes and test
npm run tauri:dev

# Commit with conventional commits
git commit -m "feat(ui): implement friends list display"

# Push and create PR
git push origin feature/friends-ui
```

### Code Style

- **Rust**: `cargo fmt` + `cargo clippy`
- **JavaScript**: ESLint + Prettier
- **Commits**: Conventional Commits format

---

## 📚 Related Projects

- **[Erupe Server](https://github.com/mrsasy89/Erupe)** - Private server implementation (Go)
- **[mhf-iel](https://github.com/rockisch/mhf-iel)** - Game launcher bypass (Rust)
- **[ButterClient](https://github.com/mrsasy89/ButterClient)** - Original Windows-only launcher (C#)
- **[MHF Patch Server](https://github.com/mrsasy89/MHF-Patch-Server)** - Update distribution (Node.js)
- **[Avalanche Server](http://avalanchemhfz.ddns.net)** - Italian public server

---

## 📜 License

**GNU General Public License v3.0**

This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

See [LICENSE](LICENSE) for full details.

---

## 🙏 Credits

- **Original ButterClient**: [LilButter](https://github.com/LilButter)
- **Linux Port & mhf-iel Integration**: [mrsasy89](https://github.com/mrsasy89)
- **mhf-iel**: [rockisch](https://github.com/rockisch)
- **Erupe Server**: Community-maintained fork
- **Avalanche Server**: Italian MHFZ community
- **CAPCOM**: Original game assets and Monster Hunter IP

---

## 💬 Support & Community

- **Issues**: [GitHub Issues](https://github.com/mrsasy89/MHFZ-Launcher/issues)
- **Discussions**: [GitHub Discussions](https://github.com/mrsasy89/MHFZ-Launcher/discussions)
- **Discord**: [Avalanche Server Discord](https://discord.gg/hfJESUsbnz)
- **Wiki**: [Documentation](https://github.com/mrsasy89/MHFZ-Launcher/wiki) (coming soon)

---

## 🎯 Roadmap Summary

| Milestone | Status | ETA |
|-----------|--------|-----|
| ✅ Windows Support | Complete | Released |
| ✅ Linux Support (mhf-iel) | **Complete** | **Dec 2025** |
| ⚙️ Friends List UI | In Progress | Jan 2026 |
| 📦 AppImage Release | Planned | Q1 2026 |
| 🌍 Multi-language | Planned | Q2 2026 |
| 🎮 Steam Deck Support | Planned | Q2 2026 |

---

## 📊 Statistics

- **Lines of Code**: ~8,500 (Rust + JS)
- **Dependencies**: 47 crates, 23 npm packages
- **Bundle Size**: ~10MB (release build)
- **Supported Platforms**: Windows 10/11, Linux (Wine 8.0+)
- **Active Servers**: 2+ (Avalanche, community servers)

---

**⚠️ Disclaimer**: This project is for educational purposes and preservation of a discontinued game. All Monster Hunter intellectual property belongs to CAPCOM. This launcher is not affiliated with or endorsed by CAPCOM.

**🎮 Happy Hunting!** Whether on Windows or Linux, may your hunts be successful and your frames be high!

---

*Last updated: December 12, 2025*  
*Launcher Version: 1.0.0-beta (Linux milestone)*  
*Tested on: Arch Linux, Wine 9.21, DXVK 2.4, Avalanche Server*
