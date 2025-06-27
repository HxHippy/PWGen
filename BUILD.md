# Building PwGen for Different Platforms

## Prerequisites

- Rust 1.70 or later
- Platform-specific dependencies (see below)

## Linux

### Dependencies
```bash
# Debian/Ubuntu
sudo apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libspeechd-dev libxkbcommon-dev libssl-dev

# Fedora
sudo dnf install libxcb-devel libxkbcommon-devel openssl-devel

# Arch
sudo pacman -S libxcb libxkbcommon
```

### Build
```bash
cargo build --release -p pwgen-gui
cargo build --release -p pwgen-cli
```

## Windows

### Dependencies
- Visual Studio 2019 or later with C++ tools
- No additional dependencies needed

### Build
```powershell
cargo build --release -p pwgen-gui
cargo build --release -p pwgen-cli
```

The GUI executable will hide the console window on Windows in release mode.

## macOS

### Dependencies
- Xcode Command Line Tools
```bash
xcode-select --install
```

### Build
```bash
cargo build --release -p pwgen-gui
cargo build --release -p pwgen-cli
```

## Creating Application Bundles

### Linux AppImage
```bash
# Install linuxdeploy
wget https://github.com/linuxdeploy/linuxdeploy/releases/download/continuous/linuxdeploy-x86_64.AppImage
chmod +x linuxdeploy-x86_64.AppImage

# Create AppImage
./linuxdeploy-x86_64.AppImage --appdir AppDir \
    --executable target/release/pwgen-gui \
    --desktop-file pwgen.desktop \
    --icon-file pwgen.png \
    --output appimage
```

### Windows Installer
Use WiX Toolset or Inno Setup to create an installer from the release executable.

### macOS App Bundle
```bash
# Create app structure
mkdir -p PwGen.app/Contents/MacOS
mkdir -p PwGen.app/Contents/Resources

# Copy executable
cp target/release/pwgen-gui PwGen.app/Contents/MacOS/PwGen

# Create Info.plist
cat > PwGen.app/Contents/Info.plist << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>PwGen</string>
    <key>CFBundleIdentifier</key>
    <string>com.pwgen.app</string>
    <key>CFBundleName</key>
    <string>PwGen</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleVersion</key>
    <string>1.0.0</string>
</dict>
</plist>
EOF
```

## Binary Locations

After building, the binaries will be located at:
- GUI: `target/release/pwgen-gui` (or `pwgen-gui.exe` on Windows)
- CLI: `target/release/pwgen-cli` (or `pwgen-cli.exe` on Windows)

## Notes

- The egui framework used for the GUI is fully cross-platform and will automatically use the native rendering backend for each platform
- All features including clipboard support work across all platforms
- The SQLite database is stored in the platform-specific data directory:
  - Linux: `~/.local/share/pwgen/`
  - Windows: `%APPDATA%\pwgen\`
  - macOS: `~/Library/Application Support/pwgen/`