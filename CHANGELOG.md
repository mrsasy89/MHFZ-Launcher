# Changelog

All notable changes to MHFZ-Launcher will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Friends list data reception (UI pending)

## [1.0.0-beta] - 2025-12-12

### 🎉 Major Milestone: Linux Support

#### Added
- **mhf-iel integration** for game launch bypass
- **config.json generation** with 25+ fields
- **Wine/DXVK auto-detection** and prefix management
- **Thread-safe configuration** using OnceLock (Rust 1.70+)
- **lib_linux.rs** module for Linux-specific functionality
- **Comprehensive README** with Linux setup guide
- **Video demo** of launcher and game startup
- **Screenshot** of launcher UI

#### Changed
- Refactored `settings.rs` to use `configparser` (removed Windows-only deps)
- Updated `config.rs` with Avalanche server endpoints
- Modified `main.rs` with Linux launch pipeline

#### Technical Details
- Wine prefix: `<game_folder>/pfx`
- DXVK HUD integration
- Proton fallback support
- Character/auth data pipeline complete

#### Tested On
- Arch Linux + Wine 9.21 + DXVK 2.4
- Avalanche server (avalanchemhfz.ddns.net:53310)
- Character: Kyuseishu (HR 999, GR 110)
- Status: ✅ Fully functional

#### Known Issues
- Friends list mapping needs UI refinement (data received correctly)
- MezFes stall display formatting

---

## [0.1.0] - 2025-12-11

### Added
- Initial Linux port
- Cross-platform INI parsing
- Avalanche server configuration
- Basic Wine integration

---

**Legend**:
- `Added` for new features
- `Changed` for changes in existing functionality
- `Fixed` for bug fixes
- `Removed` for removed features
- `Security` for security fixes
