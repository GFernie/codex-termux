# @mmmbuto/codex-cli-lts

> **Long-term support line for Linux x64, Android Termux (ARM64), and macOS arm64**

[![npm](https://img.shields.io/npm/v/@mmmbuto/codex-cli-lts?style=flat-square&logo=npm)](https://www.npmjs.org/package/@mmmbuto/codex-cli-lts)
[![downloads](https://img.shields.io/npm/dt/@mmmbuto/codex-cli-lts?style=flat-square)](https://www.npmjs.org/package/@mmmbuto/codex-cli-lts)
[![ko-fi](https://img.shields.io/badge/☕_Support-Ko--fi-FF5E5B?style=flat-square&logo=ko-fi)](https://ko-fi.com/dionanos)

---

## About

LTS (Long-term support) release line based on upstream OpenAI Codex rust-v0.80.0. This version focuses on stability and /chat compatibility with minimal features and security patches.

### Supported Platforms

- Linux x64
- Android Termux (ARM64)
- macOS arm64 (via GitHub releases/CI artifacts)

---

## Project Scope

### LTS (Multi-platform)
- Based on upstream rust-v0.80.0
- Supports /chat and /responses wire APIs
- Minimal features + security patches only
- Stable for compatibility-focused use cases

### What We Do
✅ **Use official OpenAI Codex source** (https://github.com/openai/codex)
✅ **Compile for Linux x64 + ARM64** (Android Termux native)
✅ **Apply minimal patches** only for Termux-specific issues not addressed upstream
✅ **Package as npm** for easy installation
✅ **Maintain full Apache 2.0 compliance** with OpenAI attribution

### What We DON'T Do
❌ **NO new features**
❌ **NO behavior modifications** (works exactly like upstream)
❌ **NO replacement** of official Codex

---

## Patches & Updates

### LTS Updates
- Based on rust-v0.80.0 (minimal features + security only)
- Maintains /chat wire API compatibility
- Stability-focused for production use

### Latest Patches
We apply patches for issues that:
- **Prevent Codex from working on Termux**
- **Are not addressed by upstream** (Termux is not officially supported)
- **Are minimal and well-documented**

**Current patches**: See [../patches/](../patches/) directory for full documentation.

Need help debugging upgrade alerts? See
[../docs/termux-upgrade-checks.md](../docs/termux-upgrade-checks.md) for known causes
and fix strategies.

**Found an issue?** Well-documented bug reports with reproduction steps are welcome! Open an [issue](https://github.com/DioNanos/codex-termux/issues).

---

## 📦 Installation

### Termux (Android ARM64)

```bash
# Update Termux packages and install Node.js
pkg update && pkg upgrade -y
pkg install nodejs-lts -y

# Install LTS
npm install -g @mmmbuto/codex-cli-lts

# Verify
codex --version
codex login
```

**Requirements:** Android 7+, ARM64, Node.js >=18 (recommended v22+), ~50MB storage

---

### Linux (x64)

```bash
# Install Node.js (example for Debian/Ubuntu)
sudo apt-get update
sudo apt-get install -y nodejs npm

# Install LTS
npm install -g @mmmbuto/codex-cli-lts

# Verify
codex --version
codex login
```

**Requirements:** Linux x64, Node.js >=18 (recommended v22+), ~80MB storage

---

### macOS (arm64)

```bash
# Using Homebrew
brew install node

# Download and install LTS from GitHub releases
curl -L https://github.com/DioNanos/codex-termux/releases/latest/download/codex-cli-lts-macos-arm64.tar.gz -o codex-cli-lts-macos-arm64.tar.gz
tar -xzf codex-cli-lts-macos-arm64.tar.gz
sudo mv codex codex-exec /usr/local/bin/

# Verify
codex --version
codex login
```

**Requirements:** macOS arm64, Node.js >=18 (recommended v22+), ~100MB storage

---

## 📚 Documentation

- [Installation Details](../docs/installation.md)
- [Testing](../docs/testing.md)
- [Building from Source](../BUILDING.md)
- [Test Reports](../test-reports/)
- [Full Documentation](../docs/)

---

## 🔧 Project Maintenance

Community-maintained port enabling AI-powered coding on Android Termux. Activities include ARM64 compilation, upstream synchronization, Termux compatibility patches, and documentation.

**Thank you** to all users who have reported issues, provided feedback, and helped improve this project.

---

## 📝 License

This project maintains full compliance with Apache 2.0 license from OpenAI Codex.

**Original work**: Copyright OpenAI (https://github.com/openai/codex)
**Termux port**: Minimal patches for Android compatibility

See [../LICENSE](../LICENSE) file for details.

---

## 🙏 Credits

- **OpenAI** for the amazing Codex CLI
- **Termux** community for Android terminal environment
- All contributors to upstream Codex project

---

**Maintained**: Community-driven, not affiliated with OpenAI
