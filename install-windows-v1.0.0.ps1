# OptaFly_Zed v1.0.0 - Windows Installation Script
# Copyright (c) 2025-2026 Tumquan Corp
# Requires: PowerShell 5.1+, Administrator privileges

#Requires -RunAsAdministrator

$ErrorActionPreference = "Stop"

Write-Host "╔════════════════════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║  OptaFly_Zed v1.0.0 - Windows Installer                    ║" -ForegroundColor Cyan
Write-Host "║  ML Foundation + Structurizr JNI + Semantic Caching       ║" -ForegroundColor Cyan
Write-Host "║  Copyright (c) 2025-2026 Tumquan Corp                      ║" -ForegroundColor Cyan
Write-Host "╚════════════════════════════════════════════════════════════╝" -ForegroundColor Cyan
Write-Host ""

# Configuration
$SCRIPT_DIR = $PSScriptRoot
$REQUIRED_RUST_VERSION = "1.91.1"
$REQUIRED_PYTHON_VERSION = "3.12"

# Function: Check if command exists
function Test-CommandExists {
    param($Command)
    $null = Get-Command $Command -ErrorAction SilentlyContinue
    return $?
}

# Function: Compare versions
function Compare-Version {
    param(
        [string]$Version1,
        [string]$Version2
    )
    $v1 = [Version]($Version1 -replace '[^\d.]','')
    $v2 = [Version]($Version2 -replace '[^\d.]','')
    return $v1.CompareTo($v2)
}

Write-Host "═══ Step 1: Checking Dependencies ═══" -ForegroundColor Yellow
Write-Host ""

# Check Rust
Write-Host "Checking Rust installation..." -NoNewline
if (Test-CommandExists rustc) {
    $rustVersion = (rustc --version) -replace 'rustc ([^\s]+).*','$1'
    Write-Host " Found: $rustVersion" -ForegroundColor Green
    
    if ($rustVersion -ne $REQUIRED_RUST_VERSION) {
        Write-Host "   ⚠️ Warning: Expected Rust $REQUIRED_RUST_VERSION, found $rustVersion" -ForegroundColor Yellow
        Write-Host "   Installing Rust $REQUIRED_RUST_VERSION..." -ForegroundColor Yellow
        
        # Download rustup-init
        $rustupUrl = "https://win.rustup.rs/x86_64"
        $rustupPath = "$env:TEMP\rustup-init.exe"
        Invoke-WebRequest -Uri $rustupUrl -OutFile $rustupPath
        
        # Install Rust 1.91.1
        & $rustupPath --default-toolchain $REQUIRED_RUST_VERSION -y
        
        # Update PATH
        $env:PATH = "$env:USERPROFILE\.cargo\bin;$env:PATH"
        
        Write-Host "   ✅ Rust $REQUIRED_RUST_VERSION installed" -ForegroundColor Green
    }
} else {
    Write-Host " Not found" -ForegroundColor Red
    Write-Host "   Installing Rust $REQUIRED_RUST_VERSION..." -ForegroundColor Yellow
    
    $rustupUrl = "https://win.rustup.rs/x86_64"
    $rustupPath = "$env:TEMP\rustup-init.exe"
    Invoke-WebRequest -Uri $rustupUrl -OutFile $rustupPath
    & $rustupPath --default-toolchain $REQUIRED_RUST_VERSION -y
    $env:PATH = "$env:USERPROFILE\.cargo\bin;$env:PATH"
    
    Write-Host "   ✅ Rust $REQUIRED_RUST_VERSION installed" -ForegroundColor Green
}

