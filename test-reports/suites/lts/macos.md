# LTS Test Suite (macOS arm64)

Purpose: validate an LTS build on macOS arm64 using dedicated LTS wrappers.

WARNING: This release may ship with incomplete re-validation. Run this suite
before relying on it in production.

This suite is written to avoid provider-specific instructions. It only assumes
your wrappers set whatever environment you need and then run the binaries.

## Command Selection

```bash
command -v codex-lts
command -v codex-lts-exec
command -v codex-qwen35
command -v codex-glm5
```

## Version Family Guard (Required)

Both must be `-lts`:

```bash
codex-lts --version
codex-lts --version | rg --fixed-strings "-lts"

codex-lts-exec --version
codex-lts-exec --version | rg --fixed-strings "-lts"

# Provider alias smoke
codex-qwen35 --version
codex-glm5 --version
```

If either command reports `0.96.0` and references `@openai/codex`, you are testing
the upstream Homebrew/npm package, not this repo's LTS.

## Basic Functionality

```bash
codex-lts --help
codex-lts exec --help
codex-lts-exec --help
```

Non-interactive sanity:

```bash
codex-lts-exec --json "print working directory and list files"
codex-lts-exec --json "create hello.txt with content 'hello' and then read it"
```

## Updater Sanity (Important)

The LTS updater must not suggest upgrades to non-LTS versions such as `0.96.0`.
If it does, capture:

```bash
codex-lts --version
env | rg '^CODEX_MANAGED_BY_NPM=' || true
cat ~/.config/codex/version.json 2>/dev/null || true
```
