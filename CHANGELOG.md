# Changelog - Codex Termux

All notable changes to this project will be documented in this file.

## [0.110.0-termux] - 2026-03-05

### Upstream
- OpenAI Codex rust-v0.110.0 release: https://github.com/openai/codex/releases/tag/rust-v0.110.0
- Upstream release notes and details are maintained in the link above.

### Termux Patches
- Termux patches (#1, #2, #4, #5, #6, #9, #10) revalidated after merge.
- Android audio/linker guard retained (`cpal` with `oboe-shared-stdcxx`).
- Launcher hardening retained (`codex.bin` / `codex-exec.bin` + safe `LD_LIBRARY_PATH`).

### Version
- npm package bumped: `@mmmbuto/codex-cli-termux@0.110.0-termux`
- Upstream base: `rust-v0.110.0`

---

## [0.108.0-termux] - 2026-03-04

### Upstream
- OpenAI Codex rust-v0.108.0 release: https://github.com/openai/codex/releases/tag/rust-v0.108.0
- Upstream release notes and details are maintained in the link above.

### Termux Patches
- Termux patches (#1, #2, #4, #5, #6, #9, #10) revalidated via `verify-patches.sh`.
- Launcher hardening from 0.106.2 (`codex.bin` / `codex-exec.bin` + safe `LD_LIBRARY_PATH`) retained.

### Version
- npm package bumped: `@mmmbuto/codex-cli-termux@0.108.0-termux`
- Upstream base: `rust-v0.108.0`

### Verification
- `env -u LD_LIBRARY_PATH ./bin/codex --version` -> PASS
- `env -u LD_LIBRARY_PATH ./bin/codex-exec --version` -> PASS
- `node ./bin/codex.js --version` -> PASS
- `node ./bin/codex-exec.js --version` -> PASS

---

## [0.106.2-termux] - 2026-03-02

### Packaging Fix
- Fixed direct binary invocation when `LD_LIBRARY_PATH` is missing.
- `npm-package/bin/codex` and `npm-package/bin/codex-exec` are now launcher scripts that export a safe `LD_LIBRARY_PATH` and invoke:
  - `bin/codex.bin`
  - `bin/codex-exec.bin`
- This resolves failures such as:
  - `CANNOT LINK EXECUTABLE .../bin/codex: library "libc++_shared.so" not found`
  when tools invoke package binaries directly (without Node launcher env setup).

### Version
- npm package bumped: `@mmmbuto/codex-cli-termux@0.106.2-termux`
- Upstream base remains: `rust-v0.106.0`

### Verification
- `env -u LD_LIBRARY_PATH ./bin/codex --version` -> PASS
- `env -u LD_LIBRARY_PATH ./bin/codex-exec --version` -> PASS
- `node ./bin/codex.js --version` -> PASS
- `node ./bin/codex-exec.js --version` -> PASS

---

## [0.88.0-termux] - 2026-01-22

### Upstream
- OpenAI Codex rust-v0.88.0 release: https://github.com/openai/codex/releases/tag/rust-v0.88.0
- Upstream release notes and details are maintained in link above.
- **New features:**
  * Collaboration modes and presets to streamline multi-agent workflows
  * Device-code auth as a standalone fallback in headless environments
  * Request-user-input tool for explicit agent prompts
  * Remote models and auto-enable WebSockets transport
  * Thread/fork endpoints (conversation branching)

### Termux Patches
- Termux patches (#1–#6, #9) revalidated via verify-patches.sh.
- **Patch #8 (bash execution)**: Not required (resolved upstream v0.80.0+)
- **Patch #2 (compilation)**: Updated with rustls fix (native-tls → rustls)

### Testing
- CODEX_TEST_REPORT_v0.88.0.md on SamsungWork (2026-01-22): ALL PASS
- Binary sizes: codex 65MB, codex-exec 38MB
- All Termux patches verified (#1–#6, #9)

### Documentation
- Updated patches/README.md for v0.88.0
- Updated STATUS.md with release status
- Created tag: v0.88.0-termux
- Ready for npm publish

---

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [0.80.0-termux] - 2026-01-10

### Upstream
- OpenAI Codex rust-v0.80.0 release: https://github.com/openai/codex/releases/tag/rust-v0.80.0
- Upstream release notes and details are maintained in the link above.
- **Important**: Process hardening removed from Codex CLI (upstream PR #8951)
- New features:
  * Thread/fork endpoints (conversation branching)
  * Requirements/list API
  * Elevated sandbox onboarding NUX
  * Skills explicit invocation via V2 API
  * Metrics capabilities (otel/metrics module)

### Termux Patches
- Termux patches (#1–#6, #9) revalidated via `verify-patches.sh`.
- **Patch #8 (bash execution)**: No longer required - resolved by upstream PR #8951 (process hardening removal)
- This improves bash execution in Agent mode without custom patches.

### Testing
- CODEX_TEST_SUITE v1.2 on Termux (2026-01-10): 49 tests, 49 passed / 0 failed / 0 skipped.
- Package & Binary 8/8 passed; Termux-specific 10/10 passed.
- All binaries verified: codex (60M), codex-tui (42M), codex-exec (35M), codex-app-server (38M).

### Documentation
- Updated patches/README.md for v0.80.0 with Patch #8 resolution notes
- Updated README.md with v0.80.0 version references
- Removed outdated CODEX_TEST_REPORT_v0.79.0.md

---
## [0.79.0-termux] - 2026-01-08

### Upstream
- OpenAI Codex rust-v0.79.0 release: https://github.com/openai/codex/releases/tag/rust-v0.79.0
- Upstream release notes and details are maintained in the link above.

### Termux Patches
- Termux patches (#1–#6, #8, #9) revalidated via `verify-patches.sh`.

### Testing
- CODEX_TEST_SUITE v1.2 on Termux (2026-01-08): 49 tests, 47 passed / 0 failed / 2 skipped (WebSearch unavailable; git info skipped outside repo). Package & Binary 8/8 passed; Termux-specific 10/10 passed.

### Documentation
- Added GLM-4.7 quickstart guide (later removed when docs were generalized)
- Updated README.md with GLM-4.7 setup instructions in Quickstart section

---

## [0.78.0-termux] - 2026-01-06

### Upstream
- OpenAI Codex rust-v0.78.0 release: https://github.com/openai/codex/releases/tag/rust-v0.78.0
- Upstream release notes and details are maintained in the link above.

### Termux Patches
- Termux patches (#1–#6, #8, #9) revalidated via `verify-patches.sh`.

### Testing
- CODEX_TEST_SUITE v1.2 on Termux (2026-01-06): 49 tests, 47 passed / 0 failed / 2 skipped (WebSearch unavailable; git info skipped outside repo). Package & Binary 8/8 passed; Termux-specific 10/10 passed.

---

## [0.77.1-termux] - 2026-01-04

### Upstream
- Base release: rust-v0.77.0 — https://github.com/openai/codex/releases/tag/rust-v0.77.0
- Termux build synced after rust-v0.77.0 (upstream commit range not listed here; see upstream history for details).

### Termux Patches
- Single entrypoint confirmed: `codex` for TUI; `codex exec` for automation; `codex-exec` kept as JS wrapper (no symlink).
- Termux patches (#1–#6, #8, #9) revalidated via `verify-patches.sh`.

### Testing
- CODEX_TEST_SUITE v1.2 on Termux (2026-01-04): 49 tests, 47 passed / 0 failed / 2 skipped (WebSearch disabled; git info skipped outside repo). Package & Binary 8/8 passed; Termux-specific 10/10 passed.
