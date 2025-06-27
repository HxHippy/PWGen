---
sidebar_position: 3
---

# Password Generator

Create strong, secure passwords with customizable generation options.

## Overview

PwGen's password generator creates cryptographically secure passwords using customizable rules and patterns. The generator supports various character sets, length requirements, and security patterns to meet different password policies.

## Generation Options

### Character Sets

**Uppercase Letters (A-Z):**
- Essential for password complexity
- Required by most password policies
- Provides 26 additional character options

**Lowercase Letters (a-z):**
- Base character set for most passwords
- Improves readability and typing
- Provides 26 character options

**Numbers (0-9):**
- Adds numeric complexity
- Required by many security policies
- Provides 10 additional character options

**Symbols (!@#$%^&*):**
- Significantly increases password entropy
- Common symbols: `!@#$%^&*()_+-=[]{}|;:,.<>?`
- Some systems may restrict certain symbols

### Advanced Options

**Exclude Ambiguous Characters:**
- Removes visually similar characters
- Excludes: `0` (zero), `O` (capital O), `l` (lowercase L), `I` (capital I)
- Improves password readability and reduces typing errors

**Exclude Similar Characters:**
- Removes characters that might be confused
- Excludes: `il1Lo0O`
- Helpful for passwords that need to be read aloud or written

**Pronounceable Passwords:**
- Generates passwords following syllable patterns
- Easier to remember but slightly less secure
- Good for temporary passwords or recovery codes

## Using the Generator

### GUI Generator

1. **Navigate to Generator Tab**
   - Click the **🎲 Generator** tab in the main interface

2. **Configure Settings**
   - **Length**: Set password length (8-128 characters)
   - **Character Types**: Select uppercase, lowercase, numbers, symbols
   - **Exclusions**: Choose ambiguous character handling
   - **Pattern**: Select generation pattern (random, pronounceable, etc.)

3. **Generate Password**
   - Click **🎲 Generate New Password**
   - Password appears in the preview field
   - Click **🔄 Regenerate** for new password
   - Click **📋 Copy** to copy to clipboard

4. **Save to Entry**
   - Click **💾 Save as Entry** to create new password entry
   - Or use **📝 Use in Entry** to apply to existing entry

### CLI Generator

**Basic Generation:**
```bash
# Generate with default settings (16 characters)
pwgen-cli generate

# Generate with specific length
pwgen-cli generate --length 24

# Generate multiple passwords
pwgen-cli generate --count 5
```

**Character Set Control:**
```bash
# Only uppercase and lowercase
pwgen-cli generate --length 20 --upper --lower

# Include symbols
pwgen-cli generate --length 16 --upper --lower --numbers --symbols

# Exclude ambiguous characters
pwgen-cli generate --length 16 --no-ambiguous

# Pronounceable password
pwgen-cli generate --length 12 --pronounceable
```

**Advanced Options:**
```bash
# Custom character set
pwgen-cli generate --length 16 --charset "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"

# Pattern-based generation
pwgen-cli generate --pattern "Llll-dddd-LLLL" # Letter-digit patterns

# Exclude specific characters
pwgen-cli generate --length 16 --exclude "0O1l"
```

## Generation Patterns

### Random Generation

**Default Pattern:**
- Completely random character selection
- Maximum entropy and security
- No predictable patterns

**Characteristics:**
- High entropy (3.3 bits per character for full charset)
- Passes most security audits
- Difficult to remember but most secure

### Pronounceable Generation

**Syllable-Based:**
- Follows consonant-vowel patterns
- Easier to remember and pronounce
- Slightly reduced entropy

**Pattern Examples:**
- `ba-ri-tu-mo` (8 characters)
- `ke-lar-ni-dox` (10 characters)
- `pa-tel-ka-mor-ne` (12 characters)

### Pattern-Based Generation

**Custom Patterns:**
- Use templates to define password structure
- Combine predictable structure with random content
- Good for passwords that need specific formats

**Pattern Syntax:**
- `L` = Uppercase letter
- `l` = Lowercase letter  
- `d` = Digit
- `s` = Symbol
- `?` = Any character from selected sets

**Pattern Examples:**
```bash
# Credit card PIN style
pwgen-cli generate --pattern "dddd"

# Windows license key style
pwgen-cli generate --pattern "LLLLL-LLLLL-LLLLL-LLLLL-LLLLL"

# Mixed complexity
pwgen-cli generate --pattern "Lllldddd!!"
```

## Security Considerations

### Entropy Calculation

**Character Set Size:**
- Lowercase only: 26 characters = 4.7 bits per character
- + Uppercase: 52 characters = 5.7 bits per character
- + Numbers: 62 characters = 5.95 bits per character
- + Symbols: 94 characters = 6.55 bits per character

**Password Strength:**
- 8 characters (full charset): 52.4 bits (weak)
- 12 characters (full charset): 78.6 bits (good)
- 16 characters (full charset): 104.8 bits (strong)
- 20 characters (full charset): 131 bits (very strong)

### Security Recommendations

**Length Guidelines:**
- **Minimum**: 12 characters for personal accounts
- **Recommended**: 16 characters for important accounts
- **High Security**: 20+ characters for sensitive systems
- **Passphrases**: 5-7 words for memorizable security

**Character Set Usage:**
- Always include uppercase and lowercase letters
- Include numbers for policy compliance
- Add symbols for maximum security
- Avoid if system doesn't support all symbols

## Integration with Password Entries

### Direct Integration

**GUI Integration:**
- Generator tab integrates with password entries
- One-click password generation during entry creation
- Automatic application to selected entries

**CLI Integration:**
```bash
# Generate and add to new entry
pwgen-cli password add --site github.com --username user --generate

# Update existing entry with generated password
pwgen-cli password update --site github.com --generate

# Generate with specific requirements
pwgen-cli password add --site bank.com --username user --generate --length 20 --symbols
```

### Batch Generation

**Multiple Passwords:**
```bash
# Generate multiple passwords for review
pwgen-cli generate --count 10 --length 16

# Generate with different patterns
pwgen-cli generate --count 5 --pattern "Llll-dddd-LLLL"

# Export to file
pwgen-cli generate --count 20 --output passwords.txt
```

## Customization and Profiles

### Generator Profiles

**Creating Profiles:**
```bash
# Create profile for banking passwords
pwgen-cli profile create banking \
  --length 20 \
  --upper --lower --numbers --symbols \
  --no-ambiguous

# Create profile for development
pwgen-cli profile create dev \
  --length 16 \
  --upper --lower --numbers \
  --exclude "'\"`"

# Create pronounceable profile
pwgen-cli profile create memorable \
  --length 12 \
  --pronounceable
```

**Using Profiles:**
```bash
# Generate using profile
pwgen-cli generate --profile banking

# Add entry with profile
pwgen-cli password add --site bank.com --username user --profile banking
```

### Custom Character Sets

**Defining Custom Sets:**
```bash
# Base64-like character set
pwgen-cli generate --charset "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"

# Hexadecimal passwords
pwgen-cli generate --charset "0123456789ABCDEF" --length 32

# ASCII printable without quotes
pwgen-cli generate --charset "!#$%&()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[]^_abcdefghijklmnopqrstuvwxyz{|}~"
```

## Testing and Validation

### Password Strength Testing

**Built-in Analysis:**
```bash
# Test password strength
pwgen-cli analyze strength "MyPassword123!"

# Test against common password lists
pwgen-cli analyze dictionary "password123"

# Test entropy calculation
pwgen-cli analyze entropy "random-password"
```

**Strength Metrics:**
- Entropy calculation (bits of security)
- Dictionary attack resistance
- Pattern analysis
- Character distribution analysis

### Compliance Testing

**Policy Validation:**
```bash
# Test against common policies
pwgen-cli validate policy "MyPassword123!" --policy windows
pwgen-cli validate policy "MyPassword123!" --policy unix
pwgen-cli validate policy "MyPassword123!" --policy custom.json
```

**Custom Policy Definition:**
```json
{
  "name": "Corporate Policy",
  "min_length": 12,
  "max_length": 32,
  "require_uppercase": true,
  "require_lowercase": true,
  "require_numbers": true,
  "require_symbols": true,
  "forbidden_patterns": ["123", "abc", "password"],
  "max_repeating": 2
}
```

## Performance Considerations

### Generation Speed

**Optimization Tips:**
- Simple character sets generate faster
- Pattern-based generation is slower than random
- Pronounceable passwords require more computation
- Batch generation is more efficient than individual calls

**Benchmarking:**
```bash
# Benchmark generation speed
pwgen-cli benchmark --count 1000 --length 16

# Compare different methods
pwgen-cli benchmark --methods random,pronounceable,pattern
```

### Memory Usage

**Secure Memory Handling:**
- Generated passwords are stored in secure memory
- Automatic clearing after use
- No swap file storage
- Secure clipboard handling

## Advanced Features

### Cryptographic Quality

**Random Number Generation:**
- Uses system cryptographic random number generator
- ChaCha20 for additional entropy mixing
- Seeded from multiple entropy sources
- Suitable for cryptographic applications

**Entropy Sources:**
- Operating system random number generator
- Hardware random number generators (when available)
- Process timing variations
- Memory allocation patterns

### API Integration

**Programmatic Access:**
```bash
# JSON output for scripts
pwgen-cli generate --format json --count 5

# Integration with other tools
pwgen-cli generate --format base64 --length 32

# Webhook integration
pwgen-cli generate --webhook-url "https://api.example.com/password"
```

## Troubleshooting

### Common Issues

**Weak Passwords Generated:**
- Increase password length
- Enable all character sets
- Avoid pronounceable generation for high security
- Check entropy calculations

**Policy Compliance Failures:**
- Review password policy requirements
- Adjust character set selection
- Increase password length
- Check for forbidden patterns

**Performance Issues:**
- Use simpler generation methods
- Reduce batch sizes
- Avoid complex patterns
- Check system entropy availability

### Best Practices

**Generation Strategy:**
1. Start with reasonable defaults (16 characters, all character sets)
2. Adjust based on system requirements
3. Test generated passwords against target systems
4. Create profiles for different use cases
5. Regular review and updates of generation policies

**Security Guidelines:**
1. Never reuse generated passwords
2. Store generated passwords securely
3. Use appropriate length for security level
4. Regularly update generation policies
5. Monitor for password policy changes

## Next Steps

- [Password Management](passwords) - Using generated passwords
- [Security Architecture](../security/architecture) - Understanding security
- [Import & Export](import-export) - Password migration