# 🚀 Codex CLI - Termux Edition

> **Pre-compiled OpenAI Codex for Android Termux (ARM64)**

## What This Is

Official OpenAI Codex CLI compiled for Android Termux. Since Termux is not officially supported by upstream, we apply minimal patches only for critical compatibility issues.

### What We Do:
✅ **Use official OpenAI Codex source** (https://github.com/openai/codex)
✅ **Compile for ARM64** (Android Termux native)
✅ **Apply minimal patches** only for Termux-specific issues not addressed upstream
✅ **Package as npm** for easy installation
✅ **Maintain full Apache 2.0 compliance** with OpenAI attribution

### What We DON'T Do:
❌ **NO new features**
❌ **NO behavior modifications** (works exactly like upstream)
❌ **NO replacement** of official Codex

### 🔧 Compatibility Patches

We only apply patches for issues that:
- **Prevent Codex from working on Termux**
- **Are not addressed by upstream** (Termux is not officially supported)
- **Are minimal and well-documented**

**Current patches**: See [patches/](./patches/) directory for full documentation.

**Found an issue?** Well-documented bug reports with reproduction steps are welcome! Open an [issue](https://github.com/DioNanos/codex-termux/issues).

---

## 📋 Prerequisites

```bash
# Update Termux packages
pkg update && pkg upgrade -y

# Install Node.js
pkg install nodejs-lts -y

# Verify
node --version  # v14+
npm --version   # v6+
```

**Requirements:**
- Android 7+ (Termux)
- ARM64 architecture
- Node.js ≥ 14.0.0
- ~50MB storage

---

## 📦 Installation

### Via npm (Recommended)

```bash
npm install -g @mmmbuto/codex-cli-termux
```

### Via GitHub Release

```bash
wget https://github.com/DioNanos/codex-termux/releases/download/v0.50.0-termux/codex-arm64-termux
chmod +x codex-arm64-termux
mv codex-arm64-termux ~/.local/bin/codex
codex --version
```

**Links:**
- npm: https://www.npmjs.com/package/@mmmbuto/codex-cli-termux
- Releases: https://github.com/DioNanos/codex-termux/releases
- Upstream: https://github.com/openai/codex

---

## 🎯 Quick Start

### 1. Login

```bash
# Browser login (opens default browser)
codex login

# OR via API key
echo "sk-your-key" | codex login --with-api-key
```

### 2. Use Codex

```bash
# Interactive mode
codex "write a fibonacci function"

# Execute directly
codex exec "create a simple web server"

# Help
codex --help
```

---

## 📖 Documentation

| Resource | Description |
|----------|-------------|
| [BUILDING.md](./BUILDING.md) | Compile from source guide |
| [patches/](./patches/) | Applied patches documentation |
| [Upstream Docs](https://github.com/openai/codex) | Official Codex documentation |

---

## 🆘 Support

**Codex bugs**: Report to [upstream](https://github.com/openai/codex/issues)
**Termux-specific issues**: Report [here](https://github.com/DioNanos/codex-termux/issues)

When reporting issues, please include:
- Termux version
- Android version
- Device model
- Full error output
- Steps to reproduce

---

## ⚖️ Legal

- **License**: Apache 2.0 (same as upstream)
- **Original Copyright**: © 2025 OpenAI
- **Distribution**: © 2025 DioNanos
- **Compliance**: Full attribution maintained

See [LEGAL-COMPLIANCE.md](./LEGAL-COMPLIANCE.md) for details.

---

## 🔗 Upstream

Everything comes from OpenAI Codex:
```
openai/codex (upstream)
    ↓
[Compile for ARM64 + Termux patches]
    ↓
DioNanos/codex-termux (this repo)
    ↓
@mmmbuto/codex-cli-termux (npm)
```

We track upstream regularly and merge updates.

---

**Version**: 0.50.0-termux
**Last Updated**: October 26, 2025
**Platform**: Android ARM64 (Termux)
