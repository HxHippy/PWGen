---
sidebar_position: 4
---

# Security Reporting

How to report security vulnerabilities in PwGen.

## Reporting Security Issues

If you discover a security vulnerability, please report it responsibly:

### Contact Information

- **Email**: security@hxhippy.com
- **Encrypted**: [PGP Key](https://hxhippy.com/pgp)
- **Response Time**: Within 48 hours

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
- Core PwGen applications (CLI, GUI)
- Cryptographic implementations
- Data handling and storage
- Authentication mechanisms

### Out of Scope
- Third-party dependencies (report upstream)
- Social engineering attacks
- Physical access attacks
- Brute force attacks

## Security Best Practices

### For Users
- Keep PwGen updated to latest version
- Use strong master passwords
- Enable auto-lock features
- Regular backups in secure locations

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
- [Security Advisories](https://github.com/hxhippy/pwgen/security/advisories)
- [CVE Database](https://cve.mitre.org/)

## Security Features

PwGen implements several security measures:
- AES-256-GCM encryption
- Secure memory handling
- Protection against timing attacks
- Regular security audits

## Contact

For security-related questions:
- **Email**: security@hxhippy.com
- **GitHub**: [Security tab](https://github.com/hxhippy/pwgen/security)
- **Policy**: [SECURITY.md](https://github.com/hxhippy/pwgen/blob/main/SECURITY.md)