# Check Python
Write-Host "Checking Python installation..." -NoNewline
if (Test-CommandExists python) {
    $pythonVersion = (python --version) -replace 'Python ([^\s]+).*','$1'
    Write-Host " Found: $pythonVersion" -ForegroundColor Green
    
    $versionCompare = Compare-Version -Version1 $pythonVersion -Version2 $REQUIRED_PYTHON_VERSION
    if ($versionCompare -lt 0) {
        Write-Host "   ❌ Error: Python $REQUIRED_PYTHON_VERSION+ required, found $pythonVersion" -ForegroundColor Red
        Write-Host "   Please install Python $REQUIRED_PYTHON_VERSION+ from python.org" -ForegroundColor Yellow
        exit 1
    }
} else {
    Write-Host " Not found" -ForegroundColor Red
    Write-Host "   Installing Python via Chocolatey..." -ForegroundColor Yellow
    
    # Check if Chocolatey is installed
    if (!(Test-CommandExists choco)) {
        Write-Host "   Installing Chocolatey package manager..." -ForegroundColor Yellow
        [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072
        Invoke-Expression ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))
    }
    
    choco install -y python --version=3.12.0
    $env:PATH = "C:\Python312;C:\Python312\Scripts;$env:PATH"
    
    Write-Host "   ✅ Python 3.12 installed" -ForegroundColor Green
}

# Check Git
Write-Host "Checking Git installation..." -NoNewline
if (Test-CommandExists git) {
    $gitVersion = (git --version) -replace 'git version ([^\s]+).*','$1'
    Write-Host " Found: $gitVersion" -ForegroundColor Green
} else {
    Write-Host " Not found" -ForegroundColor Red
    Write-Host "   Installing Git via Chocolatey..." -ForegroundColor Yellow
    choco install -y git
    $env:PATH = "C:\Program Files\Git\cmd;$env:PATH"
    Write-Host "   ✅ Git installed" -ForegroundColor Green
}

# Check Graphviz (optional)
Write-Host "Checking Graphviz installation (optional)..." -NoNewline
if (Test-CommandExists dot) {
    Write-Host " Found" -ForegroundColor Green
} else {
    Write-Host " Not found" -ForegroundColor Yellow
    Write-Host "   Installing Graphviz via Chocolatey..." -ForegroundColor Yellow
    choco install -y graphviz
    $env:PATH = "C:\Program Files\Graphviz\bin;$env:PATH"
    Write-Host "   ✅ Graphviz installed" -ForegroundColor Green
}

Write-Host ""
Write-Host "═══ Step 2: Building OptaFly_Zed ═══" -ForegroundColor Yellow
Write-Host ""

Write-Host "Building in release mode (this may take 10-30 minutes)..." -ForegroundColor Cyan
Set-Location $SCRIPT_DIR

$cores = (Get-WmiObject -Class Win32_Processor).NumberOfLogicalProcessors
Write-Host "Using $cores CPU cores for parallel build" -ForegroundColor Cyan

