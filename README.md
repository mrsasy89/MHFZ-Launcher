# 🎮 MHFZ-Launcher

**Cross-platform launcher for Monster Hunter Frontier Z**  
Supports Windows natively and Linux via Wine integration.

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)  
[![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20Linux-lightgrey)](#)  
[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)  
[![Tauri](https://img.shields.io/badge/Tauri-1.5-24C8DB.svg)](https://tauri.app/)

---

## 📋 Overview

MHFZ-Launcher is a modern, cross-platform game launcher for **Monster Hunter Frontier Z**, designed to work with private servers (primarily [Erupe](https://github.com/ErupeServer/Erupe)). Built with Rust (Tauri backend) + Vue.js frontend.

### 🌟 Key Features

- ✅ **Cross-platform**: Windows native + Linux (Wine)
- 🎨 **Vanilla UI**: CAPCOM-style interface (no custom branding)
- 🔧 **Modular**: Easy server switching and configuration
- 🚀 **Lightweight**: ~10MB binary size
- 🔐 **Secure**: Token-based authentication
- 📦 **Auto-patcher**: Server-side patch management
- 🌐 **Avalanche Server**: Pre-configured for immediate play

---

## 🛠️ Current Development Status

### ✅ Completed (85% - Phase 1-3)

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

### 🚧 In Progress (Phase 3 - Final Integration)

- [x] Game launch via Wine ✅ **WORKING!**
- [x] mhf-iel integration ✅ **WORKING!**
- [x] Friends list injection ✅ **WORKING!** 🎉 NEW
- [ ] **Full INI parser** 🔥 NEXT (read/write on Linux)

### 📅 Roadmap (Phase 4)

- [ ] AppImage/Flatpak packaging (Linux)
- [ ] Steam OS optimization

**Progress**: `█████████████████░░░` 85%

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
| **Friends list Fix** | ✅ Working | Fix mhf-iel integrate |

### 🚧 Known Issues

- ⚠️ GTK backend error on game exit (cosmetic, non-blocking)

### 📊 Test Results

**Last test**: December 15, 2025  
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
✅ Clean exit (code 0)
```

---

## 🚀 Quick Start

### For Players (Linux - Beta)

**Pre-configured for Avalanche MHFZ server!**

1. **Install Wine** (10.x or newer)
   ```bash
   # Arch/Manjaro
   sudo pacman -S wine wine-mono wine-gecko dxvk-bin
   
   # Ubuntu/Debian
   sudo apt install wine64 winetricks
   ```

2. **Setup Wine prefix**
   ```bash
   mkdir -p ~/Games/MHFZ/pfx
   export WINEPREFIX=~/Games/MHFZ/pfx
   WINEARCH=win32 wineboot --init
   ```

3. **Install DXVK** (optional, improves performance)
   ```bash
   WINEPREFIX=~/Games/MHFZ/pfx setup_dxvk install
   ```

4. **Download game files** (Monster Hunter Frontier Z)
   - Place in `~/Games/MHFZ/`
   - Download **mhf-iel-cli.exe** from [mhf-iel releases](https://github.com/rockisch/mhf-iel)
   - Place `mhf-iel-cli.exe` in game folder
   - Ensure `mhfo-hd.dll` (or `mhfo.dll`) and `dat/` folder are present

5. **Download launcher** from [Releases](https://github.com/mrsasy89/MHFZ-Launcher/releases)
   ```bash
   chmod +x MHFZ-Launcher
   export WINEPREFIX=~/Games/MHFZ/pfx
   ./MHFZ-Launcher
   ```

6. **Login and play!**
   - Enter Avalanche credentials
   - Select character
   - Click **START GAME**
   - Game launches via Wine automatically 🎉

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

Or use the in-launcher Settings panel (coming soon).

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

# 4. Development mode (Linux)
export WINEPREFIX="$HOME/Games/MHFZ/pfx"
export RUST_LOG=info  # Enable debug logs
npm run tauri:dev

# 5. Production build
npm run tauri:build
```

**Output location**: `src-tauri/target/release/`

---

## 🐧 Linux Setup (Wine)

### Why Wine (Not Proton)?

**MHFZ-Launcher uses Wine**, not Proton, for the following reasons:

- ✅ **Lighter weight**: No Steam overhead
- ✅ **DirectX 9 compatibility**: Wine handles D3D9 natively
- ✅ **Proven compatibility**: mhf-iel tested with Wine
- ✅ **Standalone**: No Steam dependency
- ✅ **Confirmed working**: Game tested successfully with Wine 10.20

Proton is Valve's fork of Wine optimized for Steam games, but MHFZ doesn't need its extra layers.

### Wine Prefix Configuration

```bash
# 1. Create isolated prefix
mkdir -p ~/Games/MHFZ/pfx
export WINEPREFIX=~/Games/MHFZ/pfx

# 2. Initialize 32-bit prefix (MHFZ is 32-bit)
WINEARCH=win32 wineboot --init

# 3. Install dependencies (optional)
winetricks dotnet48 vcrun2019 d3dx9 d3dcompiler_47
winetricks corefonts allfonts  # Fixes text rendering

# 4. Install DXVK (optional, recommended for performance)
setup_dxvk install

# 5. Verify setup
winecfg  # Should open without errors
```

### Game Files Location

Place MHFZ game files in your chosen directory:

```
~/Games/MHFZ/
├── mhf-iel-cli.exe  # IELess launcher (REQUIRED) ← Download from mhf-iel releases
├── mhfo.dll         # SD client (ZZ) or
├── mhfo-hd.dll      # HD client (ZZ) ← Recommended
├── mhf.ini          # Game configuration (auto-generated)
├── config.json      # mhf-iel config (auto-generated by launcher)
├── dat/             # Game data
├── pfx/             # Wine prefix (created by setup)
└── ...

```

Set the game folder in launcher settings or via environment:

```bash
export MHF_GAME_FOLDER="$HOME/Games/MHFZ"
export WINEPREFIX="$HOME/Games/MHFZ/pfx"
```

### Tested Wine Versions

| Version | Status | Notes |
|---------|--------|-------|
| Wine 10.20 | ✅ **Working** | Primary test environment |
| Wine 9.0 | ✅ Working | Stable |
| Wine 8.x | 🧪 Untested | Should work |
| Wine-Staging | ✅ Recommended | More gaming patches |

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
│   └── lib_linux.rs         # ✅ Wine launcher (WORKING!)
├── mhf-iel-master/          # Game launcher module
│   └── src/
│       ├── lib.rs           # Platform-specific entry
│       ├── mhf.rs           # Windows native launcher
│       └── linux.rs         # 🚧 Linux Wine wrapper (planned)
└── Cargo.toml
```

### lib_linux.rs Implementation

**Wine process spawning logic** (simplified):

```rust
pub fn run_linux(config: MhfConfigLinux) -> std::io::Result<()> {
    // 1. Get mhf-iel config from global storage
    let iel_config = MHF_IEL_CONFIG_GLOBAL.get();
    
    if let Some(cfg) = iel_config {
        // 2. Generate config.json for mhf-iel
        generate_mhf_iel_config(&config.game_folder, cfg)?;
        
        // 3. Find mhf-iel-cli.exe
        let iel_path = config.game_folder.join("mhf-iel-cli.exe");
        
        // 4. Setup Wine environment
        let wine_prefix = config.game_folder.join("pfx");
        
        // 5. Launch game via Wine + mhf-iel
        let mut command = Command::new("wine");
        command
            .arg(&iel_path)
            .current_dir(&config.game_folder)
            .env("WINEPREFIX", &wine_prefix)
            .env("DXVK_HUD", "fps");
        
        command.spawn()?.wait()?;
        Ok(())
    } else {
        // Fallback: Proton (only if mhf-iel unavailable)
        run_proton_fallback(config)
    }
}

```

**Next step**: Replace `mhf.exe` with `mhf-iel.exe` for direct DLL injection (bypasses CAPCOM launcher).

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
- **Wine**: Windows compatibility layer (Linux)
- **DXVK**: DirectX to Vulkan translation (optional)

---

## 🤝 Contributing

Contributions are welcome! Areas needing help:

### High Priority
1. **Testing**: Multi-distro compatibility (Ubuntu, Fedora, Debian)
2. **SteamOS**: Optimization and testing

### Medium Priority
4. **INI parser**: Full read/write support on Linux
5. **Friends list**: Cross-platform injection method
6. **Localization**: Japanese/English translations
7. **GTK exit crash**: Fix cosmetic error on game closure

### Development Workflow

```bash
# 1. Fork and clone
git clone https://github.com/YOUR_USERNAME/MHFZ-Launcher.git
cd MHFZ-Launcher

# 2. Read documentation
cat docs/README.md
cat docs/IMPLEMENTATION_PLAN.md

# 3. Create feature branch
git checkout -b feature/mhf-iel-integration

# 4. Make changes and test
export WINEPREFIX=~/Games/MHFZ/pfx
export RUST_LOG=info
npm run tauri:dev

# 5. Follow testing checklist
cat docs/TESTING_CHECKLIST.md

# 6. Commit with conventional commits
git commit -m "feat(linux): integrate mhf-iel for direct DLL injection

- Cross-compile mhf-iel.exe for Windows i686
- Update lib_linux.rs to use mhf-iel instead of mhf.exe
- Pass user token and server config via CLI args
- Bypass CAPCOM launcher entirely

Tested on: Arch Linux with Wine 10.20 + DXVK 2.7.1"

# 7. Push and create PR
git push origin feature/mhf-iel-integration
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
- ✅ **Game launch (Linux via Wine)** 🎉
- ✅ **In-game connection** 🎉
- ✅ **Gameplay** 🎉
- ✅ **config.json generation** (mhf-iel)
- ✅ **mhf-iel-cli.exe launch** 🎉
- ✅ **Game launch (Linux via Wine + mhf-iel)** 🎉
- ✅ **Friends list (implemented)**🎉

---

## 📚 Related Projects

- **[Erupe Server](https://github.com/ErupeServer/Erupe)** - Private server implementation
- **[Avalanche MHFZ](http://avalanchemhfz.ddns.net:9010)** - Public Erupe server (pre-configured)
- **[MHF Patch Server](https://github.com/mrsasy89/MHF-Patch-Server)** - Update distribution system
- **[mhf-iel](https://github.com/rockisch/mhf-iel)** - IELess launcher (DLL injection)
- **[ButterClient](https://github.com/RuriYoshinova/ButterClient)** - Original Windows-only launcher (upstream)

---

## 📜 License

GNU General Public License v3.0 - See [LICENSE](LICENSE) for details.

This project is a fork of [ButterClient](https://github.com/LilButter/ButterClient) with additional Linux support.

---

## 🙏 Credits

- **Original ButterClient**: [LilButter](https://github.com/LilButter)
- **Linux Port & mhf-iel Integration**: [mrsasy89](https://github.com/mrsasy89)
- **mhf-iel**: [rockisch](https://github.com/rockisch) - IELess launcher (now integrated!)
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
A: No! Wine runs the game natively on Linux. Performance is excellent (tested working).

**Q: Where do I get the game files?**  
A: You need a copy of Monster Hunter Frontier Z (Japanese version). Check community resources or archived game files.

**Q: Is this launcher safe?**  
A: Yes. Open source (GPL v3), no telemetry, no ads. You can review the code yourself.

**Q: Can I use this on Steam Deck?**  
A: Not yet tested, but it should work with the included Wine setup. Testing welcome!

**Q: Why Wine and not Proton?**  
A: Wine is lighter, works standalone without Steam, and MHFZ (DirectX 9) doesn't need Proton's extra features. Wine 10.20 tested working perfectly.

**Q: Does it work with other Erupe servers?**  
A: Yes! You can configure custom servers in the settings. Avalanche is just the default.

**Q: Do I need mhf.exe?**  
A: No! mhf-iel bypasses the CAPCOM launcher entirely. You only need `mhf-iel-cli.exe` and the game DLL (`mhfo-hd.dll`).


---

## 🚨 Disclaimer

This project is for **educational purposes** and **preservation** of a discontinued game. All rights to Monster Hunter Frontier Z belong to CAPCOM Co., Ltd. This launcher does not contain any game assets or copyrighted material.

**Use at your own risk.** We are not affiliated with CAPCOM.

---

## 🎯 Project Status

**Current Version**: 1.4.5-beta (Linux Wine Integration)  
**Last Updated**: December 12, 2025  
**Maintainer**: [@mrsasy89](https://github.com/mrsasy89)

### Recent Milestones 🎉

- ✅ **December 15, 2025**: Friends list fixing
- ✅ **December 14, 2025**: mhf-iel integration
- ✅ **December 11, 2025**: Wine launcher successfully tested on Arch Linux
- ✅ **December 11, 2025**: Game confirmed playable via Wine 10.20
- ✅ **December 11, 2025**: DXVK integration verified working

### Next Milestone

**v1.5.0 - Multi-distro testing (Ubuntu, Fedora, Debian, SteamOS)**
  
**ETA**: ~1 week

Goals:

- [ ] Multi-distro testing (Ubuntu, Fedora, Debian, SteamOS)

---

⭐ **Star this project** if you find it useful!  
🐛 **Report bugs** via [Issues](https://github.com/mrsasy89/MHFZ-Launcher/issues)  

**Happy Hunting! 🎮🔥**
