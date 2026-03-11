# Changelog LTS

All notable LTS changes are documented in this file.

## [0.80.5-lts] - 2026-03-10

### Summary

- LTS line remains based on upstream `rust-v0.80.0`.
- Focus: `/chat` compatibility, stability, and auth/login diagnostics.
- No broad feature uplift from latest line.

### Included Backports

- `fix: eliminate unnecessary clone() for each SSE event` (`#9238`)
- `fix: Emit response.completed immediately for Responses SSE` (`#9170`)
- `Show OAuth error descriptions in callback responses` (`#9654`)
- `Surface local login entitlement denials in browser` (`#12289`)
- `feat: add auth login diagnostics` (`#13797`) with LTS-safe conflict resolution

### Chat Compatibility Notes

- `response.completed` / `response.done` handling remains explicitly supported in SSE.
- `invalid_prompt` stays non-retryable and mapped to `InvalidRequest` in LTS behavior.
- Multi-tool-call chat aggregation behavior is present in the 0.80.x base and verified unchanged.
- chat/tool-call compatibility was hardened for third-party coding providers that require strict tool-call sequencing and JSON argument formatting.

### LTS UX / Presets

- `/plan` and `/code` are available in `tui2` as persistent runtime mode switches.
- LTS now ships curated built-in provider presets for:
  - `alibaba-coding`
  - `zai-coding`
  - `openrouter`
  - `deepseek`
- LTS now ships built-in profiles for common Qwen/GLM coding stacks.
- Qwen/GLM/DeepSeek/Kimi model families now include conservative context-window and auto-compact defaults for better long-session behavior.

### Security Notes

- OAuth/login error surfacing improved for diagnostics and support triage.
- Login flow now emits dedicated diagnostics (`codex-login.log`) with minimal UX impact.

### Not Backported

- Newer `network-proxy` hardening commits from latest were **not** backported because this LTS line
  does not include the same proxy architecture/files as upstream latest, so direct cherry-pick is
  not applicable without invasive subsystem uplift.

### Packaging / Distribution

- Package version bumped to `0.80.5-lts`.
- LTS npm package targets Termux ARM64 + Linux x64/arm64 + macOS x64/arm64.
- macOS npm packaging is built through the dedicated GitHub workflow.
- Homebrew tap initialized at `https://github.com/DioNanos/homebrew-codex-lts` (formula `codex-lts`).

### Documentation / Test Policy

- Added repo docs for release lines, packaging, testing, `/plan`, context/compact behavior, and LTS provider presets.
- LTS human validation is now split into `Core` and `Extended` suites per platform.
