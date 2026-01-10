#!/usr/bin/env bash
# OptaFly_Zed v1.0.0 - Complete Installation Script (Linux)
# Installs: Zed Editor + OptaCore + Widget-Log Semantic Caching
# Copyright (c) 2025-2026 Tumquan Corp

set -euo pipefail

# ═══════════════════════════════════════════════════════════════════════════
# Configuration
# ═══════════════════════════════════════════════════════════════════════════

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
NC='\033[0m' # No Color

# ═══════════════════════════════════════════════════════════════════════════
# Banner
# ═══════════════════════════════════════════════════════════════════════════

clear
echo -e "${CYAN}"
cat << "EOF"
╔════════════════════════════════════════════════════════════════════════════╗
║                                                                            ║
║                      OptaFly_Zed v1.0.0 Installer                          ║
║                                                                            ║
║  • Zed Editor with OptaCore Architecture Modeling                          ║
║  • Widget-Log Semantic Caching (280x faster AI responses)                  ║
║  • Complete automated setup for Linux                                      ║
║                                                                            ║
╚════════════════════════════════════════════════════════════════════════════╝
EOF
echo -e "${NC}"
echo ""
echo -e "${BLUE}Release:${NC} ${OPTAFLY_VERSION}"
echo -e "${BLUE}GitHub:${NC}  ${GITHUB_REPO}"
echo -e "${BLUE}Docs:${NC}    ${RELEASE_URL}"
echo ""
echo -e "${YELLOW}This script will install:${NC}"
echo "  1. System dependencies (build tools, Java, Python, OpenSSL)"
echo "  2. Rust toolchain ${REQUIRED_RUST_VERSION}"
echo "  3. OptaFly_Zed editor (from source)"
echo "  4. OptaCore architecture modeling engine"
echo "  5. Widget-Log semantic caching proxy"
echo "  6. All configuration files and SSL certificates"
echo ""
echo -e "${YELLOW}Estimated time: 40-60 minutes (depends on CPU)${NC}"
echo -e "${YELLOW}Disk space required: ~13 GB${NC}"
echo ""
read -p "Continue? (y/N) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "Installation cancelled."
    exit 0
fi

# ═══════════════════════════════════════════════════════════════════════════
# Logging
# ═══════════════════════════════════════════════════════════════════════════

LOG_FILE="${SCRIPT_DIR}/optafly-install-$(date +%Y%m%d-%H%M%S).log"
exec > >(tee -a "$LOG_FILE")
exec 2>&1

echo ""
echo -e "${GREEN}═══ Installation Log ═══${NC}"
echo "Version:     ${OPTAFLY_VERSION}"
echo "Date:        $(date)"
echo "Platform:    $(uname -s) $(uname -m)"
echo "Log file:    ${LOG_FILE}"
echo ""

# ═══════════════════════════════════════════════════════════════════════════
# Helper Functions
# ═══════════════════════════════════════════════════════════════════════════

log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[✓]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[⚠]${NC} $1"
}

log_error() {
    echo -e "${RED}[✗]${NC} $1"
}

command_exists() {
    command -v "$1" &>/dev/null
}

version_ge() {
    printf '%s\n%s' "$2" "$1" | sort -C -V
}

detect_distro() {
    if [ -f /etc/os-release ]; then
        . /etc/os-release
        echo "$ID"
    elif [ -f /etc/lsb-release ]; then
        . /etc/lsb-release
        echo "$DISTRIB_ID" | tr '[:upper:]' '[:lower:]'
    else
        echo "unknown"
    fi
}

# ═══════════════════════════════════════════════════════════════════════════
# Step 1: System Dependencies
# ═══════════════════════════════════════════════════════════════════════════

echo ""
echo -e "${CYAN}═══ Step 1/6: Installing System Dependencies ═══${NC}"
echo ""

DISTRO=$(detect_distro)
log_info "Detected distribution: ${DISTRO}"

