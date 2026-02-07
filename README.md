# codex-termux

Repackaged builds of the upstream Codex CLI with minimal compatibility patches
for Android Termux.

## Release Lines

- **Latest (Termux-only)**: `@mmmbuto/codex-cli-termux`
- **LTS (stability-focused)**: `@mmmbuto/codex-cli-lts`

Both packages expose:

- `codex` (interactive TUI)
- `codex-exec` (automation, e.g. `codex exec --json ...`)

## Install

Latest (Termux):

```bash
npm install -g @mmmbuto/codex-cli-termux
```

LTS (Linux x64 + Termux ARM64 via npm; macOS arm64 is distributed via CI artifacts/releases when available):

```bash
npm install -g @mmmbuto/codex-cli-lts
```

Verify:

```bash
codex --version
codex exec --help
```

## Configuration (Provider-Neutral)

All configuration guidance is kept in one place:

- `docs/configuration.md`

Note: when reading this file on npm, the `docs/` and `test-reports/` folders are
in the GitHub repository (see the `repository` link on the package page).

## Test Reports

Human-run validation reports live under:

- `test-reports/`

## Patches And Building

- Termux patch notes: `patches/`
- Build notes: `BUILDING.md`

## Notes

- This repo aims to preserve upstream behavior. Patches are limited to platform
  compatibility and safety fixes, with documentation under `patches/` and
  `docs/`.
