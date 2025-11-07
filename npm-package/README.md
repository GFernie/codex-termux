# @mmmbuto/codex-cli-termux

> OpenAI Codex CLI v0.55.0 pre-compiled for Android Termux (ARM64)

## 🔧 v0.55.1 - Auto-Update Detection Fix

**Critical fix**: Version parser now correctly detects updates with `-termux` suffix

### What was broken
Users on 0.53.x versions never saw update notifications for 0.55.0-termux

### What's fixed
Auto-update detection now works correctly across all version ranges

## Installation

```bash
npm install -g @mmmbuto/codex-cli-termux
```

## Usage

```bash
codex login
codex
```

## Requirements

- Android 7+ (Termux)
- ARM64 architecture
- Node.js ≥ 14.0.0

## About

This is a pre-compiled build of the official OpenAI Codex CLI with minimal patches for Termux compatibility.

### Total Patches Applied: 5
1. Browser login fix (termux-open-url)
2. RAM optimizations (compilation settings)
3. Version alignment
4. Auto-update URL redirect
5. **Version parser fix** (NEW in 0.55.1)

## Links

- **GitHub**: https://github.com/DioNanos/codex-termux
- **Upstream**: https://github.com/openai/codex
- **Patches**: https://github.com/DioNanos/codex-termux/blob/main/patches/README.md

## License

Apache-2.0 (same as upstream OpenAI Codex)