case "$DISTRO" in
    ubuntu|debian|pop|linuxmint)
        log_info "Installing packages via apt..."
        sudo apt update
        sudo apt install -y \
            build-essential pkg-config git curl wget \
            libssl-dev libfontconfig1-dev libfreetype6-dev \
            libxcb-xfixes0-dev libxcb-shape0-dev libxkbcommon-dev \
            libsqlite3-dev openjdk-17-jdk \
            python3 python3-venv python3-pip openssl \
            graphviz
        ;;
    fedora|rhel|centos)
        log_info "Installing packages via dnf..."
        sudo dnf install -y \
            gcc gcc-c++ make pkg-config git curl wget \
            openssl-devel fontconfig-devel freetype-devel \
            libxcb-devel libxkbcommon-devel sqlite-devel \
            java-17-openjdk-devel \
            python3 python3-pip openssl \
            graphviz
        ;;
    arch|manjaro)
        log_info "Installing packages via pacman..."
        sudo pacman -S --needed --noconfirm \
            base-devel git curl wget \
            openssl fontconfig freetype2 libxcb libxkbcommon sqlite \
            jdk17-openjdk \
            python python-pip openssl \
            graphviz
        ;;
    *)
        log_error "Unsupported distribution: ${DISTRO}"
        log_warn "Please install dependencies manually and re-run"
        exit 1
        ;;
esac

log_success "System dependencies installed"

# ═══════════════════════════════════════════════════════════════════════════
# Step 2: Rust Toolchain
# ═══════════════════════════════════════════════════════════════════════════

echo ""
echo -e "${CYAN}═══ Step 2/6: Installing Rust ${REQUIRED_RUST_VERSION} ═══${NC}"
echo ""

if command_exists rustc; then
    CURRENT_RUST=$(rustc --version | awk '{print $2}')
    log_info "Found Rust ${CURRENT_RUST}"
    
    if [ "$CURRENT_RUST" != "$REQUIRED_RUST_VERSION" ]; then
        log_warn "Rust ${REQUIRED_RUST_VERSION} required, installing..."
        rustup toolchain install ${REQUIRED_RUST_VERSION}
        rustup default ${REQUIRED_RUST_VERSION}
    fi
else
    log_info "Rust not found, installing ${REQUIRED_RUST_VERSION}..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | \
        sh -s -- --default-toolchain ${REQUIRED_RUST_VERSION} -y
    source "$HOME/.cargo/env"
fi

rustup component add rustfmt clippy
log_success "Rust ${REQUIRED_RUST_VERSION} ready"

# ═══════════════════════════════════════════════════════════════════════════
# Step 3: Build OptaFly_Zed + OptaCore
# ═══════════════════════════════════════════════════════════════════════════

echo ""
echo -e "${CYAN}═══ Step 3/6: Building OptaFly_Zed (20-30 minutes) ═══${NC}"
echo ""

cd "$SCRIPT_DIR"

CORES=$(nproc)
log_info "Building with ${CORES} CPU cores..."

log_info "Building main editor..."
cargo build --release -j ${CORES}

log_info "Building OptaCore libraries..."
cargo build --release -p optacore_struct -j ${CORES}
cargo build --release -p optacore_jni -j ${CORES}

# Verify binaries
if [ ! -f "target/release/zed" ]; then
    log_error "Main Zed binary not found!"
    exit 1
fi

if [ ! -f "target/release/liboptacore_jni.so" ]; then
    log_warn "OptaCore JNI library not found (may be optional)"
fi

log_success "OptaFly_Zed built successfully"
log_info "Binary: ${SCRIPT_DIR}/target/release/zed"

# ═══════════════════════════════════════════════════════════════════════════
# Step 4: Widget-Log Python Setup
# ═══════════════════════════════════════════════════════════════════════════

echo ""
echo -e "${CYAN}═══ Step 4/6: Setting Up Widget-Log (10-15 minutes) ═══${NC}"
echo ""

