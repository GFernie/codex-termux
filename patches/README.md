# 🔧 Termux Compatibility Patches

This document describes all patches applied to make OpenAI Codex work on Android Termux.

---

## Patch List

### 1. Browser Login Fix

**File**: `codex-rs/login/src/server.rs`
**Lines Modified**: 9
**Date Applied**: 2025-10-26
**Upstream Issue**: Browser login crashes on Termux with `ndk-context` error

#### Problem
Upstream uses `webbrowser` crate which calls `ndk-context` on Android. 
This requires an Android Activity context, which is not available in Termux (CLI environment). 

**Error:**
```
thread 'main' panicked at ndk-context-0.1.1/src/lib.rs:72:30:
android context was not initialized
```

#### Solution
On Android (`target_os = "android"`), use `termux-open-url` command directly 
instead of `webbrowser::open()`. Other platforms continue using `webbrowser` crate.

**Code:**
```rust
if opts.open_browser {
    // On Termux/Android, use termux-open-url directly to avoid ndk-context crash
    #[cfg(target_os = "android")]
    {
        let _ = std::process::Command::new("termux-open-url").arg(&auth_url).status();
    }
    #[cfg(not(target_os = "android"))]
    {
        let _ = webbrowser::open(&auth_url);
    }
}
```

**Testing:**
```bash
codex login
# Browser opens without crash ✅
```

**Impact:** 
- ✅ Enables OAuth login flow on Termux
- ✅ No changes to other platforms
- ✅ Minimal invasive patch (9 lines)

---

### 2. Compilation Optimizations

**File**: `codex-rs/Cargo.toml`
**Section**: `[profile.release]`
**Date Applied**: 2025-10-26
**Purpose**: Enable compilation on RAM-constrained devices

#### Problem
Default upstream compilation settings use aggressive optimizations:
- `lto = "fat"` - Consumes 18-22GB RAM during linking
- `codegen-units = 1` - Single-threaded, large temporary files

These settings cause Out Of Memory (OOM) errors on devices with < 32GB RAM.

#### Solution
Optimize for Termux devices (8-16GB RAM):

**Changes:**
```toml
[profile.release]
lto = false                    # Was: "fat" - saves ~4GB RAM
codegen-units = 16             # Was: 1 - enables parallel compilation
opt-level = 3                  # Keep optimization level high
```

**Impact:**
- Binary size: +5% larger (~40MB instead of 38MB)
- Compilation time: +5-10 minutes
- RAM usage: 12-14GB instead of 18-22GB
- **Result: Successful compilation on 16GB devices** ✅

---

### 3. Version Alignment

**File**: `codex-rs/Cargo.toml`
**Field**: `[workspace.package] version`
**Purpose**: Display correct version in `codex --version`

#### Change
```toml
version = "0.53.0"  # Was: "0.0.0"
```

**Reason:**
- Upstream uses `0.0.0` as placeholder
- We align with upstream release version for clarity
- npm package uses `<upstream>-termux` format (e.g. `0.53.0-termux`)

---

## Versioning Strategy

| Component | Version | Example |
|-----------|---------|---------|
| **Binary** | Upstream version | `codex-cli 0.53.0` |
| **npm package** | `<upstream>.<patch>-termux` | `0.53.0-termux` or `0.53.1-termux` |

**Why:**
- Binary version matches upstream for compatibility
- npm patch number indicates Termux-specific releases
- Clear traceability: `0.53.1-termux` = "based on upstream 0.53.0, Termux patch #1"

---

### 4. Auto-Update URL Redirect

**File**: `codex-rs/tui/src/updates.rs`
**Lines Modified**: 8
**Date Applied**: 2025-10-31
**Upstream Issue**: Auto-update checks OpenAI repo, not Termux fork

#### Problem
Upstream checks OpenAI releases for updates:
```rust
const LATEST_RELEASE_URL: &str = "https://api.github.com/repos/openai/codex/releases/latest";
```

Tag format mismatch:
- Upstream uses: `rust-v0.53.0`
- Termux fork uses: `v0.53.0-termux`

#### Solution
1. **Change URL** to point to Termux fork
2. **Update tag parser** to handle `v`-prefixed tags

**Changes:**
```rust
// Line 56: Update URL
const LATEST_RELEASE_URL: &str = "https://api.github.com/repos/DioNanos/codex-termux/releases/latest";

// Lines 81-85: Update tag parser for Termux format
let version = latest_tag_name
    .strip_prefix("v")
    .unwrap_or(&latest_tag_name)
    .to_string();
```

**Impact:**
- ✅ Auto-update now checks Termux fork releases
- ✅ Supports both `vX.Y.Z-termux` and fallback formats
- ✅ No breaking changes to existing functionality

### 5. Node.js Wrapper for Auto-Update

**File**: `npm-package/bin/codex.js` (new file)
**Lines Added**: 62
**Date Applied**: 2025-10-31
**Upstream Issue**: Binary called directly, can't execute npm auto-update

#### Problem
npm package pointed directly to Rust binary:
```json
"bin": { "codex": "bin/codex" }
```

Without `CODEX_MANAGED_BY_NPM` environment variable set, the binary can't:
- Detect it was installed via npm
- Execute `npm install -g @mmmbuto/codex-cli-termux@latest` automatically
- Show proper update instructions

#### Solution
Create Node.js wrapper script that:
1. Sets `CODEX_MANAGED_BY_NPM=1` environment variable
2. Spawns the Rust binary with all arguments
3. Forwards signals (SIGINT, SIGTERM, SIGHUP)
4. Mirrors child exit code/signal

**Changes:**
```javascript
// bin/codex.js (new file)
const env = {
  ...process.env,
  CODEX_MANAGED_BY_NPM: "1",
};

const child = spawn(binaryPath, process.argv.slice(2), {
  stdio: "inherit",
  env,
});
```

**package.json update:**
```json
{
  "type": "module",
  "bin": { "codex": "bin/codex.js" },
  "files": ["bin/codex.js", "bin/codex"]
}
```

**Impact:**
- ✅ Users get prompted to update with exact npm command
- ✅ Can auto-execute update with confirmation
- ✅ Full parity with upstream OpenAI package
- ✅ No performance overhead (direct spawn)

---

## Testing Checklist

Before each release:
- [ ] `codex --version` shows correct upstream version
- [ ] `codex login` opens browser without crash
- [ ] OAuth flow completes successfully
- [ ] Binary size < 50MB
- [ ] Compilation completes on 16GB device

---

## Contributing

Found a bug specific to Termux? Please open an issue with:
1. Steps to reproduce
2. Expected vs actual behavior
3. Device specs (RAM, Android version)
4. Error logs

We only accept patches for Termux-specific issues, not general feature requests.

---

**Last Updated**: 2025-10-27
**Based on**: OpenAI Codex 0.53.0
**Platform**: Android Termux ARM64
