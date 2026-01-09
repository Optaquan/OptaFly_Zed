# Lessons Learned: Parallel Installation on Linux

## Overview

This document captures key insights from implementing parallel installations of OptaFly_Zed on Linux, allowing development and production versions to coexist without conflicts.

---

## Key Challenge: Single-Instance Applications

### Problem
Modern desktop applications like Zed use instance locking to prevent multiple copies from running simultaneously. This is typically implemented via:
- Unix domain sockets (e.g., `zed-stable.sock`)
- DBus activation
- Process detection

### Impact
- Clicking a desktop icon may connect to an existing instance instead of launching a new one
- Different builds attempting to share the same socket fail
- Configuration conflicts between versions

### Solution
Complete isolation through environment variables and separate resource paths:

```bash
export ZED_CONFIG_DIR="$HOME/.config/project-phase-x"
export ZED_DATA_DIR="$HOME/.local/share/project-phase-x"
export ZED_SOCKET_NAME="project-phase-x.sock"
```

---

## Isolation Strategy

### 1. Configuration Directories

**Standard Application:**
```
~/.config/zed/
~/.local/share/zed/
```

**Parallel Installation:**
```
~/.config/project-variant/
~/.local/share/project-variant/
```

**Implementation:**
- Use environment variables (`XDG_CONFIG_HOME`, `XDG_DATA_HOME`)
- Create wrapper scripts that set isolation variables
- Initialize directories on first run

### 2. Socket Naming

**Problem:** Default socket names collide.

**Solution:** Use unique socket names per installation:
```bash
export SOCKET_NAME="application-variant.sock"
```

Store sockets in isolated data directories to prevent conflicts.

### 3. Network Ports

If your application uses network services (HTTP servers, debug ports), assign unique ports:

```bash
# Standard: 8443
export APP_PORT=8444  # Variant
```

---

## Desktop Integration Challenges

### Desktop Entry Files

**Location:** `~/.local/share/applications/`

**Requirements:**
1. Unique filename (e.g., `application-variant.desktop`)
2. Distinct `Name=` field for menu visibility
3. Correct `Exec=` path to wrapper script
4. Optional: `StartupWMClass=` for proper window grouping

**Example:**
```desktop
[Desktop Entry]
Version=1.0
Type=Application
Name=MyApp (Development)
Exec=/path/to/wrapper-script %U
Icon=myapp
StartupWMClass=myapp-dev
Categories=Development;IDE;
```

### Icon Differentiation

**Options:**
1. **Badge overlay** (requires ImageMagick):
   ```bash
   convert source-icon.png \
     -gravity SouthEast \
     -pointsize 120 -fill '#00FF00' \
     -annotate +20+20 'DEV' \
     output-icon.png
   ```

2. **Different icon file** with visual distinction

3. **Colored variants** of base icon

### Desktop Database Updates

After creating `.desktop` files:
```bash
update-desktop-database ~/.local/share/applications/
```

**Note:** Some desktop environments auto-detect changes, others require logout/login or manual refresh.

---

## Build Considerations

### Build Time Management

For large Rust projects:
- **Cold build:** 25-30 minutes typical
- **Incremental builds:** 2-5 minutes
- **Release builds:** Longer due to optimization

**Recommendations:**
1. Use `cargo build --release` for production binaries
2. Consider `--jobs` flag for parallel compilation
3. Monitor disk space (release builds can be >1GB per binary)
4. Use `sccache` or similar for shared build cache

### Dependency Isolation

When building parallel installations:
```bash
# Option 1: Separate source trees
cp -r project/ project-variant/
cd project-variant/
cargo build --release

# Option 2: Separate target directories
cargo build --release --target-dir=/path/to/alternate/target
```

---

## Launch Wrapper Pattern

### Purpose
Wrapper scripts set environment variables before launching the actual binary, ensuring isolation without modifying source code.

