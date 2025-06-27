#!/bin/bash

# PwGen Password Manager - Linux Installer
# Detects distribution and installs dependencies automatically

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

# Check if running as root
check_root() {
    if [[ $EUID -eq 0 ]]; then
        log_error "This script should not be run as root!"
        log_info "Please run as a regular user. The script will prompt for sudo when needed."
        exit 1
    fi
}

# Detect Linux distribution
detect_distro() {
    if [[ -f /etc/os-release ]]; then
        . /etc/os-release
        DISTRO=$ID
        VERSION=$VERSION_ID
    elif [[ -f /etc/lsb-release ]]; then
        . /etc/lsb-release
        DISTRO=$DISTRIB_ID
        VERSION=$DISTRIB_RELEASE
    else
        log_error "Unable to detect Linux distribution"
        exit 1
    fi
    
    log_info "Detected: $DISTRO $VERSION"
}

# Check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Install Rust if not present
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

# Install system dependencies based on distribution
install_dependencies() {
    case "$DISTRO" in
        ubuntu|debian|pop|elementary|linuxmint)
            log_info "Installing dependencies for Debian/Ubuntu-based system..."
            sudo apt update
            sudo apt install -y \
                build-essential \
                curl \
                git \
                pkg-config \
                libssl-dev \
                libgtk-3-dev \
                libayatana-appindicator3-dev \
                libxdo-dev \
                librsvg2-dev \
                libsoup2.4-dev \
                libjavascriptcoregtk-4.0-dev \
                libwebkit2gtk-4.0-dev \
                libglib2.0-dev \
                libcairo-dev \
                libpango1.0-dev \
                libatk1.0-dev \
                libgdk-pixbuf-2.0-dev
            ;;
        fedora|rhel|centos|rocky|almalinux)
            log_info "Installing dependencies for Red Hat-based system..."
            if command_exists dnf; then
                sudo dnf install -y \
                    gcc \
                    gcc-c++ \
                    make \
                    curl \
                    git \
                    pkg-config \
                    openssl-devel \
                    gtk3-devel \
                    libappindicator-gtk3-devel \
                    xdotool-devel \
                    librsvg2-devel \
                    libsoup-devel \
                    webkit2gtk3-devel \
                    glib2-devel \
                    cairo-devel \
                    pango-devel \
                    atk-devel \
                    gdk-pixbuf2-devel
            else
                sudo yum install -y \
                    gcc \
                    gcc-c++ \
                    make \
                    curl \
                    git \
                    pkg-config \
                    openssl-devel \
                    gtk3-devel \
                    libappindicator-gtk3-devel \
                    xdotool-devel \
                    librsvg2-devel \
                    libsoup-devel \
                    webkit2gtk3-devel \
                    glib2-devel \
                    cairo-devel \
                    pango-devel \
                    atk-devel \
                    gdk-pixbuf2-devel
            fi
            ;;
        opensuse*|sles)
            log_info "Installing dependencies for openSUSE/SLES..."
            sudo zypper install -y \
                gcc \
                gcc-c++ \
                make \
                curl \
                git \
                pkg-config \
                libopenssl-devel \
                gtk3-devel \
                libappindicator3-devel \
                xdotool-devel \
                librsvg-devel \
                libsoup-devel \
                webkit2gtk3-devel \
                glib2-devel \
                cairo-devel \
                pango-devel \
                atk-devel \
                gdk-pixbuf-devel
            ;;
        arch|manjaro|endeavouros)
            log_info "Installing dependencies for Arch-based system..."
            sudo pacman -Sy --needed --noconfirm \
                base-devel \
                curl \
                git \
                pkg-config \
                openssl \
                gtk3 \
                libappindicator-gtk3 \
                xdotool \
                librsvg \
                libsoup \
                webkit2gtk \
                glib2 \
                cairo \
                pango \
                atk \
                gdk-pixbuf2
            ;;
        alpine)
            log_info "Installing dependencies for Alpine Linux..."
            sudo apk add --no-cache \
                build-base \
                curl \
                git \
                pkgconf \
                openssl-dev \
                gtk+3.0-dev \
                libappindicator-dev \
                xdotool-dev \
                librsvg-dev \
                libsoup-dev \
                webkit2gtk-dev \
                glib-dev \
                cairo-dev \
                pango-dev \
                atk-dev \
                gdk-pixbuf-dev
            ;;
        gentoo)
            log_info "Installing dependencies for Gentoo..."
            log_warning "Please ensure you have the following packages installed:"
            echo "  - sys-devel/gcc"
            echo "  - dev-vcs/git"
            echo "  - net-misc/curl"
            echo "  - virtual/pkgconfig"
            echo "  - dev-libs/openssl"
            echo "  - x11-libs/gtk+:3"
            echo "  - dev-libs/libappindicator:3"
            echo "  - x11-misc/xdotool"
            echo "  - gnome-base/librsvg"
            echo "  - net-libs/libsoup"
            echo "  - net-libs/webkit-gtk"
            read -p "Press Enter to continue once dependencies are installed..."
            ;;
        *)
            log_warning "Unsupported distribution: $DISTRO"
            log_info "Please install the following dependencies manually:"
            echo "  - build-essential/base-devel"
            echo "  - curl, git, pkg-config"
            echo "  - openssl-dev/openssl-devel"
            echo "  - gtk3-devel"
            echo "  - libappindicator3-devel"
            echo "  - xdotool/xdotool-devel"
            echo "  - librsvg2-devel"
            echo "  - webkit2gtk-devel"
            read -p "Press Enter to continue once dependencies are installed..."
            ;;
    esac
}

