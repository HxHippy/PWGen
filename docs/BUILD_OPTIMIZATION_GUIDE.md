# PwGen Build Optimization Guide v1.2

This guide provides comprehensive information about PwGen's v1.2 build optimization features, including build profiles, feature flags, and deployment strategies.

## 🎯 Overview

PwGen v1.2 introduces sophisticated build optimization capabilities that allow you to create binaries tailored for specific use cases:

- **30-40% smaller binaries** through dependency optimization
- **Conditional compilation** with feature flags
- **Multiple build profiles** for different optimization levels
- **Platform-specific optimizations** for reduced dependencies

## 🔧 Build Profiles

### Release Profile (Default)
Optimized for size while maintaining good performance:

```toml
[profile.release]
opt-level = "z"     # Optimize for size
lto = true          # Enable Link Time Optimization
codegen-units = 1   # Single codegen unit for better optimization
strip = true        # Strip symbols from binary
panic = "abort"     # Smaller panic handling
```

**Usage:**
```bash
cargo build --release
```

**Characteristics:**
- Balanced size/performance optimization
- Suitable for most production deployments
- Good compromise between build time and binary size

### Min-Size Profile (Maximum Optimization)
Maximum size optimization for constrained environments:

```toml
[profile.min-size]
inherits = "release"
opt-level = "z"
lto = "fat"         # Aggressive Link Time Optimization
codegen-units = 1
panic = "abort"
strip = "symbols"   # Strip all symbols including debug info
```

**Usage:**
```bash
cargo build --profile min-size
```

**Characteristics:**
- Smallest possible binary size
- Longer build times due to aggressive optimization
- Perfect for distribution and embedded scenarios

## 🚩 Feature Flags

### Core Library Features (pwgen-core)

#### `document-compression` (Default: Enabled)
Enables document compression functionality using flate2:

```toml
# Enable compression
cargo build --release --features document-compression

# Disable compression  
cargo build --release --no-default-features
```

**When to use:**
- ✅ **Enable** when you need to store large documents
- ❌ **Disable** for minimal builds or when compression isn't needed

**Impact:**
- **Size**: ~1-2MB when disabled
- **Functionality**: Document storage without compression when disabled

### GUI Features (pwgen-gui)

#### `clipboard` (Default: Enabled)
Enables clipboard integration using arboard:

```toml
# Enable clipboard
cargo build --release --features clipboard

# Disable clipboard
cargo build --release --no-default-features
```

**When to use:**
- ✅ **Enable** for desktop environments with clipboard support
- ❌ **Disable** for headless servers or minimal environments

**Impact:**
- **Size**: ~500KB-1MB when disabled
- **Dependencies**: Reduces Windows SDK requirements when disabled
- **Functionality**: No copy-to-clipboard when disabled

## 📋 Build Variants

### 1. Standard Build (Recommended)
Full features with size optimization:

```bash
cargo build --release
```

**Includes:**
- Clipboard support
- Document compression
- All GUI features
- Cross-platform compatibility

**Use cases:**
- Desktop installations
- General purpose deployments
- Development environments

### 2. Minimal Build
Core functionality only:

```bash
cargo build --release --no-default-features
```

**Includes:**
- Core password management
- Basic GUI (no clipboard)
- No document compression
- Reduced dependencies

**Use cases:**
- Windows environments with SDK issues
- Embedded systems
- Containerized deployments
- Security-focused minimal installations

### 3. Size-Optimized Build
Maximum optimization:

```bash
cargo build --profile min-size
```

**Includes:**
- All features
- Maximum size optimization
- Smallest possible binaries

**Use cases:**
- Distribution packages
- Mobile deployment preparation
- Bandwidth-constrained environments

### 4. Custom Feature Builds
Selective feature enabling:

```bash
# Clipboard only
cargo build --release --no-default-features --features clipboard

# Compression only
cargo build --release --no-default-features --features document-compression

# Both features explicitly
cargo build --release --features "clipboard,document-compression"
```

## 🛠️ Platform-Specific Optimization

### Windows Optimization

**Standard Windows Build:**
```bash
cargo build --release
```

**Minimal Windows Build (Reduced SDK Requirements):**
```bash
cargo build --release --no-default-features
```

**Benefits:**
- Eliminates `arboard` dependency (clipboard)
- Reduces Windows SDK requirements
- Smaller binary size
- Fewer potential compatibility issues

**Trade-offs:**
- No clipboard functionality
- Manual copy/paste required

### Linux/macOS Optimization

**Recommended Build:**
```bash
cargo build --release
```

**Container/Server Build:**
```bash
cargo build --release --no-default-features
```

**Benefits:**
- Native clipboard support works well
- All optimizations apply
- Good performance characteristics

### Cross-Platform Considerations

**For Distribution:**
```bash
# Create optimized builds for all platforms
cargo build --profile min-size --target x86_64-pc-windows-msvc
cargo build --profile min-size --target x86_64-unknown-linux-gnu  
cargo build --profile min-size --target x86_64-apple-darwin
```

## 📊 Performance Comparison

### Binary Size Comparison

