---
sidebar_position: 4
---

# Contributing

Help make PwGen-rust even better! This guide covers everything you need to know about contributing to the project.

## Getting Started

### Prerequisites

1. **Rust Development Environment**
   - Latest stable Rust (1.75+)
   - Familiarity with Rust programming
   - Basic understanding of cryptography concepts

2. **Development Tools**
   - Git for version control
   - Your preferred IDE/editor
   - Understanding of security best practices

### Quick Setup

```bash
# Fork and clone the repository
git clone https://github.com/YOUR_USERNAME/PWGen.git
cd PWGen

# Build and test
cargo build
cargo test

# Format and lint
cargo fmt
cargo clippy
```

## Contribution Guidelines

### Code of Conduct

We are committed to providing a welcoming and inclusive experience for everyone. Please read our [Code of Conduct](https://github.com/HxHippy/PWGen/blob/main/CODE_OF_CONDUCT.md) before contributing.

### Types of Contributions

- **Bug Reports**: Help us identify and fix issues
- **Feature Requests**: Suggest new functionality
- **Code Contributions**: Submit pull requests
- **Documentation**: Improve guides and tutorials
- **Security**: Report vulnerabilities responsibly
- **Testing**: Help expand test coverage

## Development Workflow

### 1. Create an Issue

Before starting work:
- Check existing issues to avoid duplication
- Create a new issue describing your proposal
- Wait for maintainer feedback on significant changes

### 2. Fork and Branch

```bash
# Fork the repository on GitHub
# Clone your fork
git clone https://github.com/YOUR_USERNAME/PWGen.git

# Create a feature branch
git checkout -b feature/your-feature-name

# Or for bug fixes
git checkout -b fix/issue-123
```

### 3. Development

Follow our coding standards:
- Write clear, documented code
- Include comprehensive tests
- Follow Rust conventions
- Add security considerations

### 4. Testing

```bash
# Run all tests
cargo test

# Run specific tests
cargo test test_name

# Check formatting
cargo fmt --check

# Run linter
cargo clippy -- -D warnings

# Security audit
cargo audit
```

### 5. Commit and Push

```bash
# Make focused commits
git add .
git commit -m "feat: add password strength meter

- Implement zxcvbn algorithm
- Add visual strength indicator
- Include tests for edge cases"

# Push to your fork
git push origin feature/your-feature-name
```

### 6. Pull Request

Create a pull request with:
- Clear title and description
- Reference to related issues
- Screenshots for UI changes
- Test coverage information

## Coding Standards

### Rust Style

Follow standard Rust conventions:
```rust
// Good: Clear, documented functions
/// Encrypts plaintext using AES-256-GCM
/// 
/// # Arguments
/// * `plaintext` - Data to encrypt
/// * `key` - 256-bit encryption key
/// 
/// # Returns
/// Encrypted data with nonce and authentication tag
pub fn encrypt(plaintext: &[u8], key: &Key) -> Result<Vec<u8>, CryptoError> {
    // Implementation
}

// Bad: Unclear, undocumented
pub fn enc(d: &[u8], k: &Key) -> Result<Vec<u8>, CryptoError> {
    // Implementation
}
```

### Security Considerations

- **No Unsafe Code**: Avoid `unsafe` unless absolutely necessary
- **Zeroize Secrets**: Always clear sensitive data from memory
- **Constant-Time Operations**: Use constant-time comparisons for secrets
- **Input Validation**: Validate all inputs thoroughly
- **Error Handling**: Handle errors gracefully without leaking information

### Documentation

- Document all public APIs
- Include examples in doc comments
- Update README and guides as needed
- Write clear commit messages

## Security Contributions

### Reporting Vulnerabilities

**Do NOT create public issues for security vulnerabilities.**

Instead:
1. Email: security@pwgenrust.dev
2. Include detailed reproduction steps
3. Wait for acknowledgment
4. Allow time for fix before disclosure

### Security Code Reviews

Security-sensitive changes require:
- Extra careful review
- Multiple approvals
- Security-focused testing
- Documentation of security implications

## Testing Guidelines

### Test Types

1. **Unit Tests**: Test individual functions
2. **Integration Tests**: Test component interactions
3. **Security Tests**: Test cryptographic functions
4. **Performance Tests**: Benchmark critical paths

### Writing Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_encryption_roundtrip() {
        let password = "test_password";
        let encrypted = encrypt_password(password).unwrap();
        let decrypted = decrypt_password(&encrypted).unwrap();
        assert_eq!(password, decrypted);
    }

    #[test]
    fn test_invalid_key_rejection() {
        let result = decrypt_password(&[0u8; 32]);
        assert!(result.is_err());
    }
}
```

## Documentation Contributions

### Areas Needing Help

- User guides and tutorials
- API documentation
- Security documentation
- Architecture documentation
- Troubleshooting guides

### Writing Guidelines

- Clear, concise language
- Step-by-step instructions
- Code examples that work
- Screenshots where helpful
- Consider different skill levels

## Release Process

### Version Numbering

We follow [Semantic Versioning](https://semver.org/):
- `MAJOR.MINOR.PATCH`
- Major: Breaking changes
- Minor: New features
- Patch: Bug fixes

### Release Checklist

Before releases:
- [ ] All tests pass
- [ ] Documentation updated
- [ ] Changelog updated
- [ ] Security review completed
- [ ] Performance regression check

## Community

### Communication Channels

- **GitHub Issues**: Bug reports and feature requests
- **GitHub Discussions**: General discussion and questions
- **Twitter**: [@HxHippy](https://x.com/HxHippy) for updates
- **Email**: security@pwgenrust.dev for security issues

### Recognition

Contributors are recognized through:
- Credits in release notes
- Contributor list in README
- Special recognition for significant contributions

## Getting Help

### Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust Security Guidelines](https://anssi-fr.github.io/rust-guide/)
- [Project Architecture](architecture)
- [Building Guide](building)

### Questions

- Check existing [GitHub Discussions](https://github.com/HxHippy/PWGen/discussions)
- Create new discussion for general questions
- Tag maintainers for urgent issues

## License

By contributing to PwGen-rust, you agree that your contributions will be licensed under the Apache 2.0 License.

---

Thank you for contributing to PwGen-rust! Together we can build the most secure password manager possible. 🔐

