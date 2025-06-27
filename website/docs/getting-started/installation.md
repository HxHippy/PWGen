---
sidebar_position: 1
---

# Installation

Get PwGen installed on your system quickly and easily.

## Quick Install (Recommended)

### Linux

```bash
curl -sSL https://raw.githubusercontent.com/hxhippy/pwgen/main/scripts/install.sh | bash
```

### macOS

```bash
curl -sSL https://raw.githubusercontent.com/hxhippy/pwgen/main/scripts/install-macos.sh | bash
```

### Windows (PowerShell as Administrator)

```powershell
irm https://raw.githubusercontent.com/hxhippy/pwgen/main/scripts/install.ps1 | iex
```

## Package Managers

### Cargo (Rust)

```bash
cargo install --git https://github.com/hxhippy/pwgen pwgen-cli pwgen-gui
```

### Homebrew (macOS/Linux)

```bash
brew tap hxhippy/pwgen
brew install pwgen-rust
```

### AUR (Arch Linux)

```bash
yay -S pwgen-rust
```

## Manual Download

Visit our [download page](/download) to get platform-specific installers:

- **Linux**: `.deb`, `.rpm`, `.tar.gz`
- **macOS**: `.dmg`, `.pkg` 
- **Windows**: `.msi`, `.exe`

## Build from Source

### Prerequisites

- [Rust](https://rustup.rs/) 1.70 or later
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
git clone https://github.com/hxhippy/pwgen.git
cd pwgen

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
  - Linux: glibc 2.17+ or musl
  - macOS: 11.0 or later
  - Windows: 10 or later

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
2. Search [GitHub Issues](https://github.com/hxhippy/pwgen/issues)
3. Create a new issue with system details

## Next Steps

- 🚀 **[First Run](first-run)** - Set up your first vault
- 📚 **[Basic Usage](basic-usage)** - Learn the fundamentals
- 🔧 **[Configuration](../user-guide/configuration)** - Customize PwGen