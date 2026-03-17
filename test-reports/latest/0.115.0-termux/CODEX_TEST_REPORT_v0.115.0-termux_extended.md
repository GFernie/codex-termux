# CODEX TEST REPORT v0.115.0-termux (Extended)

- Date: 2026-03-17 12:02:56 CET
- Device: pixel9pro (Termux)
- Repo: `~/Dev/codex-termux`
- Scope: extended validation on top of `test-reports/suites/latest/termux.md`
- Raw log (sanitized): `test-reports/latest/0.115.0-termux/TERMUX_EXTENDED_RAW_20260317-120256_sanitized.log`
- Structured results: `test-reports/latest/0.115.0-termux/TERMUX_EXTENDED_RESULTS_20260317-120256.tsv`

Sanitization policy applied:
- Home absolute path normalized to `~` in report excerpts
- User IDs normalized to `termux_user` where applicable
- Trust-guard output contains no secrets and is kept verbatim

## Extended Checks

## EXT-100 - Patch set verification
Command: `bash verify-patches.sh`
Result: PASS
Notes:
- Critical patches #1, #2, #4, #5, #6, #9, #10, #11, #12 present
- Informational warning on extra undeclared patch files remains unchanged from existing baseline

## EXT-110 - Android feature guard (`codex-cli`)
Command:
```bash
cd codex-rs
cargo tree -p codex-cli -e features --target aarch64-linux-android | rg -e 'voice-input|cpal|oboe|oboe-sys' || true
```
Result: PASS (empty output)

## EXT-111 - Android feature guard (`codex-cloud-tasks`)
Command:
```bash
cd codex-rs
cargo tree -p codex-cloud-tasks -e features --target aarch64-linux-android | rg -e 'voice-input|cpal|oboe|oboe-sys' || true
```
Result: PASS (empty output)

## EXT-120 - Android compile guard (`codex-network-proxy`)
Command: `cargo check -p codex-network-proxy --target aarch64-linux-android`
Result: FAIL (environment/toolchain)
Failure:
```text
error: linker `aarch64-linux-android21-clang` not found
```

## EXT-121 - Android compile guard (`codex-core`)
Command: `cargo check -p codex-core --target aarch64-linux-android`
Result: FAIL (environment/toolchain)
Failure:
```text
error: linker `aarch64-linux-android21-clang` not found
```

## EXT-122 - Android compile guard (`codex-cli`)
Command: `cargo check -p codex-cli --target aarch64-linux-android`
Result: FAIL (environment/toolchain)
Failure:
```text
error: linker `aarch64-linux-android21-clang` not found
```

## EXT-130 - Trusted-directory guard behavior
Command:
```bash
cd ~/codex-test-workspace
codex-exec --sandbox workspace-write --json "print current directory"
```
Result: PASS
Expected refusal observed:
```text
Not inside a trusted directory and --skip-git-repo-check was not specified.
```

## Summary
- PASS: 4
- FAIL: 3
- SKIP: 0
- Verdict: PARTIAL PASS (core extended guards pass; Android compile guards blocked by missing NDK linker in current environment)
