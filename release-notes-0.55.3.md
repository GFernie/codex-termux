## Complete Auto-Update Fix for Termux

This release resolves the infinite update loop and "Permission denied" errors on Android/Termux.

### Critical Fixes
- **Auto-update on Android**: Now shows manual update instructions instead of failing
- **Infinite loop resolved**: Binary version synchronized to 0.55.3
- **Permission denied**: No more errors when accepting updates on Termux

### What Changed
**On Android/Termux:**
- Codex detects platform and shows friendly manual update instructions
- Users run: `npm install -g @mmmbuto/codex-cli-termux@latest` themselves
- No more "binary in use" permission errors

**On Linux/Mac:**
- Auto-update continues to work automatically (unchanged)

### Technical Details
- **Patch #7**: Platform-specific update handling with `#[cfg(target_os = "android")]`
- **Root cause**: Android cannot overwrite files in use, unlike Linux/Mac
- **Solution**: Show manual instructions on Android, auto-execute on other platforms

### All Patches Applied (7 total)
1. Browser login fix (termux-open-url)
2. RAM optimizations (lto=false, codegen-units=16)
3. Version alignment (0.55.3)
4. Auto-update URL redirect (DioNanos/codex-termux)
5. Version parser fix (-termux suffix)
6. NPM package name fix (@mmmbuto/codex-cli-termux)
7. **Manual update instructions on Android** ⭐ NEW

### Installation
```bash
npm install -g @mmmbuto/codex-cli-termux@latest
```

### Upgrade from Previous Versions
If you see an update prompt:
```bash
npm install -g @mmmbuto/codex-cli-termux@latest
```

Then restart Codex.

---

**Binary version**: 0.55.3
**npm package**: 0.55.3-termux
**Based on**: OpenAI Codex 0.55.0