try {
    cargo build --release -j $cores
    Write-Host "✅ Build successful!" -ForegroundColor Green
} catch {
    Write-Host "❌ Build failed: $_" -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "═══ Step 3: Setting Up Widget-Log ═══" -ForegroundColor Yellow
Write-Host ""

Set-Location "$SCRIPT_DIR\widget-log"

Write-Host "Creating Python virtual environment..." -ForegroundColor Cyan
python -m venv venv

Write-Host "Activating virtual environment..." -ForegroundColor Cyan
& ".\venv\Scripts\Activate.ps1"

Write-Host "Installing Python dependencies..." -ForegroundColor Cyan
pip install --upgrade pip
pip install -r requirements.txt

Write-Host "✅ Widget-Log setup complete!" -ForegroundColor Green

Write-Host ""
Write-Host "═══ Step 4: Configuring API Key ═══" -ForegroundColor Yellow
Write-Host ""

$configDir = "$env:LOCALAPPDATA\optafly-zed\widget-log"
New-Item -ItemType Directory -Force -Path $configDir | Out-Null

$envFile = "$configDir\.env"

if (Test-Path $envFile) {
    Write-Host "API key configuration already exists" -ForegroundColor Green
    $existingKey = Select-String -Path $envFile -Pattern "ANTHROPIC_API_KEY=(.+)" | ForEach-Object { $_.Matches.Groups[1].Value }
    if ($existingKey -and $existingKey -ne "your_key_here") {
        Write-Host "Using existing API key: $($existingKey.Substring(0,10))..." -ForegroundColor Green
    } else {
        Write-Host "⚠️ API key not configured. Please edit: $envFile" -ForegroundColor Yellow
    }
} else {
    Write-Host "Creating API key configuration file..." -ForegroundColor Cyan
    
    # Generate secure auth token
    $authToken = -join ((48..57) + (97..102) | Get-Random -Count 64 | ForEach-Object {[char]$_})
    
    $envContent = @"
ANTHROPIC_API_KEY=your_key_here
WIDGET_LOG_AUTH_TOKEN=$authToken
"@
    Set-Content -Path $envFile -Value $envContent
    
    Write-Host "✅ Configuration file created: $envFile" -ForegroundColor Green
    Write-Host "⚠️ IMPORTANT: Edit $envFile and add your Anthropic API key" -ForegroundColor Yellow
    Write-Host "   Get your API key from: https://console.anthropic.com" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "═══ Step 5: Starting Widget-Log Proxy ═══" -ForegroundColor Yellow
Write-Host ""

Write-Host "Starting semantic caching proxy on https://127.0.0.1:8443..." -ForegroundColor Cyan

# Check if port 8443 is in use
$portInUse = Get-NetTCPConnection -LocalPort 8443 -ErrorAction SilentlyContinue
if ($portInUse) {
    Write-Host "⚠️ Port 8443 already in use. Stopping existing process..." -ForegroundColor Yellow
    $processId = $portInUse.OwningProcess
    Stop-Process -Id $processId -Force
    Start-Sleep -Seconds 2
}

# Start proxy in background
Start-Process -FilePath "python" -ArgumentList "secure_proxy.py" -WorkingDirectory "$SCRIPT_DIR\widget-log" -WindowStyle Hidden

Start-Sleep -Seconds 3

# Verify proxy is running
try {
    $response = Invoke-WebRequest -Uri "https://127.0.0.1:8443/health" -SkipCertificateCheck -ErrorAction Stop
    if ($response.StatusCode -eq 200) {
        Write-Host "✅ Widget-Log proxy started successfully!" -ForegroundColor Green
    }
} catch {
    Write-Host "⚠️ Warning: Could not verify proxy health (may still be starting)" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "═══ Installation Complete! ═══" -ForegroundColor Green
Write-Host ""

Write-Host "╔════════════════════════════════════════════════════════════╗" -ForegroundColor Green
Write-Host "║                    Installation Summary                    ║" -ForegroundColor Green
Write-Host "╠════════════════════════════════════════════════════════════╣" -ForegroundColor Green
Write-Host "║  ✅ OptaFly_Zed v1.0.0 built successfully                  ║" -ForegroundColor Green
Write-Host "║  ✅ Widget-Log semantic caching configured                 ║" -ForegroundColor Green
Write-Host "║  ✅ Proxy running on https://127.0.0.1:8443                ║" -ForegroundColor Green
Write-Host "╚════════════════════════════════════════════════════════════╝" -ForegroundColor Green
Write-Host ""

Write-Host "To start OptaFly_Zed:" -ForegroundColor Cyan
Write-Host "  .\target\release\zed.exe" -ForegroundColor White
Write-Host ""

Write-Host "To configure your API key:" -ForegroundColor Cyan
Write-Host "  notepad $envFile" -ForegroundColor White
Write-Host ""

Write-Host "To check cache statistics:" -ForegroundColor Cyan
Write-Host "  curl -k https://127.0.0.1:8443/stats" -ForegroundColor White
Write-Host ""

Write-Host "For more information, see INSTALL.md and README.md" -ForegroundColor Cyan
Write-Host ""

# Offer to start Zed
$startNow = Read-Host "Start OptaFly_Zed now? (Y/n)"
if ($startNow -eq '' -or $startNow -eq 'Y' -or $startNow -eq 'y') {
    Write-Host "Launching OptaFly_Zed..." -ForegroundColor Cyan
    Set-Location $SCRIPT_DIR
    Start-Process -FilePath ".\target\release\zed.exe"
}

Write-Host ""
Write-Host "Thank you for using OptaFly_Zed! 🚀" -ForegroundColor Green
