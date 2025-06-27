---
sidebar_position: 7
---

# Configuration

Customize PwGen to suit your preferences and workflow.

## Configuration File

Default location: `~/.config/pwgen/config.toml`

```toml
[ui]
theme = "auto"  # auto, light, dark
font_size = 14
window_size = [1200, 800]

[security]
auto_lock_minutes = 15
clipboard_clear_seconds = 30

[generator]
default_length = 16
default_symbols = true
exclude_ambiguous = true

[cli]
default_format = "table"
colors = true
```

## Environment Variables

```bash
# Vault location
export PWGEN_VAULT_PATH="/path/to/vault.db"

# Output format
export PWGEN_FORMAT="json"

# Disable colors
export PWGEN_NO_COLOR=1
```

## GUI Settings

Access via Settings menu:
- Theme selection (light/dark/auto)
- Font size adjustment
- Window size preferences
- Auto-lock timeout
- Clipboard timeout

## CLI Configuration

### Output Formats
- `table` - Formatted table (default)
- `json` - JSON output
- `csv` - Comma-separated values
- `yaml` - YAML format

### Shell Completions

```bash
# Bash
pwgen-cli completion bash > ~/.bash_completion.d/pwgen-cli

# Zsh  
pwgen-cli completion zsh > ~/.zsh/completions/_pwgen-cli

# Fish
pwgen-cli completion fish > ~/.config/fish/completions/pwgen-cli.fish
```

## Security Settings

### Auto-lock
- Configure inactivity timeout
- Lock on screen saver
- Lock on system suspend

### Clipboard
- Clear timeout (default: 30 seconds)
- Disable clipboard history
- Primary vs clipboard selection (Linux)

## Advanced Configuration

### Custom Vault Location
```bash
pwgen-cli --vault /custom/path/vault.db
```

### Logging
```toml
[logging]
level = "info"  # error, warn, info, debug, trace
file = "~/.config/pwgen/pwgen.log"
```

### Backup Settings
```toml
[backup]
auto_backup = true
backup_interval_hours = 24
backup_retention_days = 30
backup_location = "~/.config/pwgen/backups"
```