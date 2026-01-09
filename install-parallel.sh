#!/bin/bash
# OptaFly_Zed - Parallel Installation Script for Linux
# Installs a variant alongside existing installation with complete isolation

set -e

# Configuration
VARIANT="${1:-dev}"
SRC_DIR="$(pwd)"
DEST_DIR="${SRC_DIR}_${VARIANT}"
INSTALL_DIR="$HOME/.local/opt/optafly-${VARIANT}"

echo "╔════════════════════════════════════════════════════════════╗"
echo "║  OptaFly_Zed Parallel Installation                        ║"
echo "║  Variant: ${VARIANT}                                       ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""

# Validate source directory
if [ ! -f "$SRC_DIR/Cargo.toml" ]; then
    echo "❌ Error: Not in OptaFly_Zed source directory"
    echo "   Run this script from the OptaFly_Zed root"
    exit 1
fi

# Step 1: Prepare source
echo "📦 Step 1/5: Preparing source code..."
if [ -d "$DEST_DIR" ]; then
    echo "   Variant directory exists: $DEST_DIR"
    echo "   Using existing source (run 'git pull' to update)"
    cd "$DEST_DIR"
else
    echo "   Creating variant copy..."
    cp -r "$SRC_DIR" "$DEST_DIR"
    cd "$DEST_DIR"
    echo "   ✅ Source copied to: $DEST_DIR"
fi

# Step 2: Build
echo ""
echo "🔨 Step 2/5: Building binaries (this will take 15-30 min)..."
echo "   Building main editor..."
cargo build --release 2>&1 | grep -E "Compiling|Finished|error" || true

# Build OptaCore if present
if [ -f "crates/optacore_struct/Cargo.toml" ]; then
    echo "   Building OptaCore libraries..."
    cargo build --release --package optacore_struct 2>&1 | grep -E "Compiling|Finished|error" || true
    cargo build --release --package optacore_jni 2>&1 | grep -E "Compiling|Finished|error" || true
fi

# Verify build
if [ ! -f "target/release/zed" ]; then
    echo "❌ Error: Build failed - zed binary not found"
    exit 1
fi

echo "   ✅ Built: $(ls -lh target/release/zed | awk '{print $9, $5}')"
if [ -f "target/release/liboptacore_jni.so" ]; then
    echo "   ✅ Built: $(ls -lh target/release/liboptacore_jni.so | awk '{print $9, $5}')"
fi

# Step 3: Install
echo ""
echo "📥 Step 3/5: Installing binaries..."
mkdir -p "$INSTALL_DIR/bin"
cp target/release/zed "$INSTALL_DIR/bin/"

# Copy OptaCore libraries if present
if [ -f "target/release/liboptacore_jni.so" ]; then
    mkdir -p "$INSTALL_DIR/lib"
    cp target/release/liboptacore_jni.so "$INSTALL_DIR/lib/"
fi

echo "   ✅ Installed to: $INSTALL_DIR"

# Step 4: Create launch wrapper
echo ""
echo "🚀 Step 4/5: Creating launch wrapper..."
cat > "$INSTALL_DIR/bin/launcher" << 'WRAPPER'
#!/bin/bash
# OptaFly_Zed Variant Launcher - Isolated Configuration

# Isolation: Use separate directories
export ZED_CONFIG_DIR="$HOME/.config/optafly-VARIANT_NAME"
export ZED_DATA_DIR="$HOME/.local/share/optafly-VARIANT_NAME"
export ZED_SOCKET_NAME="optafly-VARIANT_NAME.sock"

# Differentiation: Custom window title (if supported)
export ZED_WINDOW_TITLE="OptaFly (VARIANT_NAME)"

# Port isolation for Widget-Log (if present)
export WIDGET_LOG_PORT=8444

# Initialize directories on first run
if [ ! -d "$ZED_CONFIG_DIR" ]; then
    mkdir -p "$ZED_CONFIG_DIR"

    # Create variant marker
    cat > "$ZED_CONFIG_DIR/settings.json" << 'SETTINGS'
{
  "variant": "VARIANT_NAME",
  "theme": "One Dark"
}
SETTINGS
fi

