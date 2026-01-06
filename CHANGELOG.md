# Changelog - Codex Termux

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [0.78.0-termux] - 2026-01-06

### Termux Notes
- Upstream bump to OpenAI Codex rust-v0.78.0.
- Termux patches (#1–#6, #8, #9) revalidated via `verify-patches.sh`.

### Testing
- CODEX_TEST_SUITE v1.2 on Termux (2026-01-06): 49 tests, 47 passed / 0 failed / 2 skipped (WebSearch unavailable; git info skipped outside repo). Package & Binary 8/8 passed; Termux-specific 10/10 passed.

---

## [0.77.1-termux] - 2026-01-04

### Termux Notes
- Upstream bump to rust-v0.77.0 plus 63 commits after; key highlights: config sources, execpolicy wiring, TUI2 selection/copy/perf, unified exec output cap.
- Single entrypoint confirmed: `codex` for TUI; `codex exec` for automation; `codex-exec` kept as JS wrapper (no symlink).
- Termux patches (#1–#6, #8, #9) revalidated via `verify-patches.sh`.

### Testing
- CODEX_TEST_SUITE v1.2 on Termux (2026-01-04): 49 tests, 47 passed / 0 failed / 2 skipped (WebSearch disabled; git info skipped outside repo). Package & Binary 8/8 passed; Termux-specific 10/10 passed.
