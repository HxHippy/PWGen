---
sidebar_position: 1
---

# Installation

Get PwGen-rust installed on your system quickly and easily. Multiple installation methods are available for all major platforms.

## Quick Install (Recommended)

### Linux

#### Snap Store (Approved!)
```bash
# Recommended: Install from Snap Store
sudo snap install pwgen-rust
```

#### Direct Download
```bash
# Download latest release
wget https://github.com/HxHippy/PWGen/releases/latest/download/pwgen-linux-amd64.tar.gz
tar -xzf pwgen-linux-amd64.tar.gz
sudo mv pwgen-* /usr/local/bin/
```

### macOS

```bash
# Download the .dmg installer
curl -LO https://github.com/HxHippy/PWGen/releases/latest/download/pwgen-macos.dmg
# Or download the .pkg for automated deployment
curl -LO https://github.com/HxHippy/PWGen/releases/latest/download/pwgen-macos.pkg
```

### Windows

```powershell
# Download the NSIS installer
Invoke-WebRequest -Uri "https://github.com/HxHippy/PWGen/releases/latest/download/pwgen-installer.exe" -OutFile "pwgen-installer.exe"
# Run the installer
.\pwgen-installer.exe
```

## Package Managers

### Cargo (Rust)

```bash
# Build from source using Cargo
cargo install --git https://github.com/HxHippy/PWGen pwgen-cli pwgen-gui
```

### Platform-Specific Package Managers

```bash
# Snap (Linux) - Official
sudo snap install pwgen-rust

# Homebrew (macOS/Linux) - Coming Soon
# brew install pwgen-rust

# AUR (Arch Linux) - Community Maintained
# yay -S pwgen-rust-git
```

## Manual Download

Visit our [download page](/download) or [GitHub Releases](https://github.com/HxHippy/PWGen/releases/latest) to get platform-specific installers:

- **Linux**: 
  - `.tar.gz` (universal binary)
  - `.snap` (Snap Store)
  - `.AppImage` (coming soon)
- **macOS**: 
  - `.dmg` (disk image)
  - `.pkg` (installer package)
- **Windows**: 
  - `.exe` (NSIS installer)
  - `.msi` (WiX installer - coming soon)

## Build from Source

### Prerequisites

- [Rust](https://rustup.rs/) 1.75 or later
- Git

### Platform Dependencies

#### Linux

```bash
# Ubuntu/Debian
sudo apt install build-essential pkg-config libssl-dev libgtk-3-dev

# Fedora/RHEL  
sudo dnf install gcc gcc-c++ pkg-config openssl-devel gtk3-devel

# Arch Linux
sudo pacman -S base-devel pkg-config openssl gtk3
```

#### macOS

```bash
# Install via Homebrew
brew install pkg-config openssl cmake
```

#### Windows

- [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022)
- [Git for Windows](https://git-scm.com/download/win)

### Build Steps

```bash
# Clone the repository
git clone https://github.com/HxHippy/PWGen.git
cd PWGen

# Build release version
cargo build --release

# Install binaries
cargo install --path pwgen-cli
cargo install --path pwgen-gui
```

## Verification

After installation, verify everything works:

```bash
# Check CLI version
pwgen-cli --version

# Launch GUI (if installed)
pwgen-gui
```

## System Requirements

### Minimum Requirements

- **RAM**: 100 MB
- **Disk**: 50 MB free space
- **OS**: 
  - Linux: glibc 2.17+ or musl (kernel 3.2+)
  - macOS: 11.0 Big Sur or later
  - Windows: 10 version 1809 or later

### Recommended

- **RAM**: 200 MB
- **Disk**: 100 MB free space
- **Display**: 1024x768 or higher for GUI

## Troubleshooting

### Common Issues

**Linux: Missing GTK libraries**
```bash
sudo apt install libgtk-3-dev libgtk-3-0
```

**macOS: Unsigned binary warning**
- Go to System Preferences → Security & Privacy
- Click "Allow Anyway" for PwGen

**Windows: Windows Defender warning**
- Click "More info" → "Run anyway"
- Or add exclusion for PwGen directory

### Getting Help

If you encounter issues:

1. Check our [troubleshooting guide](../user-guide/troubleshooting)
2. Search [GitHub Issues](https://github.com/HxHippy/PWGen/issues)
3. Create a new issue with system details
4. Join our [Community Discussions](https://github.com/HxHippy/PWGen/discussions)

## Next Steps

- 🚀 **[First Run](first-run)** - Set up your first vault
- 📚 **[Basic Usage](basic-usage)** - Learn the fundamentals
- 🔧 **[Configuration](../user-guide/configuration)** - Customize PwGen