### Template
```bash
#!/bin/bash
# Application Variant Launcher

# Isolation
export CONFIG_DIR="$HOME/.config/app-variant"
export DATA_DIR="$HOME/.local/share/app-variant"
export SOCKET_NAME="app-variant.sock"
export APP_PORT=8444

# First-run initialization
if [ ! -d "$CONFIG_DIR" ]; then
    mkdir -p "$CONFIG_DIR"
    # Copy default configs or create markers
    cat > "$CONFIG_DIR/settings.json" << 'EOF'
{
  "variant": "development",
  "version": "experimental"
}
EOF
fi

mkdir -p "$DATA_DIR"

# Launch
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
exec "$SCRIPT_DIR/actual-binary" "$@"
```

### Key Features
- `exec` replaces shell with binary (clean process tree)
- `$@` passes all arguments through
- First-run initialization creates directories
- Absolute path resolution via `$SCRIPT_DIR`

---

## Testing Strategy

### Verification Checklist

1. **Binary Isolation**
   ```bash
   ps aux | grep app-name
   # Should show different paths for each variant
   ```

2. **Configuration Separation**
   ```bash
   ls ~/.config/
   # Verify separate directories exist
   ```

3. **Socket Isolation**
   ```bash
   ls ~/.local/share/*/  | grep .sock
   # Each variant should have unique socket
   ```

4. **Desktop Entry**
   ```bash
   desktop-file-validate ~/.local/share/applications/app-variant.desktop
   ```

5. **Launch Test**
   ```bash
   # From command line
   /path/to/wrapper-script
   
   # Via desktop (check process list)
   # Click icon, then verify with ps aux
   ```

---

## Common Pitfalls

### 1. Desktop Entry Not Appearing

**Causes:**
- Desktop database not updated
- File permissions incorrect
- Invalid `.desktop` file syntax

**Solutions:**
```bash
chmod 644 ~/.local/share/applications/app-variant.desktop
desktop-file-validate ~/.local/share/applications/app-variant.desktop
update-desktop-database ~/.local/share/applications/
```

### 2. Icon Launches Wrong Version

**Cause:** Application detects existing instance and connects to it instead of launching new one.

**Solution:** Ensure socket name isolation is working:
```bash
# Check running processes
pgrep -af application-name

# Verify different socket names
find ~/.local/share/ -name "*.sock"
```

### 3. "Application Already Running" Error

**Cause:** Socket collision or shared lock files.

**Solution:**
- Verify `SOCKET_NAME` environment variable is set
- Check for lock files in cache directories
- Ensure data directories are truly separate

### 4. Wrapper Script Not Executable

**Symptom:** Desktop icon does nothing when clicked.

**Solution:**
```bash
chmod +x /path/to/wrapper-script
```

---

## Performance Considerations

### Disk Space

Parallel installations multiply resource requirements:
- Binary size: ~2GB per Rust release build
- Dependencies: Shared via `~/.cargo`
- Build artifacts: ~5-10GB per variant in `target/`

**Recommendation:** Plan for 10-15GB per parallel installation.

### Memory Usage

Running multiple variants simultaneously:
- Each instance: 200MB-1GB RAM typical
- Shared libraries reduce overhead
- Monitor with `htop` or `top`

### Build Cache

Use `sccache` to share compilation cache across variants:
```bash
cargo install sccache
export RUSTC_WRAPPER=sccache
```

---

## Automation Recommendations

### Installation Script Template

