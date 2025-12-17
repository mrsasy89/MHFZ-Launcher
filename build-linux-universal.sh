#!/bin/bash
# build-linux-universal.sh - Build universale per distro Linux

set -e

VERSION="1.4.6"
BUILD_NAME="MHFZ-Launcher-Linux-v${VERSION}"

echo "=========================================="
echo "🐧 Building MHFZ-Launcher for Linux"
echo "   Version: ${VERSION}"
echo "   Target: Universal (all distros)"
echo "=========================================="
echo ""

# 1. Verifica dipendenze build
echo "📋 Step 1/6: Checking build dependencies..."
command -v node >/dev/null 2>&1 || { echo "❌ Node.js required"; exit 1; }
command -v npm >/dev/null 2>&1 || { echo "❌ npm required"; exit 1; }
command -v cargo >/dev/null 2>&1 || { echo "❌ Rust/Cargo required"; exit 1; }
echo "✅ Node.js $(node --version)"
echo "✅ npm $(npm --version)"
echo "✅ Rust $(rustc --version | cut -d' ' -f2)"
echo ""

# 2. Pulisci build precedenti
echo "🧹 Step 2/6: Cleaning previous builds..."
rm -rf src-tauri/target/release/bundle/
rm -rf dist/
rm -rf "$BUILD_NAME"
rm -f "${BUILD_NAME}.tar.gz"
echo "✅ Clean complete"
echo ""

# 3. Build frontend (Vue.js)
echo "📦 Step 3/6: Building frontend..."
npm install --silent
npm run build
echo "✅ Frontend built ($(du -sh dist/ | cut -f1))"
echo ""

# 4. Build backend (Rust/Tauri)
echo "🦀 Step 4/6: Building Rust binary..."
cd src-tauri
cargo build --release --features custom-protocol
cd ..
echo "✅ Binary built ($(du -sh src-tauri/target/release/app | cut -f1))"
echo ""

# 5. Crea package distribuzione
echo "📁 Step 5/6: Creating distribution package..."
mkdir -p "$BUILD_NAME"

# Copia binario
cp src-tauri/target/release/app "$BUILD_NAME/mhfz-launcher"
chmod +x "$BUILD_NAME/mhfz-launcher"

# Copia assets frontend (TUTTO IL CONTENUTO di dist/)
echo "   Copying frontend assets..."
if [ ! -d "dist" ]; then
    echo "❌ Error: dist/ directory not found!"
    echo "   Run 'npm run build' first"
    exit 1
fi

