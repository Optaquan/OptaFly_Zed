#!/bin/bash
# OptaFly_Zed Phase 2.5 - Parallel Installation Script
# Creates isolated Phase 2.5 installation alongside existing Zed

set -e

SRC_DIR="/home/ty/Documents/3-Tumquan/1_The NOW/2_Technology/OptaFly_Zed"
DEST_DIR="/home/ty/Documents/3-Tumquan/1_The NOW/2_Technology/OptaFly_Zed_Phase2.5"

echo "╔════════════════════════════════════════════════════════════╗"
echo "║  OptaFly_Zed Phase 2.5 - Parallel Installation            ║"
echo "║  ML Foundation + Structurizr JNI Integration               ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""

# Check if source exists
if [ ! -d "$SRC_DIR" ]; then
    echo "❌ Error: Source directory not found: $SRC_DIR"
    exit 1
fi

# Step 1: Copy source
echo "📦 Step 1/5: Copying source code..."
if [ -d "$DEST_DIR" ]; then
    echo "   Directory already exists. Updating..."
    cd "$DEST_DIR"
    git pull 2>/dev/null || echo "   (Not a git repo, using existing files)"
else
    echo "   Creating fresh copy..."
    cp -r "$SRC_DIR" "$DEST_DIR"
    cd "$DEST_DIR"
fi

# Step 2: Build binaries
echo ""
echo "🔨 Step 2/5: Building binaries (this will take 15-30 min)..."
echo "   Building main editor..."
cargo build --release 2>&1 | grep -E "Compiling|Finished|error" || true

echo "   Building OptaCore libraries..."
cargo build --release --package optacore_struct 2>&1 | grep -E "Compiling|Finished|error" || true
cargo build --release --package optacore_jni 2>&1 | grep -E "Compiling|Finished|error" || true

# Verify binaries
if [ ! -f "target/release/zed" ]; then
    echo "❌ Error: Build failed - zed binary not found"
    exit 1
fi

echo "   ✅ Built: $(ls -lh target/release/zed | awk '{print $9, $5}')"
echo "   ✅ Built: $(ls -lh target/release/liboptacore_jni.so | awk '{print $9, $5}')"

# Step 3: Create launch wrapper
echo ""
echo "🚀 Step 3/5: Creating launch wrapper..."
cat > target/release/optafly-phase2.5 << 'WRAPPER'
#!/bin/bash
# OptaFly Phase 2.5 Launcher - Isolated from standard Zed

# Isolation: Use separate directories
export ZED_CONFIG_DIR="$HOME/.config/optafly-phase2.5"
export ZED_DATA_DIR="$HOME/.local/share/optafly-phase2.5"
export ZED_SOCKET_NAME="optafly-phase2.5.sock"

# Differentiation: Custom window title
export ZED_WINDOW_TITLE="OptaFly Phase 2.5"

# Conflict avoidance: Different Widget-Log port
export WIDGET_LOG_PORT=8444

# Initialize directories on first run
if [ ! -d "$ZED_CONFIG_DIR" ]; then
    mkdir -p "$ZED_CONFIG_DIR"
    echo "First run: Created $ZED_CONFIG_DIR"
    
    # Create Phase 2.5 marker
    cat > "$ZED_CONFIG_DIR/settings.json" << 'SETTINGS'
{
  "// OptaFly_Zed": "Phase 2.5 Edition",
  "// Version": "2.5 - ML Foundation + Structurizr",
  "// Features": [
    "OptaCore tensor-native architecture",
    "Structurizr JNI bridge",
    "Telemetry for ML training",
    "C4 visualization"
  ],
  "// Build": "2026-01-09"
}
SETTINGS
fi

mkdir -p "$ZED_DATA_DIR"

# Launch
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
exec "$SCRIPT_DIR/zed" "$@"
WRAPPER

chmod +x target/release/optafly-phase2.5
echo "   ✅ Created: $(pwd)/target/release/optafly-phase2.5"

# Step 4: Create icon (with ImageMagick if available)
echo ""
echo "🎨 Step 4/5: Creating custom icon..."
mkdir -p resources/icons