mkdir -p "$ZED_DATA_DIR"

# Launch
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
exec "$SCRIPT_DIR/zed" "$@"
WRAPPER

# Replace placeholder
sed -i "s/VARIANT_NAME/${VARIANT}/g" "$INSTALL_DIR/bin/launcher"
chmod +x "$INSTALL_DIR/bin/launcher"

echo "   ✅ Created: $INSTALL_DIR/bin/launcher"

# Step 5: Create desktop entry
echo ""
echo "📝 Step 5/5: Creating desktop entry..."

# Try to find or create icon
ICON_PATH="zed"
if command -v convert &> /dev/null; then
    # Find source icon
    for icon_dir in /usr/share/icons/hicolor/512x512/apps /usr/share/pixmaps ~/.local/share/icons; do
        if [ -f "$icon_dir/zed.png" ]; then
            mkdir -p ~/.local/share/icons

            # Create badged icon
            convert "$icon_dir/zed.png" \
                -gravity SouthEast \
                -pointsize 120 \
                -fill '#4CAF50' \
                -stroke '#000000' \
                -strokewidth 4 \
                -font 'DejaVu-Sans-Bold' \
                -annotate +20+20 "${VARIANT^^}" \
                ~/.local/share/icons/optafly-${VARIANT}.png 2>/dev/null || true

            if [ -f ~/.local/share/icons/optafly-${VARIANT}.png ]; then
                ICON_PATH="$HOME/.local/share/icons/optafly-${VARIANT}.png"
                echo "   ✅ Created badged icon with '${VARIANT^^}' overlay"
            fi
            break
        fi
    done
else
    echo "   ⚠️  ImageMagick not found, using generic icon"
    echo "   Install with: sudo apt install imagemagick (Debian/Ubuntu)"
fi

# Create desktop entry
cat > "$HOME/.local/share/applications/optafly-${VARIANT}.desktop" << DESKTOP
[Desktop Entry]
Version=1.0
Type=Application
Name=OptaFly Zed (${VARIANT})
GenericName=Architecture-Enhanced Code Editor
Comment=OptaFly_Zed ${VARIANT} variant
TryExec=$INSTALL_DIR/bin/launcher
StartupNotify=true
Exec=$INSTALL_DIR/bin/launcher %U
Icon=$ICON_PATH
Terminal=false
Categories=Development;IDE;TextEditor;
Keywords=zed;optafly;editor;${VARIANT};
MimeType=text/plain;
StartupWMClass=optafly-zed-${VARIANT}

[Desktop Action NewWorkspace]
Exec=$INSTALL_DIR/bin/launcher --new %U
Name=Open New Workspace
DESKTOP

# Update desktop database
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
echo "   $INSTALL_DIR"
echo ""
echo "🚀 Launch Options:"
echo "   1. From terminal:"
echo "      $INSTALL_DIR/bin/launcher"
echo ""
echo "   2. From application menu:"
echo "      Search for 'OptaFly Zed (${VARIANT})'"
echo ""
echo "⚙️  Configuration & Data:"
echo "   Config: ~/.config/optafly-${VARIANT}/"
echo "   Data:   ~/.local/share/optafly-${VARIANT}/"
echo "   Socket: optafly-${VARIANT}.sock"
echo ""
echo "🎯 Isolation:"
echo "   - Separate config/data directories"
echo "   - Unique socket name prevents conflicts"
echo "   - Independent from other installations"
echo ""
echo "🔄 To uninstall this variant:"
echo "   rm -rf $INSTALL_DIR"
echo "   rm ~/.local/share/applications/optafly-${VARIANT}.desktop"
echo "   rm -rf ~/.config/optafly-${VARIANT}"
echo "   rm -rf ~/.local/share/optafly-${VARIANT}"
echo ""
echo "📖 Documentation:"
echo "   - Full guide: docs/PARALLEL_INSTALL_LINUX.md"
echo "   - Lessons learned: docs/LESSONS_LEARNED_PARALLEL_INSTALL.md"
echo ""
echo "✨ Your standard OptaFly_Zed installation remains unaffected."
echo ""
