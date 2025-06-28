---
sidebar_position: 1
---

# Welcome to PwGen-rust

Welcome to **PwGen-rust** - the advanced password and secrets manager built in Rust! 🦀

## What is PwGen-rust?

PwGen-rust is a modern, secure, and user-friendly password and secrets management solution designed for individuals, developers, and teams. Built with Rust for maximum performance and security, PwGen-rust provides enterprise-grade encryption while maintaining an intuitive user experience.

### 🎉 v1.2.0 Optimization Release

**Latest improvements:**
- **30-40% smaller binaries** through dependency optimization
- **Enhanced security** with modern cryptography (SHA-256 only)
- **Flexible build options** for different deployment scenarios
- **Cross-platform installers** (Snap, NSIS, WiX, PKG, DMG)
- **Conditional compilation** for platform-specific features

### 🎯 Key Features

- **🔒 Military-Grade Security** - AES-256-GCM encryption with secure key derivation
- **🗝️ Comprehensive Secrets** - API keys, SSH keys, documents, environment variables
- **🚀 High Performance** - Native Rust performance with minimal resource usage
- **🎨 Modern UI** - Clean, responsive interface built with egui
- **🌐 Cross-Platform** - Windows, macOS, and Linux support
- **🔧 Developer-Friendly** - CLI and GUI interfaces for all workflows

## Why Choose PwGen?

### Security First 🛡️

- **Zero-Knowledge Architecture** - Your master password never leaves your device
- **Memory Safety** - Rust's ownership model prevents common vulnerabilities
- **Encrypted Storage** - All sensitive data encrypted with AES-256-GCM
- **Audit Logging** - Comprehensive tracking of all access and modifications

### Developer Experience 💻

- **Powerful CLI** - Full command-line interface for automation and scripting
- **REST API** - Integration capabilities for custom workflows
- **Browser Extensions** - Seamless auto-fill across all major browsers
- **Import/Export** - Easy migration from other password managers

### Enterprise Ready 🏢

- **Team Collaboration** - Role-based access control and sharing
- **Backup & Restore** - Encrypted backups with integrity verification
- **Template System** - Pre-configured templates for common services
- **Compliance** - Meet security requirements for enterprise environments

## Quick Start

Get up and running with PwGen in minutes:

### 1. Download and Install

Choose your platform on our [download page](/download) or use our quick install scripts:

```bash title="Linux (Snap)"
# Manual install (pending store approval)
wget https://github.com/hxhippy/pwgen/releases/download/v1.2.0/pwgen-rust_1.2.0_amd64.snap
sudo snap install --dangerous pwgen-rust_1.2.0_amd64.snap
```

```bash title="macOS"
# Download from GitHub releases
# https://github.com/hxhippy/pwgen/releases/latest
```

```powershell title="Windows"
# Download from GitHub releases
# https://github.com/hxhippy/pwgen/releases/latest
```

### 2. Create Your First Vault

```bash title="CLI"
pwgen-cli vault create
```

Or launch the GUI application:

```bash title="GUI"
pwgen-gui
```

### 3. Add Your First Password

```bash title="CLI Example"
pwgen-cli password add --site github.com --username your-username
```

## What's Next?

- 📖 **[Getting Started Guide](getting-started/installation)** - Detailed setup instructions
- 💡 **[User Guide](user-guide/passwords)** - Learn all the features
- 🔧 **[CLI Reference](cli/overview)** - Command-line documentation
- 👩‍💻 **[Developer Guide](developers/architecture)** - Technical deep-dive

## Community & Support

- 🐛 **[Report Issues](https://github.com/hxhippy/pwgen/issues)** - Bug reports and feature requests
- 💬 **[Discussions](https://github.com/hxhippy/pwgen/discussions)** - Community support
- 🤝 **[Contributing](developers/contributing)** - Help improve PwGen
- 🔒 **[Security](security/reporting)** - Report security vulnerabilities

## Powered By Innovation

PwGen is proudly powered by:

- **[TRaViS](https://travisasm.com)** - AI-Powered EASM without asset caps
- **[Kief Studio](https://kief.studio)** - AI Integration & Technology Consulting  
- **[HxHippy](https://hxhippy.com)** - [@HxHippy](https://x.com/HxHippy) on X/Twitter

---

Ready to secure your digital life? **[Download PwGen-rust now](/download)** and experience the future of password management! 🚀