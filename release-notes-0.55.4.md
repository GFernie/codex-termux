# Release v0.55.4-termux

## 🔧 Critical Fix: Bash Execution in Agent Mode

This release fixes a **critical bug** that has existed since v0.53.0, making Agent mode completely unusable on Termux/Android.

### What's Fixed

**Problem:** When using Codex in TUI/Agent mode (interactive chat), all bash command executions failed with:
```
/data/data/com.termux/files/usr/bin/bash: Permission denied
```

**Root Causes:**
1. **Shell detection fails on Termux** - `getpwuid()` returns `/data/data/.../login` instead of actual shell
2. **LD_* environment variables removed** - Process-hardening removes `LD_LIBRARY_PATH` needed by Termux
3. **Sandbox not supported on Android** - Codex tries to use landlock/seccomp which don't exist on Android

### Patch #8 Changes

Three independent fixes, all using `#[cfg(target_os = "android")]`:

#### 1. Disable Sandbox on Android
**File:** `codex-rs/core/src/safety.rs` (3 lines)

Android/Termux does not support landlock/seccomp sandbox. Modified `get_platform_sandbox()` to return `None` on Android.

#### 2. Preserve LD_* Environment Variables
**File:** `codex-rs/process-hardening/src/lib.rs` (29 lines)

Android needs `LD_LIBRARY_PATH` to find libraries in `/data/data/com.termux/files/usr/lib`. Skip LD_* removal in process-hardening on Android only.

#### 3. Use $SHELL Instead of getpwuid()
**File:** `codex-rs/core/src/shell.rs` (56 lines)

On Android/Termux, `getpwuid()` returns "login" instead of actual shell. Use `$SHELL` environment variable which correctly points to bash/zsh.

### Testing

**Before Patch #8:**
```bash
$ codex
> Ask Codex: "run pkg help"
Error: /data/data/com.termux/files/usr/bin/bash: Permission denied ❌
```

**After Patch #8:**
```bash
$ codex --version
codex-cli 0.55.4 ✅

$ codex
> Ask Codex: "run pkg help"
Usage: pkg command [arguments] ✅
```

### Impact

- ✅ **Enables Agent mode bash execution** - Critical functionality restored
- ✅ **Shell detection fixed** - Correctly identifies bash/zsh on Termux
- ✅ **Library loading fixed** - LD_* preserved for dynamic linking
- ✅ **Sandbox disabled on Android** - Prevents sandbox-related crashes
- ✅ **No changes to other platforms** - Linux/Mac/Windows unchanged
- ✅ **Minimal invasive** - 88 lines modified across 3 files

### Installation

```bash
npm install -g @mmmbuto/codex-cli-termux@0.55.4-termux
```

Or download pre-compiled binary:
```bash
curl -LO https://github.com/DioNanos/codex-termux/releases/download/v0.55.4-termux/codex-0.55.4-termux-arm64.tar.gz
tar xzf codex-0.55.4-termux-arm64.tar.gz
mv codex ~/.local/bin/
```

### Full Changelog

**v0.55.4-termux** (2025-11-06)
- feat: Patch #8 - Fix bash execution on Android/Termux (3 files, 88 lines)
- feat: Disable sandbox on Android (core/src/safety.rs)
- feat: Preserve LD_* environment variables on Android (process-hardening/src/lib.rs)
- feat: Use $SHELL instead of getpwuid() on Android (core/src/shell.rs)
- docs: Add Patch #8 documentation
- chore: Bump version to 0.55.4

**Previous releases:**
- v0.55.3-termux: Auto-update fix (manual instructions on Android)
- v0.55.2-termux: Version alignment fix (npm package name)
- v0.55.1-termux: Version parser fix (-termux suffix handling)
- v0.55.0-termux: Initial release (browser login, RAM optimizations)

### Upstream

Based on **OpenAI Codex 0.55.0** (46 commits ahead of 0.53.0)

### Platform

- **OS:** Android 12+ (Termux)
- **Arch:** ARM64 (aarch64)
- **Binary size:** 44MB (optimized for 16GB RAM)
- **Compilation time:** ~15 minutes

---

**Note:** This patch fixes a critical bug that prevented Agent mode from working on Termux since at least v0.53.0. All previous versions (0.53.x - 0.55.3) were affected.
