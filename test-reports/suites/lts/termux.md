# LTS Test Suite (Termux / Android ARM64)

Purpose: validate an installed LTS build on Termux, using your `~/.zshrc` wrappers
for both `codex` and `codex-exec`.

WARNING: This release may ship with incomplete re-validation. Run this suite
before relying on it in production.

## Command Selection

This suite assumes dedicated LTS commands in `~/.zshrc`:

- `codex-lts`
- `codex-lts-exec`
- `codex-qwen35`
- `codex-glm5`
- `codex-deepseek` (recommended)
- `codex-kimi` (recommended)

Verify:

```bash
command -v codex-lts
command -v codex-lts-exec
command -v codex-qwen35
command -v codex-glm5
command -v codex-deepseek || true
command -v codex-kimi || true
```

## Version Family Guard (Required)

Both LTS binaries must be `-lts`:

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

## Termux Environment

```bash
uname -a
echo "$PREFIX"
echo "$HOME"
node --version
npm --version
```

## Core Tests

Help:

```bash
codex-lts --help
codex-lts exec --help
codex-lts-exec --help
```

Non-interactive sanity:

```bash
codex-lts-exec --json "print current directory and list files"
codex-lts-exec --json "create a file hello.txt with content 'hello' and then read it"
```

Termux-specific checks (optional but useful):

```bash
command -v termux-open-url || true
command -v termux-setup-storage || true
ls -la /data/data/com.termux/files/usr || true
```

## Interactive TUI: `/chat` Stability (Required)

Prefer starting with:

```bash
codex-qwen35
```

Then inside the TUI:

1. Send a normal request like:
   `say hello, print current directory, then stop`
2. Confirm the model responds normally and does not only insert blank lines
3. Send a second follow-up message in the same session
4. Confirm the session remains responsive

Repeat with:

```bash
codex-glm5
codex-deepseek || true
codex-kimi || true
```

Record specifically if Termux shows:

- multi-minute delay before first answer
- silent no-op behavior after pressing Enter
- warnings/errors about tool calls, roles, or invalid JSON
- breakage only after the first reply

## Interactive TUI: `/plan` and `/code` (Required)

Inside `codex-qwen35` and `codex-glm5`:

1. Run `/plan`
2. Ask: `plan how to add a troubleshooting note for termux install`
3. Confirm the assistant remains in planning mode and does not edit files
4. Ask one more planning follow-up
5. Confirm Plan mode persists
6. Run `/code`
7. Ask for an executable coding task
8. Confirm normal coding mode is restored

Also verify inline args:

```text
/plan investigate startup delay on termux and propose fixes
```

Expected:

- `/plan` is recognized in TUI
- plan mode persists across turns until `/code`
- no visible fake prompt injection
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
2. `create hello.txt containing hello from termux lts`
3. `read hello.txt`
4. `rename hello.txt to hello-renamed.txt and show its contents`

Expected:

- tool calls execute correctly
- file changes are applied correctly
- no malformed tool-call warnings
- no freeze after a tool-using turn

## Update Channel Guard

If you see an update jump to a non-LTS version (for example `0.80.4-lts -> 0.96.0`),
capture:

```bash
codex-lts --version
cat ~/.config/codex/version.json 2>/dev/null || true
```

and record it in a test report.
