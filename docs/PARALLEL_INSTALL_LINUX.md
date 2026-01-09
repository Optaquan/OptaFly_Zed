# Parallel Installation Guide for Linux

Run multiple OptaFly_Zed variants side-by-side without conflicts.

---

## Overview

This guide enables you to install development, testing, or experimental versions of OptaFly_Zed alongside your stable installation. Each version runs in complete isolation with separate configurations, data, and UI presence.

**Use Cases:**
- Test new features without affecting production workflow
- Run stable and development branches simultaneously
- Maintain multiple project-specific configurations
- Safe experimentation with custom builds

---

## Prerequisites

- **Linux distribution** with desktop environment
- **Rust toolchain** (1.70.0 or later)
- **Git** for source management
- **~15GB disk space** per installation
- **Optional:** ImageMagick for custom icons

```bash
# Install dependencies (Debian/Ubuntu)
sudo apt install build-essential git imagemagick

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

---

## Quick Start

### Using the Automated Script

```bash
cd OptaFly_Zed
./install-parallel.sh
```

The script will:
1. ✅ Copy source to parallel location
2. ✅ Build release binaries (20-30 minutes)
3. ✅ Create launch wrapper with isolation
4. ✅ Generate desktop entry with custom icon
5. ✅ Update application menu

### Launch Your Parallel Installation

**Option 1:** Application Menu
- Search for "OptaFly Zed (Variant)" in your launcher

**Option 2:** Command Line
```bash
~/.local/opt/optafly-variant/bin/launcher
```

---

## Manual Installation

### Step 1: Prepare Source

```bash
# Choose a variant name (alphanumeric, no spaces)
VARIANT="dev"

# Copy or clone source
cp -r OptaFly_Zed OptaFly_Zed_${VARIANT}
cd OptaFly_Zed_${VARIANT}
```

### Step 2: Build

```bash
cargo build --release

# Optional: Build additional components
cargo build --release --package optacore_struct
cargo build --release --package optacore_jni
```

**Build time:** 20-30 minutes on typical hardware.

### Step 3: Create Launch Wrapper

```bash
mkdir -p ~/.local/opt/optafly-${VARIANT}/bin
cp target/release/zed ~/.local/opt/optafly-${VARIANT}/bin/

cat > ~/.local/opt/optafly-${VARIANT}/bin/launcher << 'EOF'
#!/bin/bash

# Configuration isolation
export ZED_CONFIG_DIR="$HOME/.config/optafly-VARIANT_NAME"
export ZED_DATA_DIR="$HOME/.local/share/optafly-VARIANT_NAME"
export ZED_SOCKET_NAME="optafly-VARIANT_NAME.sock"

# Port isolation (if using Widget-Log)
export WIDGET_LOG_PORT=8444

# Initialize on first run
if [ ! -d "$ZED_CONFIG_DIR" ]; then
    mkdir -p "$ZED_CONFIG_DIR"
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
EOF

# Replace placeholder
sed -i "s/VARIANT_NAME/${VARIANT}/g" ~/.local/opt/optafly-${VARIANT}/bin/launcher

# Make executable
chmod +x ~/.local/opt/optafly-${VARIANT}/bin/launcher
```

### Step 4: Create Desktop Entry

```bash
cat > ~/.local/share/applications/optafly-${VARIANT}.desktop << EOF
[Desktop Entry]
Version=1.0
Type=Application
Name=OptaFly Zed (${VARIANT})
GenericName=Code Editor (${VARIANT})
Comment=OptaFly Zed ${VARIANT} variant
Exec=$HOME/.local/opt/optafly-${VARIANT}/bin/launcher %U
Icon=zed
Terminal=false
Categories=Development;IDE;TextEditor;
Keywords=editor;code;zed;optafly;
StartupWMClass=optafly-zed-${VARIANT}
MimeType=text/plain;

[Desktop Action NewWorkspace]
Name=New Workspace
Exec=$HOME/.local/opt/optafly-${VARIANT}/bin/launcher --new %U
EOF

# Update desktop database
update-desktop-database ~/.local/share/applications/
```

### Step 5: (Optional) Custom Icon

Create a visually distinct icon with a badge:

```bash
# Find source icon
SRC_ICON="/usr/share/icons/hicolor/512x512/apps/zed.png"

# Create badged version
convert "$SRC_ICON" \
  -gravity SouthEast \
  -pointsize 120 \
  -fill '#4CAF50' \
  -stroke '#000000' \
  -strokewidth 4 \
  -annotate +20+20 "${VARIANT^^}" \
  ~/.local/share/icons/optafly-${VARIANT}.png

# Update desktop entry to use custom icon
sed -i "s|Icon=zed|Icon=$HOME/.local/share/icons/optafly-${VARIANT}.png|" \
  ~/.local/share/applications/optafly-${VARIANT}.desktop
