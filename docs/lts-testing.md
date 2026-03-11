# LTS Testing

LTS testing is split into two layers.

## Core

Core is release-blocking. It must pass before npm publish or GitHub release.

Core validates:

- installed binary identity and `-lts` version family
- wrapper and alias resolution
- non-interactive `codex-exec --json` sanity
- `/chat` multi-turn responsiveness
- `/plan` persistence and `/code` restore
- tool-call and file-mutation round-trip
- updater/channel sanity
- required providers:
  - Qwen3.5
  - GLM5

## Extended

Extended is the broader regression checklist. It should be run before a release candidate is considered clean.

Extended validates:

- slash command coverage
- long-context and compact behavior
- review/diff behavior inside and outside a git repo
- resume/new/init flows
- provider advisory matrix:
  - DeepSeek
  - Kimi, when wrappers are available
- platform-specific package identity checks

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
