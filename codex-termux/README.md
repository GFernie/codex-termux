# @mmmbuto/codex-cli-termux

> **Latest release line for Android Termux (ARM64), tracking upstream OpenAI closely**

[![npm](https://img.shields.io/npm/v/@mmmbuto/codex-cli-termux?style=flat-square&logo=npm)](https://www.npmjs.org/package/@mmmbuto/codex-cli-termux)
[![downloads](https://img.shields.io/npm/dt/@mmmbuto/codex-cli-termux?style=flat-square)](https://www.npmjs.org/package/@mmmbuto/codex-cli-termux)
[![ko-fi](https://img.shields.io/badge/☕_Support-Ko--fi-FF5E5B?style=flat-square&logo=ko-fi)](https://ko-fi.com/dionanos)

---

## About

Latest release line tracking upstream OpenAI Codex closely, compiled specifically for Android Termux (ARM64).

### Supported Platforms

- Android Termux (ARM64) only

---

## Project Scope

### Latest (Termux-only)
- Compiles for ARM64 native on Android Termux
- Tracks upstream OpenAI closely
- Applies minimal patches for Termux-specific compatibility

### What We Do
✅ **Use official OpenAI Codex source** (https://github.com/openai/codex)
✅ **Compile for ARM64** (Android Termux native)
✅ **Apply minimal patches** only for Termux-specific issues not addressed upstream
✅ **Package as npm** for easy installation
✅ **Maintain full Apache 2.0 compliance** with OpenAI attribution

### What We DON'T Do
❌ **NO new features**
❌ **NO behavior modifications** (works exactly like upstream)
❌ **NO replacement** of official Codex

---

## Patches & Updates

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

# Install Latest
npm install -g @mmmbuto/codex-cli-termux

# Verify
codex --version
codex login
```

**Requirements:** Android 7+, ARM64, Node.js >=18 (recommended v22+), ~50MB storage

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