cd "${SCRIPT_DIR}/widget-log"

# Check Python version
if command_exists python3; then
    PYTHON_VERSION=$(python3 --version | awk '{print $2}')
    log_info "Found Python ${PYTHON_VERSION}"
    
    if ! version_ge "$PYTHON_VERSION" "$REQUIRED_PYTHON_VERSION"; then
        log_error "Python ${REQUIRED_PYTHON_VERSION}+ required, found ${PYTHON_VERSION}"
        exit 1
    fi
else
    log_error "Python 3 not found!"
    exit 1
fi

# Create virtual environment
log_info "Creating Python virtual environment..."
python3 -m venv venv

# Activate venv
source venv/bin/activate

# Upgrade pip
log_info "Upgrading pip..."
pip install --upgrade pip --quiet

# Install dependencies (includes 500MB model download)
log_info "Installing Python dependencies..."
log_warn "This will download ~500MB of ML models (sentence-transformers)"
pip install -r requirements.txt

# Pre-download sentence-transformers model
log_info "Pre-downloading semantic similarity models..."
python3 << 'EOFPYTHON'
from sentence_transformers import SentenceTransformer
print("Loading all-MiniLM-L6-v2 model...")
model = SentenceTransformer('all-MiniLM-L6-v2')
print("Model ready!")
EOFPYTHON

log_success "Widget-Log Python environment ready"

# ═══════════════════════════════════════════════════════════════════════════
# Step 5: Configuration
# ═══════════════════════════════════════════════════════════════════════════

echo ""
echo -e "${CYAN}═══ Step 5/6: Configuring OptaFly_Zed ═══${NC}"
echo ""

# Create config directory
CONFIG_DIR="$HOME/.local/share/optafly-zed/widget-log"
mkdir -p "$CONFIG_DIR"

# Generate .env if not exists
ENV_FILE="${CONFIG_DIR}/.env"
if [ ! -f "$ENV_FILE" ]; then
    log_info "Creating .env configuration..."
    
    # Generate auth token
    AUTH_TOKEN=$(openssl rand -hex 32)
    
    cat > "$ENV_FILE" << EOFENV
# OptaFly_Zed Widget-Log Configuration
# Generated: $(date)

# Anthropic API Key (REQUIRED - get from https://console.anthropic.com)
ANTHROPIC_API_KEY=your_anthropic_api_key_here

# Widget-Log Authentication
WIDGET_LOG_AUTH_TOKEN=${AUTH_TOKEN}

# Cache Configuration
WIDGET_LOG_CACHE_DIR=${CONFIG_DIR}/cache
WIDGET_LOG_SIMILARITY_THRESHOLD=0.85
WIDGET_LOG_MAX_CACHE_SIZE_GB=5
EOFENV
    
    log_success "Created ${ENV_FILE}"
    log_warn "YOU MUST EDIT THIS FILE AND ADD YOUR ANTHROPIC API KEY!"
else
    log_info "Using existing .env file"
fi

# Generate SSL certificates
CERT_DIR="${SCRIPT_DIR}/widget-log/certs"
mkdir -p "$CERT_DIR"

if [ ! -f "${CERT_DIR}/cert.pem" ]; then
    log_info "Generating self-signed SSL certificate..."
    openssl req -x509 -newkey rsa:4096 \
        -keyout "${CERT_DIR}/key.pem" \
        -out "${CERT_DIR}/cert.pem" \
        -days 3650 -nodes \
        -subj "/CN=localhost/O=OptaFly_Zed/C=US" \
        2>/dev/null
    log_success "SSL certificates generated"
else
    log_info "Using existing SSL certificates"
fi

# Configure Zed settings
log_info "Configuring Zed editor..."
cd "${SCRIPT_DIR}/widget-log"
./configure-zed.sh

log_success "Configuration complete"

