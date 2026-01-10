# OptaFly_Zed v1.0.0 - Installation Scripts Guide

## Quick Start

**One script per platform - installs EVERYTHING:**

### Linux
```bash
git clone https://github.com/Optaquan/OptaFly_Zed.git
cd OptaFly_Zed
./install.sh
```

### macOS
```bash
git clone https://github.com/Optaquan/OptaFly_Zed.git
cd OptaFly_Zed
./install-macos.sh
```

### Windows (PowerShell as Administrator)
```powershell
git clone https://github.com/Optaquan/OptaFly_Zed.git
cd OptaFly_Zed
.\install-windows.ps1
```

---

## What Gets Installed

Each script installs ALL OptaFly_Zed components:

1. ✅ **System Dependencies** (build tools, Java, Python, OpenSSL)
2. ✅ **Rust Toolchain** (1.91.1 exact version)
3. ✅ **Zed Editor** (built from source)
4. ✅ **OptaCore** (architecture modeling + Structurizr JNI)
5. ✅ **Widget-Log** (Python venv + semantic caching proxy)
6. ✅ **Configuration** (.env, SSL certs, Zed settings)

**No additional steps needed!**

---

## Installation Time

| Platform | Time | Notes |
|----------|------|-------|
| Linux | 40-60 min | Depends on CPU cores |
| macOS | 45-70 min | Xcode installation may add time |
| Windows | 50-80 min | Visual Studio Build Tools required |

**Most time is spent on Rust compilation (20-30 minutes)**

---

## What You Need

### Before Running

1. **Internet connection** (downloads dependencies)
2. **API Key** from https://console.anthropic.com
3. **Disk space**: ~13 GB
4. **RAM**: 4 GB minimum (8 GB recommended)

### Platform-Specific

**Linux:**
- `sudo` access for package installation

**macOS:**
- Xcode Command Line Tools (script will prompt)
- Homebrew (script installs if missing)

**Windows:**
- PowerShell 5.1+
- **Run as Administrator**
- Visual Studio 2022 Build Tools (manual install before script)

---

## Correct Script Names

**Use these scripts (complete installers):**

| Platform | Script | Status |
|----------|--------|--------|
| Linux | `install.sh` | ✅ Complete |
| macOS | `install-macos.sh` | ✅ Complete |
| Windows | `install-windows.ps1` | ✅ Complete |

**⚠️ OLD scripts (DO NOT USE):**
- ~~`install-phase25-parallel.sh`~~ (missing Widget-Log)
- ~~`install-parallel.sh`~~ (base Zed only)
- ~~`install-linux-v1.0.0.sh`~~ (duplicate)
- ~~`install-macos-v1.0.0.sh`~~ (duplicate)
- ~~`install-windows-v1.0.0.ps1`~~ (duplicate)

---

## After Installation

1. **Edit .env file** (required):
   ```bash
   # Linux/macOS
   nano ~/.local/share/optafly-zed/widget-log/.env
   
   # Windows
   notepad %LOCALAPPDATA%\optafly-zed\widget-log\.env
   ```
   
   Add your API key:
   ```
   ANTHROPIC_API_KEY=sk-ant-your-key-here
   ```

2. **Start OptaFly_Zed**:
   ```bash
   # Linux/macOS
   ./target/release/zed
   
   # Windows
   .\target\release\zed.exe
   ```

3. **Widget-Log starts automatically** on port 8443

---

## Troubleshooting

See [INSTALL.md](INSTALL.md) for detailed troubleshooting.

**Common issues:**

- **Build fails**: Check Rust version `rustc --version` (must be 1.91.1)
- **Python errors**: Check version `python3 --version` (must be 3.8+)
- **Proxy won't start**: Check API key in .env file
- **Permission denied**: Run with sudo (Linux) or as Administrator (Windows)

---

## Version Information

**Current Release**: v1.0.0  
**GitHub**: https://github.com/Optaquan/OptaFly_Zed  
**Release Notes**: https://github.com/Optaquan/OptaFly_Zed/releases/tag/v1.0.0

Each script logs to `optafly-install-YYYYMMDD-HHMMSS.log` for debugging.

---

**Copyright (c) 2025-2026 Tumquan Corp**
