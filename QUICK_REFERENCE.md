# 🚀 Codex Termux - Quick Reference

**Version**: 0.55.2-termux (binary 0.55.1)
**Last Updated**: 2025-11-05

---

## 📚 Documentation Index

| File | Purpose |
|------|---------|
| **README.md** | User-facing documentation (public) |
| **CHANGELOG.md** | Version history and changes |
| **UPGRADE.md** | Step-by-step upgrade instructions |
| **patches/README.md** | Technical patch documentation |
| **QUICK_REFERENCE.md** | This file - quick links |

---

## 🔧 Current Patch Status

**Total Patches**: 6 (all critical)

| # | File | Lines | Purpose | Status |
|---|------|-------|---------|--------|
| 1 | login/src/server.rs | ~10 | Browser login (termux-open-url) | ✅ Active |
| 2 | Cargo.toml | ~8 | RAM optimizations (lto, codegen) | ✅ Active |
| 3 | Cargo.toml | 1 | Version alignment | ✅ Active |
| 4 | tui/src/updates.rs | ~14 | Auto-update URL redirect | ✅ Active |
| 5 | tui/src/updates.rs | ~3 | Version parser (-termux suffix) | ✅ Active |
| 6 | tui/src/updates.rs + Cargo.toml | ~4 | NPM package name fix | ✅ Active |

---

## 📦 Release Checklist

**Before starting**:
- [ ] Read `UPGRADE.md` for detailed instructions
- [ ] Check `patches/README.md` for current patch list
- [ ] Backup current branch

**During upgrade**:
- [ ] Fetch upstream tag
- [ ] Create clean branch
- [ ] Apply all 6 patches
- [ ] Restore Termux README
- [ ] Test compilation
- [ ] Security check

**After compilation**:
- [ ] Test binary version
- [ ] Verify npm package name in binary
- [ ] Create tarball
- [ ] Create GitHub release
- [ ] Publish npm package
- [ ] Update CHANGELOG.md
- [ ] Update documentation

---

## 🎯 Quick Commands

### Upgrade from Upstream
```bash
cd ~/Dev/codex-termux
git fetch upstream
UPSTREAM_VERSION="0.56.0"  # Example
git checkout -b clean-${UPSTREAM_VERSION} rust-v${UPSTREAM_VERSION}
# Follow UPGRADE.md for patches
```

### Compilation
```bash
cd ~/Dev/codex-termux/codex-rs
cargo build --release --bin codex
# Time: ~10-15 minutes on Pixel 9 Pro
```

### Testing
```bash
./codex-rs/target/release/codex --version
strings codex-rs/target/release/codex | grep "@mmmbuto"
strings codex-rs/target/release/codex | grep -E "(dag@|/home/dag)"
```

### Release
```bash
# Tarball
tar -czf codex-${VERSION}-termux-arm64.tar.gz -C codex-rs/target/release codex

# GitHub
export GITHUB_TOKEN="ghp_..."
gh release create v${VERSION}-termux \
  --repo DioNanos/codex-termux \
  --title "v${VERSION}-termux" \
  --notes-file release-notes.md \
  codex-${VERSION}-termux-arm64.tar.gz

# npm
cd npm-package
# Update package.json version
npm publish --access public
```

---

## 🔐 Credentials Location

See private documentation:
```
~/.docs/projects/Codex-Termux.md
```

Contains:
- GitHub Personal Access Token
- npm authentication
- Session history

---

## 🐛 Common Issues & Solutions

### "ndk-context was not initialized"
- **Patch #1** missing
- Fix: Apply Android-specific browser open in `login/src/server.rs`

### Compilation OOM
- **Patch #2** missing
- Fix: Set `lto=false` and `codegen-units=16` in `Cargo.toml`

### "npm install -g @openai/codex"
- **Patch #6** missing
- Fix: Update `UpdateAction::command_args()` to use `@mmmbuto/codex-cli-termux`

### Update loop (infinite updates)
- **Patch #6** incomplete
- Fix: Synchronize binary version in `Cargo.toml` with npm version

### Version parser fails
- **Patch #5** missing
- Fix: Split on `-` before parsing in `parse_version()`

---

## 📊 Statistics

### Compilation
- **Time**: 10-15 minutes (Pixel 9 Pro)
- **RAM**: 12-14GB peak
- **Binary size**: 44MB (17MB compressed)

### Versions
- **0.50.2-termux**: Initial release (2025-10-24)
- **0.53.0-termux**: Clean release (2025-10-31)
- **0.53.1-termux**: Auto-update URL fix (2025-10-31)
- **0.53.2-termux**: Node.js wrapper (2025-11-01)
- **0.55.0-termux**: Major upstream sync (2025-11-05)
- **0.55.1-termux**: Version parser fix (2025-11-05)
- **0.55.2-termux**: NPM package fix (2025-11-05) ⭐

---

## 🔗 Links

- **GitHub**: https://github.com/DioNanos/codex-termux
- **npm**: https://www.npmjs.com/package/@mmmbuto/codex-cli-termux
- **Upstream**: https://github.com/openai/codex
- **Releases**: https://github.com/DioNanos/codex-termux/releases

---

## 📝 Notes

- All patches are **CRITICAL** - never skip any
- Always test compilation before release
- Always run security check (no sensitive data in binary)
- Keep documentation in sync with releases
- Update `~/.docs/projects/Codex-Termux.md` after each release

---

**Maintained by**: Davide A. Guglielmi (dev@mmmbuto.com)
**Platform**: Android Termux ARM64 (Pixel 9 Pro)
**License**: Apache 2.0