```

---

## Verification

### Check Installation

```bash
# Binary exists and is executable
test -x ~/.local/opt/optafly-${VARIANT}/bin/launcher && echo "✅ Launcher OK"

# Desktop entry valid
desktop-file-validate ~/.local/share/applications/optafly-${VARIANT}.desktop && echo "✅ Desktop entry OK"

# Launch test
~/.local/opt/optafly-${VARIANT}/bin/launcher --help
```

### Verify Isolation

Launch your variant, then check:

```bash
# Process shows correct binary path
ps aux | grep optafly-${VARIANT}

# Config directory created
ls ~/.config/optafly-${VARIANT}/

# Data directory separate
ls ~/.local/share/optafly-${VARIANT}/

# Socket file isolated
ls ~/.local/share/optafly-${VARIANT}/*.sock
```

Expected output: Each variant has its own directories and socket files.

---

## Configuration

### Variant-Specific Settings

Edit `~/.config/optafly-${VARIANT}/settings.json`:

```json
{
  "variant": "dev",
  "theme": "One Dark",
  "vim_mode": false,
  "tab_size": 2,
  "enable_language_server": true
}
```

### Port Configuration

If running multiple instances with Widget-Log:

```bash
# In launcher script, assign unique ports
export WIDGET_LOG_PORT=8444  # Variant 1
export WIDGET_LOG_PORT=8445  # Variant 2
```

---

## Usage

### Running Multiple Variants Simultaneously

Each variant can run at the same time:

```bash
# Terminal 1: Stable version
~/.local/opt/optafly-stable/bin/launcher ~/project-a

# Terminal 2: Dev version  
~/.local/opt/optafly-dev/bin/launcher ~/project-b
```

Or use desktop icons for both.

### Switching Between Variants

Simply launch the variant you need. Each maintains separate:
- Window state
- Open files
- Recent projects
- Extensions
- Themes

---

## Troubleshooting

### Desktop Icon Doesn't Launch

**Symptom:** Clicking icon does nothing.

**Solution:**
```bash
# Check permissions
chmod +x ~/.local/opt/optafly-${VARIANT}/bin/launcher

# Test from terminal
~/.local/opt/optafly-${VARIANT}/bin/launcher

# Check logs
journalctl --user -f | grep -i optafly
```

### "Already Running" Error

**Symptom:** App says it's already running when it's not.

**Cause:** Socket file collision.

**Solution:**
```bash
# Check socket name is unique
grep SOCKET_NAME ~/.local/opt/optafly-${VARIANT}/bin/launcher

# Remove stale socket
rm ~/.local/share/optafly-${VARIANT}/*.sock
```

### Icon Not Appearing in Menu

**Solution:**
```bash
# Update desktop database
update-desktop-database ~/.local/share/applications/

# Refresh desktop environment
# GNOME: Alt+F2, type 'r', press Enter
# KDE: kbuildsycoca5
# XFCE: xfce4-panel --restart
```

### Wrong Version Launches

**Symptom:** Clicking variant icon opens standard version.

**Cause:** Desktop entry `Exec` path incorrect.

**Solution:**
```bash
# Verify desktop entry
grep ^Exec ~/.local/share/applications/optafly-${VARIANT}.desktop

# Should point to variant launcher, not standard zed
```

---

## Maintenance

### Updating a Variant

```bash
cd OptaFly_Zed_${VARIANT}

# Pull updates
git pull

# Rebuild
cargo build --release

# Update binary
cp target/release/zed ~/.local/opt/optafly-${VARIANT}/bin/
```

### Uninstalling a Variant

```bash
# Remove launcher
rm -rf ~/.local/opt/optafly-${VARIANT}

# Remove desktop entry
rm ~/.local/share/applications/optafly-${VARIANT}.desktop
update-desktop-database ~/.local/share/applications/

# Remove config (optional - keeps your settings)
# rm -rf ~/.config/optafly-${VARIANT}
# rm -rf ~/.local/share/optafly-${VARIANT}
```

---

## Advanced Configuration

### Custom Build Flags

Optimize for your system:

```bash
# CPU-specific optimizations
RUSTFLAGS="-C target-cpu=native" cargo build --release

# Minimal debug symbols (faster builds)
cargo build --release --config profile.release.debug=1
```

### Shared Build Cache

Use `sccache` to speed up multiple builds:

```bash
cargo install sccache
export RUSTC_WRAPPER=sccache

# Builds now share compilation cache
cd OptaFly_Zed_variant1 && cargo build --release
cd OptaFly_Zed_variant2 && cargo build --release
```

### Environment Variable Reference

| Variable | Purpose | Example |
|----------|---------|---------|
| `ZED_CONFIG_DIR` | Configuration files | `~/.config/optafly-dev` |
| `ZED_DATA_DIR` | Application data | `~/.local/share/optafly-dev` |
| `ZED_SOCKET_NAME` | IPC socket name | `optafly-dev.sock` |
| `WIDGET_LOG_PORT` | Widget-Log proxy port | `8444` |
| `ZED_WINDOW_TITLE` | Custom window title | `OptaFly Dev` |

---

## Automated Installation Script

### Script Template

Save as `install-parallel.sh`:

```bash
#!/bin/bash
set -e

# Configuration
VARIANT="${1:-dev}"
SRC_DIR="$(pwd)"
INSTALL_DIR="$HOME/.local/opt/optafly-${VARIANT}"
DESKTOP_FILE="$HOME/.local/share/applications/optafly-${VARIANT}.desktop"

echo "╔════════════════════════════════════════════════╗"
echo "║  OptaFly_Zed Parallel Installation            ║"
echo "║  Variant: ${VARIANT}                           ║"
echo "╚════════════════════════════════════════════════╝"

# Step 1: Build
echo ""
echo "🔨 Building release binary..."
cargo build --release

# Step 2: Install
echo ""
echo "📦 Installing to ${INSTALL_DIR}..."
mkdir -p "${INSTALL_DIR}/bin"
cp target/release/zed "${INSTALL_DIR}/bin/"

# Step 3: Create launcher
echo ""
echo "🚀 Creating launch wrapper..."
cat > "${INSTALL_DIR}/bin/launcher" << 'EOF'
#!/bin/bash
export ZED_CONFIG_DIR="$HOME/.config/optafly-VARIANT_NAME"
export ZED_DATA_DIR="$HOME/.local/share/optafly-VARIANT_NAME"
export ZED_SOCKET_NAME="optafly-VARIANT_NAME.sock"
export WIDGET_LOG_PORT=8444

if [ ! -d "$ZED_CONFIG_DIR" ]; then
    mkdir -p "$ZED_CONFIG_DIR"
    cat > "$ZED_CONFIG_DIR/settings.json" << 'SETTINGS'
{
  "variant": "VARIANT_NAME"
}
SETTINGS
fi

mkdir -p "$ZED_DATA_DIR"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
exec "$SCRIPT_DIR/zed" "$@"
EOF

sed -i "s/VARIANT_NAME/${VARIANT}/g" "${INSTALL_DIR}/bin/launcher"
chmod +x "${INSTALL_DIR}/bin/launcher"

# Step 4: Desktop entry
echo ""
echo "📝 Creating desktop entry..."
cat > "$DESKTOP_FILE" << EOF
[Desktop Entry]
Version=1.0
Type=Application
Name=OptaFly Zed (${VARIANT})
Exec=${INSTALL_DIR}/bin/launcher %U
Icon=zed
Terminal=false
Categories=Development;IDE;
StartupWMClass=optafly-zed-${VARIANT}
EOF

update-desktop-database "$HOME/.local/share/applications/" 2>/dev/null || true

# Summary
echo ""
echo "╔════════════════════════════════════════════════╗"
echo "║  ✅ Installation Complete!                     ║"
echo "╚════════════════════════════════════════════════╝"
echo ""
echo "📂 Installation: ${INSTALL_DIR}"
echo "🚀 Launch: ${INSTALL_DIR}/bin/launcher"
echo "🎯 Desktop: Search for 'OptaFly Zed (${VARIANT})'"
echo ""
echo "⚙️  Config: ~/.config/optafly-${VARIANT}/"
echo "💾 Data:   ~/.local/share/optafly-${VARIANT}/"
echo ""
```

### Usage

```bash
chmod +x install-parallel.sh

# Install with default variant name
./install-parallel.sh

# Install with custom name
./install-parallel.sh testing
./install-parallel.sh experimental
```

---

## Best Practices

1. **Naming Convention:** Use descriptive variant names (`dev`, `stable`, `test`)
2. **Resource Management:** Monitor disk space; each build uses 2-5GB
3. **Port Assignment:** Document port numbers to avoid conflicts
4. **Testing:** Always test wrapper script from terminal before relying on desktop icon
5. **Backups:** Back up variant configs before major updates

---

## Platform Notes

### GNOME
- Icons appear in Activities overview automatically
- Use `Alt+F2` → `r` to force reload if needed

### KDE Plasma
- Entries appear in Application Menu
- Use `kbuildsycoca5` if menu doesn't update

### XFCE
- Check Applications menu after install
- May need panel restart: `xfce4-panel --restart`

### Window Managers (i3, Sway)
- Add launcher to `PATH`: `export PATH="$HOME/.local/opt/optafly-*/bin:$PATH"`
- Use dmenu/rofi to launch
- Consider adding keybindings in config

---

## Support

### Report Issues
- OptaFly_Zed: https://github.com/Optaquan/OptaFly_Zed/issues
- Include variant name and desktop environment in reports

### Community
- Share your parallel installation setups
- Contribute improvements to installation scripts

---

**Version:** 1.0  
**Last Updated:** 2026-01-09  
**Platform:** Linux (all distributions)
