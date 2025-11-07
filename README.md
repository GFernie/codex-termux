# 🚀 Codex CLI - Termux Edition

**Pre-compiled OpenAI Codex for Android Termux (ARM64)**

OpenAI Codex compiled natively for Android ARM64 (Termux), with full compatibility patches.

---

## 📦 Latest Release

**Version**: `0.56.0-termux`  
**Built**: 2025-11-07 19:56 UTC  
**Device**: Android ARM64 (Pixel 9 Pro - 16GB RAM)  
**Upstream**: OpenAI Codex rust-v0.56.0 (42 commits merged)

---

## 📥 Installation

### Via npm (Recommended)
```bash
npm install -g @mmmbuto/codex-cli-termux@latest
```

### Specific Version
```bash
npm install -g @mmmbuto/codex-cli-termux@0.56.0-termux
```

---

## ✨ Features

- ✅ **Native ARM64** compilation optimized for Termux
- ✅ **All 8 critical patches** applied and tested
- ✅ **Browser login** working (termux-open-url)
- ✅ **Agent mode bash execution** fixed (shell detection, LD_*, sandbox)
- ✅ **RAM optimized** for 8GB/16GB devices
- ✅ **Auto-update** system configured for Termux fork

---

## 🔧 Patches Applied

This build includes **8 critical patches** for Termux compatibility:

1. **Browser Login Fix** - Uses `termux-open-url` instead of ndk-context
2. **RAM Optimizations** - `lto=false`, `codegen-units=16` for low-RAM devices
3. **Version Alignment** - Binary version matches upstream (0.56.0)
4. **Auto-Update URL** - Points to DioNanos/codex-termux releases
5. **Version Parser** - Handles `-termux` suffix correctly
6. **NPM Package Name** - Uses @mmmbuto/codex-cli-termux
7. **Manual Update Instructions** - Android-specific update flow
8. **Bash Execution Fix** ⭐ - Shell detection, LD_* preservation, sandbox disabled

**All patches documented**: `patches/README.md`

---

## 📊 Build Info

- **Compilation Time**: 27m 38s (16GB RAM config)
- **Binary Size**: 46.8 MB
- **Package Size**: 17.9 MB (compressed)
- **Rust Version**: 1.90.0
- **Cargo Config**: lto=false, codegen-units=16, opt-level=3

---

## 🧪 Testing

All critical features tested before release:

```bash
# Version check
codex --version
# ✅ codex-cli 0.56.0

# Browser login (patch #1)
codex login
# ✅ Browser opens without crash

# Agent mode bash (patch #8)
codex
> run uname -a
# ✅ Command executes without permission errors

# Basic functionality
codex 'echo hello from codex'
# ✅ Works correctly
```

---

## 🔄 Changelog

### v0.56.0-termux (2025-11-07)

**Upstream Merge:**
- Merged **42 commits** from OpenAI/codex rust-v0.56.0
- Updated from v0.55.4 to v0.56.0

**Patches:**
- ✅ All 8 patches preserved and verified after merge
- ✅ Two-stage pipeline implemented (anti-deploy without test)

**Testing:**
- ✅ Build and install tested locally
- ✅ All critical patches verified working
- ✅ Browser login: PASS
- ✅ Agent mode bash: PASS

**Previous versions**: See [CHANGELOG.md](CHANGELOG.md)

---

## 🏗️ Building from Source

```bash
# Clone
git clone https://github.com/DioNanos/codex-termux.git
cd codex-termux

# Compile (optimized for 16GB RAM)
cd codex-rs
cargo build --release -j 2

# Binary location
./target/release/codex
```

**For 8GB devices**, see: `docs/COMPILATION_LOW_RAM.md`

---

## 🐛 Troubleshooting

### Installation Issues
```bash
# Clean install
npm uninstall -g @mmmbuto/codex-cli-termux
npm cache clean --force
npm install -g @mmmbuto/codex-cli-termux@latest
```

### Version Mismatch
```bash
# Check installed version
codex --version

# Check npm version
npm list -g @mmmbuto/codex-cli-termux
```

### Permission Errors
See `patches/README.md` - Patch #8 (Bash Execution Fix)

---

## 📚 Documentation

- **Patches**: `patches/README.md` (all 8 patches explained)
- **Compilation**: `docs/COMPILATION_LOW_RAM.md`
- **Pipeline**: `docs/PIPELINE.md`

---

## 🔗 Links

- **npm Package**: https://www.npmjs.com/package/@mmmbuto/codex-cli-termux
- **GitHub Repo**: https://github.com/DioNanos/codex-termux
- **Upstream**: https://github.com/openai/codex

---

## 👤 Maintainer

**Davide A. Guglielmi**  
Built on: Pixel 9 Pro (Android 15 + Termux)  
Compiler: Rust 1.90.0 (ARM64 native)

---

## ⚖️ License

Same as upstream OpenAI Codex.

---

**Last Updated**: 2025-11-07  
**Build System**: Two-stage pipeline with anti-deploy protection