# ═══════════════════════════════════════════════════════════════════════════
# Step 6: Validation
# ═══════════════════════════════════════════════════════════════════════════

echo ""
echo -e "${CYAN}═══ Step 6/6: Validation ═══${NC}"
echo ""

# Test proxy startup
log_info "Testing Widget-Log proxy..."
cd "${SCRIPT_DIR}/widget-log"

# Start proxy in background
./start-proxy.sh &
PROXY_PID=$!

# Wait for startup
sleep 3

# Test health endpoint
if curl -k -s https://127.0.0.1:8443/health | grep -q "ok"; then
    log_success "Widget-Log proxy is running (PID: ${PROXY_PID})"
else
    log_warn "Proxy health check failed (may still be starting)"
fi

# Test binary
if "${SCRIPT_DIR}/target/release/zed" --version &>/dev/null; then
    VERSION=$("${SCRIPT_DIR}/target/release/zed" --version 2>/dev/null || echo "unknown")
    log_success "OptaFly_Zed binary validated: ${VERSION}"
else
    log_warn "Could not validate Zed binary"
fi

# ═══════════════════════════════════════════════════════════════════════════
# Installation Complete
# ═══════════════════════════════════════════════════════════════════════════

echo ""
echo -e "${GREEN}═══════════════════════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}                    Installation Complete! 🎉                               ${NC}"
echo -e "${GREEN}═══════════════════════════════════════════════════════════════════════════${NC}"
echo ""
echo -e "${CYAN}╔═══════════════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║                        Next Steps                                     ║${NC}"
echo -e "${CYAN}╠═══════════════════════════════════════════════════════════════════════╣${NC}"
echo -e "${CYAN}║${NC}  1. Configure your Anthropic API key:                               ${CYAN}║${NC}"
echo -e "${CYAN}║${NC}     ${YELLOW}nano ${ENV_FILE}${NC}"
echo -e "${CYAN}║${NC}                                                                      ${CYAN}║${NC}"
echo -e "${CYAN}║${NC}  2. Start OptaFly_Zed:                                              ${CYAN}║${NC}"
echo -e "${CYAN}║${NC}     ${YELLOW}${SCRIPT_DIR}/target/release/zed${NC}"
echo -e "${CYAN}║${NC}                                                                      ${CYAN}║${NC}"
echo -e "${CYAN}║${NC}  3. Widget-Log proxy is running on https://127.0.0.1:8443           ${CYAN}║${NC}"
echo -e "${CYAN}║${NC}                                                                      ${CYAN}║${NC}"
echo -e "${CYAN}║${NC}  4. Check cache statistics:                                         ${CYAN}║${NC}"
echo -e "${CYAN}║${NC}     ${YELLOW}curl -k https://127.0.0.1:8443/stats | jq '.'${NC}"
echo -e "${CYAN}║${NC}                                                                      ${CYAN}║${NC}"
echo -e "${CYAN}╚═══════════════════════════════════════════════════════════════════════╝${NC}"
echo ""
echo -e "${BLUE}Release Information:${NC}"
echo "  Version:    ${OPTAFLY_VERSION}"
echo "  Release:    ${RELEASE_URL}"
echo "  Docs:       ${GITHUB_REPO}/blob/main/INSTALL.md"
echo "  Log:        ${LOG_FILE}"
echo ""
echo -e "${BLUE}Features Installed:${NC}"
echo "  ✓ OptaFly_Zed Editor"
echo "  ✓ OptaCore Architecture Modeling"
echo "  ✓ Widget-Log Semantic Caching (280x faster AI)"
echo "  ✓ Structurizr JNI Bridge"
echo ""
echo -e "${YELLOW}Important:${NC} Edit ${ENV_FILE}"
echo -e "${YELLOW}          and add your Anthropic API key before first use!${NC}"
echo ""
echo -e "${GREEN}Thank you for using OptaFly_Zed!${NC}"
echo ""