if command -v convert &> /dev/null; then
    # Find source icon
    if [ -f "$HOME/.local/zed.app/share/icons/hicolor/512x512/apps/zed.png" ]; then
        SRC_ICON="$HOME/.local/zed.app/share/icons/hicolor/512x512/apps/zed.png"
    elif [ -f "/usr/share/icons/hicolor/512x512/apps/zed.png" ]; then
        SRC_ICON="/usr/share/icons/hicolor/512x512/apps/zed.png"
    else
        SRC_ICON=""
    fi
    
    if [ -n "$SRC_ICON" ]; then
        # Add "2.5" badge to differentiate
        convert "$SRC_ICON" \
            -gravity SouthEast \
            -pointsize 120 \
            -fill '#00FF00' \
            -stroke '#000000' \
            -strokewidth 4 \
            -font 'DejaVu-Sans-Bold' \
            -annotate +20+20 '2.5' \
            resources/icons/optafly-phase2.5.png
        echo "   ✅ Created badged icon with '2.5' overlay"
    else
        echo "   ⚠️  Zed icon not found, using generic"
        touch resources/icons/optafly-phase2.5.png
    fi
else
    echo "   ⚠️  ImageMagick not found, using generic icon"
    echo "   Install with: sudo apt install imagemagick"
    touch resources/icons/optafly-phase2.5.png
fi

# Step 5: Create desktop entry
echo ""
echo "📝 Step 5/5: Creating desktop entry..."

ICON_PATH="$DEST_DIR/resources/icons/optafly-phase2.5.png"
if [ ! -f "$ICON_PATH" ] || [ ! -s "$ICON_PATH" ]; then
    # Fallback to standard zed icon
    ICON_PATH="zed"
fi

cat > "$HOME/.local/share/applications/optafly-zed-phase2.5.desktop" << DESKTOP
[Desktop Entry]
Version=1.0
Type=Application
Name=OptaFly Zed (Phase 2.5)
GenericName=Architecture-Enhanced Code Editor
Comment=OptaFly_Zed with OptaCore ML Foundation & Structurizr Integration
TryExec=$DEST_DIR/target/release/optafly-phase2.5
StartupNotify=true
Exec=$DEST_DIR/target/release/optafly-phase2.5 %U
Icon=$ICON_PATH
Terminal=false
Categories=Utility;TextEditor;Development;IDE;
Keywords=zed;optafly;architecture;optacore;phase2.5;ml;structurizr;
MimeType=text/plain;application/x-zerosize;
StartupWMClass=optafly-zed-phase2.5
X-GNOME-FullName=OptaFly Zed Phase 2.5 Edition
Actions=NewWorkspace;

[Desktop Action NewWorkspace]
Exec=$DEST_DIR/target/release/optafly-phase2.5 --new %U
Name=Open New Workspace (Phase 2.5)
DESKTOP

# Refresh application database
if command -v update-desktop-database &> /dev/null; then
    update-desktop-database "$HOME/.local/share/applications" 2>/dev/null || true
fi

echo "   ✅ Installed desktop entry"

# Summary
echo ""
echo "╔════════════════════════════════════════════════════════════╗"
echo "║  ✅ Installation Complete!                                 ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""
echo "📂 Installation Directory:"
echo "   $DEST_DIR"
echo ""
echo "🚀 Launch Options:"
echo "   1. From terminal:"
echo "      $DEST_DIR/target/release/optafly-phase2.5"
echo ""
echo "   2. From application menu:"
echo "      Search for 'OptaFly Zed (Phase 2.5)'"
echo ""
echo "⚙️  Configuration & Data:"
echo "   Config: ~/.config/optafly-phase2.5/"
echo "   Data:   ~/.local/share/optafly-phase2.5/"
echo "   Socket: optafly-phase2.5.sock"
echo "   Port:   8444 (Widget-Log)"
echo ""
echo "🎯 Differentiation:"
echo "   - Window title: 'OptaFly Phase 2.5'"
echo "   - Dock icon: Badge with '2.5' (if ImageMagick installed)"
echo "   - Menu entry: 'OptaFly Zed (Phase 2.5)'"
echo "   - Separate data/config directories"
echo ""
echo "🔄 To uninstall:"
echo "   rm ~/.local/share/applications/optafly-zed-phase2.5.desktop"
echo "   rm -rf ~/.config/optafly-phase2.5"
echo "   rm -rf ~/.local/share/optafly-phase2.5"
echo "   rm -rf '$DEST_DIR'"
echo ""
echo "✨ Phase 2.5 Features:"
echo "   - OptaCore tensor-native architecture engine"
echo "   - Structurizr JNI bridge for Java integration"
echo "   - Telemetry infrastructure for ML training"
echo "   - Production-grade C4 visualization"
echo ""
