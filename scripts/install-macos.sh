#!/bin/bash

# PwGen Password Manager - macOS Installer
# Automated installation script for macOS

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Show help
show_help() {
    cat << EOF
PwGen Password Manager - macOS Installer

USAGE:
    ./install-macos.sh [OPTIONS]

OPTIONS:
    --help              Show this help message
    --no-homebrew       Skip Homebrew installation
    --force             Force reinstallation

EXAMPLES:
    ./install-macos.sh
    ./install-macos.sh --force
    ./install-macos.sh --no-homebrew

EOF
}

# Parse command line arguments
SKIP_HOMEBREW=false
FORCE_INSTALL=false

while [[ $# -gt 0 ]]; do
    case $1 in
        --help)
            show_help
            exit 0
            ;;
        --no-homebrew)
            SKIP_HOMEBREW=true
            shift
            ;;
        --force)
            FORCE_INSTALL=true
            shift
            ;;
        *)
            log_error "Unknown option: $1"
            show_help
            exit 1
            ;;
    esac
done

# Check if running on macOS
check_macos() {
    if [[ "$(uname)" != "Darwin" ]]; then
        log_error "This script is for macOS only!"
        exit 1
    fi
    
    MACOS_VERSION=$(sw_vers -productVersion)
    log_info "Detected macOS $MACOS_VERSION"
    
    # Check if macOS version is supported (10.14+)
    MAJOR_VERSION=$(echo $MACOS_VERSION | cut -d. -f1)
    MINOR_VERSION=$(echo $MACOS_VERSION | cut -d. -f2)
    
    if [[ $MAJOR_VERSION -lt 10 ]] || [[ $MAJOR_VERSION -eq 10 && $MINOR_VERSION -lt 14 ]]; then
        log_warning "macOS 10.14 (Mojave) or later is recommended"
    fi
}

# Check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Install Xcode Command Line Tools
install_xcode_tools() {
    if xcode-select -p &>/dev/null; then
        log_success "Xcode Command Line Tools are already installed"
        return 0
    fi
    
    log_info "Installing Xcode Command Line Tools..."
    log_info "A dialog will appear - please click 'Install'"
    
    xcode-select --install
    
    # Wait for installation to complete
    log_info "Waiting for Xcode Command Line Tools installation to complete..."
    while ! xcode-select -p &>/dev/null; do
        sleep 5
    done
    
    log_success "Xcode Command Line Tools installed successfully"
}

# Install Homebrew
install_homebrew() {
    if $SKIP_HOMEBREW; then
        log_info "Skipping Homebrew installation as requested"
        return 0
    fi
    
    if command_exists brew; then
        log_success "Homebrew is already installed"
        return 0
    fi
    
    log_info "Installing Homebrew..."
    /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
    
    # Add Homebrew to PATH for Apple Silicon Macs
    if [[ "$(uname -m)" == "arm64" ]]; then
        echo 'eval "$(/opt/homebrew/bin/brew shellenv)"' >> ~/.zprofile
        eval "$(/opt/homebrew/bin/brew shellenv)"
    else
        echo 'eval "$(/usr/local/bin/brew shellenv)"' >> ~/.zprofile
        eval "$(/usr/local/bin/brew shellenv)"
    fi
    
    log_success "Homebrew installed successfully"
}

# Install Rust
install_rust() {
    if command_exists rustc && command_exists cargo; then
        RUST_VERSION=$(rustc --version | cut -d' ' -f2)
        log_success "Rust is already installed (version $RUST_VERSION)"
        return 0
    fi
    
    log_info "Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source ~/.cargo/env
    log_success "Rust installed successfully"
}

# Install system dependencies
install_dependencies() {
    log_info "Installing system dependencies..."
    
    # Core dependencies
    local packages=(
        "pkg-config"
        "openssl"
        "cmake"
        "gtk+3"
        "adwaita-icon-theme"
        "librsvg"
        "libsoup@2"
        "webkit2gtk"
    )
    
    if command_exists brew; then
        log_info "Using Homebrew to install dependencies..."
        
        # Update Homebrew
        brew update
        
        for package in "${packages[@]}"; do
            log_info "Installing $package..."
            brew install "$package" || log_warning "Failed to install $package"
        done
        
        # Install additional tools that might be useful
        brew install --cask xquartz || log_warning "Failed to install XQuartz"
        
    else
        log_warning "Homebrew not available. Please install the following dependencies manually:"
        for package in "${packages[@]}"; do
            echo "  - $package"
        done
        read -p "Press Enter to continue once dependencies are installed..."
    fi
}

# Set environment variables for building
setup_build_environment() {
    log_info "Setting up build environment..."
    
    # Set PKG_CONFIG_PATH for dependencies
    if command_exists brew; then
        export PKG_CONFIG_PATH="$(brew --prefix)/lib/pkgconfig:$PKG_CONFIG_PATH"
        export LIBRARY_PATH="$(brew --prefix)/lib:$LIBRARY_PATH"
        export CPATH="$(brew --prefix)/include:$CPATH"
        
        # Set OpenSSL paths for Apple Silicon Macs
        if [[ "$(uname -m)" == "arm64" ]]; then
            export OPENSSL_DIR="$(brew --prefix openssl)"
            export OPENSSL_LIB_DIR="$(brew --prefix openssl)/lib"
            export OPENSSL_INCLUDE_DIR="$(brew --prefix openssl)/include"
        fi
    fi
}

