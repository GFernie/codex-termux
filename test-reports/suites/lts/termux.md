# LTS Test Suite (Termux / Android ARM64)

Purpose: validate an installed LTS build on Termux, using your `~/.zshrc` wrappers
for both `codex` and `codex-exec`.

WARNING: This release may ship with incomplete re-validation. Run this suite
before relying on it in production.

## Command Selection

This suite assumes you have a shell function/alias in `~/.zshrc` that selects
your preferred provider/profile for the `codex` CLI. The example wrapper name
used below is `codex-glm-a`. Adjust to whatever you actually use.

Verify:

```bash
command -v codex-glm-a
command -v codex-exec

# Optional: if you also wrap codex-exec via ~/.zshrc, keep using it.
command -v codex-glm-a-exec || true
```

## Version Family Guard (Required)

Both must be `-lts`:

```bash
codex-glm-a --version
codex-glm-a --version | rg --fixed-strings "-lts"

codex-exec --version
codex-exec --version | rg --fixed-strings "-lts"
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
codex-glm-a --help
codex-glm-a exec --help
codex-exec --help
```

Non-interactive sanity:

```bash
codex-exec --json "print current directory and list files"
codex-exec --json "create a file hello.txt with content 'hello' and then read it"
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
codex-glm-a --version
cat ~/.config/codex/version.json 2>/dev/null || true
```

and record it in a test report.
