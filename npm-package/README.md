# 🤖 Codex CLI - Latest Termux & LTS Termux/Linux/macOS

> **Built from upstream OpenAI Codex v0.112.0, compiled for Android Termux (ARM64) + LTS Termux/Linux/macOS**

[![npm termux](https://img.shields.io/npm/v/@mmmbuto/codex-cli-termux?style=flat-square&logo=npm)](https://www.npmjs.org/package/@mmmbuto/codex-cli-termux)
[![npm lts](https://img.shields.io/npm/v/@mmmbuto/codex-cli-lts?style=flat-square&logo=npm)](https://www.npmjs.org/package/@mmmbuto/codex-cli-lts)

---

![Codex Termux Header](./docs/assets/codex-termux-header.jpg)

## About

Built from upstream OpenAI Codex source, compiled for Linux x64 and Android Termux. Since Termux is not officially supported by upstream, we apply minimal patches only for critical compatibility issues.

### Release Lines

- **Latest**: Termux-only, tracks upstream OpenAI closely
- **LTS**: Long-term support based on upstream rust-v0.80.0, minimal features + security patches to maintain /chat compatibility

**Supported Platforms:**
- Android Termux (ARM64): Latest + LTS
- Linux x64: LTS
- macOS arm64: LTS (via npm + GitHub releases/CI artifacts)

---

## Project Scope

### Latest (Termux-only)
- Compiles for ARM64 native on Android Termux
- Tracks upstream OpenAI closely
- Applies minimal patches for Termux-specific compatibility

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
❌ **NO broad behavior modifications** outside required Termux compatibility
❌ **NO replacement** of official Codex

---

## Patches & Updates

### Latest Patches
We apply patches for issues that:
- **Prevent Codex from working on Termux**
- **Are not addressed by upstream** (Termux is not officially supported)
- **Are minimal and well-documented**

**Current patches**: See [patches/README.md](./patches/README.md) for full documentation.

**Termux build note:** the published latest Termux package disables voice/realtime audio.
This avoids Android linker failures such as missing `libOpenSLES.so` while keeping the
rest of the upstream CLI behavior intact.

### LTS Updates
- Based on rust-v0.80.0 (minimal features + security only)
- Maintains /chat wire API compatibility
- Stability-focused for production use

Need help debugging upgrade alerts? See
[docs/termux-upgrade-checks.md](./docs/termux-upgrade-checks.md) for known causes
and fix strategies.

**Found an issue?** Well-documented bug reports with reproduction steps are welcome! Open an [issue](https://github.com/DioNanos/codex-termux/issues).

---

## 📦 Installation

### Termux (Android ARM64)

```bash
# Update Termux packages and install Node.js
pkg update && pkg upgrade -y
pkg install nodejs-lts -y

# Latest (tracks upstream)
npm install -g @mmmbuto/codex-cli-termux

# OR LTS (stable, /chat compatible)
npm install -g @mmmbuto/codex-cli-lts

# Verify
codex --version
codex login
```

**Requirements:** Android 7+, ARM64, Node.js >=18 (recommended v22+), ~50MB storage

**Current latest limitation:** voice/realtime audio is intentionally disabled in the
published Termux build to keep packaged binaries free of Android-only audio linker
dependencies.

---

### Linux (x64)

```bash
# Install Node.js (example for Debian/Ubuntu)
sudo apt-get update
sudo apt-get install -y nodejs npm

# LTS (Linux support)
npm install -g @mmmbuto/codex-cli-lts

# Verify
codex --version
codex login
```

**Requirements:** Linux x64, Node.js >=18 (recommended v22+), ~80MB storage

---

### macOS (arm64)

```bash
# Using npm
npm install -g @mmmbuto/codex-cli-lts

# Verify
codex --version
codex login
```

**Requirements:** macOS arm64, Node.js >=18 (recommended v22+), ~100MB storage

---

## 📚 Documentation

- [Installation Details](./docs/installation.md)
- [Testing](./docs/testing.md)
- [Building from Source](./BUILDING.md)
- [Test Reports](./test-reports/)
- [Full Documentation](./docs/)

---

## 🔧 Project Maintenance

Community-maintained port enabling AI-powered coding on Android Termux. Activities include ARM64 compilation, upstream synchronization, Termux compatibility patches, and documentation.

**Thank you** to all users who have reported issues, provided feedback, and helped improve this project.

---

## 📝 License

This project maintains full compliance with Apache 2.0 license from OpenAI Codex.

**Original work**: Copyright OpenAI (https://github.com/openai/codex)
**Termux port**: Minimal patches for Android compatibility

See [LICENSE](./LICENSE) file for details.

---

## 🙏 Credits

- **OpenAI** for the amazing Codex CLI
- **Termux** community for Android terminal environment
- All contributors to upstream Codex project

---

## License

Original project by OpenAI: https://github.com/openai/codex<br>
Apache License 2.0 (upstream OpenAI Codex)<br>
Termux-port maintenance by WellaNet.Dev<br>
Made in Italy 🇮🇹