# Build and install PwGen
build_and_install() {
    log_info "Building PwGen Password Manager..."
    
    # Ensure we're in the project directory
    if [[ ! -f "Cargo.toml" ]]; then
        log_error "Cargo.toml not found. Please run this script from the project root directory."
        exit 1
    fi
    
    # Set up build environment
    setup_build_environment
    
    # Build the project
    log_info "Compiling project (this may take a few minutes)..."
    cargo build --release
    
    # Create installation directories
    mkdir -p ~/Applications/PwGen.app/Contents/MacOS
    mkdir -p ~/Applications/PwGen.app/Contents/Resources
    
    # Install binaries
    cp target/release/pwgen-cli ~/.local/bin/ 2>/dev/null || {
        mkdir -p ~/.local/bin
        cp target/release/pwgen-cli ~/.local/bin/
    }
    cp target/release/pwgen-gui ~/Applications/PwGen.app/Contents/MacOS/
    
    # Create Info.plist for the app bundle
    cat > ~/Applications/PwGen.app/Contents/Info.plist << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>pwgen-gui</string>
    <key>CFBundleIdentifier</key>
    <string>com.pwgen.passwordmanager</string>
    <key>CFBundleName</key>
    <string>PwGen</string>
    <key>CFBundleDisplayName</key>
    <string>PwGen Password Manager</string>
    <key>CFBundleVersion</key>
    <string>1.0.0</string>
    <key>CFBundleShortVersionString</key>
    <string>1.0.0</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleInfoDictionaryVersion</key>
    <string>6.0</string>
    <key>LSMinimumSystemVersion</key>
    <string>10.14</string>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>LSApplicationCategoryType</key>
    <string>public.app-category.utilities</string>
    <key>NSHumanReadableCopyright</key>
    <string>Copyright © 2024 PwGen. All rights reserved.</string>
    <key>CFBundleIconFile</key>
    <string>PWGenLogo</string>
</dict>
</plist>
EOF

    # Copy icon if available
    if [[ -f "ui/PWGenLogo.png" ]]; then
        cp ui/PWGenLogo.png ~/Applications/PwGen.app/Contents/Resources/
    elif [[ -f "assets/PWGenLogo.png" ]]; then
        cp assets/PWGenLogo.png ~/Applications/PwGen.app/Contents/Resources/
    fi
    
    # Make the app executable
    chmod +x ~/Applications/PwGen.app/Contents/MacOS/pwgen-gui
    
    # Add ~/.local/bin to PATH if not already there
    SHELL_RC=""
    if [[ "$SHELL" == *"zsh"* ]]; then
        SHELL_RC="$HOME/.zshrc"
    elif [[ "$SHELL" == *"bash"* ]]; then
        SHELL_RC="$HOME/.bash_profile"
    fi
    
    if [[ -n "$SHELL_RC" ]] && [[ ":$PATH:" != *":$HOME/.local/bin:"* ]]; then
        echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$SHELL_RC"
        log_info "Added ~/.local/bin to PATH in $SHELL_RC"
        log_info "Please run 'source $SHELL_RC' or restart your terminal"
    fi
    
    log_success "PwGen installed successfully!"
}

# Create uninstaller
create_uninstaller() {
    cat > ~/.local/bin/pwgen-uninstall << 'EOF'
#!/bin/bash

echo "Uninstalling PwGen Password Manager..."

# Remove CLI binary
rm -f ~/.local/bin/pwgen-cli

# Remove app bundle
rm -rf ~/Applications/PwGen.app

# Remove this uninstaller
rm -f ~/.local/bin/pwgen-uninstall

echo "PwGen uninstalled successfully!"
echo "Note: User data in ~/Library/Application Support/pwgen/ was preserved"
EOF

    chmod +x ~/.local/bin/pwgen-uninstall
    log_info "Uninstaller created at ~/.local/bin/pwgen-uninstall"
}

# Create launcher script for terminal
create_launcher() {
    cat > ~/.local/bin/pwgen-gui << 'EOF'
#!/bin/bash
exec ~/Applications/PwGen.app/Contents/MacOS/pwgen-gui "$@"
EOF

    chmod +x ~/.local/bin/pwgen-gui
    log_info "Terminal launcher created at ~/.local/bin/pwgen-gui"
}

# Main installation function
main() {
    echo "======================================"
    echo "  PwGen Password Manager Installer"
    echo "======================================"
    echo
    
    check_macos
    
    log_info "Starting installation process..."
    
    install_xcode_tools
    install_homebrew
    install_rust
    install_dependencies
    build_and_install
    create_launcher
    create_uninstaller
    
    echo
    echo "======================================"
    log_success "Installation completed successfully!"
    echo "======================================"
    echo
    log_info "You can now:"
    echo "  • Find 'PwGen' in your Applications folder"
    echo "  • Run 'pwgen-gui' from Terminal"
    echo "  • Run 'pwgen-cli --help' for command-line usage"
    echo "  • Run 'pwgen-uninstall' to remove PwGen"
    echo
    log_info "Note: If commands are not found, restart your terminal or run:"
    if [[ "$SHELL" == *"zsh"* ]]; then
        echo "  source ~/.zshrc"
    else
        echo "  source ~/.bash_profile"
    fi
}

# Run main function
main "$@"