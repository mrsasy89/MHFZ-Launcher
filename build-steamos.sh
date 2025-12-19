#!/bin/bash
set -e

VERSION="1.4.7"
BUILD_NAME="MHFZ-Launcher-SteamOS-v${VERSION}"
APPIMAGE_NAME="mhfz.AppImage"

echo "=================================================="
echo "   MHFZ-Launcher - SteamOS/Linux Build"
echo "   Version: ${VERSION}"
echo "   Target: AppImage - Hidden Terminal"
echo "=================================================="

# 1. Verifica dipendenze
echo "🔍 [1/5] Checking dependencies..."
command -v node >/dev/null 2>&1 || { echo "❌ Node.js required"; exit 1; }
command -v npm >/dev/null 2>&1 || { echo "❌ npm required"; exit 1; }
command -v cargo >/dev/null 2>&1 || { echo "❌ Rust/Cargo required"; exit 1; }
command -v appimagetool >/dev/null 2>&1 || {
    echo "⚠️  appimagetool not found. Installing..."
    wget https://github.com/AppImage/AppImageKit/releases/download/continuous/appimagetool-x86_64.AppImage -O /tmp/appimagetool
    chmod +x /tmp/appimagetool
    APPIMAGETOOL="/tmp/appimagetool"
}
[ -z "$APPIMAGETOOL" ] && APPIMAGETOOL="appimagetool"

echo "✅ Dependencies OK"

# 2. Clean
echo "🧹 [2/5] Cleaning previous builds..."
rm -rf src-tauri/target/release/bundle
rm -rf dist
rm -rf AppDir
rm -f "${APPIMAGE_NAME}"
echo "✅ Clean complete"

# 3. Build frontend
echo "🎨 [3/5] Building frontend..."
npm install --silent
npm run build
echo "✅ Frontend built ($(du -sh dist 2>/dev/null | cut -f1))"

# 4. Build Rust binary
echo "⚙️  [4/5] Building Rust binary with embedded frontend..."
cd src-tauri
cargo build --release --features custom-protocol
cd ..

BINARY_PATH="src-tauri/target/release/app"

if [ ! -f "$BINARY_PATH" ]; then
    echo "❌ ERROR: Binary not found at $BINARY_PATH"
    exit 1
fi

echo "✅ Binary built: $(basename $BINARY_PATH) ($(du -sh $BINARY_PATH | cut -f1))"

# 5. Create AppImage
echo "📦 [5/5] Creating AppImage..."
mkdir -p AppDir/usr/bin
mkdir -p AppDir/usr/share/applications
mkdir -p AppDir/usr/share/icons/hicolor/128x128/apps

# Crea wrapper script principale
cat > AppDir/usr/bin/mhfz-launcher << 'WRAPPER_EOF'
#!/bin/bash
# 🎮 MHFZ-Launcher - Font Fix Wrapper

export FONTCONFIG_PATH="/etc/fonts"
export FONTCONFIG_FILE="/etc/fonts/fonts.conf"

if [ -z "$XDG_DATA_DIRS" ]; then
    export XDG_DATA_DIRS="/usr/share:/usr/local/share"
fi

if [ -d "$HOME/.cache/fontconfig" ] && [ -w "$HOME/.cache/fontconfig" ]; then
    rm -rf "$HOME/.cache/fontconfig" 2>/dev/null || true
fi

export GTK_THEME="${GTK_THEME:-Adwaita}"
export WEBKIT_DISABLE_COMPOSITING_MODE=1
export WEBKIT_DISABLE_DMABUF_RENDERER=1
export PANGO_RC_FILE=/etc/pango/pangorc

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
exec "$SCRIPT_DIR/mhfz-launcher.bin" "$@"
WRAPPER_EOF

chmod +x AppDir/usr/bin/mhfz-launcher

# Copia il binario
cp "$BINARY_PATH" AppDir/usr/bin/mhfz-launcher.bin
chmod +x AppDir/usr/bin/mhfz-launcher.bin

# ✅ CRITICAL: Crea wrapper per terminale nascosto
cat > AppDir/usr/bin/mhfz-terminal-wrapper << 'TERM_WRAPPER'
#!/bin/bash
# Lancia in un terminale virtuale ma completamente nascosto

# Usa setsid per creare una nuova sessione + nohup per detach completo
nohup setsid "${0%/*}/mhfz-launcher" </dev/null >/dev/null 2>&1 &

# Esci immediatamente così il terminale si chiude subito
exit 0
TERM_WRAPPER

chmod +x AppDir/usr/bin/mhfz-terminal-wrapper

echo "✅ Binary packaged with terminal wrapper"

# Copia icona
if [ -f src-tauri/icons/128x128.png ]; then
    cp src-tauri/icons/128x128.png AppDir/usr/share/icons/hicolor/128x128/apps/mhfz-launcher.png
    cp src-tauri/icons/128x128.png AppDir/mhfz-launcher.png
    echo "✅ Icon packaged"
else
    echo "⚠️  Creating placeholder icon"
    echo -n 'iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNk+M9QDwADhgGAWjR9awAAAABJRU5ErkJggg==' | base64 -d > AppDir/mhfz-launcher.png
fi

# Crea .desktop file - USA IL WRAPPER
cat > AppDir/mhfz-launcher.desktop << 'EOF'
[Desktop Entry]
Type=Application
Name=MHFZ Launcher
Comment=Monster Hunter Frontier Z Launcher
Exec=mhfz-terminal-wrapper
Icon=mhfz-launcher
Terminal=true
Categories=Game;
StartupNotify=false
X-KDE-SubstituteUID=false
EOF

# Crea AppRun - ✅ FORZA TERM=xterm
cat > AppDir/AppRun << 'EOF'
#!/bin/bash
SELF="$(readlink -f "${0}")"
HERE="${SELF%/*}"

# ✅ CRITICAL FIX: Forza TERM per simulare ambiente terminale
export TERM=xterm
export FONTCONFIG_PATH="/etc/fonts"
export FONTCONFIG_FILE="/etc/fonts/fonts.conf"

if [ -z "$XDG_DATA_DIRS" ]; then
    export XDG_DATA_DIRS="/usr/share:/usr/local/share"
fi

if [ -d "$HOME/.cache/fontconfig" ] && [ -w "$HOME/.cache/fontconfig" ]; then
    rm -rf "$HOME/.cache/fontconfig" 2>/dev/null || true
fi

export GTK_THEME="${GTK_THEME:-Adwaita}"
export WEBKIT_DISABLE_COMPOSITING_MODE=1
export WEBKIT_DISABLE_DMABUF_RENDERER=1
export PANGO_RC_FILE=/etc/pango/pangorc

export PATH="${HERE}/usr/bin:${PATH}"
export LD_LIBRARY_PATH="${HERE}/usr/lib:${LD_LIBRARY_PATH}"

exec "${HERE}/usr/bin/mhfz-terminal-wrapper" "$@"
EOF
chmod +x AppDir/AppRun


# Genera AppImage
echo "🔨 Generating AppImage..."
$APPIMAGETOOL AppDir "${APPIMAGE_NAME}"

# Cleanup
rm -rf AppDir

echo ""
echo "=================================================="
echo "   ✅ BUILD COMPLETE!"
echo "=================================================="
echo "📦 AppImage: ./${APPIMAGE_NAME}"
echo "📏 Size: $(du -h "${APPIMAGE_NAME}" | cut -f1)"
echo "=================================================="
