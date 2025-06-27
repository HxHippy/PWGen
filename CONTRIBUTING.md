# Contributing to PwGen

🎉 First off, thank you for considering contributing to PwGen! It's people like you that make PwGen such a great tool.

## 📋 Table of Contents

- [Code of Conduct](#code-of-conduct)
- [How Can I Contribute?](#how-can-i-contribute)
- [Development Setup](#development-setup)
- [Pull Request Process](#pull-request-process)
- [Coding Standards](#coding-standards)
- [Security Guidelines](#security-guidelines)
- [Documentation](#documentation)
- [Testing](#testing)

## 📜 Code of Conduct

This project and everyone participating in it is governed by our [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

## 🚀 How Can I Contribute?

### 🐛 Reporting Bugs

Before creating bug reports, please check the [issue tracker](https://github.com/your-username/pwgen/issues) as you might find that the issue has already been reported.

When filing a bug report, please include:

- **Clear description** of the issue
- **Steps to reproduce** the behavior
- **Expected behavior** vs actual behavior
- **Environment details** (OS, Rust version, etc.)
- **Screenshots** if applicable
- **Log output** with `RUST_LOG=debug` if relevant

### 🎯 Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues. When creating an enhancement suggestion, please include:

- **Clear title** and description
- **Detailed explanation** of the proposed functionality
- **Use cases** and examples
- **Possible implementation** approach (if you have ideas)

### 💻 Code Contributions

#### Good First Issues

Look for issues labeled with:
- `good first issue` - Perfect for newcomers
- `help wanted` - We'd love community input
- `documentation` - Help improve our docs

#### Priority Areas

We especially welcome contributions in:
- **Security audits** and improvements
- **Cross-platform compatibility** testing
- **Performance optimizations**
- **User interface** improvements
- **Documentation** and tutorials
- **Test coverage** expansion

## 🛠️ Development Setup

### Prerequisites

- **Rust** 1.70+ ([Install Rust](https://rustup.rs/))
- **Git**
- **Platform-specific dependencies** (see README.md)

### Setting Up Your Development Environment

1. **Fork the repository** on GitHub

2. **Clone your fork:**
   ```bash
   git clone https://github.com/your-username/pwgen.git
   cd pwgen
   ```

3. **Add the upstream repository:**
   ```bash
   git remote add upstream https://github.com/original-owner/pwgen.git
   ```

4. **Install dependencies:**
   ```bash
   cargo build
   ```

5. **Run tests to ensure everything works:**
   ```bash
   cargo test
   ```

6. **Run the application:**
   ```bash
   # GUI
   cargo run --bin pwgen-gui
   
   # CLI
   cargo run --bin pwgen-cli -- --help
   ```

### Development Workflow

1. **Create a feature branch:**
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes** following our coding standards

3. **Test your changes:**
   ```bash
   cargo test
   cargo clippy
   cargo fmt --check
   ```

4. **Commit your changes:**
   ```bash
   git commit -m "feat: add your feature description"
   ```

5. **Push to your fork:**
   ```bash
   git push origin feature/your-feature-name
   ```

6. **Create a Pull Request**

## 🔄 Pull Request Process

### Before Submitting

- [ ] **Rebase** your branch on the latest upstream main
- [ ] **Run tests** and ensure they pass
- [ ] **Run linting** with `cargo clippy`
- [ ] **Format code** with `cargo fmt`
- [ ] **Update documentation** if needed
- [ ] **Add tests** for new functionality

### PR Title Format

Use [Conventional Commits](https://www.conventionalcommits.org/) format:

- `feat: add new password generator options`
- `fix: resolve encryption key derivation issue`
- `docs: update installation instructions`
- `test: add unit tests for storage module`
- `refactor: improve error handling`
- `perf: optimize database queries`

### PR Description Template

```markdown
## Description
Brief description of what this PR does.

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update
- [ ] Performance improvement
- [ ] Refactoring

## Testing
- [ ] Unit tests added/updated
- [ ] Integration tests added/updated
- [ ] Manual testing performed

## Screenshots (if applicable)
Add screenshots to help explain your changes.

## Checklist
- [ ] My code follows the project's style guidelines
- [ ] I have performed a self-review of my code
- [ ] I have commented my code, particularly in hard-to-understand areas
- [ ] I have made corresponding changes to the documentation
- [ ] My changes generate no new warnings
- [ ] I have added tests that prove my fix is effective or that my feature works
- [ ] New and existing unit tests pass locally with my changes
```

### Review Process

1. **Automated checks** must pass (CI/CD pipeline)
2. **Code review** by at least one maintainer
3. **Security review** for security-related changes
4. **Final approval** by a maintainer

## 🎨 Coding Standards

### Rust Style

We follow the standard Rust style guide:

- **Use `rustfmt`** for code formatting
- **Use `clippy`** for linting
- **Follow naming conventions:**
  - `snake_case` for functions and variables
  - `PascalCase` for types and structs
  - `SCREAMING_SNAKE_CASE` for constants

### Code Organization

```rust
// File structure for new modules
pub mod new_module {
    use crate::common_imports;
    
    // Re-exports
    pub use self::submodule::PublicStruct;
    
    // Public types
    pub struct PublicStruct;
    
    // Public functions
    pub fn public_function() -> Result<()> {
        // Implementation
    }
    
    // Private types and functions
    struct PrivateStruct;
    
    fn private_helper() {
        // Implementation
    }
    
    // Tests
    #[cfg(test)]
    mod tests {
        use super::*;
        
        #[test]
        fn test_public_function() {
            // Test implementation
        }
    }
}
```

### Error Handling

- **Use `Result<T, Error>` for fallible functions**
- **Implement custom error types** when needed
- **Provide descriptive error messages**
- **Use `?` operator** for error propagation

```rust
use crate::{Result, Error};

pub fn example_function() -> Result<String> {
    let data = fetch_data()
        .map_err(|e| Error::DataFetch(format!("Failed to fetch: {}", e)))?;
    
    Ok(process_data(data)?)
}
```

### Documentation

- **Document all public APIs** with `///` comments
- **Include examples** in documentation
- **Use `cargo doc` to verify** documentation builds

```rust
/// Generates a secure password with the specified configuration.
///
/// # Arguments
///
/// * `config` - Password generation configuration
///
/// # Returns
///
/// Returns a `Result<String, Error>` containing the generated password
/// or an error if generation fails.
///
/// # Examples
///
/// ```
/// use pwgen_core::generator::{PasswordConfig, PasswordGenerator};
///
/// let config = PasswordConfig::default();
/// let password = PasswordGenerator::generate(&config)?;
/// assert!(password.len() >= config.length);
/// ```
pub fn generate_password(config: &PasswordConfig) -> Result<String> {
    // Implementation
}
```

## 🔒 Security Guidelines

### Security-First Development

- **Never log sensitive data** (passwords, keys, etc.)
- **Use secure random generators** for cryptographic operations
- **Implement constant-time comparisons** for sensitive data
- **Zero sensitive data** after use
- **Validate all inputs** thoroughly

### Cryptographic Standards

- **Use established libraries** (ring, etc.)
- **Follow current best practices** (AES-256-GCM, PBKDF2, etc.)
- **Document cryptographic decisions**
- **Never implement custom crypto**

### Example: Secure Data Handling

```rust
use zeroize::Zeroize;

pub struct SecureString {
    data: Vec<u8>,
}

impl SecureString {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }
    
    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }
}

impl Drop for SecureString {
    fn drop(&mut self) {
        self.data.zeroize();
    }
}
```

## 📚 Documentation

### Types of Documentation

1. **Code Documentation** - Inline comments and docstrings
2. **User Documentation** - README, user guides, tutorials
3. **Developer Documentation** - Architecture, API references
4. **Example Documentation** - Code examples and demos

### Documentation Standards

- **Keep documentation up-to-date** with code changes
- **Use clear, concise language**
- **Include practical examples**
- **Test code examples** to ensure they work

### Building Documentation

```bash
# Build API documentation
cargo doc --open

# Check documentation for warnings
cargo doc --document-private-items

# Spell check (if available)
codespell docs/
```

## 🧪 Testing

### Test Types

1. **Unit Tests** - Test individual functions and modules
2. **Integration Tests** - Test component interactions
3. **End-to-End Tests** - Test complete workflows
4. **Security Tests** - Test security-critical functionality

### Writing Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;

    #[test]
    fn test_password_generation() {
        let config = PasswordConfig::default();
        let password = PasswordGenerator::generate(&config).unwrap();
        
        assert_eq!(password.len(), config.length);
        assert!(password.chars().all(|c| config.charset.contains(c)));
    }

    #[test]
    fn test_error_handling() {
        let invalid_config = PasswordConfig { length: 0, ..Default::default() };
        let result = PasswordGenerator::generate(&invalid_config);
        
        assert!(result.is_err());
    }
}
```

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_password_generation

# Run tests with coverage (if available)
cargo tarpaulin
```

## 🎖️ Recognition

Contributors will be recognized in:
- **GitHub contributors page**
- **CHANGELOG.md** for significant contributions
- **README.md** acknowledgments section
- **Release notes** for major features

## 📞 Getting Help

If you need help with contributing:

1. **Check existing documentation** and issues
2. **Ask in GitHub Discussions**
3. **Contact maintainers** via email
4. **Join our community** channels

## 📄 License

By contributing to PwGen, you agree that your contributions will be licensed under the Apache License 2.0.

---

**Thank you for contributing to PwGen! 🚀**