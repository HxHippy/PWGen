---
sidebar_position: 6
---

# Troubleshooting

Common issues and solutions for PwGen.

## Installation Issues

### Command Not Found

**Problem**: `pwgen-cli: command not found` after installation

**Solution**:
```bash
# Add cargo bin to PATH
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc

# Or install system-wide
sudo cp ~/.cargo/bin/pwgen-* /usr/local/bin/
```

### Build Errors

**Problem**: Compilation fails during installation

**Solution**:
```bash
# Update Rust
rustup update

# Install build dependencies (Linux)
sudo apt install build-essential pkg-config libssl-dev

# Install build dependencies (macOS)  
xcode-select --install
```

## Runtime Issues

### Vault Won't Open

**Problem**: "Failed to open vault" error

**Solutions**:
- Verify vault file exists and has correct permissions
- Check master password is correct
- Try restoring from backup if vault is corrupted

### Performance Issues

**Problem**: Slow loading with large vaults

**Solutions**:
- Use search and filtering to narrow results
- Consider archiving old entries
- Increase available memory

## GUI Issues

### Won't Start

**Problem**: GUI application fails to launch

**Solutions**:
```bash
# Check for missing libraries (Linux)
sudo apt install libgtk-3-dev

# Check display settings
echo $DISPLAY

# Try from terminal to see error messages
pwgen-gui
```

## Security Issues

### Forgotten Master Password

**Problem**: Can't remember master password

**Solutions**:
- Master passwords cannot be recovered by design
- Restore from backup with known password
- Start fresh if no backups available

### Suspected Breach

**Problem**: Possible security compromise

**Actions**:
1. Change master password immediately
2. Review all stored passwords
3. Update compromised passwords
4. Check access logs if available

## Getting Help

- 📖 [Documentation](../getting-started/installation)
- 🐛 [GitHub Issues](https://github.com/hxhippy/pwgen/issues)
- 💬 [Discussions](https://github.com/hxhippy/pwgen/discussions)