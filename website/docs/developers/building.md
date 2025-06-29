---
sidebar_position: 3
---

# Building from Source

Complete guide to building PwGen-rust from source code on all supported platforms.

## Prerequisites

### Required Tools

1. **Rust Toolchain**
   ```bash
   # Install rustup (Rust installer)
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   
   # Use minimum required version
   rustup install 1.75.0
   rustup default 1.75.0
   ```

2. **Git**
   ```bash
   # Ubuntu/Debian
   sudo apt install git
   
   # macOS
   brew install git
   
   # Windows: Download from git-scm.com
   ```

3. **Build Tools**
   ```bash
   # Ubuntu/Debian
   sudo apt install build-essential pkg-config
   
   # CentOS/RHEL/Fedora
   sudo dnf install gcc gcc-c++ pkg-config
   
   # macOS (install Xcode Command Line Tools)
   xcode-select --install
   
   # Windows (install Visual Studio Build Tools)
   # Download from visualstudio.microsoft.com
   ```

### Platform-Specific Dependencies

#### Linux

**Ubuntu/Debian:**
```bash
# GUI dependencies
sudo apt install libgtk-3-dev libgtk-3-0

# Optional: For system integration
sudo apt install libxss1 libgconf-2-4 libnss3 libxtst6 libatspi2.0-0 libdrm2 libxcomposite1 libxdamage1 libxrandr2 libgbm1 libxss1 libasound2

# SSL support
sudo apt install libssl-dev
```

**CentOS/RHEL/Fedora:**
```bash
# GUI dependencies
sudo dnf install gtk3-devel

# SSL support
sudo dnf install openssl-devel
```

**Arch Linux:**
```bash
# All dependencies
sudo pacman -S base-devel gtk3 openssl
```

#### macOS

```bash
# Install dependencies via Homebrew
brew install pkg-config openssl cmake

# Set environment variables for OpenSSL
echo 'export PATH="/opt/homebrew/bin:$PATH"' >> ~/.zshrc
echo 'export LDFLAGS="-L/opt/homebrew/lib"' >> ~/.zshrc
echo 'export CPPFLAGS="-I/opt/homebrew/include"' >> ~/.zshrc
```

#### Windows

1. **Visual Studio Build Tools 2022**
   - Download and install from Microsoft
   - Include C++ build tools
   - Include Windows 10/11 SDK

2. **Optional: vcpkg for dependencies**
   ```powershell
   git clone https://github.com/Microsoft/vcpkg.git
   cd vcpkg
   .\bootstrap-vcpkg.bat
   ```

## Source Code

### Clone Repository

```bash
# Clone the repository
git clone https://github.com/HxHippy/PWGen.git
cd PWGen

# Check available branches/tags
git branch -a
git tag -l

# Use specific version (recommended for production)
git checkout v1.2.0
```

### Repository Structure

```
PWGen/
├── Cargo.toml          # Workspace configuration
├── Cargo.lock          # Dependency lockfile
├── pwgen-core/         # Core library
├── pwgen-cli/          # Command-line interface
├── pwgen-gui/          # Desktop GUI application
├── pwgen-server/       # API server (in development)
├── scripts/            # Build and installation scripts
├── docs/               # Additional documentation
├── tests/              # Integration tests
└── .github/            # CI/CD workflows
```

## Build Process

### Default Build

```bash
# Build all components (debug mode)
cargo build

# Build specific component
cargo build -p pwgen-cli
cargo build -p pwgen-gui
cargo build -p pwgen-core

# Run tests
cargo test

# Format code
cargo fmt

# Run linter
cargo clippy -- -D warnings
```

### Release Build

```bash
# Build optimized release version
cargo build --release

# Binaries will be in target/release/
ls target/release/pwgen-*
```

### Feature-Specific Builds

PwGen-rust supports conditional compilation for different use cases:

