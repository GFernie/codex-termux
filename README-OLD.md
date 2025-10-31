# 🚀 Codex CLI for Termux

> **⚠️ IMPORTANT: This is ONLY a pre-compiled distribution. We make NO modifications to the source code.**

## What This Is

This repository contains **pre-compiled OpenAI Codex CLI binaries for Android Termux & Linux ARM64**.

### What We Do:
✅ **Take official OpenAI Codex source code** (https://github.com/openai/codex)
✅ **Compile it for ARM64 architecture** (Termux/Linux)
✅ **Apply minimal Termux compatibility patch** (browser login fix only)
✅ **Package it as npm module** for easy installation
✅ **Keep OpenAI's copyright and license** (Apache 2.0)

### What We DON'T Do:
❌ **NO feature additions or removals**
❌ **NO functionality changes**
❌ **NO behavior alterations**
❌ **NO forking or replacing** upstream

### ⚠️ Applied Patches:
**1. Termux Browser Login Fix** (`login/src/server.rs`)
- **Problem**: Upstream uses `webbrowser` crate which requires Android Activity context
- **Fix**: Use `termux-open-url` command on Android instead of `webbrowser::open()`
- **Lines Changed**: 9 lines (1 file)
- **Impact**: Fixes crash on `codex login`, no other changes
- **Commit**: [View patch](https://github.com/DioNanos/codex-termux/commit/754b506d)

---

## 📋 Prerequisites (Termux)

```bash
# Update package lists
pkg update && pkg upgrade -y

# Install Node.js (choose one)
pkg install nodejs-lts -y    # LTS version (v22.x - recommended)
# OR
pkg install nodejs -y        # Latest version (v24.x)

# Verify installation
node --version  # Should be v14.0.0+
npm --version   # Should be v6.0.0+
```

**Requirements:**
- Android 7.0+ (Termux)
- ARM64 architecture
- Node.js ≥ 14.0.0
- npm ≥ 6.0.0
- ~50MB free storage

**Available Node.js versions in Termux:**
- `nodejs-lts` → v22.x (Long Term Support - **recommended**)
- `nodejs` → v24.x (Latest/Current)

**Want to compile from source?** See [BUILDING.md](./BUILDING.md)

---

## 📦 Installation

### Method 1: npm (Recommended)

```bash
npm install -g @mmmbuto/codex-cli-termux
```

### Method 2: Manual Download

Download the latest release from [GitHub Releases](https://github.com/DioNanos/codex-termux/releases):

```bash
# Download and extract
wget https://github.com/DioNanos/codex-termux/releases/download/v0.50.0-termux/codex-arm64-termux
chmod +x codex-arm64-termux
mv codex-arm64-termux ~/.local/bin/codex

# Verify installation
codex --version
```

**Links:**
- 📦 **npm package**: https://www.npmjs.com/package/@mmmbuto/codex-cli-termux
- 📥 **GitHub Releases**: https://github.com/DioNanos/codex-termux/releases
- 📋 **Upstream source**: https://github.com/openai/codex

---

## 🎯 Quick Start

### Authentication (Required)

⚠️ **IMPORTANT for Termux**: Browser-based login doesn't work on Android. Use API key instead:

```bash
# Get your API key from: https://platform.openai.com/api-keys

# Login with API key (recommended for Termux)
echo "your-openai-api-key" | codex login --with-api-key

# Or set environment variable
export OPENAI_API_KEY="your-api-key"

# Verify login
codex login status
```

### Usage

```bash
# Interactive mode
codex "write a fibonacci function"

# Execute directly
codex exec "create a simple web server"

# Get help
codex --help
```

For full usage guide, see **[npm documentation](https://www.npmjs.com/package/@mmmbuto/codex-cli-termux)**.

---

## 📖 Documentation

| Document | Purpose |
|----------|---------|
| **[docs/PIPELINE.md](./docs/PIPELINE.md)** | 🤖 Automated build & release pipeline |
| **[LEGAL-COMPLIANCE.md](./LEGAL-COMPLIANCE.md)** | ⚖️ Legal status & compliance |
| **[npm Package](https://www.npmjs.com/package/@mmmbuto/codex-cli-termux)** | 📖 Complete usage guide |
| **[Original Codex](https://github.com/openai/codex)** | 🔗 Upstream source & development |
| **[OpenAI Codex Docs](https://developers.openai.com/codex/cli)** | 📚 Official documentation |

---

## 🔗 Upstream & Source

**Everything comes from OpenAI. We only recompile it.**

- **Original Source**: https://github.com/openai/codex
- **Official Documentation**: https://developers.openai.com/codex/cli
- **Our Changes**: ZERO (just compilation for ARM64)

```
openai/codex (source)
    ↓
[Compile for ARM64]
    ↓
DioNanos/codex-termux (this repo)
    ↓
@mmmbuto/codex-cli-termux (npm)
```

---

## ⚖️ Legal & License

### License
- **Type**: Apache 2.0
- **Original**: OpenAI
- **Distribution**: Fully compliant

### Copyright
- **Original**: © 2025 OpenAI
- **Distribution**: © 2025 DioNanos
- **Status**: ✅ Full attribution preserved

### What's Legal?
✅ Re-distributing pre-compiled binaries (Apache 2.0 allows this)
✅ Creating npm package wrapper
✅ Adding for platforms OpenAI doesn't officially support
✅ Commercial use (with license compliance)

📄 **[See full legal analysis](./LEGAL-COMPLIANCE.md)**

---

## 🆘 Support & Issues

### For Codex CLI bugs:
Report to **[OpenAI upstream](https://github.com/openai/codex/issues)**
(We don't modify code, so upstream is the authority)

### For Termux-specific issues:
**[Open issue here](https://github.com/DioNanos/codex-termux/issues)**

### For npm installation issues:
**[npm Package](https://www.npmjs.com/package/@mmmbuto/codex-cli-termux)**

---

## 🔐 Security

- **No modifications**: Binary is identical to upstream
- **Source tracking**: Can always compare with openai/codex
- **License compliance**: Full Apache 2.0 + attribution
- **No hidden changes**: All code is open-source

---

## 📊 Repository Structure

```
codex-termux/                    ← This repository
├── README.md                    ← You are here
├── LEGAL-COMPLIANCE.md          ← Legal documentation
├── codex-rs/                    ← Upstream source (synced)
├── LICENSE                      ← Apache 2.0 + dual copyright
└── [other upstream files]
```

**Note**: This repo is a **clone of upstream** with:
- ✅ Added legal compliance documentation
- ✅ Added .gitignore for builds
- ✅ Configured for Termux distribution
- ❌ NO source code changes

---

## 👤 Author & Distribution

**Distribution & Termux Build**: @DioNanos
- GitHub: https://github.com/DioNanos
- npm org: https://www.npmjs.com/org/mmmbuto

**Original Project**: OpenAI (https://github.com/openai/codex)

---

## 🎯 Why This Exists

OpenAI Codex CLI doesn't officially support:
- ✗ Android Termux
- ✗ Linux ARM64 (pre-compiled)

This distribution:
- ✅ Makes Codex available on Termux
- ✅ No compilation needed (pre-built)
- ✅ Simple npm install
- ✅ Fully legal & compliant

---

## ✨ Installation Methods

### Method 1: npm (Recommended)
```bash
npm install -g @mmmbuto/codex-cli-termux
codex --version
```

### Method 2: GitHub Release
Download from: https://github.com/DioNanos/codex-termux/releases/tag/v0.50.0-termux

### Method 3: From Source
```bash
git clone https://github.com/DioNanos/codex-termux.git
cd codex-termux/codex-rs
cargo build --release
```

---

## 📋 Key Facts

| Fact | Answer |
|------|--------|
| Do you modify Codex? | ❌ NO - we only compile it |
| Is this official? | ⚠️ NO - it's a distribution |
| Is it legal? | ✅ YES - Apache 2.0 allows it |
| Do you own it? | ❌ NO - OpenAI owns Codex |
| Can I use it? | ✅ YES - Apache 2.0 licensing |
| Must I credit OpenAI? | ✅ YES - we do, automatically |

---

## 🚀 Get Started

```bash
# Install
npm install -g @mmmbuto/codex-cli-termux

# Login
codex login

# Use it
codex "write hello world in python"
```

Full docs: https://www.npmjs.com/package/@mmmbuto/codex-cli-termux

---

## 📞 Quick Links

| Purpose | Link |
|---------|------|
| Install | `npm install -g @mmmbuto/codex-cli-termux` |
| Documentation | https://www.npmjs.com/package/@mmmbuto/codex-cli-termux |
| Legal | [LEGAL-COMPLIANCE.md](./LEGAL-COMPLIANCE.md) |
| Original | https://github.com/openai/codex |
| Issues | https://github.com/DioNanos/codex-termux/issues |
| Release | https://github.com/DioNanos/codex-termux/releases |

---

**Status**: ✅ Production Ready
**Version**: 0.50.0-termux
**License**: Apache 2.0
**Last Updated**: October 26, 2025
