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
command -v codex-deepseek || true
command -v codex-kimi || true
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
codex-deepseek --version || true
codex-kimi --version || true
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

## Interactive TUI: `/chat` Stability (Required)

Prefer:

```bash
codex-qwen35
```

Inside the TUI:

1. Send a normal request such as:
   `say hello, print current directory, then stop`
2. Confirm the first answer arrives without hanging or blank-line no-op behavior
3. Send a second follow-up in the same chat
4. Confirm the session remains responsive

Repeat with:

```bash
codex-glm5
codex-deepseek || true
codex-kimi || true
```

Record if any provider:

- hangs before first answer
- emits invalid tool-call / JSON / role warnings
- stops responding after one turn

## Interactive TUI: `/plan` and `/code` (Required)

Inside `codex-qwen35` and `codex-glm5`:

1. Run `/plan`
2. Ask: `plan how to add a troubleshooting note for macOS install`
3. Confirm planning behavior persists and no edits are executed
4. Ask a second planning follow-up
5. Run `/code`
6. Ask a normal coding request
7. Confirm coding mode is restored

Also verify:

```text
/plan investigate startup issues on mac arm and propose a fix plan
```

Expected:

- `/plan` is recognized
- plan mode persists until `/code`
- no visible fake prompt text
- no regressions on `/compact`, `/diff`, `/review`

## Tools and Mutation Smoke (Required)

Use a disposable directory:

```bash
tmpdir="$(mktemp -d)"
cd "$tmpdir"
codex-lts
```

Inside the TUI, ask:

1. `list files in this directory`
2. `create hello.txt containing hello from mac lts`
3. `read hello.txt`
4. `rename hello.txt to hello-renamed.txt and show its contents`

Expected:

- tool execution works
- file mutation works
- no malformed tool-call warnings
- no freeze after tool usage

## Updater Sanity (Important)

The LTS updater must not suggest upgrades to non-LTS versions such as `0.96.0`.
If it does, capture:

```bash
codex-lts --version
env | rg '^CODEX_MANAGED_BY_NPM=' || true
cat ~/.config/codex/version.json 2>/dev/null || true
```
