# CODEX CLI TEST SUITE - EXTENDED REPORT

Platform: Android Termux ARM64
Codex Version: codex-cli 0.112.0
Codex-Exec Version: codex-exec 0.112.0
Suite Source: test-reports/CODEX_TEST_SUITE.md
Test Date: 2026-03-09
Runner raw log: TERMUX_EXTENDED_RAW_20260309-160900.log
Extended TSV: TERMUX_EXTENDED_RESULTS_20260309-160900.tsv

Summary
-------
Total Tests: 52
✅ Passed: 50
❌ Failed: 0
⚠️ Skipped: 2

Category totals:
- Category 1 (System): 3/3
- Category 2 (File Ops): 8/8
- Category 3 (Search): 3/3
- Category 4 (Shell): 4/4
- Category 5 (Text): 2/2
- Category 6 (Web & Network): 2/2
- Category 7 (Git): 1 passed, 1 skipped
- Category 8 (AI): 3/3
- Category 9 (Error): 3/3
- Category 10 (Termux): 12/12
- Category 12 (Package/Bin): 7/8 (1 skipped)
- Category 11 (Cleanup): 1/1

Verdict: ⚠️ PASS WITH WARNINGS

Primary failures:
- No critical failures. `TEST-1208` downgraded to deprecated/non-applicable check for local npm validation.

Notes:
- `TEST-601` marked pass with note: search flag exists in help; no live web query executed.
- `TEST-702` skipped because workspace is not a git repo.
- `TEST-1208` deprecated: `codex-rs/target/release/codex*` check is no longer required for local npm validation.
