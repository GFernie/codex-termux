# CODEX CLI TEST SUITE - EXTENDED REPORT

Platform: Android Termux ARM64
Codex Version: codex-cli 0.113.0
Codex-Exec Version: codex-exec 0.113.0
Suite Source: test-reports/CODEX_TEST_SUITE.md and test-reports/suites/latest/termux.md
Test Date: 2026-03-10
Runner raw log: TERMUX_EXTENDED_RAW_20260310-194448.log
Extended TSV: TERMUX_EXTENDED_RESULTS_20260310-194448.tsv

Summary
-------
Total Tests: 52
✅ Passed: 49
❌ Failed: 0
⚠️ Skipped: 3

Category totals:
- Category 1 (System): 3/3
- Category 2 (File Ops): 8/8
- Category 3 (Search): 3/3
- Category 4 (Shell): 4/4
- Category 5 (Text): 2/2
- Category 6 (Web & Network): 1 passed, 1 skipped
- Category 7 (Git): 1 passed, 1 skipped
- Category 8 (AI): 3/3
- Category 9 (Error): 3/3
- Category 10 (Termux): 12/12
- Category 12 (Package/Bin): 7/8 (1 skipped)
- Category 11 (Cleanup): 1/1

Verdict: ✅ PASS

Notes:
- Execution mode: fully manual, command-by-command (no custom script runner).
- TEST-601 skipped in local harness (no search flag exposed in help output).
- TEST-702 skipped because workspace is not a git repo.
- TEST-1208 skipped as deprecated/non-applicable for npm-only validation.
- v0.113.0 wrapper routing guard passed (`fork`/`debug` routed correctly).