# Build and install PwGen
build_and_install() {
    log_info "Building PwGen Password Manager..."
    
    # Ensure we're in the project directory
    if [[ ! -f "Cargo.toml" ]]; then
        log_error "Cargo.toml not found. Please run this script from the project root directory."
        exit 1
    fi
    
    # Build the project
    cargo build --release
    
    # Create installation directories
    mkdir -p ~/.local/bin
    mkdir -p ~/.local/share/applications
    mkdir -p ~/.local/share/icons/hicolor/256x256/apps
    
    # Install binaries
    cp target/release/pwgen-cli ~/.local/bin/
    cp target/release/pwgen-gui ~/.local/bin/
    
    # Install desktop entry
    if [[ -f "pwgen.desktop" ]]; then
        sed "s|Exec=pwgen-gui|Exec=$HOME/.local/bin/pwgen-gui|g" pwgen.desktop > ~/.local/share/applications/pwgen.desktop
    else
        # Create desktop entry
        cat > ~/.local/share/applications/pwgen.desktop << EOF
[Desktop Entry]
Name=PwGen Password Manager
Comment=Secure password management application
Exec=$HOME/.local/bin/pwgen-gui
Icon=PWGenLogo
Type=Application
Categories=Utility;Security;
Keywords=password;security;vault;
StartupNotify=true
EOF
    fi
    
    # Install icon
    if [[ -f "ui/PWGenLogo.png" ]]; then
        cp ui/PWGenLogo.png ~/.local/share/icons/hicolor/256x256/apps/PWGenLogo.png
    elif [[ -f "assets/PWGenLogo.png" ]]; then
        cp assets/PWGenLogo.png ~/.local/share/icons/hicolor/256x256/apps/PWGenLogo.png
    fi
    
    # Update desktop database
    if command_exists update-desktop-database; then
        update-desktop-database ~/.local/share/applications/
    fi
    
    # Update icon cache
    if command_exists gtk-update-icon-cache; then
        gtk-update-icon-cache ~/.local/share/icons/hicolor/
    fi
    
    # Add ~/.local/bin to PATH if not already there
    if [[ ":$PATH:" != *":$HOME/.local/bin:"* ]]; then
        echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
        log_info "Added ~/.local/bin to PATH in ~/.bashrc"
        log_info "Please run 'source ~/.bashrc' or restart your terminal"
    fi
    
    log_success "PwGen installed successfully!"
}

# Create uninstaller
create_uninstaller() {
    cat > ~/.local/bin/pwgen-uninstall << 'EOF'
#!/bin/bash

echo "Uninstalling PwGen Password Manager..."

# Remove binaries
rm -f ~/.local/bin/pwgen-cli
rm -f ~/.local/bin/pwgen-gui

# Remove desktop entry
rm -f ~/.local/share/applications/pwgen.desktop

# Remove icon
rm -f ~/.local/share/icons/hicolor/256x256/apps/PWGenLogo.png

# Remove this uninstaller
rm -f ~/.local/bin/pwgen-uninstall

# Update caches
if command -v update-desktop-database >/dev/null 2>&1; then
    update-desktop-database ~/.local/share/applications/
fi

if command -v gtk-update-icon-cache >/dev/null 2>&1; then
    gtk-update-icon-cache ~/.local/share/icons/hicolor/
fi

echo "PwGen uninstalled successfully!"
echo "Note: User data in ~/.local/share/pwgen/ was preserved"
EOF

    chmod +x ~/.local/bin/pwgen-uninstall
    log_info "Uninstaller created at ~/.local/bin/pwgen-uninstall"
}

# Main installation function
main() {
    echo "======================================"
    echo "  PwGen Password Manager Installer"
    echo "======================================"
    echo
    
    check_root
    detect_distro
    
    log_info "Starting installation process..."
    
    install_rust
    install_dependencies
    build_and_install
    create_uninstaller
    
    echo
    echo "======================================"
    log_success "Installation completed successfully!"
    echo "======================================"
    echo
    log_info "You can now:"
    echo "  • Run 'pwgen-gui' to start the graphical interface"
    echo "  • Run 'pwgen-cli --help' for command-line usage"
    echo "  • Find PwGen in your applications menu"
    echo "  • Run 'pwgen-uninstall' to remove PwGen"
    echo
    log_info "Note: If commands are not found, run 'source ~/.bashrc' or restart your terminal"
}

# Run main function
main "$@"