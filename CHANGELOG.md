# Changelog - Codex Termux

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

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
