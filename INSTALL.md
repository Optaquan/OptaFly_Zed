# OptaFly_Zed v1.0.0 - Installation Guide

Complete platform-specific installation instructions for OptaFly_Zed with Widget-Log semantic caching and OptaCore architecture modeling.

---

## Table of Contents

- [Quick Start (Recommended)](#quick-start-recommended)
- [System Requirements](#system-requirements)
- [Platform-Specific Installation](#platform-specific-installation)
  - [Linux](#linux-installation)
  - [macOS](#macos-installation)
  - [Windows](#windows-installation)
- [Manual Installation](#manual-installation)
- [Cargo Install Method](#cargo-install-method)
- [Verification](#verification)
- [Troubleshooting](#troubleshooting)
- [Upgrade from Previous Versions](#upgrade-from-previous-versions)

---

## Quick Start (Recommended)

The fastest way to install OptaFly_Zed v1.0.0:

```bash
# Clone repository
git clone https://github.com/Optaquan/OptaFly_Zed.git
cd OptaFly_Zed

# Run platform-specific installer
./install-phase25-parallel.sh      # Linux
./install-mac.sh                     # macOS
./install-windows.ps1                # Windows (PowerShell)
```

These scripts automatically:
- ✅ Check and install dependencies
- ✅ Build OptaFly_Zed in release mode
- ✅ Set up Widget-Log with virtual environment
- ✅ Configure API keys
- ✅ Start semantic caching proxy
- ✅ Launch OptaFly_Zed

---

## System Requirements

### Minimum Requirements

| Component | Requirement | Purpose |
|-----------|-------------|---------|
| **Rust** | 1.91.1 | Core editor build |
| **Python** | 3.12+ | Widget-Log proxy |
| **RAM** | 4 GB | Build and runtime |
| **Disk Space** | 5 GB | Source, build, cache |
| **CPU** | 2 cores | Minimum performance |

### Recommended Requirements

| Component | Recommendation | Benefit |
|-----------|----------------|---------|
| **Rust** | 1.91.1 (exact) | Tested version |
| **Python** | 3.12+ | Best compatibility |
| **RAM** | 8 GB+ | Faster builds |
| **Disk Space** | 10 GB+ | Room for cache growth |
| **CPU** | 4+ cores | Parallel compilation |
| **GPU** | Optional | Future FAISS-GPU support |

### Platform-Specific Dependencies

**All Platforms:**
- Git 2.0+
- Internet connection (initial setup)
- Anthropic API key ([get one here](https://console.anthropic.com))

**Linux:**
- Build essentials (gcc, g++, make)
- pkg-config
- OpenSSL development libraries
- Graphviz (optional, for OptaCore diagrams)

**macOS:**
- Xcode Command Line Tools
- Homebrew (recommended)

**Windows:**
- Visual Studio 2022 Build Tools
- Windows SDK

---

## Platform-Specific Installation

### Linux Installation

#### Ubuntu / Debian

```bash
# 1. Install system dependencies
sudo apt update
sudo apt install -y build-essential pkg-config libssl-dev \
    python3 python3-pip python3-venv graphviz git curl

# 2. Install Rust 1.91.1
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- \
    --default-toolchain 1.91.1 -y
source "$HOME/.cargo/env"

# 3. Verify Rust version
rustc --version  # Should output: rustc 1.91.1

# 4. Clone OptaFly_Zed
git clone https://github.com/Optaquan/OptaFly_Zed.git
cd OptaFly_Zed

# 5. Run installer (automated)
chmod +x install-phase25-parallel.sh
./install-phase25-parallel.sh
```

#### Fedora / RHEL / CentOS

```bash
# 1. Install system dependencies
sudo dnf install -y gcc gcc-c++ make pkg-config openssl-devel \
    python3 python3-pip graphviz git curl

# 2. Install Rust 1.91.1
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- \
    --default-toolchain 1.91.1 -y
source "$HOME/.cargo/env"

# 3. Clone and install
git clone https://github.com/Optaquan/OptaFly_Zed.git
cd OptaFly_Zed
chmod +x install-phase25-parallel.sh
./install-phase25-parallel.sh
```

#### Arch Linux

```bash
# 1. Install system dependencies
sudo pacman -S --needed base-devel openssl python python-pip graphviz git curl

# 2. Install Rust 1.91.1
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- \
    --default-toolchain 1.91.1 -y
source "$HOME/.cargo/env"

# 3. Clone and install
git clone https://github.com/Optaquan/OptaFly_Zed.git
cd OptaFly_Zed
chmod +x install-phase25-parallel.sh
./install-phase25-parallel.sh
```

---

### macOS Installation

#### Prerequisites

```bash
# 1. Install Xcode Command Line Tools
xcode-select --install

# 2. Install Homebrew (if not already installed)
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# 3. Install dependencies via Homebrew
brew install python@3.12 graphviz git

# 4. Install Rust 1.91.1
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- \
    --default-toolchain 1.91.1 -y
source "$HOME/.cargo/env"
```

#### Automated Installation

```bash
# Clone and run macOS installer
git clone https://github.com/Optaquan/OptaFly_Zed.git
cd OptaFly_Zed
chmod +x install-mac.sh
./install-mac.sh
```

---

### Windows Installation

#### Prerequisites

1. **Install Visual Studio Build Tools**
   - Download: [Visual Studio 2022 Build Tools](https://visualstudio.microsoft.com/downloads/)
   - Select "Desktop development with C++"
   - Install Windows 10/11 SDK

2. **Install Chocolatey** (package manager)
   ```powershell
   # Run in Administrator PowerShell
   Set-ExecutionPolicy Bypass -Scope Process -Force
   [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072
   iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))
   ```

3. **Install Dependencies**
   ```powershell
   # Run in Administrator PowerShell
   choco install -y git python graphviz
   ```

4. **Install Rust 1.91.1**
   - Download: [rustup-init.exe](https://win.rustup.rs/)
   - Run: `rustup-init.exe --default-toolchain 1.91.1`
   - Restart terminal after installation

#### Automated Installation

```powershell
# Run in PowerShell (Administrator)
git clone https://github.com/Optaquan/OptaFly_Zed.git
cd OptaFly_Zed
.\install-windows.ps1
```

---

## Manual Installation

For advanced users who want full control:

### Step 1: Install Dependencies

See platform-specific sections above for dependency installation.

### Step 2: Clone Repository

```bash
git clone https://github.com/Optaquan/OptaFly_Zed.git
cd OptaFly_Zed
```

### Step 3: Build OptaFly_Zed

```bash
# Full release build (10-30 minutes depending on hardware)
cargo build --release

# Or build with more parallel jobs (faster on multi-core systems)
cargo build --release -j 8  # Use 8 cores
```

**Build output:** `target/release/zed` (Linux/macOS) or `target/release/zed.exe` (Windows)

### Step 4: Set Up Widget-Log

```bash
cd widget-log

# Create Python virtual environment
python3 -m venv venv

# Activate virtual environment
source venv/bin/activate  # Linux/macOS
# OR
venv\Scripts\activate  # Windows

# Install dependencies
pip install -r requirements.txt
```

### Step 5: Configure API Key

Create or edit `~/.local/share/optafly-zed/widget-log/.env`:

```bash
# Linux/macOS
mkdir -p ~/.local/share/optafly-zed/widget-log
cat > ~/.local/share/optafly-zed/widget-log/.env << EOF
ANTHROPIC_API_KEY=your_key_here
WIDGET_LOG_AUTH_TOKEN=$(openssl rand -hex 32)
EOF
```

```powershell
# Windows
New-Item -ItemType Directory -Force -Path "$env:LOCALAPPDATA\optafly-zed\widget-log"
Set-Content "$env:LOCALAPPDATA\optafly-zed\widget-log\.env" @"
ANTHROPIC_API_KEY=your_key_here
WIDGET_LOG_AUTH_TOKEN=$(-join ((48..57) + (97..102) | Get-Random -Count 64 | ForEach-Object {[char]$_}))
"@
```

Get your API key from: [console.anthropic.com](https://console.anthropic.com)

### Step 6: Start Widget-Log Proxy

```bash
cd widget-log
./start-proxy.sh  # Linux/macOS
# OR
.\start-proxy.bat  # Windows
```

### Step 7: Run OptaFly_Zed

```bash
cd ..
./target/release/zed  # Linux/macOS
# OR
.\target\release\zed.exe  # Windows
```

---

## Cargo Install Method

**Coming Soon:** OptaFly_Zed will be published to crates.io as `optafly-zed` v1.0.0.

Once published, installation will be as simple as:

```bash
# Install from crates.io
cargo install optafly-zed --version 1.0.0 --features widget-log,optacore

# Run
optafly-zed
```

**Note:** This method is planned for v1.0.1+ after initial release validation.

---

## Verification

### 1. Check Rust Version

```bash
rustc --version
# Expected: rustc 1.91.1 (c9c80678c 2024-12-16)
```

If version is incorrect:
```bash
rustup default 1.91.1
```

### 2. Check Python Version

```bash
python3 --version
# Expected: Python 3.12.0 or higher
```

### 3. Verify Build Success

```bash
./target/release/zed --version
# Expected: zed 1.0.0
```

### 4. Check Widget-Log Proxy

```bash
# Proxy should be running
ps aux | grep secure_proxy

# Health check
curl -k https://127.0.0.1:8443/health
# Expected: {"status":"ok"}
```

### 5. Test Cache Statistics

```bash
# Get auth token
TOKEN=$(grep WIDGET_LOG_AUTH_TOKEN ~/.local/share/optafly-zed/widget-log/.env | cut -d= -f2)

# Query stats
curl -k -H "Authorization: Bearer $TOKEN" https://127.0.0.1:8443/stats | jq '.'
# Expected: JSON with cache statistics
```

---

## Troubleshooting

### Build Issues

#### Error: "Rust version too old"

```bash
# Update to Rust 1.91.1
rustup update 1.91.1
rustup default 1.91.1
```

#### Error: "linker `cc` not found"

**Linux:**
```bash
# Ubuntu/Debian
sudo apt install build-essential

# Fedora/RHEL
sudo dnf install gcc gcc-c++

# Arch
sudo pacman -S base-devel
```

**macOS:**
```bash
xcode-select --install
```

**Windows:**
Install Visual Studio Build Tools (see prerequisites above)

#### Error: "Could not find OpenSSL"

**Linux:**
```bash
# Ubuntu/Debian
sudo apt install libssl-dev pkg-config

# Fedora/RHEL
sudo dnf install openssl-devel pkg-config

# Arch
sudo pacman -S openssl pkg-config
```

**macOS:**
```bash
brew install openssl@3
export OPENSSL_DIR=$(brew --prefix openssl@3)
```

#### Build takes too long

```bash
# Use more CPU cores (replace 8 with your core count)
cargo build --release -j 8

# Use faster linker (Linux only)
sudo apt install mold
export RUSTFLAGS="-C link-arg=-fuse-ld=mold"
cargo build --release
```

### Widget-Log Issues

#### Proxy won't start

**Check Python version:**
```bash
python3 --version  # Must be 3.12+
```

**Check dependencies:**
```bash
cd widget-log
source venv/bin/activate
pip list | grep -E "(anthropic|sentence-transformers|faiss)"
```

**Reinstall dependencies:**
```bash
cd widget-log
source venv/bin/activate
pip install -r requirements.txt --upgrade
```

#### Port 8443 already in use

**Find what's using the port:**
```bash
# Linux/macOS
sudo lsof -i :8443

# Windows
netstat -ano | findstr :8443
```

**Kill the process or change Widget-Log port** in `widget-log/config.yaml`

#### API key not recognized

**Verify .env file:**
```bash
cat ~/.local/share/optafly-zed/widget-log/.env
# Should contain: ANTHROPIC_API_KEY=sk-ant-...
```

**Test API key directly:**
```bash
curl https://api.anthropic.com/v1/messages \
  -H "x-api-key: YOUR_KEY" \
  -H "anthropic-version: 2023-06-01" \
  -H "content-type: application/json" \
  -d '{"model":"claude-3-5-sonnet-20241022","max_tokens":10,"messages":[{"role":"user","content":"Hi"}]}'
```

### Runtime Issues

#### Zed crashes on startup

**Check logs:**
```bash
# Linux
tail -f ~/.local/share/optafly-zed/logs/Zed.log

# macOS
tail -f ~/Library/Logs/OptaFly_Zed/Zed.log

# Windows
Get-Content "$env:LOCALAPPDATA\optafly-zed\logs\Zed.log" -Wait
```

#### Cache not working

**Verify proxy connection:**
```bash
curl -k https://127.0.0.1:8443/health
```

**Check Zed settings:**
```bash
# Linux/macOS
cat ~/.config/zed/settings.json | grep "127.0.0.1:8443"

# Windows
Get-Content "$env:APPDATA\Zed\settings.json" | Select-String "127.0.0.1:8443"
```

**View Widget-Log logs:**
```bash
tail -f ~/.local/share/optafly-zed/widget-log/logs/widget-log.log
```

---

## Upgrade from Previous Versions

### From v0.99 to v1.0.0

```bash
cd OptaFly_Zed

# Pull latest changes
git fetch origin
git checkout v1.0.0

# Rebuild
cargo clean
cargo build --release

# Update Widget-Log dependencies
cd widget-log
source venv/bin/activate
pip install -r requirements.txt --upgrade

# Restart proxy
./stop-proxy.sh
./start-proxy.sh
```

### From v0.95-0.98 to v1.0.0

**Major changes:**
- Rust toolchain pinned to 1.91.1
- Python dependencies pinned with exact versions
- New installation scripts

**Recommended:** Fresh installation to avoid compatibility issues.

```bash
# Backup your settings
cp -r ~/.config/zed ~/.config/zed.backup
cp -r ~/.local/share/optafly-zed ~/.local/share/optafly-zed.backup

# Fresh clone and install
cd ..
git clone https://github.com/Optaquan/OptaFly_Zed.git OptaFly_Zed_v1.0.0
cd OptaFly_Zed_v1.0.0
./install-phase25-parallel.sh  # Or platform-specific script
```

---

## Getting Help

**For installation issues:**
- 🐛 GitHub Issues: [Optaquan/OptaFly_Zed/issues](https://github.com/Optaquan/OptaFly_Zed/issues)
- 📚 Documentation: [README.md](README.md), [BUILD_INSTRUCTIONS.md](BUILD_INSTRUCTIONS.md)
- 💬 Discussions: [GitHub Discussions](https://github.com/Optaquan/OptaFly_Zed/discussions)

**When reporting issues, include:**
- Operating system and version
- Rust version (`rustc --version`)
- Python version (`python3 --version`)
- Error messages and logs
- Steps to reproduce

---

## Next Steps

After successful installation:

1. **Configure Zed Settings**: Customize editor preferences in `~/.config/zed/settings.json`
2. **Test Widget-Log**: Make AI queries and verify caching with `curl stats endpoint`
3. **Explore OptaCore**: Try architecture modeling with `crates/optacore_jni/examples/`
4. **Read Documentation**: Check [CONTRIBUTING.md](CONTRIBUTING.md) for development guidelines
5. **Join Community**: Star the repo, report issues, contribute improvements!

---

**Copyright (c) 2025-2026 Tumquan Corp**  
**OptaFly_Zed is a derivative work of Zed editor**  
**Licensed under AGPL-3.0 with MIT/Apache-2.0 for extracted components**
