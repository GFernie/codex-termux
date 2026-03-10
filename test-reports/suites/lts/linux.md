# LTS Test Suite (Linux)

Purpose: validate an installed LTS build (both `codex` and `codex-exec`) on Linux.

WARNING: This release may ship with incomplete re-validation. Run this suite
before relying on it in production.

## Command Selection

This suite assumes you have dedicated LTS commands in `~/.zshrc`:

- `codex-lts`
- `codex-lts-exec`
- `codex-qwen35`
- `codex-glm5`

Verify commands resolve:

```bash
command -v codex-lts
command -v codex-lts-exec
command -v codex-qwen35
command -v codex-glm5
```

## Version Family Guard (Required)

Both LTS binaries must report an `-lts` version:

```bash
codex-lts --version
codex-lts --version | rg --fixed-strings "-lts"

codex-lts-exec --version
codex-lts-exec --version | rg --fixed-strings "-lts"

# Provider alias smoke
codex-qwen35 --version
codex-glm5 --version
```

## Basic Functionality

Help/usage:

```bash
codex-lts --help
codex-lts exec --help
codex-lts-exec --help
```

Non-interactive sanity (no secrets):

```bash
codex-lts-exec --json "print working directory and list files"
codex-lts-exec --json "create a file named hello.txt with content 'hello' and then read it"
```

File operations:

```bash
tmpdir="$(mktemp -d)"
cd "$tmpdir"
printf "a\nb\nc\n" > a.txt
codex-lts-exec --json "count lines in a.txt and write the count to out.txt"
cat out.txt
```

## Update Banner Sanity (Optional)

The updater should not suggest a jump to non-LTS tags.

```bash
codex-lts --search --help >/dev/null 2>&1 || true
```

If you see: `0.80.x-lts -> 0.96.0` (or any non-`-lts`), that is a bug.
