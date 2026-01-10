#!/usr/bin/env bash
# OptaFly_Zed Phase 2.5 - Parallel Installation Script
# Creates isolated Phase 2.5 installation alongside existing Zed
# Copyright (c) 2025–2026 Tumquan Corp

set -euo pipefail

# ──────────────────────────────────────────────────────────────────────────────
# Configuration & Paths
# ──────────────────────────────────────────────────────────────────────────────

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DEST_DIR="$SCRIPT_DIR"                  # Install in current directory (repo root)

echo "╔════════════════════════════════════════════════════════════╗"
echo "║  OptaFly_Zed Phase 2.5 - Intelligent Parallel Installer   ║"
echo "║  ML Foundation + Structurizr JNI + Semantic Caching       ║"
echo "║  Copyright (c) 2025–2026 Tumquan Corp                      ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""

# ──────────────────────────────────────────────────────────────────────────────
# Dependency Checking & Installation
# ──────────────────────────────────────────────────────────────────────────────

check_command() {
    if command -v "$1" &>/dev/null; then
        echo "   ✅ $1 found"
        return 0
    else
        echo "   ❌ $1 not found"
        return 1
    fi
}

check_rust() {
    if command -v rustc &>/dev/null && command -v cargo &>/dev/null; then
        local version=$(rustc --version | awk '{print $2}')
        echo "   ✅ Rust $version"
        # Zed typically wants latest stable (as of 2026); 1.82 is minimum
        if [[ "$version" > "1.82.0" ]] || [[ "$version" == "1.82.0" ]]; then
            return 0
        else
            echo "   ⚠️ Rust $version is too old (want ≥1.82.0)"
            return 1
        fi
    else
        echo "   ❌ Rust toolchain missing"
        return 1
    fi
}

check_python() {
    if command -v python3 &>/dev/null; then
        local version=$(python3 -c 'import sys; print(".".join(map(str, sys.version_info[:2])))')
        echo "   ✅ Python $version"
        local major="${version%%.*}"
        local minor="${version#*.}"
        if [[ "$major" -ge 3 ]] && [[ "$minor" -ge 8 ]]; then
            return 0
        else
            echo "   ⚠️ Python $version too old (want ≥3.8)"
            return 1
        fi
    else
        echo "   ❌ Python 3 missing"
        return 1
    fi
}

install_dependencies() {
    echo "🔧 Auto-installing missing dependencies..."
    echo ""

    local pkg_mgr=""
    local install_cmd=""
    local update_cmd=""

    if command -v apt &>/dev/null; then
        pkg_mgr="apt"
        update_cmd="sudo apt update"
        install_cmd="sudo apt install -y"
    elif command -v dnf &>/dev/null; then
        pkg_mgr="dnf"
        update_cmd="sudo dnf check-update || true"
        install_cmd="sudo dnf install -y"
    elif command -v pacman &>/dev/null; then
        pkg_mgr="pacman"
        update_cmd="sudo pacman -Sy"
        install_cmd="sudo pacman -S --noconfirm"
    elif command -v brew &>/dev/null; then
        pkg_mgr="brew"
        update_cmd="brew update"
        install_cmd="brew install"
    else
        echo "   ⚠️ No supported package manager detected."
        echo "   Please install dependencies manually:"
        echo "   - Rust: https://rustup.rs"
        echo "   - Python 3.8+: https://python.org"
        echo "   - Build tools: gcc, clang, pkg-config, libssl-dev"
        return 1
    fi

    echo "   Using $pkg_mgr..."
    echo ""

    $update_cmd

    # Rust (always via rustup — safest & up-to-date)
    if ! check_rust &>/dev/null; then
        echo "   → Installing latest Rust via rustup..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --no-modify-path
        source "$HOME/.cargo/env"
        echo "   ✅ Rust installed"
    fi

    # Python + pip
    if ! check_python &>/dev/null; then
        echo "   → Installing Python 3..."
        case "$pkg_mgr" in
            apt)
                $install_cmd python3 python3-pip python3-venv
                ;;
            dnf)
                $install_cmd python3 python3-pip
                ;;
            pacman)
                $install_cmd python python-pip
                ;;
            brew)
                $install_cmd python
                ;;
        esac
        echo "   ✅ Python installed"
    fi

    # Core build deps (Zed needs quite a few on Linux)
    echo "   → Installing core build dependencies..."
    case "$pkg_mgr" in
        apt)
            $install_cmd build-essential pkg-config libssl-dev clang libclang-dev \
                libgtk-4-dev libadwaita-1-dev git curl || true
            ;;
        dnf)
            $install_cmd gcc gcc-c++ pkg-config openssl-devel clang clang-devel \
                gtk4-devel libadwaita-devel git curl || true
            ;;
        pacman)
            $install_cmd base-devel openssl clang gtk4 libadwaita git curl || true
            ;;
        brew)
            $install_cmd pkg-config openssl git curl || true
            ;;
    esac

    # Optional but very useful
    echo "   → Installing optional dependencies..."
    case "$pkg_mgr" in
        apt|dnf|pacman)
            $install_cmd graphviz || echo "   (Graphviz skipped - optional)"
            ;;
        brew)
            $install_cmd graphviz || echo "   (Graphviz skipped - optional)"
            ;;
    esac

    echo ""
    echo "   ✅ Dependencies installation complete!"
    echo ""
}

