# 🔄 Upgrade Guide - Codex Termux

## Quick Reference for Next Upgrade

**Current Version**: 0.55.0 (binary 0.55.1)
**Last Upgrade**: 2025-11-05
**Patches Applied**: 6

---

## 📋 Pre-Upgrade Checklist

Before fetching new upstream version:

1. **Backup current state**:
   ```bash
   cd ~/Dev/codex-termux
   git branch backup-$(date +%Y%m%d)
   git push origin backup-$(date +%Y%m%d)
   ```

2. **Review current patches**:
   ```bash
   cat patches/README.md
   ```

3. **Check upstream changelog**:
   ```bash
   git fetch upstream
   git log upstream/main --oneline | head -20
   ```

---

## 🔧 Upgrade Workflow

### Step 1: Fetch Upstream

```bash
cd ~/Dev/codex-termux
git fetch upstream

# Check latest upstream tag
git tag -l "rust-v*" | sort -V | tail -5

# Example: Upgrade to rust-v0.56.0
UPSTREAM_VERSION="0.56.0"
UPSTREAM_TAG="rust-v${UPSTREAM_VERSION}"
```

### Step 2: Create Clean Branch

```bash
# Create new branch from upstream tag
git checkout -b clean-${UPSTREAM_VERSION} ${UPSTREAM_TAG}

# Verify clean checkout
git log --oneline -5
```

### Step 3: Apply All Patches

Apply patches in order. **Reference**: `patches/README.md`

#### **Patch #1: Browser Login Fix**
**File**: `codex-rs/login/src/server.rs`
**Lines**: Around line 10-42

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

#### **Patch #2: RAM Optimizations**
**File**: `codex-rs/Cargo.toml`
**Section**: `[profile.release]`

```toml
[profile.release]
lto = false                    # Disable LTO (saves ~4GB RAM)
strip = "symbols"
codegen-units = 16             # Use 16 parallel units (was 1)
opt-level = 3                  # Keep optimization level high
```

#### **Patch #3: Version Alignment**
**File**: `codex-rs/Cargo.toml`
**Field**: `[workspace.package] version`

```toml
# Set to match upstream version
version = "0.56.0"  # Example: update to new version
```

#### **Patch #4: Auto-Update URL Redirect**
**File**: `codex-rs/tui/src/updates.rs`
**Lines**: 56, 81-85

```rust
// Line 56: Update URL
const LATEST_RELEASE_URL: &str = "https://api.github.com/repos/DioNanos/codex-termux/releases/latest";

// Lines 81-85: Update tag parser
let version = latest_tag_name
    .strip_prefix("v")
    .unwrap_or(&latest_tag_name)
    .to_string();
```

#### **Patch #5: Version Parser Fix**
**File**: `codex-rs/tui/src/updates.rs`
**Function**: `parse_version()` (around line 139-147)

```rust
fn parse_version(v: &str) -> Option<(u64, u64, u64)> {
    let mut iter = v.trim().split('.');
    let maj = iter.next()?.parse::<u64>().ok()?;
    let min = iter.next()?.parse::<u64>().ok()?;
    // Handle suffixes like "0-termux" by splitting on '-' and taking first part
    let pat_str = iter.next()?;
    let pat = pat_str.split('-').next()?.parse::<u64>().ok()?;
    Some((maj, min, pat))
}
```

#### **Patch #6: NPM Package Name Fix**
**File**: `codex-rs/tui/src/updates.rs`
**Function**: `UpdateAction::command_args()` (around line 180-186)

```rust
impl UpdateAction {
    pub fn command_args(self) -> (&'static str, &'static [&'static str]) {
        match self {
            UpdateAction::NpmGlobalLatest => ("npm", &["install", "-g", "@mmmbuto/codex-cli-termux@latest"]),
            UpdateAction::BunGlobalLatest => ("bun", &["install", "-g", "@mmmbuto/codex-cli-termux@latest"]),
            UpdateAction::BrewUpgrade => ("brew", &["upgrade", "--cask", "codex"]),
        }
    }
}
```

### Step 4: Restore Termux README

```bash
# Save upstream README if needed
mv README.md README.upstream.md

# Restore Termux README from origin/main
git show origin/main:README.md > README.md

# Update version numbers in README
sed -i "s/0.55.0/${UPSTREAM_VERSION}/g" README.md
```

### Step 5: Update Documentation

1. **Update patches/README.md**:
   - Update "Based on" version
   - Update dates
   - Add any new patches if needed

