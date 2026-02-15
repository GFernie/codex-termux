# Codex Test Suite Final Report (from `CODEX_TEST_SUITE.md`)

## Summary
- Total tests: 50
- Passed: 47
- Failed: 1
- Skipped: 2

## Critical Failures
- `TEST-1208` Upstream Crate Inventory: Cargo workspace present but could not confirm compiled release binaries

## Warnings
- (none)

## Notes
- No `git` trusted-directory / `safe.directory` (“dubious ownership”) behavior encountered in logs.
- Suite header claims 82 tests, but the file contains 50 `TEST-xxxx` sections; all `TEST-xxxx` sections present were executed sequentially.
- Report saved to: `test-reports/latest/0.100.0-termux/CODEX_TEST_REPORT_v0.100.0-termux_extended.md` (suite template path was updated after this run)
- Runner log: `/data/data/com.termux/files/usr/tmp/codex-test-run-20260213T000649/run.log`

## Environment Summary
- Platform: Android Termux ARM64 (`uname -m`: `aarch64`)
- User: `u0_a458`
- Shell: `/data/data/com.termux/files/usr/bin/zsh`
- `PREFIX`: `/data/data/com.termux/files/usr`
- Node: `v25.3.0`
- npm: `11.8.0`
- Codex: `codex-cli 0.100.0`
- Android SDK (via `getprop`): `36`

## Per-Test Results (Concise)
- `TEST-000`: PASS - Environment Preparation
- `TEST-101`: PASS - Display Codex Version
- `TEST-102`: PASS - Environment Context
- `TEST-103`: PASS - Platform Detection
- `TEST-201`: PASS - Create Text File
- `TEST-202`: PASS - Read File
- `TEST-203`: PASS - Modify File (Append)
- `TEST-204`: PASS - Modify File (Edit/Replace)
- `TEST-205`: PASS - Create Directory Structure
- `TEST-206`: PASS - List Directory Contents
- `TEST-207`: PASS - Create Multiple Files
- `TEST-208`: PASS - Delete File
- `TEST-301`: PASS - Find Files by Pattern
- `TEST-302`: PASS - Search File Contents (Grep)
- `TEST-303`: PASS - Recursive Directory Search
- `TEST-401`: PASS - Simple Shell Command
- `TEST-402`: PASS - Command with Output Capture
- `TEST-403`: PASS - Command Chain (Pipes)
- `TEST-404`: PASS - Package Manager Test
- `TEST-501`: PASS - JSON File Operations
- `TEST-502`: PASS - Multi-line File Creation
- `TEST-601`: SKIPPED - Web Search (Codex WebSearch tool not available in this local terminal runner)
- `TEST-602`: PASS - Network Connectivity
- `TEST-701`: PASS - Git Repository Detection
- `TEST-702`: SKIPPED - Git Information (Not in a git repo in test workspace)
- `TEST-801`: PASS - Code Analysis
- `TEST-802`: PASS - Problem Solving
- `TEST-803`: PASS - Documentation Generation
- `TEST-901`: PASS - Handle Non-existent File
- `TEST-902`: PASS - Handle Invalid Command
- `TEST-903`: PASS - Handle Permission Issues
- `TEST-1001`: PASS - Termux Paths
- `TEST-1002`: PASS - Termux Shell Detection
- `TEST-1003`: PASS - Termux-API Availability
- `TEST-1004`: PASS - Termux Package Manager
- `TEST-1005`: PASS - Termux Storage Paths
- `TEST-1006`: PASS - Termux Environment Variables
- `TEST-1007`: PASS - Android-Specific Commands
- `TEST-1008`: PASS - Library Path Preservation
- `TEST-1009`: PASS - Termux Browser Open
- `TEST-1010`: PASS - Android Permissions
- `TEST-1201`: PASS - Verify codex-tui Binary
- `TEST-1202`: PASS - Verify codex-exec Binary
- `TEST-1203`: PASS - Verify codex-exec JSON Flag
- `TEST-1204`: PASS - NPM Package Structure
- `TEST-1205`: PASS - Binary Version Consistency
- `TEST-1206`: PASS - Package.json Bin Entries
- `TEST-1207`: PASS - Global Command Availability
- `TEST-1208`: FAIL - Upstream Crate Inventory (Cargo workspace present but could not confirm compiled release binaries)
- `TEST-1101`: PASS - Remove Test Files
