# Codex v0.55.6-termux Release Notes

**Release Date**: November 7, 2025
**Binary Version**: 0.55.6
**npm Package**: @mmmbuto/codex-cli-termux@0.55.6-termux

## 🆕 What's New

### Upstream Sync (42 commits)

Major sync with OpenAI Codex upstream including:

**Critical Fixes**:
- Fix apply_patch rename/move path resolution (#5486)
- Don't retry "insufficient_quota" errors (#6340)
- Fix backtracking past /status in TUI (#6335)

**New Features**:
- Enable CTRL-n/CTRL-p for navigating slash commands, files, history (#1994)
- Support models with single reasoning effort (#6300)
- Clarify GPT-5 Codex should not amend commits unless requested (#6333)
- Remove shell tool when unified exec is enabled (#6345)

**Improvements**:
- Widen sandbox to allow certificate ops when network enabled (#5980)
- Improved TUI navigation and chat composer
- Better error handling and rate limiting
- New app-server v2 APIs
- Freeform unified exec output formatting (#6233)

### Termux-Specific (Patch #8 maintained)

All Android/Termux fixes from Patch #8 are maintained:
- ✅ Disabled unsupported sandbox on Android
- ✅ Preserved LD_* environment variables required by Termux
- ✅ Fixed shell detection using $SHELL on Android

## 📦 Installation

```bash
npm install -g @mmmbuto/codex-cli-termux@0.55.6-termux
```

## 🔧 Compilation Details

- **Platform**: Pixel 9 Pro (Android 15 + Termux)
- **Architecture**: ARM64 (aarch64)
- **Rust Version**: Latest stable
- **Binary Size**: ~44-45 MB

## ✅ Testing

- ✅ Binary version check: `codex --version` → `codex-cli 0.55.6`
- ✅ Agent mode bash execution (Patch #8)
- ✅ TUI navigation (CTRL-n/CTRL-p)
- ✅ Login flow with termux-open-url

## 📋 Full Changelog

See CHANGELOG.md for complete commit history.
