# Applied Patches

This document lists all patches applied to OpenAI Codex for Termux compatibility.

## Policy

We **only** apply patches that:
1. Fix critical issues preventing Codex from working on Termux
2. Are not addressed by upstream (Termux is not officially supported)
3. Are minimal and well-documented
4. Do not change Codex behavior or features

All patches are open source and documented here for full transparency.

---

## Current Patches

### 1. Browser Login Fix

**File**: `login/src/server.rs`
**Lines Modified**: 9
**Date Applied**: 2025-10-26
**Commit**: [754b506d](https://github.com/DioNanos/codex-termux/commit/754b506d)

#### Problem
Upstream uses `webbrowser` crate which calls `ndk-context` on Android. This requires an Android Activity context, which is not available in Termux (CLI environment). Results in crash:
```
thread 'main' panicked at ndk-context-0.1.1/src/lib.rs:72:30:
android context was not initialized
```

#### Solution
On Android (`target_os = "android"`), use `termux-open-url` command directly instead of `webbrowser::open()`. Other platforms continue using `webbrowser` crate.

#### Code Change
```rust
// BEFORE
if opts.open_browser {
    let _ = webbrowser::open(&auth_url);
}

// AFTER
if opts.open_browser {
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

#### Impact
- ✅ Browser login now works on Termux
- ✅ No behavior change on other platforms
- ✅ No feature additions or removals
- ✅ Identical user experience to upstream

#### Testing
Tested on:
- Device: Pixel 9 Pro
- Android: 15
- Termux: googleplay.2025.10.05
- Result: Browser opens successfully, login works

---

## Reporting Issues

Found a Termux-specific bug? Please report with:
- Termux version (`echo $TERMUX_VERSION`)
- Android version
- Device model
- Full error output
- Steps to reproduce

Open an issue: https://github.com/DioNanos/codex-termux/issues

---

**Last Updated**: 2025-10-26
