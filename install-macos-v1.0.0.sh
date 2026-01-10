#!/usr/bin/env bash
# OptaFly_Zed v1.0.0 - macOS Installation Script
# Copyright (c) 2025-2026 Tumquan Corp

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REQUIRED_RUST_VERSION="1.91.1"
REQUIRED_PYTHON_VERSION="3.12"

echo "╔════════════════════════════════════════════════════════════╗"
echo "║  OptaFly_Zed v1.0.0 - macOS Installer                      ║"
echo "║  ML Foundation + Structurizr JNI + Semantic Caching       ║"
echo "║  Copyright (c) 2025-2026 Tumquan Corp                      ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""

# Check if command exists
command_exists() {
    command -v "$1" &>/dev/null
}

# Compare version numbers
version_ge() {
    [ "$(printf '%s\n' "$1" "$2" | sort -V | head -n1)" = "$2" ]
}

echo "═══ Step 1: Checking Dependencies ═══"
echo ""

# Check Xcode Command Line Tools
echo -n "Checking Xcode Command Line Tools... "
if xcode-select -p &>/dev/null; then
    echo "✅ Found"
else
    echo "❌ Not found"
    echo "   Installing Xcode Command Line Tools..."
    xcode-select --install
    echo "   Please complete the installation and re-run this script"
    exit 1
fi

# Check Homebrew
echo -n "Checking Homebrew... "
if command_exists brew; then
    echo "✅ Found"
else
    echo "❌ Not found"
    echo "   Installing Homebrew..."
    /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
    
    # Add Homebrew to PATH
    if [[ $(uname -m) == "arm64" ]]; then
        eval "$(/opt/homebrew/bin/brew shellenv)"
    else
        eval "$(/usr/local/bin/brew shellenv)"
    fi
    echo "✅ Homebrew installed"
fi

# Check Rust
echo -n "Checking Rust installation... "
if command_exists rustc; then
    RUST_VERSION=$(rustc --version | awk '{print $2}')
    echo "Found: $RUST_VERSION"
    
    if [ "$RUST_VERSION" != "$REQUIRED_RUST_VERSION" ]; then
        echo "   ⚠️ Warning: Expected Rust $REQUIRED_RUST_VERSION, found $RUST_VERSION"
        echo "   Installing Rust $REQUIRED_RUST_VERSION..."
        rustup toolchain install $REQUIRED_RUST_VERSION
        rustup default $REQUIRED_RUST_VERSION
        echo "   ✅ Rust $REQUIRED_RUST_VERSION installed"
    fi
else
    echo "❌ Not found"
    echo "   Installing Rust $REQUIRED_RUST_VERSION..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain $REQUIRED_RUST_VERSION -y
    source "$HOME/.cargo/env"
    echo "   ✅ Rust $REQUIRED_RUST_VERSION installed"
fi

# Check Python
echo -n "Checking Python installation... "
if command_exists python3; then
    PYTHON_VERSION=$(python3 --version | awk '{print $2}')
    echo "Found: $PYTHON_VERSION"
    
    if ! version_ge "$PYTHON_VERSION" "$REQUIRED_PYTHON_VERSION"; then
        echo "   ⚠️ Warning: Python $REQUIRED_PYTHON_VERSION+ required, found $PYTHON_VERSION"
        echo "   Installing Python $REQUIRED_PYTHON_VERSION via Homebrew..."
        brew install python@3.12
        echo "   ✅ Python 3.12 installed"
    fi
else
    echo "❌ Not found"
    echo "   Installing Python 3.12 via Homebrew..."
    brew install python@3.12
    echo "   ✅ Python 3.12 installed"
fi

# Check Graphviz (optional)
echo -n "Checking Graphviz installation (optional)... "
if command_exists dot; then
    echo "✅ Found"
else
    echo "❌ Not found"
    echo "   Installing Graphviz via Homebrew..."
    brew install graphviz
    echo "   ✅ Graphviz installed"
fi

echo ""
echo "═══ Step 2: Building OptaFly_Zed ═══"
echo ""

cd "$SCRIPT_DIR"

echo "Building in release mode (this may take 10-30 minutes)..."
CORES=$(sysctl -n hw.ncpu)
echo "Using $CORES CPU cores for parallel build"

