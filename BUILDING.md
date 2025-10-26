# 🔨 Building Codex from Source (Termux)

This guide is for **advanced users** who want to compile Codex from source on Android Termux.

**For most users**, we recommend using the pre-compiled npm package:
```bash
npm install -g @mmmbuto/codex-cli-termux
```

---

## 📋 Prerequisites

### Install Build Dependencies

```bash
# Update packages
pkg update && pkg upgrade -y

# Install required packages
pkg install -y \
  rust \
  binutils \
  git \
  nodejs-lts \
  python

# Verify installations
rustc --version  # Should be 1.75+
cargo --version
node --version   # Should be v18+
git --version
```

### System Requirements

| Requirement | Minimum | Recommended |
|-------------|---------|-------------|
| **Android Version** | 7.0+ | 12+ |
| **Architecture** | ARM64 | ARM64 |
| **RAM** | 4GB | 8GB+ |
| **Free Storage** | 2GB | 4GB |
| **Compilation Time** | 60-90 min | 30-60 min |

---

## 🚀 Build Steps

### 1. Clone Repository

```bash
# Clone the repository
git clone https://github.com/DioNanos/codex-termux.git
cd codex-termux

# Check current branch
git branch
```

### 2. Navigate to Rust Source

```bash
cd codex-rs
```

### 3. Build Release Binary

```bash
# Build optimized release binary
cargo build --release

# This will take 30-90 minutes depending on your device
# Progress will be shown in the terminal
```

**Note**: Compilation is memory-intensive. If you encounter out-of-memory errors, see the [Low RAM Configuration](#low-ram-configuration) section below.

### 4. Install Binary

```bash
# Copy compiled binary to local bin
mkdir -p ~/.local/bin
cp target/release/codex-cli ~/.local/bin/codex

# Make it executable
chmod +x ~/.local/bin/codex

# Add to PATH if not already (add to ~/.bashrc or ~/.zshrc)
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

### 5. Verify Installation

```bash
# Check version
codex --version

# Should output: codex-cli X.Y.Z
```

---

## ⚙️ Low RAM Configuration

If compilation fails with out-of-memory errors on devices with **less than 8GB RAM**, edit the Cargo.toml optimization settings:

### Edit Cargo.toml

```bash
cd ~/codex-termux/codex-rs
nano Cargo.toml
```

Find the `[profile.release]` section and modify:

```toml
[profile.release]
opt-level = 2           # Changed from 3 (less optimization, less RAM)
lto = false            # Changed from "fat" (disable link-time optimization)
codegen-units = 16     # Changed from 1 (more parallel jobs, less RAM per job)
```

Save and retry compilation:

```bash
cargo clean
cargo build --release
```

---

## 🔄 Update to Latest Version

```bash
cd ~/codex-termux

# Fetch latest changes from upstream
git fetch origin
git pull origin main

# Rebuild
cd codex-rs
cargo build --release

# Reinstall
cp target/release/codex-cli ~/.local/bin/codex
```

---

## 🐛 Troubleshooting

### Out of Memory During Compilation

**Solution**: Use low RAM configuration (see above) or reduce parallel jobs:

```bash
cargo build --release -j 1  # Use only 1 parallel job
```

### Missing Dependencies

```bash
# Reinstall all build dependencies
pkg install -y rust binutils git nodejs-lts python
```

### Binary Not Found After Install

```bash
# Check if ~/.local/bin is in PATH
echo $PATH

# If not, add it
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

### Compilation Errors

```bash
# Clean build cache and retry
cd ~/codex-termux/codex-rs
cargo clean
cargo build --release
```

---

## 📚 Additional Resources

- **Upstream Source**: https://github.com/openai/codex
- **Rust Installation**: https://www.rust-lang.org/tools/install
- **Termux Wiki**: https://wiki.termux.com/wiki/Main_Page
- **Cargo Documentation**: https://doc.rust-lang.org/cargo/

---

## ❓ Why Compile from Source?

**Advantages:**
- ✅ Latest development features
- ✅ Custom optimizations for your device
- ✅ Learning experience

**Disadvantages:**
- ❌ Takes 30-90 minutes
- ❌ Requires 2-4GB storage
- ❌ Memory-intensive (may fail on low RAM devices)
- ❌ Manual updates required

**For most users, the npm package is better:**
```bash
npm install -g @mmmbuto/codex-cli-termux
```

---

**Last Updated**: October 26, 2025
**Tested on**: Pixel 9 Pro, Android 15, Termux (ARM64)
