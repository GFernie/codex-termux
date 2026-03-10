# @mmmbuto/codex-cli-lts

LTS packaging of upstream OpenAI Codex for stable Termux and desktop usage.

[![npm lts](https://img.shields.io/npm/v/@mmmbuto/codex-cli-lts?style=flat-square&logo=npm)](https://www.npmjs.org/package/@mmmbuto/codex-cli-lts)

---

## Supported Targets

- Android Termux (`android-arm64`)
- Linux (`linux-x64`)
- macOS (`darwin-arm64`)

This package exposes:

- `codex` (interactive TUI)
- `codex-exec` (automation, e.g. `codex exec --json ...`)

## Install

```bash
npm install -g @mmmbuto/codex-cli-lts
```

## Verify

```bash
codex --version
codex exec --help
```

## Policy

- LTS line is stability-focused
- Backports are restricted to reliability/security fixes
- `/chat` compatibility is treated as release-blocking

Full docs, test reports, and patch history are in the GitHub repository.