if cargo build --release -j "$CORES"; then
    echo "✅ Build successful!"
else
    echo "❌ Build failed"
    exit 1
fi

echo ""
echo "═══ Step 3: Setting Up Widget-Log ═══"
echo ""

cd "$SCRIPT_DIR/widget-log"

echo "Creating Python virtual environment..."
python3 -m venv venv

echo "Activating virtual environment..."
source venv/bin/activate

echo "Installing Python dependencies..."
pip install --upgrade pip
pip install -r requirements.txt

echo "✅ Widget-Log setup complete!"

echo ""
echo "═══ Step 4: Configuring API Key ═══"
echo ""

CONFIG_DIR="$HOME/.local/share/optafly-zed/widget-log"
mkdir -p "$CONFIG_DIR"

ENV_FILE="$CONFIG_DIR/.env"

if [ -f "$ENV_FILE" ]; then
    echo "API key configuration already exists"
    EXISTING_KEY=$(grep "ANTHROPIC_API_KEY=" "$ENV_FILE" | cut -d= -f2)
    if [ -n "$EXISTING_KEY" ] && [ "$EXISTING_KEY" != "your_key_here" ]; then
        echo "Using existing API key: ${EXISTING_KEY:0:10}..."
    else
        echo "⚠️ API key not configured. Please edit: $ENV_FILE"
    fi
else
    echo "Creating API key configuration file..."
    
    # Generate secure auth token
    AUTH_TOKEN=$(openssl rand -hex 32)
    
    cat > "$ENV_FILE" << EOF
ANTHROPIC_API_KEY=your_key_here
WIDGET_LOG_AUTH_TOKEN=$AUTH_TOKEN
EOF
    
    echo "✅ Configuration file created: $ENV_FILE"
    echo "⚠️ IMPORTANT: Edit $ENV_FILE and add your Anthropic API key"
    echo "   Get your API key from: https://console.anthropic.com"
fi

echo ""
echo "═══ Step 5: Starting Widget-Log Proxy ═══"
echo ""

echo "Starting semantic caching proxy on https://127.0.0.1:8443..."

# Check if port 8443 is in use
if lsof -Pi :8443 -sTCP:LISTEN -t >/dev/null 2>&1; then
    echo "⚠️ Port 8443 already in use. Stopping existing process..."
    PID=$(lsof -Pi :8443 -sTCP:LISTEN -t)
    kill -9 "$PID" 2>/dev/null || true
    sleep 2
fi

# Start proxy in background
cd "$SCRIPT_DIR/widget-log"
nohup python3 secure_proxy.py > /dev/null 2>&1 &
PROXY_PID=$!

sleep 3

# Verify proxy is running
if curl -k -s https://127.0.0.1:8443/health | grep -q "ok"; then
    echo "✅ Widget-Log proxy started successfully! (PID: $PROXY_PID)"
else
    echo "⚠️ Warning: Could not verify proxy health (may still be starting)"
fi

echo ""
echo "═══ Installation Complete! ═══"
echo ""

echo "╔════════════════════════════════════════════════════════════╗"
echo "║                    Installation Summary                    ║"
echo "╠════════════════════════════════════════════════════════════╣"
echo "║  ✅ OptaFly_Zed v1.0.0 built successfully                  ║"
echo "║  ✅ Widget-Log semantic caching configured                 ║"
echo "║  ✅ Proxy running on https://127.0.0.1:8443                ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""

echo "To start OptaFly_Zed:"
echo "  ./target/release/zed"
echo ""

echo "To configure your API key:"
echo "  nano $ENV_FILE"
echo ""

echo "To check cache statistics:"
echo "  TOKEN=\$(grep WIDGET_LOG_AUTH_TOKEN $ENV_FILE | cut -d= -f2)"
echo "  curl -k -H \"Authorization: Bearer \$TOKEN\" https://127.0.0.1:8443/stats | jq '.'"
echo ""

echo "For more information, see INSTALL.md and README.md"
echo ""

# Offer to start Zed
read -p "Start OptaFly_Zed now? (Y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]] || [[ -z $REPLY ]]; then
    echo "Launching OptaFly_Zed..."
    cd "$SCRIPT_DIR"
    ./target/release/zed &
fi

echo ""
echo "Thank you for using OptaFly_Zed! 🚀"
