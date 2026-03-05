=====================================
CODEX CLI TEST SUITE - FINAL REPORT
=====================================

Platform: Android Termux ARM64
Codex Version: codex-cli 0.110.0 / codex-exec 0.110.0
Test Date: 2026-03-05 (CET)
Suite Source: test-reports/suites/latest/termux.md
Primary Raw Log: TERMUX_SAFE_RAW_20260305-090607.log
Non-Interactive #1 Log: TEST_noninteractive_1.jsonl
Non-Interactive #2 Log: TEST_noninteractive_2.jsonl
Network Smoke Log: TEST_network_smoke.jsonl

SUMMARY:
--------
Total Checks: 12
Passed: 12
Failed: 0
Skipped: 0

SECTION BREAKDOWN:
------------------
1. Install Guard: 2/2 passed
2. Version Guard: 1/1 passed
3. Core Tests: 4/4 passed
4. v0.104.0 Regression Guard: 2/2 passed
5. v0.108.0 Dependency Crash Guard: 2/2 passed
6. Termux Checks: 1/1 passed

CRITICAL FAILURES:
------------------
None.

WARNINGS:
---------
None.

NOTES:
------
- Global package verified: @mmmbuto/codex-cli-termux@0.110.0-termux.
- Global commands verified:
  - codex -> /data/data/com.termux/files/usr/bin/codex
  - codex-exec -> /data/data/com.termux/files/usr/bin/codex-exec
- Dependency crash guards validated:
  - TEST-1011 feature chain (`oboe-shared-stdcxx`)
  - TEST-1012 ELF linkage (`libc++_shared.so`, no `libc++_static`)

DETAILED RESULTS:
-----------------
- Install Guard / npm package: PASS
  - @mmmbuto/codex-cli-termux@0.110.0-termux found globally.

- Install Guard / global commands: PASS
  - codex => /data/data/com.termux/files/usr/bin/codex
  - codex-exec => /data/data/com.termux/files/usr/bin/codex-exec

- Version Guard: PASS
  - codex --version => codex-cli 0.110.0
  - codex-exec --version => codex-exec 0.110.0

- Core / workspace reset: PASS
  - Workspace created at ~/codex-test-workspace.

- Core / help commands: PASS
  - help output returned for codex, codex exec, codex-exec.

- Core / codex-exec json sanity #1: PASS
  - JSON event stream returned, shell command completed, no crash/panic.

- Core / codex-exec json sanity #2: PASS
  - hello.txt created and read back successfully.

- Regression / binary architecture guard: PASS
  - Launchers are scripts; codex.bin and codex-exec.bin are ARM64 Android ELF.

- Regression / network-path smoke: PASS
  - No crash/panic. First status detected: HTTP/2 200.

- Dependency guard / feature chain: PASS
  - cargo tree shows cpal -> oboe -> oboe-sys with shared-stdcxx features.

- Dependency guard / linkage: PASS
  - readelf shows libc++_shared.so; no libc++_static references.

- Termux checks: PASS
  - uname => Android detected
  - PREFIX => /data/data/com.termux/files/usr
  - node => v25.3.0
  - npm => 11.11.0
  - termux-open-url => /data/data/com.termux/files/usr/bin/termux-open-url

VERDICT: PASS
