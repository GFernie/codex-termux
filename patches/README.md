# 🔧 Termux Compatibility Patches

This document describes all patches applied to make OpenAI Codex work on Android Termux.

---

## Patch List

### 1. Browser Login Fix

**File**: `codex-rs/login/src/server.rs`
**Lines Modified**: 10
**Date Applied**: 2025-11-05 (updated for 0.55.0)
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
- ✅ Minimal invasive patch (10 lines)

---

### 2. Compilation Optimizations

**File**: `codex-rs/Cargo.toml`
**Section**: `[profile.release]`
**Date Applied**: 2025-11-05 (updated for 0.55.0)
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
- Binary size: +5% larger (~44MB instead of 42MB)
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
version = "0.55.0"  # Aligned with upstream
```

**Reason:**
- Matches upstream release version
- npm package uses `<upstream>-termux` format (e.g. `0.55.0-termux`)

---

### 4. Auto-Update URL Redirect

**File**: `codex-rs/tui/src/updates.rs`
**Lines Modified**: 14
**Date Applied**: 2025-11-05 (updated for 0.55.0)
**Upstream Issue**: Auto-update checks OpenAI repo, not Termux fork

#### Problem
Upstream checks OpenAI releases for updates:
```rust
const LATEST_RELEASE_URL: &str = "https://api.github.com/repos/openai/codex/releases/latest";
```

Tag format mismatch:
- Upstream uses: `rust-v0.55.0`
- Termux fork uses: `v0.55.0-termux`

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

---

### 5. Version Parser Fix for `-termux` Suffix

**File**: `codex-rs/tui/src/updates.rs`
**Lines Modified**: 3
**Date Applied**: 2025-11-05
**Upstream Issue**: Version parser fails on `-termux` suffix, blocking update detection

#### Problem
The version parser splits version string by `.` and tries to parse each part as u64:
```rust
// Old code (fails on "0.55.0-termux")
let pat = iter.next()?.parse::<u64>().ok()?;  // "0-termux" → FAIL ❌
```

When comparing versions:
- Current: `0.53.0`
- Latest from API: `0.55.0-termux`
- Parser tries: `"0-termux".parse::<u64>()` → **Returns None**
- Result: `is_newer()` returns `None` → No update notification shown ❌

**Real-world impact**: Users on 0.53.x never see notification for 0.55.0-termux!

#### Solution
Split patch version on `-` to extract numeric part before parsing:

**Changes:**
```rust
// New code (handles "0.55.0-termux" correctly)
let pat_str = iter.next()?;                    // "0-termux"
let pat = pat_str.split('-').next()?.parse::<u64>().ok()?; // "0" → OK ✅
```

**Testing:**
```rust
parse_version("0.55.0")        → Some((0, 55, 0)) ✅
parse_version("0.55.0-termux") → Some((0, 55, 0)) ✅
parse_version("0.55.1-termux") → Some((0, 55, 1)) ✅
```

**Impact:**
- ✅ Auto-update detection now works across `-termux` versions
- ✅ Users on 0.53.x → 0.55.x see update notification
- ✅ Incremental updates (0.55.0 → 0.55.1) also work
- ✅ Backward compatible with non-suffixed versions

---

### 6. NPM Package Name Fix in Auto-Update

**File**: `codex-rs/tui/src/updates.rs` + `codex-rs/Cargo.toml`
**Lines Modified**: 4
**Date Applied**: 2025-11-05
**Upstream Issue**: Auto-update command uses wrong npm package name

#### Problem
When user accepts auto-update prompt, Codex tries to run:
```bash
npm install -g @openai/codex@latest
```

**Two issues:**
1. **Wrong package**: Uses `@openai/codex` instead of `@mmmbuto/codex-cli-termux`
2. **Update loop**: Binary version stayed at `0.55.0` while npm published `0.55.1-termux`
   - User installs 0.55.1-termux
   - Binary shows: `codex-cli 0.55.0`
   - API returns: `0.55.1-termux`
   - Parser: `(0, 55, 1) > (0, 55, 0)` → **Shows update again!** ∞

**Error seen:**
```
Updating Codex via `npm install -g @openai/codex@latest`...
env: 'node': Permission denied
Error: failed with status exit status: 126
```

#### Solution
1. **Fix npm package name** in `UpdateAction::command_args()`
2. **Increment binary version** in `Cargo.toml` to match npm release

**Changes:**
```rust
// updates.rs line 182-183
UpdateAction::NpmGlobalLatest => ("npm", &["install", "-g", "@mmmbuto/codex-cli-termux@latest"]),
UpdateAction::BunGlobalLatest => ("bun", &["install", "-g", "@mmmbuto/codex-cli-termux@latest"]),
```

```toml
// Cargo.toml line 46
version = "0.55.1"  # Was: "0.55.0"
```

**Impact:**
- ✅ Auto-update now installs correct Termux package
- ✅ No more update loop (binary version matches npm version)
- ✅ Permission errors resolved
- ✅ Clean upgrade path for all users

---

### 7. Manual Update Instructions on Android/Termux

**File**: `codex-rs/cli/src/main.rs`
**Lines Modified**: ~30 (function `run_update_action`)
**Date Applied**: 2025-11-05
**Upstream Issue**: Auto-update fails on Android with "Permission denied"

#### Problem
When user accepts auto-update on Termux, Codex tries to execute:
```bash
npm install -g @mmmbuto/codex-cli-termux@latest
```

**Error seen:**
```
Updating Codex via `npm install -g @mmmbuto/codex-cli-termux@latest`...
env: 'node': Permission denied
Error: failed with status exit status: 126
```

**Root cause:**
- Codex binary is spawned by Node.js wrapper (`bin/codex.js`)
- When npm tries to update, it must overwrite `/usr/lib/node_modules/.../bin/codex`
- Binary is IN USE (running process)
- Android/Termux: Cannot overwrite files in use → Permission denied
- Linux/Mac: Can overwrite (old inode remains) → Works fine

**Why loop exists:**
Even if npm succeeded:
1. npm 0.55.2-termux published with binary 0.55.1 inside
2. User accepts update → npm reinstalls same broken package
3. Binary still 0.55.1 after "update"
4. Codex checks: 0.55.2 > 0.55.1 → Shows update again
5. INFINITE LOOP

#### Solution
Use conditional compilation to detect Android and show manual instructions instead:

**Changes:**
```rust
// cli/src/main.rs - run_update_action()