```bash
#!/bin/bash
set -e

SRC_DIR="$(pwd)"
VARIANT_NAME="app-variant"
INSTALL_DIR="$HOME/.local/opt/$VARIANT_NAME"

echo "Installing $VARIANT_NAME..."

# Build
cargo build --release

# Create installation directory
mkdir -p "$INSTALL_DIR/bin"

# Copy binary
cp target/release/app-binary "$INSTALL_DIR/bin/"

# Create wrapper
cat > "$INSTALL_DIR/bin/launcher" << 'EOF'
#!/bin/bash
export CONFIG_DIR="$HOME/.config/VARIANT_PLACEHOLDER"
export DATA_DIR="$HOME/.local/share/VARIANT_PLACEHOLDER"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
exec "$SCRIPT_DIR/app-binary" "$@"
EOF

sed -i "s/VARIANT_PLACEHOLDER/$VARIANT_NAME/g" "$INSTALL_DIR/bin/launcher"
chmod +x "$INSTALL_DIR/bin/launcher"

# Create desktop entry
cat > "$HOME/.local/share/applications/$VARIANT_NAME.desktop" << EOF
[Desktop Entry]
Name=MyApp ($VARIANT_NAME)
Exec=$INSTALL_DIR/bin/launcher %U
Icon=myapp
Type=Application
Categories=Development;
EOF

update-desktop-database "$HOME/.local/share/applications/"

echo "Installation complete!"
echo "Launch: $INSTALL_DIR/bin/launcher"
```

---

## Platform-Specific Notes

### Linux Desktop Environments

**GNOME:**
- May cache desktop entries aggressively
- Use `Alt+F2` → `r` to reload shell
- Check Activities search for new entries

**KDE Plasma:**
- Usually auto-detects new `.desktop` files
- Use `kbuildsycoca5` to force cache rebuild

**XFCE:**
- Update with `xfce4-panel --restart`
- Menu may need manual refresh

**i3/Sway:**
- Use `dmenu` or `rofi` launchers
- Add to `~/.local/bin` and include in `$PATH`

### Distribution Considerations

**Debian/Ubuntu:**
```bash
sudo apt install imagemagick  # For icon badges
```

**Fedora/RHEL:**
```bash
sudo dnf install ImageMagick
```

**Arch Linux:**
```bash
sudo pacman -S imagemagick
```

---

## Security Considerations

### File Permissions

Wrapper scripts should be user-executable only:
```bash
chmod 700 /path/to/wrapper-script  # User only
# OR
chmod 755 /path/to/wrapper-script  # User + read by others
```

### Configuration Files

Sensitive data (API keys, tokens) in variant configs:
```bash
chmod 600 ~/.config/app-variant/secrets.json
```

### Shared Resources

Be cautious with shared caches or temporary files:
- Use variant-specific temp directories
- Clear sensitive data on exit
- Avoid world-writable paths

---

## Maintenance

### Updating Variants

```bash
cd /path/to/variant-source
git pull
cargo build --release
# Copy new binary to installation
cp target/release/app /path/to/install/bin/
```

### Uninstalling Variants

```bash
# Remove desktop entry
rm ~/.local/share/applications/app-variant.desktop
update-desktop-database ~/.local/share/applications/

# Remove configuration
rm -rf ~/.config/app-variant
rm -rf ~/.local/share/app-variant

# Remove installation
rm -rf /path/to/installation
```

### Logs and Debugging

Enable logging for troubleshooting:
```bash
export RUST_LOG=debug
/path/to/wrapper-script 2>&1 | tee debug.log
```

---

## Summary

Successful parallel installations require:

1. **Complete Isolation:** Config, data, sockets, ports
2. **Wrapper Scripts:** Environment setup before launch
3. **Unique Desktop Entries:** Clear naming and paths
4. **Process Verification:** Always test with `ps` and `pgrep`
5. **Documentation:** Track variant-specific configurations

**Key Insight:** Single-instance detection is the primary challenge. Solve it through comprehensive environment isolation, not application modification.

---

## Further Reading

- [XDG Base Directory Specification](https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html)
- [Desktop Entry Specification](https://specifications.freedesktop.org/desktop-entry-spec/desktop-entry-spec-latest.html)
- [Cargo Build Guide](https://doc.rust-lang.org/cargo/guide/build-cache.html)

---

**Document Version:** 1.0  
**Last Updated:** 2026-01-09  
**Status:** Production
