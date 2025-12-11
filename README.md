# 🎮 MHFZ-Launcher

**Cross-platform launcher for Monster Hunter Frontier Z**  
Supports Windows natively and Linux via Wine/Proton integration.

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)  
[![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20Linux-lightgrey)](#)  
[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)  
[![Tauri](https://img.shields.io/badge/Tauri-1.5-24C8DB.svg)](https://tauri.app/)

---

## 📋 Overview

MHFZ-Launcher is a modern, cross-platform game launcher for **Monster Hunter Frontier Z**, designed to work with private servers (primarily [Erupe](https://github.com/ErupeServer/Erupe)). Built with Rust (Tauri backend) + Vue.js frontend.

### 🌟 Key Features

- ✅ **Cross-platform**: Windows native + Linux (Wine/Proton)
- 🎨 **Vanilla UI**: CAPCOM-style interface (no custom branding)
- 🔧 **Modular**: Easy server switching and configuration
- 🚀 **Lightweight**: ~10MB binary size
- 🔐 **Secure**: Token-based authentication
- 📦 **Auto-patcher**: Server-side patch management
- 🌐 **Avalanche Server**: Pre-configured for immediate play

---

## 🛠️ Current Development Status

### ✅ Completed (70% - Phase 1-2)

- [x] Backend refactoring (removed Windows-only dependencies)
- [x] Cross-platform INI parsing (conditional compilation)
- [x] Wine/Proton integration architecture
- [x] Branding cleanup (vanilla CAPCOM style)
- [x] **Server configuration system** ✨ NEW
- [x] **Avalanche MHFZ server pre-configured** ✨ NEW
- [x] Character selection UI
- [x] Login/authentication system

### 🚧 In Progress (Phase 3)

- [ ] **Game launch mechanism** (Wine wrapper implementation) 🔥 NEXT
- [ ] Full INI parser (read/write on Linux)
- [ ] Offline patcher system
- [ ] Friends list injection (Linux-compatible method)

### 📅 Roadmap (Phase 4)

- [ ] Auto-update system
- [ ] Multi-language support (EN/IT)
- [ ] AppImage/Flatpak packaging (Linux)
- [ ] Steam Deck optimization

**Progress**: `██████████████░░░░░░` 70%

---

## 📚 Documentation

Comprehensive documentation is available in the [`docs/`](docs/) folder:

- **[📊 ANALYSIS.md](docs/ANALYSIS.md)** - Detailed analysis of Linux porting changes
- **[🛠️ IMPLEMENTATION_PLAN.md](docs/IMPLEMENTATION_PLAN.md)** - Step-by-step implementation guide
- **[✅ TESTING_CHECKLIST.md](docs/TESTING_CHECKLIST.md)** - Complete testing procedures
- **[📖 docs/README.md](docs/README.md)** - Documentation index and quick start

---

## 🚀 Quick Start

### For Players (Stable Release - Coming Soon)

**Pre-configured for Avalanche MHFZ server!**

1. Download latest release from [Releases](https://github.com/mrsasy89/MHFZ-Launcher/releases)
2. Extract and run `MHFZ-Launcher`
3. Enter your Avalanche server credentials
4. Click "START GAME" - it just works! 🎉

### For Developers (Build from Source)

See [Build Instructions](#build-instructions) below.

---

## 🎮 Server Configuration

### Avalanche MHFZ Server (Pre-configured)

The launcher comes pre-configured with the **Avalanche** server:

```rust
Server: Avalanche
URL: http://avalanchemhfz.ddns.net
Launcher Port: 9010  // Patch/login server
Game Port: 53310     // In-game connection
Version: ZZ          // Monster Hunter Frontier Z
```

**No manual configuration needed!** Just login and play.

### Custom Server Setup

To add your own server, edit `ButterClient/config.json`:

```json
{
  "endpoints": [
    {
      "name": "My Server",
      "url": "http://myserver.example.com",
      "launcher_port": 9010,
      "game_port": 53310,
      "version": "ZZ",
      "is_remote": true
    }
  ]
}
```

Or use the in-launcher Settings panel (coming soon).

---

## 🛠️ Build Instructions

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
sudo pacman -S webkit2gtk base-devel wine wine-mono wine-gecko

# Ubuntu / Debian
sudo apt install libwebkit2gtk-4.0-dev build-essential wine64 winetricks
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
export WINEPREFIX="$HOME/Games/MHFZ/pfx"  # Linux only
npm run tauri:dev

# 5. Production build
npm run tauri:build
```

**Output location**: `src-tauri/target/release/`

---

## 🐧 Linux Setup (Wine/Proton)

### Wine Prefix Configuration

```bash
# 1. Create isolated prefix
mkdir -p ~/Games/MHFZ/pfx
export WINEPREFIX=~/Games/MHFZ/pfx

# 2. Initialize 32-bit prefix (MHFZ is 32-bit)
WINEARCH=win32 wineboot --init

# 3. Install dependencies
winetricks dotnet48 vcrun2019 d3dx9 d3dcompiler_47
winetricks corefonts allfonts  # Fixes text rendering

# 4. Verify setup
winecfg  # Should open without errors
```

### Using GE-Proton (Recommended for Gaming)

If you have Steam installed:

```bash
# Download GE-Proton
# https://github.com/GloriousEggroll/proton-ge-custom/releases

# Extract to ~/.steam/steam/compatibilitytools.d/
# Then use via Steam compatibility tool
```

### Game Files Location

Place MHFZ game files in your chosen directory:

```
~/Games/MHFZ/
├── mhf.exe          # Main executable (F5) or
├── mhfo.dll         # SD client (ZZ) or
├── mhfo-hd.dll      # HD client (ZZ)
├── mhf.ini          # Game configuration
├── dat/             # Game data
└── ...
```

Set the game folder in launcher settings or via environment:

```bash
export MHF_GAME_FOLDER="$HOME/Games/MHFZ"
```

---

## ⚙️ Configuration Files

### Launcher Config (`ButterClient/config.json`)

Stored in game folder, contains:
- Server endpoints
- User preferences (theme, language)
- Last selected character
- Window settings

### Game Settings (`mhf.ini`)

Stored in game folder, controls:
- Graphics quality (HD/SD)
- Resolution (fullscreen/windowed)
- Sound volume
- Input settings

**Note**: On Linux, `mhf.ini` uses default values if file is missing (Wine handles actual game settings).

---

## 🔧 Technical Architecture

### Backend (Rust/Tauri)

```
src-tauri/
├── src/
│   ├── main.rs              # Tauri entry point + state management
│   ├── config.rs            # ✅ Server endpoints (Avalanche pre-configured)
│   ├── settings.rs          # ✅ Cross-platform INI parser
│   ├── endpoint.rs          # Server connection logic
│   ├── patcher.rs           # Update system
│   ├── server.rs            # HTTP client for auth/API
│   └── lib_linux.rs         # 🚧 Wine launcher (in progress)
├── mhf-iel-master/          # Game launcher module
│   └── src/
│       ├── lib.rs           # Platform-specific entry
│       ├── mhf.rs           # Windows native launcher
│       └── linux.rs         # 🚧 Linux Wine wrapper (planned)
└── Cargo.toml
```

### Frontend (Vue.js)

```
src/
├── Classic.vue              # Classic UI (default, CAPCOM style)
├── Modern.vue               # Modern UI (alternative)
├── Settings.vue             # Configuration panel
└── store.js                 # Vuex state management
```

### Key Technologies

- **Tauri**: Cross-platform desktop framework (Rust + Web)
- **Vue.js**: Reactive UI framework
- **Reqwest**: HTTP client for server communication
- **Tokio**: Async runtime
- **Wine/Proton**: Windows compatibility layer (Linux)

---

## 🤝 Contributing

Contributions are welcome! Areas needing help:

### High Priority
1. **Game launch (Linux)**: Complete Wine wrapper in `src-tauri/src/lib_linux.rs`
2. **Testing**: Multi-distro compatibility (Ubuntu, Fedora, Debian)
3. **Steam Deck**: Optimization and testing

### Medium Priority
4. **INI parser**: Full read/write support on Linux
5. **Friends list**: Cross-platform injection method
6. **Localization**: Italian/English translations

### Development Workflow

```bash
# 1. Fork and clone
git clone https://github.com/YOUR_USERNAME/MHFZ-Launcher.git
cd MHFZ-Launcher

# 2. Read documentation
cat docs/README.md
cat docs/IMPLEMENTATION_PLAN.md

# 3. Create feature branch
git checkout -b feature/wine-launcher

# 4. Make changes and test
npm run tauri:dev
# Follow TESTING_CHECKLIST.md

# 5. Commit with conventional commits
git commit -m "feat(linux): implement Wine game launcher

- Add Wine process spawning
- Detect wine64/wine automatically
- Handle WINEPREFIX environment

Tested on: Arch Linux with Wine 9.0"

# 6. Push and create PR
git push origin feature/wine-launcher
```

See [IMPLEMENTATION_PLAN.md](docs/IMPLEMENTATION_PLAN.md) for detailed step-by-step guides.

---

## 🧪 Testing

### Tested Environments ✅

| Platform | Status | Notes |
|----------|--------|-------|
| **Arch Linux** | ✅ Working | Wine 9.0, primary dev environment |
| **Windows 10** | ✅ Working | Native build |
| **Windows 11** | 🧪 Untested | Should work (Windows 10 compatible) |
| **Ubuntu 22.04** | 🧪 Untested | Should work (needs testing) |
| **Debian 12** | 🧪 Untested | Should work (needs testing) |
| **Steam Deck** | 🧪 Untested | Planned support |

### Test Coverage

- ✅ Compilation (Windows + Linux)
- ✅ UI rendering
- ✅ Login to Avalanche server
- ✅ Character list loading
- ✅ Settings persistence
- 🚧 Game launch (Windows only, Linux WIP)
- 🚧 Patching system
- ❌ Friends list (not yet implemented)

See [TESTING_CHECKLIST.md](docs/TESTING_CHECKLIST.md) for comprehensive test procedures.

---

## 📚 Related Projects

- **[Erupe Server](https://github.com/ErupeServer/Erupe)** - Private server implementation
- **[Avalanche MHFZ](http://avalanchemhfz.ddns.net:9010)** - Public Erupe server (pre-configured)
- **[MHF Patch Server](https://github.com/mrsasy89/MHF-Patch-Server)** - Update distribution system
- **[ButterClient](https://github.com/RuriYoshinova/ButterClient)** - Original Windows-only launcher (upstream)

---

## 📜 License

GNU General Public License v3.0 - See [LICENSE](LICENSE) for details.

This project is a fork of [ButterClient](https://github.com/RuriYoshinova/ButterClient) with additional Linux support.

---

## 🙏 Credits

- **Original ButterClient**: [RuriYoshinova](https://github.com/RuriYoshinova)
- **Linux Port**: [mrsasy89](https://github.com/mrsasy89)
- **Avalanche Server**: Community-maintained Erupe instance
- **Erupe Server**: Community-developed private server
- **CAPCOM**: Original game assets and Monster Hunter Frontier Z

---

## 💬 Support & Community

- **Issues**: [GitHub Issues](https://github.com/mrsasy89/MHFZ-Launcher/issues)
- **Discussions**: [GitHub Discussions](https://github.com/mrsasy89/MHFZ-Launcher/discussions)
- **Documentation**: [docs/](docs/) folder
- **Avalanche Discord**: (link if available)

### FAQ

**Q: Can I play on official CAPCOM servers?**  
A: No. Official servers were shut down in 2019. This launcher works with private servers only.

**Q: Do I need a Windows PC to play on Linux?**  
A: No! Wine/Proton runs the game natively on Linux. Performance is excellent.

**Q: Where do I get the game files?**  
A: You need a copy of Monster Hunter Frontier Z (Japanese version). Check community resources.

**Q: Is this launcher safe?**  
A: Yes. Open source (GPL v3), no telemetry, no ads. You can review the code yourself.

**Q: Can I use this on Steam Deck?**  
A: Not yet tested, but it should work with Proton. Testing welcome!

---

## 🚨 Disclaimer

This project is for **educational purposes** and **preservation** of a discontinued game. All rights to Monster Hunter Frontier Z belong to CAPCOM Co., Ltd. This launcher does not contain any game assets or copyrighted material.

**Use at your own risk.** We are not affiliated with CAPCOM.

---

## 🎯 Project Status

**Current Version**: 0.1.0 (Pre-release)  
**Last Updated**: December 11, 2025  
**Maintainer**: [@mrsasy89](https://github.com/mrsasy89)

**Next Milestone**: v0.2.0 - Wine launcher implementation (Step 4)  
**ETA**: ~1 week

---

⭐ **Star this project** if you find it useful!  
🐛 **Report bugs** via [Issues](https://github.com/mrsasy89/MHFZ-Launcher/issues)  
💻 **Contribute** following our [guidelines](docs/IMPLEMENTATION_PLAN.md)

**Happy Hunting! 🎮🔥**
