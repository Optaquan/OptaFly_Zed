#!/usr/bin/env bash
# OptaFly_Zed v1.0.0 - Complete Installation Script (macOS)
# Installs: Zed Editor + OptaCore + Widget-Log Semantic Caching
# Copyright (c) 2025-2026 Tumquan Corp

set -euo pipefail

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REQUIRED_RUST_VERSION="1.91.1"
REQUIRED_PYTHON_VERSION="3.8"
OPTAFLY_VERSION="v1.0.0"
GITHUB_REPO="https://github.com/Optaquan/OptaFly_Zed"
RELEASE_URL="${GITHUB_REPO}/releases/tag/${OPTAFLY_VERSION}"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

# Banner
clear
echo -e "${CYAN}"
cat << "EOF"
╔════════════════════════════════════════════════════════════════════════════╗
║                                                                            ║
║                  OptaFly_Zed v1.0.0 Installer (macOS)                      ║
║                                                                            ║
║  • Zed Editor with OptaCore Architecture Modeling                          ║
║  • Widget-Log Semantic Caching (280x faster AI responses)                  ║
║  • Complete automated setup for macOS                                      ║
║                                                                            ║
╚════════════════════════════════════════════════════════════════════════════╝
EOF
echo -e "${NC}"
echo ""
echo -e "${BLUE}Release:${NC} ${OPTAFLY_VERSION}"
echo -e "${BLUE}GitHub:${NC}  ${GITHUB_REPO}"
echo -e "${BLUE}Docs:${NC}    ${RELEASE_URL}"
echo ""

read -p "Continue installation? (y/N) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    exit 0
fi

# Logging
LOG_FILE="${SCRIPT_DIR}/optafly-install-$(date +%Y%m%d-%H%M%S).log"
exec > >(tee -a "$LOG_FILE")
exec 2>&1

echo "Installation started: $(date)"
echo "Platform: macOS $(sw_vers -productVersion)"
echo "Architecture: $(uname -m)"
echo ""

# Helper functions
command_exists() {
    command -v "$1" &>/dev/null
}

# Step 1: Xcode and Homebrew
echo "═══ Step 1/6: Xcode and Homebrew ═══"
echo ""

if ! xcode-select -p &>/dev/null; then
    echo "Installing Xcode Command Line Tools..."
    xcode-select --install
    echo "Please complete Xcode installation and re-run this script"
    exit 1
fi

if ! command_exists brew; then
    echo "Installing Homebrew..."
    /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
    
    # Add Homebrew to PATH
    if [[ $(uname -m) == "arm64" ]]; then
        eval "$(/opt/homebrew/bin/brew shellenv)"
    else
        eval "$(/usr/local/bin/brew shellenv)"
    fi
fi

echo "✓ Xcode and Homebrew ready"

# Step 2: System Dependencies
echo ""
echo "═══ Step 2/6: System Dependencies ═══"
echo ""

brew install pkg-config openssl openjdk@17 python@3.12 graphviz

# Set JAVA_HOME
export JAVA_HOME=$(brew --prefix openjdk@17)
echo "export JAVA_HOME=$(brew --prefix openjdk@17)" >> ~/.zshrc

echo "✓ System dependencies installed"

# Step 3: Rust
echo ""
echo "═══ Step 3/6: Rust ${REQUIRED_RUST_VERSION} ═══"
echo ""

if command_exists rustc; then
    CURRENT_RUST=$(rustc --version | awk '{print $2}')
    if [ "$CURRENT_RUST" != "$REQUIRED_RUST_VERSION" ]; then
        rustup toolchain install ${REQUIRED_RUST_VERSION}
        rustup default ${REQUIRED_RUST_VERSION}
    fi
else
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | \
        sh -s -- --default-toolchain ${REQUIRED_RUST_VERSION} -y
    source "$HOME/.cargo/env"
fi

echo "✓ Rust ${REQUIRED_RUST_VERSION} ready"

# Step 4: Build OptaFly_Zed
echo ""
echo "═══ Step 4/6: Building OptaFly_Zed (20-30 minutes) ═══"
echo ""

cd "$SCRIPT_DIR"
CORES=$(sysctl -n hw.ncpu)

cargo build --release -j ${CORES}
cargo build --release -p optacore_struct -j ${CORES}
cargo build --release -p optacore_jni -j ${CORES}

echo "✓ OptaFly_Zed built"

# Step 5: Widget-Log
echo ""
echo "═══ Step 5/6: Widget-Log Setup ═══"
echo ""

cd "${SCRIPT_DIR}/widget-log"

python3 -m venv venv
source venv/bin/activate
pip install --upgrade pip --quiet
pip install -r requirements.txt

# Pre-download models
python3 << 'EOFPYTHON'
from sentence_transformers import SentenceTransformer
SentenceTransformer('all-MiniLM-L6-v2')
EOFPYTHON

echo "✓ Widget-Log ready"

# Step 6: Configuration
echo ""
echo "═══ Step 6/6: Configuration ═══"
echo ""

CONFIG_DIR="$HOME/.local/share/optafly-zed/widget-log"
mkdir -p "$CONFIG_DIR"

ENV_FILE="${CONFIG_DIR}/.env"
if [ ! -f "$ENV_FILE" ]; then
    AUTH_TOKEN=$(openssl rand -hex 32)
    cat > "$ENV_FILE" << EOFENV
ANTHROPIC_API_KEY=your_anthropic_api_key_here
WIDGET_LOG_AUTH_TOKEN=${AUTH_TOKEN}
WIDGET_LOG_CACHE_DIR=${CONFIG_DIR}/cache
WIDGET_LOG_SIMILARITY_THRESHOLD=0.85
WIDGET_LOG_MAX_CACHE_SIZE_GB=5
EOFENV
fi

# SSL certs
CERT_DIR="${SCRIPT_DIR}/widget-log/certs"
mkdir -p "$CERT_DIR"

if [ ! -f "${CERT_DIR}/cert.pem" ]; then
    openssl req -x509 -newkey rsa:4096 \
        -keyout "${CERT_DIR}/key.pem" \
        -out "${CERT_DIR}/cert.pem" \
        -days 3650 -nodes \
        -subj "/CN=localhost/O=OptaFly_Zed/C=US" \
        2>/dev/null
fi

# Configure Zed
cd "${SCRIPT_DIR}/widget-log"
./configure-zed.sh

echo "✓ Configuration complete"

# Done
echo ""
echo "═══════════════════════════════════════════════════════════════════════════"
echo "                    Installation Complete! 🎉"
echo "═══════════════════════════════════════════════════════════════════════════"
echo ""
echo "Next Steps:"
echo "  1. Edit ${ENV_FILE}"
echo "     Add your Anthropic API key"
echo ""
echo "  2. Start OptaFly_Zed:"
echo "     ${SCRIPT_DIR}/target/release/zed"
echo ""
echo "  3. Widget-Log proxy will start automatically"
echo ""
echo "Release: ${RELEASE_URL}"
echo "Log:     ${LOG_FILE}"
echo ""