# ──────────────────────────────────────────────────────────────────────────────
# Main Execution
# ──────────────────────────────────────────────────────────────────────────────

echo "🔍 Step 1/6: Checking dependencies..."
echo ""
MISSING=0
check_rust || MISSING=$((MISSING+1))
check_python || MISSING=$((MISSING+1))
check_command git || MISSING=$((MISSING+1))
check_command graphviz || echo "   ⚠️ Graphviz missing (optional for DOT visualization)"

echo ""

if (( MISSING > 0 )); then
    echo "⚠️ Found $MISSING missing required dependencies"
    echo ""
    read -p "Install missing dependencies automatically? (y/N) " -n1 -r
    echo ""
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        install_dependencies

        # Re-check after installation
        MISSING=0
        check_rust || MISSING=$((MISSING+1))
        check_python || MISSING=$((MISSING+1))
        check_command git || MISSING=$((MISSING+1))

        if (( MISSING > 0 )); then
            echo ""
            echo "❌ Some dependencies failed to install. Please install manually."
            exit 1
        fi
    else
        echo "❌ Please install dependencies manually and re-run."
        echo ""
        echo "Quick install commands:"
        echo "  Rust:   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
        echo "  Python: sudo apt install python3 python3-pip  # or equivalent for your OS"
        echo "  Build:  sudo apt install build-essential pkg-config libssl-dev"
        exit 1
    fi
fi

echo "✅ All required dependencies are installed!"
echo ""

# ──────────────────────────────────────────────────────────────────────────────
# Widget-Log Python Dependencies
# ──────────────────────────────────────────────────────────────────────────────

if [[ -f "widget-log/requirements.txt" ]]; then
    echo "🐍 Step 2/6: Installing Widget-Log Python dependencies..."

    # Check if venv exists, create if not
    if [[ ! -d "widget-log/venv" ]]; then
        echo "   Creating Python virtual environment..."
        python3 -m venv widget-log/venv
    fi

    echo "   Installing Python packages..."
    widget-log/venv/bin/pip install --upgrade pip setuptools wheel
    widget-log/venv/bin/pip install -r widget-log/requirements.txt

    echo "   ✅ Widget-Log dependencies installed"
    echo ""
else
    echo "⚠️ Step 2/6: widget-log/requirements.txt not found, skipping Python setup"
    echo ""
fi

# ──────────────────────────────────────────────────────────────────────────────
# Building OptaFly_Zed
# ──────────────────────────────────────────────────────────────────────────────

echo "🔨 Step 3/6: Building OptaFly_Zed (this may take 10–30 minutes)..."
echo "   This is a good time for a coffee break! ☕"
echo ""

cd "$DEST_DIR"

# Build with progress indication
echo "   → Building main editor..."
cargo build --release 2>&1 | grep -E "Compiling|Finished|error" || cargo build --release

