# LTS Testing

LTS testing is split into two layers and is now intentionally broader than a simple smoke checklist.

## Core

Core is release-blocking. It must pass before npm publish or GitHub release.

Core validates:

- installed binary identity and `-lts` version family
- wrapper and alias resolution
- built-in provider/profile discovery
- CLI help and feature sanity
- non-interactive `codex-exec --json` behavior
- slash palette baseline, including `/plan`
- `/chat` multi-turn responsiveness
- persistent `/plan` and `/code` restore
- automatic workspace inspection flows
- automatic file mutation flows
- git-aware `/diff` and `/review`
- updater/channel sanity
- platform-specific environment capture

Required providers for Core:

- Qwen3.5
- GLM5

## Extended

Extended is the broader regression checklist. It should be run before a release candidate is considered clean.

Extended validates:

- the complete visible slash-command surface
- `model`, `approvals`, `status`, `new`, `resume`, `init`, `skills`, `mention`, `logout`, and `feedback`
- long-session `/compact`
- tool families:
  - shell execution and command-output capture
  - inspect/search/read
  - patch-style editing
  - one-file edit
  - multi-file edit
  - git-aware review/diff
  - MCP resource flows
  - web-search flows when enabled
- provider advisory matrix:
  - DeepSeek
  - Kimi, when wrappers are available
- failure-mode behavior:
  - outside-git `/diff` and `/review`
  - missing-file mention/read
  - unavailable MCP
  - other clear, recoverable tool failures
- platform-specific ergonomics and package identity checks

## What "complete" means here

The suites are designed to cover the operator-visible LTS surface and the main automatic jobs a Codex-style CLI must handle:

- chat reliably over multiple turns
- switch between planning and coding
- inspect a workspace automatically
- run shell commands when needed and use the output correctly
- search and read relevant files
- mutate one or more files safely
- show/review git diffs
- compact long sessions
- resume or restart sessions
- use MCP and web search when configured

Not everything is a release blocker:

- hidden debug-only slash commands are excluded
- advisory providers are documented separately from the required providers
- MCP and web search tests are conditional on local configuration

## Platform tailoring

Each platform has its own Core and Extended suite because the risks differ:

- Linux focuses on the local npm/package tree and general workstation behavior
- Termux focuses on input responsiveness, mobile terminal stability, and Android/Termux filesystem quirks
- macOS focuses on the final arm64 package/artifact behavior from GitHub Actions

## Suite Locations

- Linux Core: [`../test-reports/suites/lts/core/linux.md`](../test-reports/suites/lts/core/linux.md)
- Linux Extended: [`../test-reports/suites/lts/extended/linux.md`](../test-reports/suites/lts/extended/linux.md)
- Termux Core: [`../test-reports/suites/lts/core/termux.md`](../test-reports/suites/lts/core/termux.md)
- Termux Extended: [`../test-reports/suites/lts/extended/termux.md`](../test-reports/suites/lts/extended/termux.md)
- macOS Core: [`../test-reports/suites/lts/core/macos.md`](../test-reports/suites/lts/core/macos.md)
- macOS Extended: [`../test-reports/suites/lts/extended/macos.md`](../test-reports/suites/lts/extended/macos.md)

## Reports

Store release reports under `test-reports/lts/<version>/`.

Use the shared template:

- [`../test-reports/lts/REPORT_TEMPLATE.md`](../test-reports/lts/REPORT_TEMPLATE.md)

Each report should capture:

- platform
- binary source
- provider matrix
- Core result
- Extended result
- blockers
- advisory issues
- raw error snippets when relevant