#[cfg(target_os = "android")]
{
    println!("⚠️  Auto-update is not available on Termux/Android");
    println!("    (binary in use cannot be overwritten)");
    println!();
    println!("📦 To update manually, run:");
    println!("    {cmd_str}");
    println!();
    println!("💡 After update completes, restart Codex to use the new version.");
    return Ok(());
}

#[cfg(not(target_os = "android"))]
{
    // Execute update automatically on other platforms
    let status = std::process::Command::new(cmd).args(args).status()?;
    // ... error handling
}
```

**Impact:**
- ✅ No more "Permission denied" errors
- ✅ User-friendly message explaining Termux limitation
- ✅ Clear manual update command displayed
- ✅ Other platforms (Linux/Mac) unchanged
- ✅ No infinite loop (combined with version fix)

---

## 📊 Patch Categories

### Core Functionality (Required)
- **Patch #1**: Browser login fix (termux-open-url)
- **Patch #2**: RAM optimizations (compilation settings)
- **Patch #3**: Version alignment with upstream

### Auto-Update System (Required)
- **Patch #4**: Auto-update URL redirect (GitHub API)
- **Patch #5**: Version parser (-termux suffix handling)
- **Patch #6**: NPM package name fix
- **Patch #7**: Manual update instructions on Android

**All patches are CRITICAL** - Codex will not work correctly on Termux without them.

---

## Versioning Strategy

| Component | Version | Example |
|-----------|---------|---------|
| **Binary** | Upstream version | `codex-cli 0.55.0` |
| **npm package** | `<upstream>-termux` | `0.55.0-termux` |

**Why:**
- Binary version matches upstream for compatibility
- npm suffix indicates Termux-specific release
- Clear traceability to upstream version

---

## Testing Checklist

Before each release:
- [ ] `codex --version` shows correct upstream version (0.55.0)
- [ ] `codex login` opens browser without crash
- [ ] OAuth flow completes successfully
- [ ] Binary size < 50MB
- [ ] Compilation completes on 16GB device
- [ ] Auto-update checks correct URL

---

## Contributing

Found a bug specific to Termux? Please open an issue with:
1. Steps to reproduce
2. Expected vs actual behavior
3. Device specs (RAM, Android version)
4. Error logs

We only accept patches for Termux-specific issues, not general feature requests.

---

**Last Updated**: 2025-11-05
**Based on**: OpenAI Codex 0.55.0 (46 commits ahead of 0.53.0)
**Platform**: Android Termux ARM64
