#!/bin/bash
# check-dependencies.sh - Verifica dipendenze su distro target

echo "🔍 Checking MHFZ-Launcher dependencies..."
echo ""

# Controlla librerie richieste
LIBS=(
    "libwebkit2gtk-4.0.so.37"
    "libwebkit2gtk-4.1.so.0"
    "libgtk-3.so.0"
    "libglib-2.0.so.0"
    "libssl.so.3"
    "libssl.so.1.1"
)

FOUND=0
MISSING=0

# ✅ Check essential fonts
echo ""
echo "Checking fonts..."
FONTS_OK=true

if ! fc-list | grep -iq "liberation\|dejavu"; then
    echo "⚠️  Liberation/DejaVu fonts missing"
    FONTS_OK=false
fi

for lib in "${LIBS[@]}"; do
    if ldconfig -p | grep -q "$lib"; then
        echo "✅ $lib"
        FOUND=$((FOUND + 1))
    else
        echo "❌ $lib (missing)"
        MISSING=$((MISSING + 1))
    fi
done

echo ""
if [ $MISSING -eq 0 ]; then
    echo "✅ All dependencies satisfied!"
    echo "   You can run: ./launch.sh"
else
    echo "⚠️  $MISSING dependencies missing"
    echo ""
    echo "📦 Install missing libraries:"

    # Rileva distro
    if [ -f /etc/os-release ]; then
        . /etc/os-release
        case "$ID" in
            arch|manjaro|endeavouros)
                echo "   sudo pacman -S webkit2gtk gtk3 wine ttf-liberation ttf-dejavu"
                ;;
            ubuntu|debian|pop|linuxmint)
                echo "   sudo apt install libwebkit2gtk-4.0-37 libgtk-3-0 wine-stable fonts-liberation fonts-dejavu"
                ;;
            fedora|rhel|centos)
                echo "   sudo dnf install webkit2gtk3 gtk3 wine liberation-sans-fonts dejavu-sans-fonts"
                ;;
            opensuse*)
                echo "   sudo zypper install webkit2gtk3 gtk3 wine liberation-fonts dejavu-fonts"
                ;;
            *)
                echo "   Check your distro package manager for 'webkit2gtk'"
                ;;
        esac
    fi
fi

echo ""