# Copia tutto da dist/ direttamente nella root del package
cp -r dist/* "$BUILD_NAME/"

# Verifica che i file essenziali siano stati copiati
if [ ! -f "$BUILD_NAME/index.html" ]; then
    echo "❌ Error: index.html not found in package!"
    exit 1
fi

echo "   ✅ Assets copied:"
echo "      - index.html: $(du -sh "$BUILD_NAME/index.html" | cut -f1)"
if [ -d "$BUILD_NAME/assets" ]; then
    echo "      - assets/: $(du -sh "$BUILD_NAME/assets" | cut -f1)"
    # Lista fonts se presenti
    if [ -d "$BUILD_NAME/assets/fonts" ]; then
        FONT_COUNT=$(find "$BUILD_NAME/assets/fonts" -type f | wc -l)
        echo "      - fonts: $FONT_COUNT files"
    fi
fi

# === LAUNCHER SCRIPT ===
cat > "$BUILD_NAME/launch.sh" << 'LAUNCHER'
#!/bin/bash
# MHFZ-Launcher - Linux Universal Edition

# === Ottieni directory dello script ===
SCRIPT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)

# === Setup PATH per trovare wine ===
export PATH="/usr/bin:/usr/local/bin:/bin:/sbin:$HOME/.local/bin:$PATH"
export DISPLAY="${DISPLAY:-:0}"

# === Configurazione Wine ===
export WINEPREFIX="${WINEPREFIX:-$HOME/.local/share/MHFZ/pfx}"
export WINEDEBUG="${WINEDEBUG:--all}"

# Trova wine
WINE_CMD=$(command -v wine 2>/dev/null || echo "/usr/bin/wine")

# === Assicurati che Wine sia disponibile ===
if [ ! -x "$WINE_CMD" ] && ! command -v wine &>/dev/null; then
    # Mostra errore grafico se lanciato da desktop
    if [ -z "$TERM" ] || [ "$TERM" = "dumb" ]; then
        zenity --error --title="MHFZ Launcher Error" \
               --text="Wine not found!\n\nPlease install Wine:\n  sudo pacman -S wine (Arch)\n  sudo apt install wine (Ubuntu)" 2>/dev/null || \
        notify-send -u critical "MHFZ Launcher" "Wine not found! Install Wine to continue."
    else
        echo "❌ ERROR: Wine not found in PATH!"
        echo ""
        echo "Install Wine:"
        echo "  Arch: sudo pacman -S wine"
        echo "  Ubuntu: sudo apt install wine-stable"
    fi
    exit 1
fi


# === Configurazione Wine ===
export WINEPREFIX="${WINEPREFIX:-\$HOME/.local/share/MHFZ/pfx}"

# === Ottimizzazioni WebKitGTK ===
export WEBKIT_DISABLE_COMPOSITING_MODE=1
export FONTCONFIG_PATH="/etc/fonts"
export FONTCONFIG_FILE="/etc/fonts/fonts.conf"
export XDG_DATA_DIRS="/usr/share:/usr/local/share:$HOME/.local/share"

# === Assicurati che Wine sia disponibile ===
if ! command -v wine &>/dev/null; then
    # Mostra errore grafico se lanciato da desktop
    if [ -z "$TERM" ] || [ "$TERM" = "dumb" ]; then
        zenity --error --title="MHFZ Launcher Error" \
               --text="Wine not found!\n\nPlease install Wine:\n  sudo pacman -S wine (Arch)\n  sudo apt install wine (Ubuntu)" 2>/dev/null || \
        notify-send -u critical "MHFZ Launcher" "Wine not found! Install Wine to continue."
    else
        echo "❌ ERROR: Wine not found in PATH!"
        echo ""
        echo "Install Wine:"
        echo "  Arch: sudo pacman -S wine"
        echo "  Ubuntu: sudo apt install wine-stable"
    fi
    exit 1
fi

# === Log per debug (opzionale) ===
LOG_FILE="$HOME/.local/share/MHFZ/launcher.log"
mkdir -p "$(dirname "$LOG_FILE")"

{
    echo "=== MHFZ Launcher Started at $(date) ==="
    echo "SCRIPT_DIR: $SCRIPT_DIR"
    echo "WINEPREFIX: $WINEPREFIX"
    echo "WINE: $(which wine)"
    echo "WINE VERSION: $(wine --version 2>&1)"
    echo "PATH: $PATH"
    echo "=========================================="
} >> "$LOG_FILE"

# === Lancia applicazione ===
cd "$SCRIPT_DIR" || exit 1

# Esegui il binario e logga stdout/stderr
exec ./mhfz-launcher "$@" 2>&1 | tee -a "$LOG_FILE"

LAUNCHER

chmod +x "$BUILD_NAME/launch.sh"

# === WINE WRAPPER SCRIPT (per font) ===
cat > "$BUILD_NAME/wine-wrapper.sh" << 'WINEWRAP'
#!/bin/bash
# Wine wrapper con font environment

# Setup font rendering
export FONTCONFIG_PATH="/etc/fonts"
export FONTCONFIG_FILE="/etc/fonts/fonts.conf"
export XDG_DATA_DIRS="/usr/share:/usr/local/share:$HOME/.local/share"

# Lancia Wine con tutti gli argomenti passati
exec wine "$@"
WINEWRAP

chmod +x "$BUILD_NAME/wine-wrapper.sh"

# === CHECK DEPENDENCIES SCRIPT ===
cat > "$BUILD_NAME/check-dependencies.sh" << 'CHECKDEPS'
#!/bin/bash
# Verifica dipendenze sistema

echo "🔍 Checking MHFZ-Launcher dependencies..."
echo ""

DEPS_OK=true

# Librerie richieste
LIBS=(
    "libwebkit2gtk-4.0.so.37"
    "libwebkit2gtk-4.1.so.0"
    "libgtk-3.so.0"
    "libglib-2.0.so.0"
    "libssl.so.3"
    "libssl.so.1.1"
)

for lib in "${LIBS[@]}"; do
    if ldconfig -p 2>/dev/null | grep -q "$lib"; then
        echo "✅ $lib"
    else
        echo "❌ $lib (missing)"
        DEPS_OK=false
    fi
done

echo ""

if [ "$DEPS_OK" = true ]; then
    echo "✅ All dependencies satisfied!"
    echo ""
    echo "🚀 Run launcher:"
    echo "   ./launch.sh"
    exit 0
fi

echo "⚠️  Some dependencies are missing!"
echo ""
echo "📦 Installation commands:"
echo ""

# Rileva distro
if [ -f /etc/os-release ]; then
    . /etc/os-release

    case "$ID" in
        arch|manjaro|endeavouros)
            echo "Arch Linux / Manjaro:"
            echo "  sudo pacman -S webkit2gtk gtk3 wine"
            ;;
        ubuntu|debian|pop|linuxmint)
            echo "Ubuntu / Debian:"
            echo "  sudo apt update"
            echo "  sudo apt install libwebkit2gtk-4.0-37 libgtk-3-0 wine-stable"
            ;;
        fedora|rhel|centos|rocky)
            echo "Fedora / RHEL:"
            echo "  sudo dnf install webkit2gtk3 gtk3 wine"
            ;;
        opensuse*|suse*)
            echo "openSUSE:"
            echo "  sudo zypper install webkit2gtk3 gtk3 wine"
            ;;
        *)
            echo "Other Linux:"
            echo "  Install: webkit2gtk, gtk3, wine"
            ;;
    esac
else
    echo "Install: webkit2gtk, gtk3, wine via your package manager"
fi

exit 1
CHECKDEPS

chmod +x "$BUILD_NAME/check-dependencies.sh"

# === INSTALL SCRIPT (optional) ===
cat > "$BUILD_NAME/install.sh" << 'INSTALL'
#!/bin/bash
# Installazione in ~/.local/bin (opzionale)

SCRIPT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
INSTALL_DIR="$HOME/.local/share/mhfz-launcher"
BIN_DIR="$HOME/.local/bin"

echo "📦 Installing MHFZ-Launcher..."
echo ""

# Crea directory
mkdir -p "$INSTALL_DIR"
mkdir -p "$BIN_DIR"

# Copia files
cp -r "$SCRIPT_DIR"/* "$INSTALL_DIR/"

# Crea symlink in PATH
ln -sf "$INSTALL_DIR/launch.sh" "$BIN_DIR/mhfz-launcher"

# Crea .desktop file
mkdir -p "$HOME/.local/share/applications"
cat > "$HOME/.local/share/applications/mhfz-launcher.desktop" << DESKTOP
[Desktop Entry]
Name=MHF-Z Launcher
Comment=Monster Hunter Frontier Z Launcher
Exec=$INSTALL_DIR/launch.sh
Path=$INSTALL_DIR
Icon=$INSTALL_DIR/icon.png
Terminal=false
Type=Application
Categories=Game;
DESKTOP

echo "✅ Installation complete!"
echo ""
echo "🚀 Launch from terminal:"
echo "   mhfz-launcher"
echo ""
echo "Or search 'MHF-Z Launcher' in application menu"
INSTALL

chmod +x "$BUILD_NAME/install.sh"

# === README ===
cat > "$BUILD_NAME/README.md" << 'README'
# MHFZ-Launcher - Linux Edition

Cross-platform launcher for Monster Hunter Frontier Z with Wine integration.

## System Requirements

- Linux kernel 5.4+
- WebKitGTK 4.0 or 4.1
- GTK3
- Wine 8.0+ (for game launch)
- 100MB disk space

## Compatible Distributions

✅ **Tested:**
- Arch Linux / Manjaro / EndeavourOS
- Ubuntu 22.04+ / Debian 12+
- Fedora 38+
- Pop!_OS 22.04+
- Linux Mint 21+

⚠️ **May work (untested):**
- openSUSE Tumbleweed
- Gentoo (with webkit2gtk)

## Quick Start

### 1. Extract Archive

tar -xzf MHFZ-Launcher-Linux-v*.tar.gz
cd MHFZ-Launcher-Linux-v*/


