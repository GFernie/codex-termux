# CODEX CLI TEST SUITE - FINAL REPORT (Termux)

Platform: Android Termux ARM64
Codex Version: codex-cli 0.113.0
Codex-Exec Version: codex-exec 0.113.0
Test Date: 2026-03-10
Test Duration: ~manual run

SUMMARY:
--------
Total Tests: 52
✅ Passed: 49
❌ Failed: 0
⚠️ Skipped: 3

CATEGORY BREAKDOWN:
-------------------
1. System Information: 3/3 passed
2. File Operations: 8/8 passed
3. Search & Discovery: 3/3 passed
4. Shell Execution: 4/4 passed
5. Text Processing: 2/2 passed
6. Web & Network: 1/2 passed, 1 skipped
7. Git Operations: 1/2 passed, 1 skipped
8. AI Capabilities: 3/3 passed
9. Error Handling: 3/3 passed
10. Termux-Specific: 12/12 passed
11. Package & Binary Verification: 7/8 passed, 1 skipped
12. Cleanup: 1/1 passed

RESULTS:
--------
- **PASS** TEST-000
- **PASS** TEST-101
- **PASS** TEST-102
- **PASS** TEST-103
- **PASS** TEST-201
- **PASS** TEST-202
- **PASS** TEST-203
- **PASS** TEST-204
- **PASS** TEST-205
- **PASS** TEST-206
- **PASS** TEST-207
- **PASS** TEST-208
- **PASS** TEST-301
- **PASS** TEST-302
- **PASS** TEST-303
- **PASS** TEST-401
- **PASS** TEST-402
- **PASS** TEST-403
- **PASS** TEST-404
- **PASS** TEST-501
- **PASS** TEST-502
- **SKIP** TEST-601
- **PASS** TEST-602
- **PASS** TEST-701
- **SKIP** TEST-702
- **PASS** TEST-801
- **PASS** TEST-802
- **PASS** TEST-803
- **PASS** TEST-901
- **PASS** TEST-902
- **PASS** TEST-903
- **PASS** TEST-1001
- **PASS** TEST-1002
- **PASS** TEST-1003
- **PASS** TEST-1004
- **PASS** TEST-1005
- **PASS** TEST-1006
- **PASS** TEST-1007
- **PASS** TEST-1008
- **PASS** TEST-1009
- **PASS** TEST-1010
- **PASS** TEST-1011
- **PASS** TEST-1012
- **PASS** TEST-1201
- **PASS** TEST-1202
- **PASS** TEST-1203
- **PASS** TEST-1204
- **PASS** TEST-1205
- **PASS** TEST-1206
- **PASS** TEST-1207
- **SKIP** TEST-1208
- **PASS** TEST-1101

CRITICAL FAILURES:
------------------
No critical failures.

WARNINGS:
---------
- TEST-601 skipped: web-search flag is not exposed in local `codex-exec --help` harness.
- TEST-702 skipped: test workspace is intentionally not a git repository.
- TEST-1208 skipped: legacy upstream crate inventory check is deprecated for npm-only validation.

EXTRA GUARDS (v0.113.0):
------------------------
- Wrapper routing guard passed:
  - `node codex.js fork --help` -> `Usage: codex fork`
  - `node codex.js debug --help` -> `Usage: codex debug`
- Binary architecture guard passed: launchers are shell scripts, binaries are Android ARM64 ELF.

NOTES:
------
- Suite source: `test-reports/suites/latest/termux.md`
- Primary raw log: `TERMUX_SAFE_RAW_20260310-194448.log`
- Extended TSV: `TERMUX_EXTENDED_RESULTS_20260310-194448.tsv`
- Non-Interactive #1 Log: `TEST_noninteractive_1.jsonl`
- Non-Interactive #2 Log: `TEST_noninteractive_2.jsonl`
- Non-Interactive #3 Log: `TEST_noninteractive_3.jsonl`

VERDICT: ✅ PASS