```bash
# Minimal build (CLI only, no GUI dependencies)
cargo build --release --no-default-features --features "cli"

# GUI only
cargo build --release --no-default-features --features "gui"

# Maximum security build
cargo build --release --features "secure"

# Development build with debug features
cargo build --features "dev-tools,logging"
```

### Cross-Compilation

#### Linux to Windows

```bash
# Install target
rustup target add x86_64-pc-windows-gnu

# Install cross-compilation tools
sudo apt install gcc-mingw-w64-x86-64

# Build
cargo build --release --target x86_64-pc-windows-gnu
```

#### macOS to Linux

```bash
# Install target
rustup target add x86_64-unknown-linux-gnu

# Note: May require Docker or additional setup for dependencies
```

## Build Optimization

### Performance Optimization

```toml
# Cargo.toml profile for maximum performance
[profile.release]
opt-level = 3           # Maximum optimization
lto = "fat"             # Link-time optimization
codegen-units = 1       # Single codegen unit for better optimization
panic = "abort"         # Smaller binary size
strip = true            # Remove debug symbols
```

### Size Optimization

```toml
# Cargo.toml profile for minimal size
[profile.min-size]
inherits = "release"
opt-level = "z"         # Optimize for size
lto = true
panic = "abort"
strip = true
```

```bash
# Build with size optimization
cargo build --profile min-size
```

### Security-Focused Build

```toml
# Cargo.toml profile for security
[profile.secure]
inherits = "release"
overflow-checks = true  # Runtime integer overflow checks
debug-assertions = true # Enable debug assertions in release
```

## Platform-Specific Builds

### Linux Packaging

#### AppImage

```bash
# Install linuxdeploy
wget https://github.com/linuxdeploy/linuxdeploy/releases/download/continuous/linuxdeploy-x86_64.AppImage
chmod +x linuxdeploy-x86_64.AppImage

# Build and package
cargo build --release
./scripts/build-appimage.sh
```

#### Snap Package

```bash
# Install snapcraft
sudo apt install snapcraft

# Build snap
snapcraft
```

#### Debian Package

```bash
# Install cargo-deb
cargo install cargo-deb

# Build .deb package
cargo deb -p pwgen-gui
```

### macOS Packaging

#### App Bundle

```bash
# Build GUI application
cargo build --release -p pwgen-gui

# Create app bundle
./scripts/create-macos-app.sh

# Sign and notarize (requires Apple Developer account)
codesign --sign "Developer ID Application: Your Name" PwGen.app
xcrun notarytool submit PwGen.dmg
```

#### Installer Package

```bash
# Create .pkg installer
pkgbuild --root dist/macos --identifier dev.pwgenrust.pwgen --version 1.2.0 PwGen.pkg
```

### Windows Packaging

#### NSIS Installer

```bash
# Install NSIS (on Windows)
# Download from nsis.sourceforge.io

# Build executable
cargo build --release --target x86_64-pc-windows-msvc

# Create installer
makensis scripts/windows-installer.nsi
```

#### MSI Installer (WiX)

```bash
# Install WiX Toolset
# Download from wixtoolset.org

# Build MSI
candle scripts/pwgen.wxs
light pwgen.wixobj -out PwGen.msi
```

## Testing

### Unit Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_encryption

# Run tests for specific crate
cargo test -p pwgen-core
```

### Integration Tests

```bash
# Run integration tests
cargo test --test integration

# Run with real vault file
PWGEN_TEST_VAULT=/tmp/test.vault cargo test
```

### Security Tests

```bash
# Install cargo-audit for vulnerability scanning
cargo install cargo-audit

# Check for vulnerabilities
cargo audit

# Install cargo-deny for dependency analysis
cargo install cargo-deny

# Check licenses and security
cargo deny check
```

### Fuzzing

```bash
# Install cargo-fuzz
cargo install cargo-fuzz

# Run fuzzing tests
cargo fuzz run fuzz_encryption
cargo fuzz run fuzz_parser
```

## Continuous Integration

### GitHub Actions

The project includes CI/CD workflows:

```yaml
# .github/workflows/ci.yml
name: CI
on: [push, pull_request]
jobs:
  test:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
        rust: [stable, beta]
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@master
        with:
          toolchain: ${{ matrix.rust }}
      - run: cargo test --all-features
