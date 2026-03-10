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

Verify:

```bash
command -v codex-lts
command -v codex-lts-exec
command -v codex-qwen35
command -v codex-glm5
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

## Update Channel Guard

If you see an update jump to a non-LTS version (for example `0.80.4-lts -> 0.96.0`),
capture:

```bash
codex-lts --version
cat ~/.config/codex/version.json 2>/dev/null || true
```

and record it in a test report.