if [[ -f "target/release/zed" ]]; then
    echo "   ✅ Built: zed ($(ls -lh target/release/zed | awk '{print $5}'))"
else
    echo "   ❌ Build failed - zed binary not found"
    exit 1
fi

# Build OptaCore components if they exist
if [[ -d "crates/optacore_struct" ]]; then
    echo "   → Building OptaCore libraries..."
    cargo build --release --package optacore_struct 2>&1 | grep -E "Compiling|Finished|error" || true
    cargo build --release --package optacore_jni 2>&1 | grep -E "Compiling|Finished|error" || true

    if [[ -f "target/release/liboptacore_jni.so" ]] || [[ -f "target/release/liboptacore_jni.dylib" ]]; then
        echo "   ✅ Built: OptaCore JNI library"
    fi
fi

echo ""
echo "✅ Build complete!"
echo ""

# ──────────────────────────────────────────────────────────────────────────────
# Parallel Isolation Setup
# ──────────────────────────────────────────────────────────────────────────────

echo "🎨 Step 4/6: Setting up parallel isolation..."
echo ""

# Create isolated config/data directories
mkdir -p ~/.config/optafly-phase2.5
mkdir -p ~/.local/share/optafly-phase2.5
mkdir -p ~/.local/bin

echo "   ✅ Created isolated directories"
echo "      Config: ~/.config/optafly-phase2.5/"
echo "      Data:   ~/.local/share/optafly-phase2.5/"

# Create wrapper script
echo "   → Creating launcher script..."
cat > ~/.local/bin/optafly-zed <<EOF
#!/usr/bin/env bash
# OptaFly_Zed Phase 2.5 Launcher
# Copyright (c) 2025–2026 Tumquan Corp

# Isolation: Use separate config/data directories
export ZED_CONFIG_DIR="\$HOME/.config/optafly-phase2.5"
export ZED_DATA_DIR="\$HOME/.local/share/optafly-phase2.5"
export ZED_SOCKET_NAME="optafly-phase2.5.sock"

# Differentiation
export ZED_WINDOW_TITLE="OptaFly Phase 2.5"

# Widget-Log on different port to avoid conflicts
export WIDGET_LOG_PORT=8444

# Launch
exec "$DEST_DIR/target/release/zed" "\$@"
EOF

chmod +x ~/.local/bin/optafly-zed

echo "   ✅ Created launcher: ~/.local/bin/optafly-zed"
echo ""

# ──────────────────────────────────────────────────────────────────────────────
# Desktop Integration
# ──────────────────────────────────────────────────────────────────────────────

echo "🖥️ Step 5/6: Creating desktop entry..."
echo ""

# Determine icon path
ICON_PATH="zed"
if [[ -f "$HOME/.local/zed.app/share/icons/hicolor/512x512/apps/zed.png" ]]; then
    ICON_PATH="$HOME/.local/zed.app/share/icons/hicolor/512x512/apps/zed.png"
elif [[ -f "/usr/share/icons/hicolor/512x512/apps/zed.png" ]]; then
    ICON_PATH="/usr/share/icons/hicolor/512x512/apps/zed.png"
fi

mkdir -p ~/.local/share/applications

cat > ~/.local/share/applications/optafly-zed-phase2.5.desktop <<EOF
[Desktop Entry]
Version=1.0
Type=Application
Name=OptaFly Zed (Phase 2.5)
GenericName=Architecture-Enhanced Code Editor
Comment=Zed with Widget-Log caching + OptaCore architecture modeling
TryExec=$HOME/.local/bin/optafly-zed
Exec=$HOME/.local/bin/optafly-zed %U
Icon=$ICON_PATH
Terminal=false
Categories=Utility;TextEditor;Development;IDE;
Keywords=zed;optafly;editor;code;architecture;widget-log;optacore;
MimeType=text/plain;application/x-zerosize;
StartupWMClass=optafly-zed-phase2.5
StartupNotify=true
X-GNOME-FullName=OptaFly Zed Phase 2.5 Edition