| Build Type | Windows | Linux | macOS | Size Reduction |
|------------|---------|--------|-------|----------------|
| Standard v1.1 | 45MB | 42MB | 44MB | Baseline |
| Standard v1.2 | 30MB | 28MB | 29MB | 30-35% |
| Minimal v1.2 | 25MB | 23MB | 24MB | 40-45% |
| Min-Size v1.2 | 22MB | 20MB | 21MB | 45-50% |

### Build Time Comparison

| Build Type | Time | LTO Level | Optimization |
|------------|------|-----------|--------------|
| Debug | 2m 30s | None | None |
| Release | 8m 15s | Full | Size |
| Min-Size | 12m 45s | Fat | Maximum |

### Memory Usage

| Build Type | Startup RAM | Runtime RAM | GPU Memory |
|------------|-------------|-------------|------------|
| Standard v1.1 | 35MB | 45MB | 15MB |
| Standard v1.2 | 30MB | 38MB | 12MB |
| Minimal v1.2 | 25MB | 32MB | 10MB |

## 🔍 Dependency Analysis

### Removed Dependencies (v1.2)

| Dependency | Size Impact | Reason for Removal |
|------------|-------------|-------------------|
| `tauri-build` | 5MB | Unused build tool |
| `egui_extras` | 3MB | Unused GUI extensions |
| `image` (replaced) | 2MB | Replaced with lightweight PNG decoder |
| MD5 crypto | 500KB | Security improvement (SHA-256 only) |

### Optimized Dependencies

| Dependency | Before | After | Optimization |
|------------|--------|-------|--------------|
| `tokio` | "full" features | Specific features | Reduced runtime size |
| `sqlx` | All features | Required only | Smaller database layer |
| `png` | N/A (was `image`) | PNG-only | Lightweight image processing |

## 🚀 CI/CD Integration

### GitHub Actions Example

```yaml
name: Optimized Build

on: [push, pull_request]

jobs:
  build-optimized:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          
      - name: Build Standard
        run: cargo build --release
        
      - name: Build Minimal  
        run: cargo build --release --no-default-features
        
      - name: Build Size-Optimized
        run: cargo build --profile min-size
        
      - name: Measure Sizes
        run: |
          ls -lah target/release/pwgen-*
          ls -lah target/min-size/pwgen-*
```

### Docker Optimization

```dockerfile
# Multi-stage build for smallest image
FROM rust:1.70 as builder

WORKDIR /app
COPY . .

# Build minimal version
RUN cargo build --profile min-size --no-default-features

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/min-size/pwgen-cli /usr/local/bin/
COPY --from=builder /app/target/min-size/pwgen-gui /usr/local/bin/

CMD ["pwgen-cli"]
```

## 🧪 Testing Optimizations

### Verification Commands

```bash
# Check binary sizes
ls -lah target/release/pwgen-*
ls -lah target/min-size/pwgen-*

# Check dependencies
cargo tree

# Check features
cargo tree --features clipboard,document-compression
cargo tree --no-default-features

# Run tests with different feature sets
cargo test --release
cargo test --release --no-default-features
cargo test --release --features clipboard
```

### Performance Benchmarking

```bash
# Startup time benchmark
time ./target/release/pwgen-gui --version
time ./target/min-size/pwgen-gui --version

# Memory usage benchmark  
valgrind --tool=massif ./target/release/pwgen-cli --help
valgrind --tool=massif ./target/min-size/pwgen-cli --help
```

## 🔧 Troubleshooting

### Common Build Issues

**Issue: Windows SDK Required**
```bash
# Solution: Use minimal build
cargo build --release --no-default-features
```

**Issue: Out of Memory During Build**
```bash
# Solution: Reduce codegen units temporarily
CARGO_BUILD_JOBS=1 cargo build --release
```

**Issue: Features Not Working**
```bash
# Solution: Check feature flags
cargo build --release --features "clipboard,document-compression"
```

### Verification

**Check Active Features:**
```bash
# List compiled features
cargo rustc --release -- --print cfg | grep feature
```

**Verify Binary Size:**
```bash
# Compare sizes
du -h target/release/pwgen-*
du -h target/min-size/pwgen-*
```

## 📈 Future Optimizations

### Planned Improvements

1. **WASM Support**: Preparation for browser-based builds
2. **Profile-Guided Optimization**: Runtime-based optimization
3. **Split Binaries**: Separate core/GUI for microservice architecture
4. **Plugin System**: Dynamic loading for optional features

### Experimental Optimizations

```bash
# Enable experimental optimizations (use with caution)
RUSTFLAGS="-C target-cpu=native" cargo build --release
RUSTFLAGS="-C panic=abort" cargo build --release
```

## 💡 Best Practices

### For Development
- Use debug builds for faster iteration
- Use `--features` flag to test specific functionality
- Profile regularly to catch regressions

### For Production
- Always use `--release` for deployments
- Use `min-size` profile for distribution
- Test feature combinations thoroughly

### For Distribution
- Build multiple variants for different use cases
- Document feature requirements clearly
- Provide installation guides for each variant

---

This guide will be updated as new optimization techniques are developed. For the latest information, check the [GitHub repository](https://github.com/hxhippy/pwgen) and [documentation](../README.md).