```

### Local CI Simulation

```bash
# Install act to run GitHub Actions locally
curl https://raw.githubusercontent.com/nektos/act/master/install.sh | sudo bash

# Run CI locally
act
```

## Troubleshooting

### Common Build Issues

#### Missing Dependencies

**Error:** `error: Microsoft Visual C++ 14.0 is required`
```bash
# Solution: Install Visual Studio Build Tools
```

**Error:** `fatal error: 'gtk/gtk.h' file not found`
```bash
# Solution: Install GTK development headers
sudo apt install libgtk-3-dev
```

#### Compilation Errors

**Error:** `error: linking with 'cc' failed`
```bash
# Solution: Install build tools
sudo apt install build-essential
```

**Error:** `error: could not find system library 'openssl'`
```bash
# Ubuntu/Debian
sudo apt install libssl-dev

# macOS
brew install openssl
export OPENSSL_DIR=/opt/homebrew/opt/openssl
```

#### Performance Issues

**Slow compilation:**
```bash
# Use more CPU cores
export CARGO_BUILD_JOBS=4

# Use faster linker (Linux)
sudo apt install lld
export RUSTFLAGS="-C link-arg=-fuse-ld=lld"
```

**Large binary size:**
```bash
# Strip debug symbols
cargo build --release
strip target/release/pwgen-gui

# Use UPX compression (optional)
upx --best target/release/pwgen-gui
```

## Advanced Topics

### Custom Build Scripts

Create custom build configurations:

```toml
# build.rs
fn main() {
    // Custom build logic
    println!("cargo:rustc-link-lib=ssl");
    
    // Platform-specific compilation
    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-lib=crypt32");
    }
}
```

### Environment Variables

Control build behavior:

```bash
# Debug logging during build
export RUST_LOG=debug

# Custom features
export PWGEN_FEATURES="secure,audit"

# Build with custom config
export PWGEN_CONFIG_PATH=/custom/config.toml
```

### Memory Safety

Verify memory safety:

```bash
# Install Miri for unsafe code checking
rustup component add miri

# Run with Miri
cargo miri test
```

## Release Process

### Preparing a Release

```bash
# Update version numbers
sed -i 's/version = "1.1.0"/version = "1.2.0"/' Cargo.toml

# Update changelog
echo "## [1.2.0] - $(date +%Y-%m-%d)" >> CHANGELOG.md

# Run full test suite
cargo test --all-features

# Build all targets
cargo build --release --all-features
```

### Creating Release Artifacts

```bash
# Run release script
./scripts/prepare-release.sh v1.2.0

# This creates:
# - Linux: .tar.gz, .deb, .AppImage
# - macOS: .dmg, .pkg
# - Windows: .exe, .msi
# - Source: .tar.gz with verification
```

## Resources

### Build Tools

- [Rust Installation](https://rustup.rs/)
- [Cargo Book](https://doc.rust-lang.org/cargo/)
- [Cross-compilation Guide](https://rust-lang.github.io/rustup/cross-compilation.html)

### Platform-Specific

- [Linux Packaging](https://wiki.debian.org/HowToPackageForDebian)
- [macOS Deployment](https://developer.apple.com/documentation/xcode/distributing-your-app-for-beta-testing-and-releases)
- [Windows Packaging](https://docs.microsoft.com/en-us/windows/msix/)

### Security

- [Rust Security Guidelines](https://anssi-fr.github.io/rust-guide/)
- [Supply Chain Security](https://github.com/RustSec/rustsec)
- [Cargo Audit](https://github.com/RustSec/rustsec/tree/main/cargo-audit)

---

For additional help with building, see our [GitHub Discussions](https://github.com/HxHippy/PWGen/discussions) or [open an issue](https://github.com/HxHippy/PWGen/issues).