[Desktop Action NewWindow]
Name=New Window
Exec=$HOME/.local/bin/optafly-zed --new
EOF

# Refresh desktop database
if command -v update-desktop-database &>/dev/null; then
    update-desktop-database ~/.local/share/applications 2>/dev/null || true
fi

echo "   ✅ Desktop entry created"
echo ""

# ──────────────────────────────────────────────────────────────────────────────
# Widget-Log Setup
# ──────────────────────────────────────────────────────────────────────────────

echo "🔒 Step 6/6: Configuring Widget-Log semantic caching..."
echo ""

WIDGET_LOG_DIR="$HOME/.local/share/optafly-phase2.5/widget-log"
mkdir -p "$WIDGET_LOG_DIR"

if [[ ! -f "$WIDGET_LOG_DIR/.env" ]]; then
    echo "   → Creating Widget-Log configuration..."
    cat > "$WIDGET_LOG_DIR/.env" <<EOF
# Widget-Log Configuration
# Add your Anthropic API key here: https://console.anthropic.com
ANTHROPIC_API_KEY=

# Auto-generated auth token
WIDGET_LOG_AUTH_TOKEN=$(openssl rand -hex 32)

# Server configuration
WIDGET_LOG_PORT=8444
WIDGET_LOG_HOST=127.0.0.1
EOF
    echo "   ✅ Created: $WIDGET_LOG_DIR/.env"
    echo ""
    echo "   ⚠️ IMPORTANT: Add your Anthropic API key to:"
    echo "      $WIDGET_LOG_DIR/.env"
    echo ""
else
    echo "   ℹ️ Widget-Log config already exists"
fi

# ──────────────────────────────────────────────────────────────────────────────
# Installation Summary
# ──────────────────────────────────────────────────────────────────────────────

echo "╔════════════════════════════════════════════════════════════╗"
echo "║  ✅ Installation Complete!                                 ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""
echo "📂 Installation Directory:"
echo "   $DEST_DIR"
echo ""
echo "🚀 Launch Options:"
echo "   1. From terminal:"
echo "      optafly-zed"
echo ""
echo "   2. From application menu:"
echo "      Search for 'OptaFly Zed (Phase 2.5)'"
echo ""
echo "   3. Direct binary:"
echo "      $DEST_DIR/target/release/zed"
echo ""
echo "⚙️ Configuration & Data:"
echo "   Config: ~/.config/optafly-phase2.5/"
echo "   Data:   ~/.local/share/optafly-phase2.5/"
echo "   Logs:   ~/.local/share/optafly-phase2.5/logs/"
echo "   Socket: optafly-phase2.5.sock (port 8444)"
echo ""
echo "🔑 Next Steps:"
echo "   1. Add your Anthropic API key:"
echo "      nano ~/.local/share/optafly-phase2.5/widget-log/.env"
echo ""
echo "   2. Start Widget-Log proxy (for semantic caching):"
echo "      cd $DEST_DIR/widget-log"
echo "      ./start-proxy.sh"
echo ""
echo "   3. Launch OptaFly_Zed:"
echo "      optafly-zed"
echo ""
echo "✨ Phase 2.5 Features:"
echo "   ⚡ Widget-Log: 280x faster AI responses via semantic caching"
echo "   🏗️ OptaCore: Tensor-based architecture modeling"
echo "   🔗 Structurizr: JNI bridge for Java integration"
echo "   📊 C4 Diagrams: Professional DOT export & visualization"
echo "   🤖 ML Ready: Telemetry infrastructure for neural layout models"
echo ""
echo "📚 Documentation:"
echo "   README:     $DEST_DIR/README.md"
echo "   License:    $DEST_DIR/LICENSE.md"
echo "   Changelog:  $DEST_DIR/CHANGELOG.md"
echo "   Install:    $DEST_DIR/PHASE25_PARALLEL_INSTALL.md"
echo ""
echo "🐛 Issues & Support:"
echo "   https://github.com/Optaquan/OptaFly_Zed/issues"
echo ""
echo "Happy coding with OptaFly_Zed! 🚀"
echo "Copyright (c) 2025–2026 Tumquan Corp"
echo ""
