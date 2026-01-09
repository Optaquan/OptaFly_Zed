# OptaFly_Zed - Complete Build Instructions

**Universal build guide for all platforms and components**

---

## 📋 Table of Contents

1. [Prerequisites](#prerequisites)
2. [Building OptaFly_Zed (Main Editor)](#building-optafly_zed-main-editor)
3. [Building OptaCore (Architecture Optimization)](#building-optacore-architecture-optimization)
4. [Building OptaCore JNI Bridge (Java Integration)](#building-optacore-jni-bridge-java-integration)
5. [Platform-Specific Instructions](#platform-specific-instructions)
6. [Troubleshooting](#troubleshooting)
7. [Testing](#testing)

---

## Prerequisites

### Required for All Builds

#### Rust Toolchain
```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Update to latest stable
rustup update stable
rustup default stable

# Verify installation
rustc --version  # Should be 1.75.0 or newer
cargo --version
```

#### Build Tools

**Linux (Ubuntu/Debian)**:
```bash
sudo apt update
sudo apt install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    libfontconfig1-dev \
    libfreetype6-dev \
    libxcb-xfixes0-dev \
    libxcb-shape0-dev \
    libxkbcommon-dev \
    libsqlite3-dev \
    git \
    curl
```

**macOS**:
```bash
# Install Xcode Command Line Tools
xcode-select --install

# Install Homebrew (if not installed)
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install dependencies
brew install pkg-config openssl
```

**Windows**:
```powershell
# Install Visual Studio 2022 (Community Edition)
# Download from: https://visualstudio.microsoft.com/downloads/
# Select "Desktop development with C++" workload

# Install Rust using rustup-init.exe
# Download from: https://win.rustup.rs/
```

---

## Building OptaFly_Zed (Main Editor)

### Quick Build (All Platforms)

```bash
# 1. Clone repository
git clone https://github.com/Optaquan/OptaFly_Zed.git
cd OptaFly_Zed

# 2. Initialize submodules
git submodule update --init --recursive

# 3. Build (Debug)
cargo build

# 4. Build (Release - Optimized)
cargo build --release

# 5. Run
./target/release/zed
```

### Build Time Expectations

| Configuration | Time (First Build) | Incremental |
|---------------|-------------------|-------------|
| Debug         | 15-30 minutes     | 30-60s      |
| Release       | 20-40 minutes     | 1-2 minutes |

**Note**: First build downloads and compiles 800+ dependencies. Subsequent builds are much faster.

---

### Platform-Specific Build Commands

#### Linux
```bash
# Standard build
cargo build --release

# With system libraries (faster linking)
cargo build --release --features system-libs

# Output: target/release/zed
```

#### macOS
```bash
# Universal binary (Intel + Apple Silicon)
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin

# Create universal binary
lipo -create \
    target/x86_64-apple-darwin/release/zed \
    target/aarch64-apple-darwin/release/zed \
    -output zed-universal

# Or build for current architecture only
cargo build --release

# Output: target/release/zed
```

#### Windows
```powershell
# Ensure you're in PowerShell or cmd with MSVC environment

# Standard build
cargo build --release

# Output: target\release\zed.exe
```

---

## Building OptaCore (Architecture Optimization)

OptaCore is a tensor-based C4 architecture modeling library with force-directed layout optimization.

### Prerequisites

**All platforms**: Rust toolchain (see above)

### Build Commands

```bash
cd OptaFly_Zed

# Build OptaCore library
cargo build --release --package optacore_struct

# Run tests
cargo test --package optacore_struct

# Expected output:
# running 24 tests
# test result: ok. 24 passed; 0 failed
```

### Build with Optional Features

```bash
# With telemetry (ML training data collection)
cargo build --release --package optacore_struct --features telemetry

# With GPU acceleration (requires CUDA/Metal/Vulkan)
cargo build --release --package optacore_struct --features wgpu

# With WASM support (browser-based)
cargo build --release --package optacore_struct --target wasm32-unknown-unknown
```

### Output Location

- **Library**: `target/release/liboptacore_struct.rlib`
- **Examples**: `target/release/examples/`

### Run Examples

```bash
# Telemetry demo
cargo run --release --package optacore_struct --example telemetry_demo

# Visualization demo
cargo run --release --package optacore_struct --example visualize

# Output files: telemetry_demo.jsonl, *.dot files
```

---

## Building OptaCore JNI Bridge (Java Integration)

Enables Java applications (Structurizr) to use OptaCore via JNI.

### Additional Prerequisites

#### Java Development Kit (JDK)

**All platforms**:
```bash
# Verify Java is installed
java -version   # Should be 11 or newer
javac -version

# If not installed:

# Linux
sudo apt install openjdk-17-jdk

# macOS
brew install openjdk@17

# Windows
# Download from https://adoptium.net/
```

#### Set JAVA_HOME

**Linux/macOS**:
```bash
export JAVA_HOME=/usr/lib/jvm/java-17-openjdk-amd64  # Linux
export JAVA_HOME=/opt/homebrew/opt/openjdk@17        # macOS

# Add to ~/.bashrc or ~/.zshrc for persistence
```

**Windows**:
```powershell
# Set environment variable permanently
setx JAVA_HOME "C:\Program Files\Java\jdk-17"

# Verify
echo %JAVA_HOME%
```

### Build Commands

```bash
cd OptaFly_Zed

# Build JNI library
cargo build --release --package optacore_jni

# Output locations:
# Linux:   target/release/liboptacore_jni.so
# macOS:   target/release/liboptacore_jni.dylib
# Windows: target/release/optacore_jni.dll
```

### Verify Build

```bash
# Check file size (should be ~15MB)
ls -lh target/release/liboptacore_jni.so  # Linux
ls -lh target/release/liboptacore_jni.dylib  # macOS
dir target\release\optacore_jni.dll  # Windows

# Run tests
cargo test --release --package optacore_jni

# Expected output:
# running 3 tests
# test result: ok. 3 passed; 0 failed
```

### Build for Multiple Platforms (Cross-Compilation)

#### From Linux

```bash
# Install cross-compilation toolchains
rustup target add x86_64-unknown-linux-gnu
rustup target add aarch64-unknown-linux-gnu
rustup target add x86_64-pc-windows-gnu

# Build for different targets
cargo build --release --package optacore_jni --target x86_64-unknown-linux-gnu
cargo build --release --package optacore_jni --target aarch64-unknown-linux-gnu

# Windows requires mingw
sudo apt install mingw-w64
cargo build --release --package optacore_jni --target x86_64-pc-windows-gnu
```

#### From macOS

```bash
# Add targets
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin

# Build universal binary components
cargo build --release --package optacore_jni --target x86_64-apple-darwin
cargo build --release --package optacore_jni --target aarch64-apple-darwin

# Create universal library
lipo -create \
    target/x86_64-apple-darwin/release/liboptacore_jni.dylib \
    target/aarch64-apple-darwin/release/liboptacore_jni.dylib \
    -output target/release/liboptacore_jni-universal.dylib
```

---

### Testing Java Integration

```bash
cd crates/optacore_jni/java

# Compile Java wrapper
javac com/optafly/structurizr/OptaCoreJNI.java

# Run demo (Linux/macOS)
export LD_LIBRARY_PATH=../../../target/release:$LD_LIBRARY_PATH
java -Djava.library.path=../../../target/release com.optafly.structurizr.OptaCoreJNI

# Run demo (Windows)
set PATH=%PATH%;..\..\..\target\release
java -Djava.library.path=..\..\..\target\release com.optafly.structurizr.OptaCoreJNI

# Expected output:
# OptaCore JNI Version: 0.1.0
# Library loaded: true
# [Demo workflow output...]
```

### Enable Debug Logging

```bash
# Set before running Java
export OPTACORE_JNI_DEBUG=1  # Linux/macOS
set OPTACORE_JNI_DEBUG=1     # Windows

# Then run Java application - you'll see:
# [OptaCore JNI] Entering parseDsl
# [OptaCore JNI] Parsing DSL (234 chars)
# [OptaCore JNI] Success: 1456 bytes
```

---

## Platform-Specific Instructions

### Linux (Ubuntu/Debian)

#### Full Build Sequence

```bash
# 1. Install all dependencies
sudo apt update
sudo apt install -y \
    build-essential pkg-config libssl-dev libfontconfig1-dev \
    libfreetype6-dev libxcb-xfixes0-dev libxcb-shape0-dev \
    libxkbcommon-dev libsqlite3-dev git curl openjdk-17-jdk

# 2. Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# 3. Clone and build
git clone https://github.com/Optaquan/OptaFly_Zed.git
cd OptaFly_Zed

# 4. Build everything
cargo build --release                            # Main editor
cargo build --release --package optacore_struct  # OptaCore
cargo build --release --package optacore_jni     # JNI bridge

# 5. Run tests
./script/clippy  # Lint check
cargo test --workspace

# 6. Run OptaFly_Zed
./target/release/zed
```

#### Common Issues

**Missing libssl**:
```bash
sudo apt install libssl-dev pkg-config
```

**Missing X11 libraries**:
```bash
sudo apt install libxcb-xfixes0-dev libxcb-shape0-dev libxkbcommon-dev
```

---

### macOS (Intel & Apple Silicon)

#### Full Build Sequence

```bash
# 1. Install Xcode Command Line Tools
xcode-select --install

# 2. Install Homebrew dependencies
brew install pkg-config openssl openjdk@17

# 3. Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# 4. Clone and build
git clone https://github.com/Optaquan/OptaFly_Zed.git
cd OptaFly_Zed

# 5. Build for current architecture
cargo build --release                            # Main editor
cargo build --release --package optacore_struct  # OptaCore
cargo build --release --package optacore_jni     # JNI bridge

# 6. Run tests
./script/clippy
cargo test --workspace

# 7. Run OptaFly_Zed
./target/release/zed
```

#### Build Universal Binary (Optional)

```bash
# Add both targets
rustup target add x86_64-apple-darwin aarch64-apple-darwin

# Build for both architectures
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin

# Create universal binary
lipo -create \
    target/x86_64-apple-darwin/release/zed \
    target/aarch64-apple-darwin/release/zed \
    -output target/release/zed-universal
```

#### Common Issues

**Xcode not found**:
```bash
sudo xcode-select --reset
xcode-select --install
```

**OpenSSL linking errors**:
```bash
export PKG_CONFIG_PATH="/opt/homebrew/opt/openssl@3/lib/pkgconfig"
cargo clean
cargo build --release
```

---

### Windows (MSVC)

#### Full Build Sequence

```powershell
# 1. Install Visual Studio 2022 Community
# Download: https://visualstudio.microsoft.com/downloads/
# Select: "Desktop development with C++"

# 2. Install Rust
# Download: https://win.rustup.rs/
# Run rustup-init.exe and follow prompts

# 3. Install Java JDK
# Download: https://adoptium.net/temurin/releases/
# Install and set JAVA_HOME

# 4. Open "x64 Native Tools Command Prompt for VS 2022"

# 5. Clone and build
git clone https://github.com/Optaquan/OptaFly_Zed.git
cd OptaFly_Zed

# 6. Build components
cargo build --release                            # Main editor
cargo build --release --package optacore_struct  # OptaCore
cargo build --release --package optacore_jni     # JNI bridge

# 7. Run tests
cargo test --workspace

# 8. Run OptaFly_Zed
target\release\zed.exe
```

#### Common Issues

**MSVC not found**:
```powershell
# Install Visual Studio with C++ tools
# OR install Build Tools standalone:
# https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022
```

**Link errors**:
```powershell
# Ensure you're in VS Native Tools prompt, not regular PowerShell
# Shortcut: Start Menu → Visual Studio 2022 → x64 Native Tools Command Prompt
```

**OpenSSL missing**:
```powershell
# Install via vcpkg
git clone https://github.com/microsoft/vcpkg
cd vcpkg
.\bootstrap-vcpkg.bat
.\vcpkg install openssl:x64-windows
```

---

## Troubleshooting

### Build Failures

#### "Could not compile X"

```bash
# Clear build cache
cargo clean

# Update dependencies
cargo update

# Rebuild
cargo build --release
```

#### "Linking with 'cc' failed"

```bash
# Linux: Install build-essential
sudo apt install build-essential

# macOS: Install Xcode tools
xcode-select --install

# Windows: Use VS Native Tools prompt
```

#### "Could not find JNI headers"

```bash
# Ensure JAVA_HOME is set
echo $JAVA_HOME  # Linux/macOS
echo %JAVA_HOME%  # Windows

# If not set:
export JAVA_HOME=/path/to/jdk  # Linux/macOS
setx JAVA_HOME "C:\Path\To\JDK"  # Windows (permanent)
set JAVA_HOME="C:\Path\To\JDK"   # Windows (current session)

# Rebuild
cargo clean
cargo build --release --package optacore_jni
```

---

### Performance Issues

#### Slow Builds

```bash
# Use parallel compilation
export CARGO_BUILD_JOBS=8  # Adjust to your CPU core count

# Use sccache for incremental builds
cargo install sccache
export RUSTC_WRAPPER=sccache
```

#### Out of Memory

```bash
# Reduce parallel jobs
export CARGO_BUILD_JOBS=2

# Or build in debug mode (faster, less memory)
cargo build
```

---

### Runtime Issues

#### "Library not found" (JNI)

```bash
# Linux
export LD_LIBRARY_PATH=/path/to/target/release:$LD_LIBRARY_PATH

# macOS
export DYLD_LIBRARY_PATH=/path/to/target/release:$DYLD_LIBRARY_PATH

# Windows
set PATH=%PATH%;C:\path\to\target\release
```

#### "UnsatisfiedLinkError" (Java)

```bash
# Verify library exists
ls target/release/liboptacore_jni.so  # Should show ~15MB file

# Check JAVA_HOME
java -XshowSettings:properties 2>&1 | grep java.home

# Rebuild JNI library
cargo clean
cargo build --release --package optacore_jni
```

---

## Testing

### Run All Tests

```bash
# Full test suite
cargo test --workspace --release

# Expected output:
# running 27+ tests
# test result: ok. 27 passed; 0 failed
```

### Component-Specific Tests

```bash
# OptaFly_Zed main
cargo test --release

# OptaCore
cargo test --release --package optacore_struct

# OptaCore JNI
cargo test --release --package optacore_jni

# With debug output
cargo test --release -- --nocapture
```

### Lint Check

```bash
# Run Clippy (Rust linter)
./script/clippy

# Or manually
cargo clippy --workspace --all-targets --all-features
```

---

## Build Artifacts

After successful build, you'll have:

### Main Editor
```
target/release/zed              # Linux/macOS executable
target/release/zed.exe          # Windows executable
```

### OptaCore Library
```
target/release/liboptacore_struct.rlib  # Rust library
target/release/examples/telemetry_demo  # Example binaries
target/release/examples/visualize
```

### JNI Bridge
```
target/release/liboptacore_jni.so      # Linux
target/release/liboptacore_jni.dylib   # macOS
target/release/optacore_jni.dll        # Windows
```

---

## Quick Reference

### Build Commands Cheat Sheet

```bash
# Main editor (debug)
cargo build

# Main editor (release)
cargo build --release

# OptaCore only
cargo build --release -p optacore_struct

# JNI bridge only
cargo build --release -p optacore_jni

# Everything
cargo build --release --workspace

# Run tests
cargo test --workspace

# Clean build
cargo clean && cargo build --release

# Check without building
cargo check

# Format code
cargo fmt

# Lint
./script/clippy
```

---

## Getting Help

- **Build Issues**: [GitHub Issues](https://github.com/Optaquan/OptaFly_Zed/issues)
- **OptaCore Docs**: `crates/optacore_jni/README.md`
- **JNI Integration**: `crates/optacore_jni/QUICKSTART.md`
- **Zed Docs**: `docs/src/development/`

---

## Next Steps

After successful build:

1. **Run OptaFly_Zed**: `./target/release/zed`
2. **Configure Widget-Log**: See main [README.md](./README.md)
3. **Try OptaCore**: See [crates/optacore_jni/QUICKSTART.md](./crates/optacore_jni/QUICKSTART.md)
4. **Contribute**: See [CONTRIBUTING.md](./CONTRIBUTING.md)

---

**Build Time Summary**:
- First build: 20-40 minutes (downloads dependencies)
- Incremental: 30 seconds - 2 minutes
- Clean rebuild: 15-30 minutes

**Disk Space Required**:
- Source: ~500MB
- Dependencies: ~2GB
- Build artifacts: ~3GB
- **Total**: ~5.5GB

---

**Happy Building! 🚀**