### 2. Check Dependencies

./check-dependencies.sh


If missing packages, follow installation commands shown.

### 3. Run Launcher

./launch.sh


### 4. (Optional) System Installation


./install.sh


This installs to `~/.local/share/mhfz-launcher` and adds to application menu.

## Wine Configuration

First run requires Wine prefix setup:

export WINEPREFIX="$HOME/.local/share/MHFZ/pfx"
WINEARCH=win32 wineboot --init


Recommended: Install DXVK for better performance:

Arch
sudo pacman -S wine-mono wine-gecko dxvk

Ubuntu
sudo apt install wine-mono wine-gecko

DXVK: https://github.com/doitsujin/dxvk/releases


## Game Files Setup

1. Download Monster Hunter Frontier Z game files
2. Place in `~/Games/MHFZ/` (or custom location)
3. Download `mhf-iel-cli.exe` from [mhf-iel releases](https://github.com/rockisch/mhf-iel)
4. Place in game folder
5. Configure game folder path in launcher settings

## Troubleshooting

### Launcher won't start

Check dependencies
./check-dependencies.sh

Test binary directly
./mhfz-launcher

Check logs
journalctl --user -xe


### Wine not working

Verify Wine installation
wine --version

Test Wine prefix
WINEPREFIX=~/.local/share/MHFZ/pfx wine cmd /c echo OK


### Missing libraries error

Install webkit2gtk and gtk3 for your distribution (see check-dependencies.sh).

## Support

- GitHub Issues: https://github.com/mrsasy89/MHFZ-Launcher/issues
- Discord: [Monster Hunter Old Gen](https://discord.gg/UdQ4cy5TbU)

## License

GPL v3.0 - See LICENSE file
README

# Placeholder icon (se non esiste)
if [ -f "src-tauri/icons/128x128.png" ]; then
    cp "src-tauri/icons/128x128.png" "$BUILD_NAME/icon.png"
else
    echo "⚠️  WARNING: Icon not found at src-tauri/icons/128x128.png"
    echo "            Creating empty placeholder file."
    echo "            Add a valid icon or build may appear without icon!"
    echo ""
    touch "$BUILD_NAME/icon.png"
fi


echo "✅ Package created: $BUILD_NAME/"
echo ""

# 6. Crea tarball
echo "📦 Step 6/6: Creating archive..."
tar -czf "${BUILD_NAME}.tar.gz" "$BUILD_NAME"
ARCHIVE_SIZE=$(du -sh "${BUILD_NAME}.tar.gz" | cut -f1)
echo "✅ Archive created: ${BUILD_NAME}.tar.gz ($ARCHIVE_SIZE)"
echo ""

# Summary
echo "=========================================="
echo "✅ Build Complete!"
echo "=========================================="
echo ""
echo "📦 Distribution package:"
echo "   ./$BUILD_NAME/"
echo ""
echo "📦 Archive (for distribution):"
echo "   ./${BUILD_NAME}.tar.gz ($ARCHIVE_SIZE)"
echo ""
echo "📄 Package contents:"
ls -1 "$BUILD_NAME/" | sed 's/^/   /'
echo ""
echo "🐧 Compatibility:"
echo "   ✅ Arch Linux / Manjaro"
echo "   ✅ Ubuntu / Debian"
echo "   ✅ Fedora / RHEL"
echo "   ✅ Pop!_OS / Linux Mint"
echo ""
echo "📋 User installation:"
echo "   tar -xzf ${BUILD_NAME}.tar.gz"
echo "   cd $BUILD_NAME"
echo "   ./check-dependencies.sh"
echo "   ./launch.sh"
echo ""
echo "🚀 Or system-wide install:"
echo "   cd $BUILD_NAME && ./install.sh"
echo ""
