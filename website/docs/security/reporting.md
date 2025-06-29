---
sidebar_position: 4
---

# Security Reporting

How to report security vulnerabilities in PwGen-rust responsibly and effectively.

## Reporting Security Issues

If you discover a security vulnerability, please report it responsibly:

### Contact Information

- **Primary Email**: security@pwgenrust.dev
- **Alternative**: security@hxhippy.com
- **Response Time**: Within 24-48 hours
- **Follow Updates**: [@HxHippy](https://x.com/HxHippy)

### What to Include

1. **Description**: Clear description of the vulnerability
2. **Steps**: How to reproduce the issue
3. **Impact**: Potential security impact
4. **Environment**: Operating system, version, etc.

### What NOT to Include

- Do not publicly disclose the vulnerability
- Do not exploit the vulnerability beyond verification
- Do not access data that isn't yours

## Security Response Process

1. **Acknowledgment**: We'll acknowledge receipt within 48 hours
2. **Investigation**: We'll investigate and assess the issue
3. **Fix**: We'll develop and test a fix
4. **Disclosure**: Coordinated disclosure after fix is available
5. **Credit**: We'll credit reporters (if desired)

## Scope

### In Scope
- Core PwGen-rust applications (CLI, GUI)
- Cryptographic implementations (AES-256-GCM, Argon2)
- Data handling and storage (SQLite encryption)
- Authentication mechanisms
- Memory safety issues
- Key management

### Out of Scope
- Third-party dependencies (report upstream)
- Social engineering attacks
- Physical access attacks
- Brute force attacks

## Security Best Practices

### For Users
- Keep PwGen-rust updated to latest version
- Use strong, unique master passwords (12+ characters)
- Enable auto-lock features (default: 15 minutes)
- Regular encrypted backups in secure locations
- Verify downloads with SHA-256 checksums

### For Developers
- Follow secure coding practices
- Regular security audits
- Dependency vulnerability scanning
- Code review process

## Bug Bounty

We currently don't offer monetary rewards, but we provide:
- Public acknowledgment (if desired)
- Mention in release notes
- Direct contact with development team

## Previous Security Issues

We maintain transparency about past security issues:
- [Security Advisories](https://github.com/HxHippy/PWGen/security/advisories)
- [CVE Database](https://cve.mitre.org/)
- [Release Notes](https://github.com/HxHippy/PWGen/releases)

## Security Features

PwGen-rust implements defense-in-depth security:
- **AES-256-GCM encryption** with authenticated encryption
- **Argon2 key derivation** (memory-hard, side-channel resistant)
- **Rust memory safety** preventing buffer overflows and use-after-free
- **Zeroization** of sensitive data in memory
- **Protection against timing attacks**
- **Local-first architecture** (no network exposure)
- **Regular dependency audits**

## Contact

For security-related questions:
- **Email**: security@pwgenrust.dev
- **GitHub**: [Security tab](https://github.com/HxHippy/PWGen/security)
- **Policy**: [SECURITY.md](https://github.com/HxHippy/PWGen/blob/main/SECURITY.md)
- **Community**: [GitHub Discussions](https://github.com/HxHippy/PWGen/discussions)