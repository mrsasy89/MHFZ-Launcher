#!/bin/bash
set -e

VERSION="1.4.7"
BUILD_NAME="MHFZ-Launcher-v${VERSION}"
APPIMAGE_NAME="mhfz.AppImage"

echo "=================================================="
echo "   MHFZ-Launcher - SteamOS/Linux Build"
echo "   Version: ${VERSION}"
echo "   Target: AppImage"
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

# ✅ Wrapper principale - Preserva ambiente + forza fontconfig
cat > AppDir/usr/bin/mhfz-launcher << 'WRAPPER_EOF'
#!/bin/bash
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Preserva ambiente ma forza fontconfig + X11
export FONTCONFIG_PATH="${FONTCONFIG_PATH:-/etc/fonts}"
export FONTCONFIG_FILE="${FONTCONFIG_FILE:-/etc/fonts/fonts.conf}"
export XDG_DATA_DIRS="${XDG_DATA_DIRS:-/usr/share:/usr/local/share}"
export XAUTHORITY="${XAUTHORITY:-$HOME/.Xauthority}"
export WEBKIT_DISABLE_COMPOSITING_MODE=1
export WEBKIT_DISABLE_DMABUF_RENDERER=1

exec "$SCRIPT_DIR/mhfz-launcher.bin" "$@"
WRAPPER_EOF

chmod +x AppDir/usr/bin/mhfz-launcher

# Copia il binario
cp "$BINARY_PATH" AppDir/usr/bin/mhfz-launcher.bin
chmod +x AppDir/usr/bin/mhfz-launcher.bin

echo "✅ Binary packaged with wrapper"

# Copia icona
if [ -f src-tauri/icons/128x128.png ]; then
    cp src-tauri/icons/128x128.png AppDir/usr/share/icons/hicolor/128x128/apps/mhfz-launcher.png
    cp src-tauri/icons/128x128.png AppDir/mhfz-launcher.png
    echo "✅ Icon packaged"
else
    echo "⚠️  Creating placeholder icon"
    echo -n 'iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNk+M9QDwADhgGAWjR9awAAAABJRU5ErkJggg==' | base64 -d > AppDir/mhfz-launcher.png
fi

# Crea .desktop file - lancia DIRETTO senza terminale
cat > AppDir/mhfz-launcher.desktop << 'EOF'
[Desktop Entry]
Type=Application
Name=MHFZ Launcher
Comment=Monster Hunter Frontier Z Launcher
Exec=mhfz-launcher
Icon=mhfz-launcher
Terminal=false
Categories=Game;
StartupNotify=true
EOF

# Crea AppRun - lancia diretto
cat > AppDir/AppRun << 'EOF'
#!/bin/bash
SELF="$(readlink -f "${0}")"
HERE="${SELF%/*}"

export PATH="${HERE}/usr/bin:${PATH}"
export LD_LIBRARY_PATH="${HERE}/usr/lib:${LD_LIBRARY_PATH}"

exec "${HERE}/usr/bin/mhfz-launcher" "$@"
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
echo ""
echo "ℹ️  This build:"
echo "   • Launches directly (no terminal)"
echo "   • Creates Wine prefix on first launch"
echo "   • Auto-installs Japanese fonts from fonts/ folder"
echo "=================================================="
