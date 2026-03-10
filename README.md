# codex-termux LTS

Stability-first packaging of upstream OpenAI Codex for Android Termux and LTS desktop usage.

[![npm lts](https://img.shields.io/npm/v/@mmmbuto/codex-cli-lts?style=flat-square&logo=npm)](https://www.npmjs.org/package/@mmmbuto/codex-cli-lts)
[![license](https://img.shields.io/badge/license-Apache--2.0-blue?style=flat-square)](./LICENSE)

---

## Scope

- Upstream base: `openai/codex` (`rust-v0.80.0` line)
- Branch goal: keep `/chat` compatibility and stable behavior over time
- Backports policy: security and reliability only, no feature drift

## Release Lines

- `main` branch: latest Termux line (`@mmmbuto/codex-cli-termux`)
- `lts` branch: long-term support line (`@mmmbuto/codex-cli-lts`)

## Distribution Channels (LTS)

- Android Termux (ARM64): npm package `@mmmbuto/codex-cli-lts`
- Linux/macOS: primary channel is Homebrew tap `homebrew-codex-lts`
- Linux x64 + macOS arm64 npm artifacts: optional channel built via GitHub Actions

## Install

Termux:

```bash
npm install -g @mmmbuto/codex-cli-lts
```

Verify:

```bash
codex --version
codex exec --help
```

## Documentation

- Configuration: `docs/configuration.md`
- LTS changelog: `CHANGELOG_LTS.md`
- Patch notes: `patches/README.md`
- Build notes: `BUILDING.md`
- Test reports: `test-reports/`

## Compatibility Rules

- Preserve upstream behavior whenever possible
- Keep Termux compatibility non-regressive
- Treat `/chat` as release-blocking for LTS
