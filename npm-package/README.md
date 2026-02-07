# 🤖 Codex CLI - Termux Edition

> **Built from upstream OpenAI Codex source, compiled for Android Termux (ARM64)**

[![npm](https://img.shields.io/npm/v/@mmmbuto/codex-cli-termux?style=flat-square&logo=npm)](https://www.npmjs.org/package/@mmmbuto/codex-cli-termux)
[![downloads](https://img.shields.io/npm/dt/@mmmbuto/codex-cli-termux?style=flat-square)](https://www.npmjs.org/package/@mmmbuto/codex-cli-termux)
[![ko-fi](https://img.shields.io/badge/☕_Support-Ko--fi-FF5E5B?style=flat-square&logo=ko-fi)](https://ko-fi.com/dionanos)

---

## What This Is

Built from upstream OpenAI Codex source, compiled for Android Termux. Since Termux is not officially supported by upstream, we apply minimal patches only for critical compatibility issues.

### Termux Edition (0.98.0)

This repo maintains **two release lines**:

- **Latest (main)**: Termux-only, tracks upstream more closely (current: **v0.98.0-termux** based on `rust-v0.98.0`).
- **LTS (lts)**: Long-term support based on upstream `rust-v0.80.0`, stable for compatibility.
  The LTS line supports **both /chat and /responses** wire APIs, and receives **security and stability backports only**.

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

**Current patches**: See [patches/](../patches/) directory for full documentation.

Need help debugging upgrade alerts? See
[docs/termux-upgrade-checks.md](../docs/termux-upgrade-checks.md) for known causes
and fix strategies.

**Found an issue?** Well-documented bug reports with reproduction steps are welcome! Open an [issue](https://github.com/DioNanos/codex-termux/issues).

---

## 📋 Prerequisites

```bash
# Update Termux packages
pkg update && pkg upgrade -y

# Install Node.js
pkg install nodejs-lts -y

# Verify
node --version  # v18+ (recommended v22+)
npm --version   # v9+
```

**Requirements:**
- Android 7+ (Termux)
- ARM64 architecture
- Node.js >=18 (recommended >=22)
- ~50MB storage

---

## 📦 Installation

### Via npm (Recommended)

```bash
npm install -g @mmmbuto/codex-cli-termux
```

### Version policy (LTS vs Latest)

This repo maintains **two release lines**:

- **LTS:** `@mmmbuto/codex-cli-lts` - based on upstream `rust-v0.80.0`, stable for long-term compatibility
- **Latest:** `@mmmbuto/codex-cli-termux` - tracks upstream more closely (current: **v0.98.0-termux** based on `rust-v0.98.0`)

Install a specific version when needed:

```bash
# LTS (stable, long-term support)
npm install -g @mmmbuto/codex-cli-lts@latest

# Latest (tracks upstream)
npm install -g @mmmbuto/codex-cli-termux@latest
```

Tip: check what npm considers "latest":

```bash
npm view @mmmbuto/codex-cli-termux dist-tags
npm view @mmmbuto/codex-cli-termux versions --json
```

### Verify Installation

```bash
codex --version
# Output: codex-cli 0.98.0

codex login
# Opens browser for authentication
```

**Links:**
- npm: https://www.npmjs.org/package/@mmmbuto/codex-cli-termux
- Releases: https://github.com/DioNanos/codex-termux/releases
- Upstream: https://github.com/openai/codex
- Web UI: [NexusCLI](https://github.com/DioNanos/nexuscli) - Optional web interface for Codex/Claude/Gemini

---

## 🚀 Usage

Same as official Codex CLI:

```bash
# Login to OpenAI
codex login

# Start chat
codex

# Help
codex --help
```

For full documentation, see [OpenAI Codex docs](https://developers.openai.com/codex).

### Non-Interactive Mode (Automation)

```bash
# Execute a prompt non-interactively
codex "Generate a hello world in Python"

# JSON output for tooling integration
codex --json "List files in current directory"
```

## 🧪 Testing & Validation

**v0.98.0-termux** (pending): run tests after build — report will be added as `CODEX_TEST_REPORT_v0.98.0.md`

**LTS validation** (2026-02-02): All categories PASS — see [CODEX_TEST_REPORT_v0.80.3-lts_termux.md](../CODEX_TEST_REPORT_v0.80.3-lts_termux.md)

**LTS Linux** (2026-01-31): 62 tests, 60 passed / 0 failed / 11 skipped — see [CODEX_TEST_REPORT_v0.80.3-lts_linux.md](../CODEX_TEST_REPORT_v0.80.3-lts_linux.md)

---

## License

See [LICENSE](../LICENSE) file for details.

---

**Version**: Based on OpenAI Codex rust-v0.98.0 with Termux compatibility patches
**Platform**: Android Termux ARM64
**Maintained**: Community-driven, not affiliated with OpenAI
