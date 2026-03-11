# codex-termux

Stability-first fork of OpenAI Codex CLI with two maintained release lines:

- `main`: latest Termux-focused line (`@mmmbuto/codex-cli-termux`)
- `lts`: long-term support line based on upstream `0.80.x` (`@mmmbuto/codex-cli-lts`)

[![npm lts](https://img.shields.io/npm/v/@mmmbuto/codex-cli-lts?style=flat-square&logo=npm)](https://www.npmjs.org/package/@mmmbuto/codex-cli-lts)
[![license](https://img.shields.io/badge/license-Apache--2.0-blue?style=flat-square)](./LICENSE)

## What LTS Optimizes For

- preserve `/chat` usability and avoid silent stalls
- keep tool and file-mutation flows compatible on third-party coding providers
- provide stable Termux/Linux/macOS arm64 packaging
- prefer conservative backports and predictable behavior over feature churn

## Release Lines

- Release line guide: [`docs/release-lines.md`](./docs/release-lines.md)
- Latest changelog: [`CHANGELOG.md`](./CHANGELOG.md)
- LTS changelog: [`CHANGELOG_LTS.md`](./CHANGELOG_LTS.md)

## Distribution Matrix

### Latest (`main`)

- npm: Termux / Android ARM64

### LTS (`lts`)

- npm: Termux / Android ARM64
- npm: Linux x64
- npm: macOS arm64
- GitHub Actions builds the macOS arm64 artifact used by the final npm package
- Homebrew tap can be used as an additional distribution channel, not as the source of truth for npm packaging

Build and packaging details live in:

- [`BUILDING.md`](./BUILDING.md)
- [`docs/install.md`](./docs/install.md)
- [`docs/lts-packaging.md`](./docs/lts-packaging.md)

## LTS Built-In Provider Presets

LTS now ships curated built-in providers and profiles for common coding stacks, so you can start with `--profile` instead of hand-writing every provider stanza.

Built-in providers:

- `alibaba-coding`
- `zai-coding`
- `openrouter`
- `deepseek`

Built-in profiles:

- `qwen35-coding`
- `qwen35-plan`
- `qwen3-coder-plus`
- `qwen3-coder-next`
- `glm5-coding`
- `glm5-plan`
- `glm5-zai-coding`
- `glm47-zai-coding`
- `openrouter-qwen`
- `deepseek-coding`

Provider and profile details:

- [`docs/lts-provider-presets.md`](./docs/lts-provider-presets.md)
- [`docs/lts-context-and-compact.md`](./docs/lts-context-and-compact.md)

## LTS `/plan`

LTS backports a persistent Plan mode adapted to the `0.80.x` codebase:

- `/plan` enters planning mode
- `/plan <request>` enters planning mode and submits the request
- `/code` returns to normal coding mode

Behavior notes:

- it is a real runtime mode switch, not fake prompt text injected into chat
- it is designed to avoid regressing normal `/chat` behavior

Details:

- [`docs/lts-plan-mode.md`](./docs/lts-plan-mode.md)
- [`docs/slash_commands.md`](./docs/slash_commands.md)

## Testing

LTS validation is split into:

- `Core`: release-blocking
- `Extended`: broader regression coverage

Suites and reports:

- [`docs/lts-testing.md`](./docs/lts-testing.md)
- [`test-reports/README.md`](./test-reports/README.md)
- [`test-reports/suites/README.md`](./test-reports/suites/README.md)

## Repo Docs

- Config: [`docs/configuration.md`](./docs/configuration.md)
- Install: [`docs/install.md`](./docs/install.md)
- Build: [`BUILDING.md`](./BUILDING.md)
- Slash commands: [`docs/slash_commands.md`](./docs/slash_commands.md)
- Provider presets: [`docs/lts-provider-presets.md`](./docs/lts-provider-presets.md)
- Context and compact: [`docs/lts-context-and-compact.md`](./docs/lts-context-and-compact.md)
- Test policy: [`docs/lts-testing.md`](./docs/lts-testing.md)
