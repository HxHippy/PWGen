# Security Policy

## 🛡️ Security Overview

PwGen takes security seriously. This document outlines our security practices, how to report vulnerabilities, and our response process.

## 🔒 Supported Versions

We provide security updates for the following versions:

| Version | Supported          |
| ------- | ------------------ |
| 1.x.x   | ✅ Yes             |
| 0.x.x   | ❌ No (Development)|

## 🚨 Reporting a Vulnerability

### Where to Report

**DO NOT** create public GitHub issues for security vulnerabilities.

Instead, please report security issues via:

- **Email**: security@example.com (replace with actual contact)
- **Subject**: `[SECURITY] Brief description of the issue`
- **Encryption**: Use our PGP key for sensitive reports (key ID: coming soon)

### What to Include

Please include the following information:

1. **Description** of the vulnerability
2. **Steps to reproduce** the issue
3. **Potential impact** and severity assessment
4. **Affected versions** (if known)
5. **Proof of concept** (if applicable)
6. **Suggested mitigation** (if you have ideas)

### Response Timeline

- **Initial Response**: Within 24-48 hours
- **Assessment**: Within 1 week
- **Fix Development**: Depends on severity (see below)
- **Public Disclosure**: After fix is released

### Severity Levels

- **Critical**: Fix within 24-48 hours
- **High**: Fix within 1 week  
- **Medium**: Fix within 2-4 weeks
- **Low**: Fix in next regular release

## 🔐 Security Architecture

### Encryption Standards

- **Algorithm**: AES-256-GCM for symmetric encryption
- **Key Derivation**: PBKDF2 with SHA-256, minimum 100,000 iterations
- **Random Generation**: Cryptographically secure random number generation
- **Memory Protection**: Secure memory handling and zeroization

### Data Protection

- **At Rest**: All sensitive data encrypted in SQLite database
- **In Memory**: Sensitive data cleared after use
- **In Transit**: No network transmission of sensitive data (local-only)
- **Backups**: Encrypted with same standards as main database

### Authentication

- **Master Password**: Required for all vault access
- **No Storage**: Master password never stored, only derived key
- **Session Management**: Automatic timeout and manual lock
- **Brute Force Protection**: Rate limiting and secure key derivation

## 🛠️ Security Features

### Built-in Protections

- **Memory Safety**: Rust's ownership model prevents buffer overflows
- **Input Validation**: All user inputs validated and sanitized
- **Error Handling**: No sensitive information in error messages
- **Audit Logging**: All access and modifications logged
- **Integrity Checks**: Database integrity verification

### Security Hardening

- **Minimal Dependencies**: Reduced attack surface
- **Static Analysis**: Regular security scanning
- **Code Review**: Security-focused code review process
- **Testing**: Security-specific test cases

## 🔍 Security Testing

### What We Test

- **Cryptographic Implementation**: Verification of encryption/decryption
- **Key Derivation**: PBKDF2 implementation and parameters
- **Memory Management**: Proper zeroization of sensitive data
- **Input Validation**: Fuzzing and boundary testing
- **Authentication**: Master password verification
- **Database Security**: Encrypted storage verification

### External Audits

We welcome security researchers and auditors to review our codebase. If you're interested in conducting a security audit, please contact us.

## 🚀 Secure Development Process

### Development Practices

- **Secure by Design**: Security considerations from initial design
- **Code Review**: All code reviewed for security issues
- **Static Analysis**: Automated security scanning in CI/CD
- **Dependency Scanning**: Regular vulnerability scanning of dependencies
- **Threat Modeling**: Regular threat assessment updates

### Release Process

- **Security Testing**: All releases undergo security testing
- **Vulnerability Scanning**: Automated scanning before release
- **Dependency Updates**: Regular updates to address vulnerabilities
- **Security Documentation**: Updates to security documentation

## 📋 Security Checklist for Contributors

When contributing to PwGen, please ensure:

- [ ] No hardcoded secrets or credentials
- [ ] Proper input validation and sanitization
- [ ] Secure error handling (no information leakage)
- [ ] Appropriate use of cryptographic functions
- [ ] Memory safety and secure deletion
- [ ] Documentation of security-relevant changes

## 🔗 Security Resources

### Documentation

- [Rust Security Guidelines](https://doc.rust-lang.org/stable/std/mem/fn.forget.html)
- [OWASP Password Storage Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html)
- [Cryptographic Best Practices](https://crypto.stanford.edu/~dabo/cryptobook/)

### Dependencies

Our cryptographic dependencies:
- **ring**: Cryptographic primitives
- **sqlx**: Database driver with prepared statements
- **zeroize**: Secure memory clearing
- **uuid**: Secure UUID generation

## 🏅 Security Hall of Fame

We recognize security researchers who help improve PwGen's security:

<!-- Future contributors will be listed here -->

*Be the first to contribute to our security!*

## ⚖️ Responsible Disclosure

We believe in responsible disclosure and will:

- **Acknowledge** your contribution publicly (if desired)
- **Credit** you in release notes and documentation
- **Work with you** on disclosure timeline
- **Provide updates** on fix progress

We ask that you:

- **Give us time** to fix issues before public disclosure
- **Provide reasonable detail** to help us understand the issue
- **Avoid accessing** or modifying other users' data
- **Don't perform testing** that could harm users or systems

## 📞 Contact Information

- **Security Email**: security@example.com (replace with actual)
- **General Issues**: GitHub Issues (for non-security issues)
- **Discussions**: GitHub Discussions

## 🔄 Updates to This Policy

This security policy may be updated periodically. Check the Git history for changes and updates.

---

**Thank you for helping keep PwGen secure! 🛡️**