2. **Update UPGRADE.md** (this file):
   - Update "Current Version"
   - Update "Last Upgrade" date

3. **Check for conflicts**:
   ```bash
   # Search for potential conflicts
   cd codex-rs
   grep -r "openai/codex" --include="*.rs" | grep -v "target/"
   grep -r "rust-v" --include="*.rs" | grep -v "target/"
   ```

### Step 6: Commit Changes

```bash
git add -A
git commit -m "chore: Apply Termux patches for v${UPSTREAM_VERSION}

Applied patches:
- Browser login fix (termux-open-url)
- RAM optimizations (lto=false, codegen-units=16)
- Version alignment (${UPSTREAM_VERSION})
- Auto-update URL redirect (DioNanos/codex-termux)
- Version parser fix (-termux suffix)
- NPM package name fix (@mmmbuto/codex-cli-termux)

Based on: OpenAI Codex ${UPSTREAM_VERSION}"
```

### Step 7: Test Compilation

```bash
cd codex-rs
cargo build --release --bin codex

# Monitor compilation (background)
# Expected time: 10-15 minutes on Pixel 9 Pro
```

### Step 8: Test Binary

```bash
./target/release/codex --version
# Should show: codex-cli <UPSTREAM_VERSION>

# Security check
strings target/release/codex | grep -E "(dag@|mmmbuto\.com|alpacalibre|/home/dag|192\.168)"
# Should return empty (no sensitive data)

# Check npm package name
strings target/release/codex | grep "@mmmbuto/codex-cli-termux"
# Should find the correct package name
```

### Step 9: Merge to Main

```bash
git checkout main
git reset --hard clean-${UPSTREAM_VERSION}
git push origin main --force-with-lease

# Create tag
git tag -a v${UPSTREAM_VERSION}-termux -m "Release ${UPSTREAM_VERSION}-termux"
git push origin v${UPSTREAM_VERSION}-termux
```

### Step 10: Release

```bash
# Create tarball
tar -czf codex-${UPSTREAM_VERSION}-termux-arm64.tar.gz -C codex-rs/target/release codex

# GitHub release
export GITHUB_TOKEN="<token>"
gh release create v${UPSTREAM_VERSION}-termux \
  --repo DioNanos/codex-termux \
  --title "v${UPSTREAM_VERSION}-termux" \
  --notes "..." \
  codex-${UPSTREAM_VERSION}-termux-arm64.tar.gz

# npm release
cd npm-package
# Update package.json version
npm publish --access public
```

---

## 🔍 Common Issues

### Issue 1: Compilation OOM
**Solution**: Check `[profile.release]` settings in Cargo.toml
- Ensure `lto = false`
- Ensure `codegen-units = 16`

### Issue 2: Browser Login Crash
**Solution**: Check `login/src/server.rs` has Android-specific code
- Must use `termux-open-url` on Android
- Must NOT use `webbrowser::open()` on Android

### Issue 3: Auto-Update Wrong Package
**Solution**: Check `tui/src/updates.rs`
- URL must point to `DioNanos/codex-termux`
- Package name must be `@mmmbuto/codex-cli-termux`

### Issue 4: Update Loop
**Solution**: Synchronize versions
- Binary version in `Cargo.toml` must match npm base version
- npm version uses `-termux` suffix
- Example: binary `0.56.0` → npm `0.56.0-termux`

---

## 📝 Post-Upgrade Checklist

- [ ] Binary compiled successfully
- [ ] `codex --version` shows correct version
- [ ] Security check passed (no sensitive data)
- [ ] README.md updated with new version
- [ ] patches/README.md updated
- [ ] GitHub release created
- [ ] npm package published
- [ ] Tag pushed to GitHub
- [ ] Documentation updated

---

## 📊 Patch Summary

| Patch # | File | Purpose | Critical? |
|---------|------|---------|-----------|
| 1 | login/src/server.rs | Browser login fix | ✅ YES |
| 2 | Cargo.toml | RAM optimizations | ✅ YES |
| 3 | Cargo.toml | Version alignment | ⚠️ YES |
| 4 | tui/src/updates.rs | Auto-update URL | ✅ YES |
| 5 | tui/src/updates.rs | Version parser | ✅ YES |
| 6 | tui/src/updates.rs | NPM package name | ✅ YES |

**All patches are CRITICAL** - Do not skip any!

---

**Last Updated**: 2025-11-05
**Next Review**: When upstream releases new version
