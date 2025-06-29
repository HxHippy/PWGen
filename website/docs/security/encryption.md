---
sidebar_position: 2
---

# Encryption

Deep dive into PwGen-rust's state-of-the-art encryption implementation and security architecture.

## Overview

PwGen-rust implements military-grade encryption using industry-standard algorithms and best practices. Our encryption architecture is designed to protect your sensitive data even in the event of database compromise.

## Encryption Standards

### AES-256-GCM (Galois/Counter Mode)

PwGen-rust uses **AES-256-GCM** for all data encryption, providing:

- **256-bit encryption keys** - Maximum security strength
- **Authenticated encryption** - Ensures data integrity and authenticity
- **NIST approved** - Meets [NIST SP 800-38D](https://nvlpubs.nist.gov/nistpubs/Legacy/SP/nistspecialpublication800-38d.pdf) standards
- **Performance optimized** - Hardware acceleration support where available

**Why AES-256-GCM?**
- Combines encryption and authentication in a single operation
- Prevents tampering and ensures data hasn't been modified
- Widely supported and thoroughly vetted by security experts
- Optimal balance of security and performance

### Argon2 Key Derivation

For key derivation from your master password, PwGen-rust uses **Argon2**, the winner of the Password Hashing Competition:

- **Memory-hard algorithm** - Resistant to GPU and ASIC attacks
- **Configurable parameters** - Time and memory costs can be adjusted
- **Side-channel resistant** - Protects against timing attacks
- **Future-proof** - Designed to remain secure as hardware improves

**Argon2 vs PBKDF2:**
- Argon2 is specifically designed to be memory-hard
- Provides better protection against parallel attacks
- More resistant to custom hardware attacks
- Recommended by [NIST SP 800-132](https://nvlpubs.nist.gov/nistpubs/Legacy/SP/nistspecialpublication800-132.pdf) for new applications

## Encryption Process

### 1. Master Password Processing

```
User Input → Argon2 → 256-bit Master Key
```

When you enter your master password:
1. Password is combined with a unique salt
2. Processed through Argon2 with configured iterations
3. Produces a 256-bit master key
4. Original password is immediately cleared from memory

### 2. Data Encryption

```
Plaintext → AES-256-GCM → Ciphertext + Authentication Tag
```

For each piece of data:
1. Generate a unique 96-bit nonce (never reused)
2. Encrypt data using AES-256-GCM with the master key
3. Produce ciphertext and 128-bit authentication tag
4. Store nonce, ciphertext, and tag together

### 3. Data Decryption

```
Ciphertext + Tag → Verify → AES-256-GCM → Plaintext
```

When accessing data:
1. Retrieve nonce, ciphertext, and authentication tag
2. Verify authentication tag (fails if data tampered)
3. Decrypt only if verification succeeds
4. Clear decrypted data from memory after use

## Security Features

### Zero-Knowledge Architecture

- **Local-only encryption** - All encryption happens on your device
- **No key escrow** - We never have access to your master password or keys
- **No recovery backdoors** - Lost master password means data is unrecoverable
- **Client-side security** - Server (when available) never sees unencrypted data

### Memory Security

PwGen-rust leverages Rust's memory safety features:

- **Automatic zeroization** - Sensitive data cleared from memory after use
- **No buffer overflows** - Rust's ownership system prevents memory vulnerabilities
- **Stack protection** - Sensitive data never stored in swap files
- **Secure random generation** - Uses OS-provided CSPRNG for all random values

### Key Security

- **Unique salts** - Every vault uses a unique salt for key derivation
- **No key reuse** - Each data item encrypted with unique nonce
- **Forward secrecy** - Compromising one item doesn't affect others
- **Key rotation support** - Change master password without re-encrypting data

## Implementation Details

### Cryptographic Libraries

PwGen-rust uses well-audited Rust cryptography libraries:

- **`ring`** - For AES-GCM encryption (same library used by AWS)
- **`argon2`** - Reference implementation of Argon2
- **`getrandom`** - Secure random number generation
- **`zeroize`** - Secure memory clearing

### Storage Format

Encrypted data is stored with:
```
[Version][Salt][Nonce][Ciphertext][AuthTag]
```

- **Version**: Allows for future algorithm upgrades
- **Salt**: Unique per vault (32 bytes)
- **Nonce**: Unique per encryption (12 bytes)
- **Ciphertext**: Variable length encrypted data
- **AuthTag**: Authentication tag (16 bytes)

## Threat Model

### What We Protect Against

- **Database theft** - Encrypted data useless without master password
- **Memory dumps** - Sensitive data zeroized after use
- **Tampering** - Authentication tags detect any modifications
- **Brute force** - Argon2 makes password cracking expensive
- **Side channels** - Constant-time operations where possible

### What We Cannot Protect Against

- **Compromised system** - Keyloggers or malware on your device
- **Weak master passwords** - Use strong, unique master password
- **Physical access** - Unlocked vault on unattended computer
- **User errors** - Sharing master password or leaving vault unlocked

## Best Practices

### Master Password Selection

1. **Length**: Minimum 12 characters, recommend 16+
2. **Complexity**: Mix of character types or passphrase
3. **Uniqueness**: Never used elsewhere
4. **Memorability**: You must remember it - no recovery possible

### Operational Security

1. **Regular backups** - Encrypted backups protect against data loss
2. **Auto-lock** - Configure appropriate timeout for your environment
3. **Secure machine** - Keep OS and software updated
4. **Physical security** - Lock computer when stepping away

### Compliance

PwGen-rust's encryption meets requirements for:
- **GDPR** - Strong encryption for personal data protection
- **HIPAA** - Suitable for healthcare data encryption
- **PCI DSS** - Appropriate for payment card data
- **SOX** - Adequate controls for financial data

## Technical Specifications

### Algorithm Parameters

**AES-256-GCM:**
- Key size: 256 bits
- Block size: 128 bits
- Nonce size: 96 bits
- Tag size: 128 bits

**Argon2id:**
- Memory: 64 MB (configurable)
- Iterations: 3 (configurable)
- Parallelism: 1
- Output: 256 bits

### Performance Characteristics

- **Encryption speed**: ~1GB/s on modern hardware with AES-NI
- **Key derivation**: 100-500ms (configurable for security/speed trade-off)
- **Memory usage**: ~100MB including Argon2 memory requirements
- **Database overhead**: ~5% for encryption metadata

## Future Enhancements

### Planned Improvements

- **Post-quantum algorithms** - Preparing for quantum computing threats
- **Hardware key support** - YubiKey and similar devices
- **Key rotation** - Automated master key rotation
- **Multiple key slots** - Support for multiple master passwords

### Algorithm Agility

PwGen-rust is designed to support algorithm updates:
- Version field allows detection of encryption version
- Modular design enables algorithm swapping
- Backward compatibility for reading old formats
- Migration tools for algorithm upgrades

## Resources

### Further Reading

- [NIST Cryptographic Standards](https://csrc.nist.gov/publications/sp)
- [Rust Crypto Libraries](https://github.com/RustCrypto)
- [OWASP Cryptographic Storage](https://cheatsheetseries.owasp.org/cheatsheets/Cryptographic_Storage_Cheat_Sheet.html)
- [Argon2 Specification](https://github.com/P-H-C/phc-winner-argon2)

### Security Audits

While PwGen-rust has not yet undergone formal security audit, we:
- Use well-audited cryptographic libraries
- Follow established best practices
- Welcome security research and responsible disclosure
- Plan formal audit as project matures

For security concerns, please see our [Security Policy](reporting).