# 🎮 MHFZ-Launcher

**Cross-platform launcher for Monster Hunter Frontier Z**  
Supports Windows natively and Linux via Wine/Proton integration.

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)  
[![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20Linux-lightgrey)](#)  
[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)  
[![Tauri](https://img.shields.io/badge/Tauri-1.5-24C8DB.svg)](https://tauri.app/)

---

## 📋 Overview

MHFZ-Launcher is a modern, cross-platform game launcher for **Monster Hunter Frontier Z**, designed to work with private servers (primarily [Erupe](https://github.com/mrsasy89/Erupe)). Built with Rust (Tauri backend) + Vue.js frontend.

### 🌟 Key Features

- ✅ **Cross-platform**: Windows native + Linux (Wine/Proton)
- 🎨 **Vanilla UI**: CAPCOM-style interface (no custom branding)
- 🔧 **Modular**: Easy server switching and configuration
- 🚀 **Lightweight**: ~10MB binary size
- 🔐 **Secure**: Token-based authentication
- 📦 **Auto-patcher**: Server-side patch management

---

## 🛠️ Current Development Status

### ✅ Completed (Phase 1)

- [x] Backend refactoring (removed Windows-only dependencies)
- [x] Cross-platform INI parsing (`configparser` instead of Win32 API)
- [x] Wine/Proton integration architecture
- [x] Branding cleanup (vanilla CAPCOM style)
- [x] Server configuration system
- [x] Character selection UI

### 🚧 In Progress (Phase 2)

- [ ] **Game launch mechanism** (Wine wrapper implementation)
- [ ] Game folder validation
- [ ] Offline patcher system
- [ ] Friends list injection (Linux-compatible method)

### 📅 Roadmap (Phase 3)

- [ ] Auto-update system
- [ ] Multi-language support (EN/JP/IT)
- [ ] AppImage/Flatpak packaging (Linux)
- [ ] Steam Deck optimization

---

## 🚀 Build Instructions

### Prerequisites

#### All Platforms
- [Rust](https://rustup.rs/) (1.70+, **nightly** toolchain required)
- [Node.js](https://nodejs.org/) (16+)
- [npm](https://www.npmjs.com/) (8+)

#### Linux Additional
- Wine or Proton (GE-Proton recommended)
- WebKitGTK development libraries

```bash
# Arch Linux / Manjaro
sudo pacman -S webkit2gtk base-devel

# Ubuntu / Debian
sudo apt install libwebkit2gtk-4.0-dev build-essential
```

### Build Steps

```bash
# 1. Clone the repository
git clone https://github.com/mrsasy89/MHFZ-Launcher.git
cd MHFZ-Launcher

# 2. Set Rust nightly toolchain
rustup override set nightly

# 3. Install dependencies
npm install

# 4. Development mode
npm run tauri:dev

# 5. Production build
npm run tauri:build
```

**Output location**: `src-tauri/target/release/` (or `i686-pc-windows-msvc` for Windows)

---

## 🐧 Linux Setup (Wine/Proton)

### Wine Prefix Configuration

```bash
# 1. Create isolated prefix
mkdir -p ~/Games/MHFZ/pfx
export WINEPREFIX=~/Games/MHFZ/pfx

# 2. Install dependencies
winetricks dotnet48 vcrun2019 d3dx9 d3dcompiler_47 dinput xinput
winetricks corefonts allfonts  # Fixes text rendering

# 3. Verify setup
winecfg  # Should open without errors
```

### Using GE-Proton (Recommended)

If you have Steam installed:

```bash
# Point to Proton runtime
export WINEPREFIX=~/.local/share/Steam/steamapps/compatdata/MHFZ/pfx
export PROTON_PATH=~/.local/share/Steam/compatdata/GE-Proton10-25
```

---

## ⚙️ Configuration

### Server Setup

Edit `ButterClient/config.json` (or use in-launcher settings):

```json
{
  "current_endpoint": {
    "name": "Erupe Server",
    "host": "avalanchemhfz.ddns.net",
    "game_port": 53310,
    "patch_port": 8094,
    "version": "ZZ"
  },
  "game_folder": "/path/to/MHFZ"
}
```

### Game Files

Place MHFZ game files in your chosen directory:

```
MHFZ/
├── mhf.exe          # Main executable (F5) or
├── mhfo.dll         # SD client (ZZ) or
├── mhfo-hd.dll      # HD client (ZZ)
├── mhf.ini          # Game configuration
├── dat/             # Game data
└── ...
```

---

## 🔧 Technical Architecture

### Backend (Rust/Tauri)

```
src-tauri/
├── src/
│   ├── main.rs              # Tauri entry point + state management
│   ├── config.rs            # Server endpoints configuration
│   ├── settings.rs          # INI parser (cross-platform)
│   ├── endpoint.rs          # Server connection logic
│   ├── patcher.rs           # Update system
│   └── server.rs            # HTTP client for auth/API
├── mhf-iel-master/          # Game launcher module
│   └── src/
│       ├── lib.rs           # Entry point (Wine wrapper)
│       └── mhf.rs           # Game initialization (Windows-native)
└── Cargo.toml
```

### Frontend (Vue.js)

```
src/
├── Classic.vue              # Classic UI (default)
├── Modern.vue               # Modern UI (alternative)
├── Settings.vue             # Configuration panel
└── store.js                 # Vuex state management
```

---

## 🤝 Contributing

Contributions are welcome! Areas needing help:

1. **Linux game launch**: Complete Wine wrapper in `mhf-iel-master/src/lib.rs`
2. **Friends list**: Cross-platform injection method
3. **Testing**: Windows 10/11 + various Linux distros
4. **Localization**: Japanese/Italian translations

### Development Workflow

```bash
# Create feature branch
git checkout -b feature/wine-launcher

# Make changes and test
npm run tauri:dev

# Commit with conventional commits
git commit -m "feat(linux): implement Wine game launcher"

# Push and create PR
git push origin feature/wine-launcher
```

---

## 📚 Related Projects

- [Erupe Server](https://github.com/mrsasy89/Erupe) - Private server implementation
- [ButterClient](https://github.com/mrsasy89/ButterClient) - Original Windows-only launcher
- [MHF Patch Server](https://github.com/mrsasy89/MHF-Patch-Server) - Update distribution system

---

## 📜 License

GNU General Public License v3.0 - See [LICENSE](LICENSE) for details.

---

## 🙏 Credits

- **Original ButterClient**: [LilButter](https://github.com/LilButter)
- **Linux Port**: [mrsasy89](https://github.com/mrsasy89)
- **Erupe Server**: Community-maintained
- **CAPCOM**: Original game assets

---

## 💬 Support

- **Issues**: [GitHub Issues](https://github.com/mrsasy89/MHFZ-Launcher/issues)
- **Discussions**: [GitHub Discussions](https://github.com/mrsasy89/MHFZ-Launcher/discussions)
- **Wiki**: Coming soon

---

**⚠️ Disclaimer**: This project is for educational purposes and preservation of a discontinued game. CAPCOM owns all rights to Monster Hunter Frontier Z.
