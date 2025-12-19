# 🎮 MHFZ-Launcher

**Cross-platform launcher for Monster Hunter Frontier Z**  
Supports Windows natively and Linux via Wine integration.

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)  
[![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20Linux-lightgrey)](#)  
[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)  
[![Tauri](https://img.shields.io/badge/Tauri-1.5-24C8DB.svg)](https://tauri.app/)

---

## 📋 Overview

MHFZ-Launcher is a modern, cross-platform game launcher for **Monster Hunter Frontier Z**, designed to work with private servers (primarily [Erupe](https://github.com/mrsasy89/Erupe)). Built with Rust (Tauri backend) + Vue.js frontend.

### 🌟 Key Features

- ✅ **Cross-platform**: Windows native + Linux (Wine)
- 🎨 **Vanilla UI**: CAPCOM-style interface (no custom branding)
- 🔧 **Modular**: Easy server switching and configuration
- 🚀 **Lightweight**: ~10MB binary size
- 🔐 **Secure**: Token-based authentication
- 📦 **Auto-patcher**: Server-side patch management
- 🌐 **Avalanche Server**: Pre-configured for immediate play
- ⚙️ **Full Settings Control**: Game configuration (graphics, audio, controls)
- 🐧 **AppImage Support**: Portable Linux distribution **NEW!** ✨
- 🎌 **Auto Japanese Fonts**: Automatic font installation on Linux **NEW!** ✨
- 📁 **Smart Defaults**: Game folder defaults to `Current Folder/game` **NEW!** ✨

---

## 🛠️ Current Development Status

### ✅ Completed (95% - Phase 1-3)

- [x] Backend refactoring (removed Windows-only dependencies)
- [x] Cross-platform INI parsing (conditional compilation)
- [x] Wine integration architecture
- [x] Branding cleanup (vanilla CAPCOM style)
- [x] **Server configuration system** ✨
- [x] **Avalanche MHFZ server pre-configured** ✨
- [x] Character selection UI
- [x] Login/authentication system
- [x] **Wine launcher core (lib_linux.rs)** 
- [x] **Successful game launch on Linux**
- [x] **Game launch via Wine** ✅ WORKING!
- [x] **mhf-iel integration** ✅ WORKING!
- [x] **Friends list injection** ✅ WORKING! 🎉
- [x] **Full INI parser** ✅ COMPLETED! 🎉
- [x] **Wine prefix auto-creation** ✅ **NEW!** 🎉
- [x] **Japanese fonts auto-installation** ✅ **NEW!** 🎉
- [x] **AppImage packaging** ✅ **NEW!** 🎉
- [x] **Default game folder (Current Folder/game)** ✅ **NEW!** 🎉

### 📅 Roadmap (Phase 4)

- [ ] Flatpak packaging (Linux)
- [ ] Steam OS optimization
- [ ] Multi-distro testing (Ubuntu, Fedora, Debian)

**Progress**: `███████████████████░` 95%

---

## 🐧 Linux Support Status

### ✅ What Works (Tested on Arch Linux)

| Feature | Status | Notes |
|---------|--------|-------|
| **Launcher UI** | ✅ Working | Tauri + WebKitGTK |
| **Login to Avalanche** | ✅ Working | HTTP auth |
| **Character selection** | ✅ Working | API integration |
| **Game launch (Wine)** | ✅ **Working!** | Wine 10.20 tested |
| **DXVK support** | ✅ Working | Vulkan renderer |
| **Game execution** | ✅ **Working!** | Confirmed playable |
| **mhf-iel integration** | ✅ Working | Direct DLL injection |
| **config.json generation** | ✅ Working | 25+ fields |
| **Friends list Fix** | ✅ Working | mhf-iel integrated |
| **Game Settings (mhf.ini)** | ✅ Working | Full read/write |
| **Wine Prefix Auto-Creation** | ✅ **Working!** | First launch setup ✨ **NEW!** |
| **Japanese Fonts** | ✅ **Working!** | Auto-install from fonts/ ✨ **NEW!** |
| **AppImage Distribution** | ✅ **Working!** | Portable package ✨ **NEW!** |
| **Default Game Folder** | ✅ **Working!** | Auto-detects game/ subfolder ✨ **NEW!** |

### 🚧 Known Issues

- ⚠️ GTK backend error on game exit (cosmetic, non-blocking)

### 📊 Test Results

**Last test**: December 19, 2025  
**Environment**: Arch Linux + Wine 10.20 + DXVK 2.7.1

```
✅ Login successful
✅ Character list loaded
✅ config.json generated correctly
✅ [Friends Injector] Injection complete!
✅ mhf-iel-cli.exe launched via Wine
✅ Game started (bypassed CAPCOM launcher)
✅ In-game connection established
✅ Gameplay confirmed working
✅ Game settings read/write working (mhf.ini)
✅ Wine prefix auto-created on first launch ← NEW!
✅ Japanese fonts auto-installed ← NEW!
✅ AppImage double-click launch working ← NEW!
✅ Default game folder detection ← NEW!
✅ Clean exit (code 0)
```

---

## 🚀 Quick Start

### For Players (Linux - AppImage) **NEW!** ✨

**Pre-configured for Avalanche MHFZ server!**

#### Option 1: AppImage (Recommended - No Installation Required)

1. **Download AppImage** from [Releases](https://github.com/mrsasy89/MHFZ-Launcher/releases)
   ```bash
   chmod +x MHFZ-Launcher-x86_64.AppImage
   ```

2. **Prepare game files structure**
   ```
   ~/Games/MHFZ/
   ├── MHFZ-Launcher-x86_64.AppImage  # The launcher
   ├── game/                          # Game files folder (NEW!)
   │   ├── mhf-iel-cli.exe           # IELess launcher
   │   ├── mhfo-hd.dll               # HD client
   │   ├── mhf.ini                   # Config (auto-generated)
   │   └── dat/                      # Game data
   └── fonts/                         # Japanese fonts (NEW!)
       ├── msgothic.ttc
       ├── msmincho.ttc
       └── ...
   ```

3. **Double-click AppImage and play!**
   - First launch takes 1-2 minutes (Wine prefix creation)
   - Japanese fonts are auto-installed
   - Subsequent launches are instant
   - No terminal needed! 🎉

**What happens on first launch:**
- ✅ Wine prefix created automatically at `game/pfx`
- ✅ Japanese fonts copied from `fonts/` to Wine prefix
- ✅ XAUTHORITY configured for display server
- ✅ FONTCONFIG variables set for correct rendering
- ✅ All done transparently, no user action needed!

#### Option 2: Manual Wine Setup

1. **Install Wine** (10.x or newer)
   ```bash
   # Arch/Manjaro
   sudo pacman -S wine wine-mono wine-gecko dxvk-bin
   
   # Ubuntu/Debian
   sudo apt install wine64 winetricks
   ```

2. **Download game files** and launcher
   - Create folder structure as shown above
   - Ensure `fonts/` folder contains Japanese fonts

3. **Run launcher**
   ```bash
   ./MHFZ-Launcher-x86_64.AppImage
   # Or if using binary:
   ./MHFZ-Launcher
   ```

4. **Login and play!**
   - Enter Avalanche credentials
   - Select character
   - Click **START GAME**
   - Game launches automatically 🎉

### Game Folder Configuration **NEW!**

The launcher now defaults to `Current Folder/game` for the game path. This means:

- **Before**: You had to manually configure the game folder
- **After**: Just place game files in a `game/` subfolder next to the launcher

**Example structure:**
```
/home/user/MHFZ/
├── MHFZ-Launcher           # Launcher executable
└── game/                   # Default game folder (auto-detected)
    ├── mhf-iel-cli.exe
    ├── mhfo-hd.dll
    └── dat/
```

You can still override this in **Settings → Advanced → Game Folder** if needed.

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
Game Port: 54001     // In-game connection
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
      "game_port": 54001,
      "version": "ZZ",
      "is_remote": true
    }
  ]
}
```

Or use the in-launcher Settings panel.

---

## ⚙️ Game Settings Configuration

### Implemented Settings

The launcher now **reads and writes** game settings from `mhf.ini` on **both Windows and Linux**:

#### 🖥️ Display
- ✅ **HD Version** (Graphics quality: Classic vs HD)
- ✅ **Fullscreen Mode** (Windowed vs Fullscreen)
- ✅ **Window Resolution** (Custom width/height)
- ✅ **Fullscreen Resolution** (Monitor resolution)

### Cross-Platform INI Parser

Custom Rust INI parser (`ini_parser.rs`) that:

- ✅ **Preserves file format**: Maintains original line endings (CRLF on Windows, LF on Linux)
- ✅ **Non-destructive**: Only modifies changed settings
- ✅ **Auto-creates**: Generates default `mhf.ini` if missing
- ✅ **Error handling**: Detailed logging for debugging
- ✅ **Same code**: Identical behavior on Windows and Linux

**Total configurable options**: 36 settings available in `mhf.ini`  
**Currently exposed in UI**: 6 (graphics/display)  
**Planned for next release**: 30 additional options

---

## 🐧 Linux Implementation Details **UPDATED!** ✨

### Wine Prefix Auto-Creation **NEW!**

The launcher now automatically creates and configures the Wine prefix on first launch:

**What it does:**
1. Checks if `game/pfx` exists
2. If not, runs `wineboot --init` automatically
3. Takes 1-2 minutes on first launch
4. Subsequent launches are instant

**Benefits:**
- ✅ No manual `wineboot` commands needed
- ✅ No large (~500MB) prefix in distribution
- ✅ Smaller download size (~100MB vs ~600MB)
- ✅ Better portability across Linux systems

### Japanese Fonts Auto-Installation **NEW!**

Fonts are now installed automatically from the `fonts/` folder:

**Setup:**
```
~/Games/MHFZ/
└── fonts/                    # Place fonts here
    ├── msgothic.ttc         # Required for UI
    ├── msmincho.ttc         # Required for text
    ├── meiryo.ttc           # Optional
    └── meiryob.ttc          # Optional
```

**Installation process:**
1. Launcher checks if fonts are installed in Wine prefix
2. If not, copies from `fonts/` to `pfx/drive_c/windows/Fonts/`
3. Happens automatically on first launch
4. One-time operation (~50MB)

**Font sources:**
- Extract from Windows 10/11: `C:\Windows\Fonts\`
- Download from Japanese font packs
- Minimum required: `msgothic.ttc`, `msmincho.ttc`

### AppImage Wrapper Script **NEW!**

The AppImage includes a wrapper script that:

**Environment setup:**
```bash
export XAUTHORITY="$HOME/.Xauthority"        # X11 auth
export FONTCONFIG_PATH="/etc/fonts"          # System fonts
export FONTCONFIG_FILE="/etc/fonts/fonts.conf"
export WINEPREFIX="$APPDIR/game/pfx"        # Wine prefix
```

**Benefits:**
- ✅ No terminal window popup
- ✅ Desktop double-click works
- ✅ Proper X11 authentication
- ✅ System fonts integration

### Wine Prefix Configuration

**Default location:** `game/pfx` (relative to launcher)

**Structure after setup:**
```
game/pfx/
├── drive_c/
│   └── windows/
│       └── Fonts/              # Auto-installed fonts
│           ├── msgothic.ttc
│           └── msmincho.ttc
├── dosdevices/
└── system.reg
```

**Environment variables:**
```bash
WINEPREFIX="$HOME/Games/MHFZ/game/pfx"   # Prefix location
WINEARCH=win32                            # 32-bit (MHFZ requirement)
XAUTHORITY="$HOME/.Xauthority"            # X11 display auth
```

### Tested Wine Versions

| Version | Status | Notes |
|---------|--------|-------|
| Wine 10.20 | ✅ **Working** | Primary test environment |
| Wine 9.0 | ✅ Working | Stable |
| Wine 8.x | 🧪 Untested | Should work |
| Wine-Staging | ✅ Recommended | More gaming patches |

---

## 🛠️ Build Instructions

### Prerequisites

#### All Platforms
- [Rust](https://rustup.rs/) (1.70+, **nightly** toolchain required)
- [Node.js](https://nodejs.org/) (16+)
- [npm](https://www.npmjs.com/) (8+)

#### Linux Additional
- **Wine** (10.x or newer recommended)
- WebKitGTK development libraries
- DXVK (optional, for Vulkan rendering)

```bash
# Arch Linux / Manjaro
sudo pacman -S webkit2gtk base-devel wine wine-mono wine-gecko dxvk-bin

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
npm run tauri:dev

# 5. Production build
npm run tauri:build

# 6. Build AppImage (Linux only)
./build-steamos.sh
```

**Output locations:**
- Binary: `src-tauri/target/release/MHFZ-Launcher`
- AppImage: `AppImage/MHFZ-Launcher-x86_64.AppImage`

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

**Auto-generated** if missing with sensible defaults.

---

## 🔧 Technical Architecture

### Backend (Rust/Tauri)

```
src-tauri/
├── src/
│   ├── main.rs              # Tauri entry point
│   ├── config.rs            # Server config (Avalanche)
│   ├── settings.rs          # Cross-platform settings
│   ├── ini_parser.rs        # Custom INI parser
│   ├── endpoint.rs          # Server connection
│   ├── patcher.rs           # Update system
│   ├── server.rs            # HTTP client
│   └── lib_linux.rs         # Wine launcher (UPDATED!)
│       ├── create_wine_prefix()      # NEW: Auto prefix creation
│       ├── install_japanese_fonts()  # NEW: Auto font install
│       └── run_linux()              # Main launcher logic
└── Cargo.toml
```

### Frontend (Vue.js)

```
src/
├── settings/
│   ├── SettingsList.vue     # Settings UI
│   ├── SettingsCheckbox.vue
│   ├── SettingsItem.vue
│   └── SettingsButton.vue
├── Classic.vue              # Classic UI (CAPCOM style)
├── Modern.vue               # Modern UI
└── store.js                 # Vuex state (UPDATED!)
    └── effectiveFolder      # NEW: Defaults to Current Folder/game
```

### Key Technologies

- **Tauri**: Cross-platform desktop framework (Rust + Web)
- **Vue.js**: Reactive UI framework
- **Reqwest**: HTTP client for server communication
- **Tokio**: Async runtime
- **Wine**: Windows compatibility layer (Linux)
- **DXVK**: DirectX to Vulkan translation (optional)
- **AppImage**: Portable Linux application format **NEW!**

---

## 🤝 Contributing

Contributions are welcome! Areas needing help:

### High Priority
1. **Testing**: Multi-distro compatibility (Ubuntu, Fedora, Debian)
2. **SteamOS**: Optimization and testing

### Medium Priority
3. **GTK exit crash**: Fix cosmetic error on game closure
4. **Flatpak packaging**: Alternative to AppImage

### Development Workflow

```bash
# 1. Fork and clone
git clone https://github.com/YOUR_USERNAME/MHFZ-Launcher.git
cd MHFZ-Launcher

# 2. Create feature branch
git checkout -b feature/my-feature

# 3. Make changes and test
npm run tauri:dev

# 4. Commit with conventional commits
git commit -m "feat(linux): add feature X"

# 5. Push and create PR
git push origin feature/my-feature
```

---

## 🧪 Testing

### Tested Environments ✅

| Platform | Status | Notes |
|----------|--------|-------|
| **Arch Linux** | ✅ **Working** | Wine 10.20 + DXVK 2.7.1, primary dev environment |
| **Windows 10** | ✅ Working | Native build |
| **Windows 11** | ✅ Working | Windows 10 compatible |
| **Ubuntu 22.04** | 🧪 Untested | Should work (needs testing) |
| **Debian 12** | 🧪 Untested | Should work (needs testing) |
| **Steam OS** | 🧪 Untested | Planned support |

### Test Coverage

- ✅ Compilation (Windows + Linux)
- ✅ UI rendering
- ✅ Login to Avalanche server
- ✅ Character list loading
- ✅ Settings persistence
- ✅ Game launch (Linux via Wine)
- ✅ In-game connection
- ✅ Gameplay
- ✅ config.json generation (mhf-iel)
- ✅ mhf-iel-cli.exe launch
- ✅ Friends list
- ✅ Wine prefix auto-creation **NEW!**
- ✅ Japanese fonts auto-install **NEW!**
- ✅ AppImage packaging **NEW!**
- ✅ Default game folder detection **NEW!**

---

## 📚 Related Projects

- **[Erupe Server](https://github.com/mrsasy89/Erupe)** - Private server implementation
- **[MHF Patch Server](https://github.com/mrsasy89/MHF-Patch-Server)** - Update distribution system
- **[mhf-iel](https://github.com/mrsasy89/mhf-iel)** - IELess launcher (DLL injection)
- **[ButterClient](https://github.com/LilButter/ButterClient)** - Original Windows-only launcher (upstream)

---

## 📜 License

GNU General Public License v3.0 - See [LICENSE](LICENSE) for details.

This project is a fork of [ButterClient](https://github.com/LilButter/ButterClient) with additional Linux support.

---

## 🙏 Credits

- **Original ButterClient**: [LilButter](https://github.com/LilButter)
- **Linux Port & Enhancements**: [mrsasy89](https://github.com/mrsasy89)
- **mhf-iel**: [rockisch](https://github.com/rockisch) - IELess launcher
- **Avalanche Server**: Community-maintained Erupe instance
- **Erupe Server**: Community-developed private server
- **CAPCOM**: Original game assets and Monster Hunter Frontier Z

---

## 💬 Support & Community

- **Issues**: [GitHub Issues](https://github.com/mrsasy89/MHFZ-Launcher/issues)
- **Discussions**: [GitHub Discussions](https://github.com/mrsasy89/MHFZ-Launcher/discussions)
- **Monster Hunter Old Gen Discord**: [Join here](https://discord.gg/UdQ4cy5TbU)

### FAQ

**Q: Can I play on official CAPCOM servers?**  
A: No. Official servers were shut down in 2019. This launcher works with private servers only.

**Q: Do I need a Windows PC to play on Linux?**  
A: No! Wine runs the game natively on Linux. Performance is excellent.

**Q: Do I need to manually setup Wine?**  
A: **Not anymore!** The AppImage auto-creates the Wine prefix on first launch. Just double-click and play.

**Q: What about Japanese fonts?**  
A: **Auto-installed!** Just place fonts in a `fonts/` folder next to the launcher. They're installed automatically on first launch.

**Q: Where do I get the game files?**  
A: You need a copy of Monster Hunter Frontier Z (Japanese version). Check community resources.

**Q: Is this launcher safe?**  
A: Yes. Open source (GPL v3), no telemetry, no ads. Review the code yourself.

**Q: Can I use this on Steam Deck?**  
A: Not yet tested, but AppImage should work. Testing welcome!

**Q: Why Wine and not Proton?**  
A: Wine is lighter, standalone, and MHFZ (DirectX 9) runs perfectly. Wine 10.20 tested working.

**Q: Does it work with other Erupe servers?**  
A: Yes! Configure custom servers in Settings. Avalanche is just the default.

**Q: Where does the game folder need to be?**  
A: By default, just create a `game/` subfolder next to the launcher. Or configure a custom path in Settings → Advanced.

---

## 🚨 Disclaimer

This project is for **educational purposes** and **preservation** of a discontinued game. All rights to Monster Hunter Frontier Z belong to CAPCOM Co., Ltd. This launcher does not contain any game assets or copyrighted material.

**Use at your own risk.** We are not affiliated with CAPCOM.

---

## 🎯 Project Status

**Current Version**: 1.4.7 (Linux AppImage + Auto-Setup)  
**Last Updated**: December 19, 2025  
**Maintainer**: [@mrsasy89](https://github.com/mrsasy89)

### Recent Milestones 🎉

- ✅ **December 19, 2025**: Default game folder to Current Folder/game ✨
- ✅ **December 19, 2025**: Japanese fonts auto-installation ✨
- ✅ **December 19, 2025**: Wine prefix auto-creation ✨
- ✅ **December 19, 2025**: AppImage packaging ✨
- ✅ **December 16, 2025**: Full INI parser implementation (cross-platform)
- ✅ **December 15, 2025**: Friends list fixing
- ✅ **December 14, 2025**: mhf-iel integration
- ✅ **December 11, 2025**: Wine launcher successfully tested on Arch Linux
- ✅ **December 11, 2025**: Game confirmed playable via Wine 10.20

### Next Milestone

**v1.5.0 - Multi-distro Support**
  
**ETA**: ~1-2 weeks

Goals:
- [ ] Ubuntu 22.04/24.04 testing
- [ ] Fedora 39/40 testing
- [ ] Debian 12 testing
- [ ] Steam OS testing
- [ ] Flatpak packaging

---

⭐ **Star this project** if you find it useful!  
🐛 **Report bugs** via [Issues](https://github.com/mrsasy89/MHFZ-Launcher/issues)  
📦 **Download AppImage** from [Releases](https://github.com/mrsasy89/MHFZ-Launcher/releases)

**Happy Hunting! 🎮🔥**
