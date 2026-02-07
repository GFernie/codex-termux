# LTS Test Suite (Linux)

Purpose: validate an installed LTS build (both `codex` and `codex-exec`) on Linux.

WARNING: This release may ship with incomplete re-validation. Run this suite
before relying on it in production.

## Command Selection

This suite assumes you have a shell function/alias in `~/.zshrc` that selects
your preferred provider/profile for the `codex` CLI. The example wrapper name
used below is `codex-glm-a`. Adjust to whatever you actually use.

Verify commands resolve:

```bash
command -v codex-glm-a
command -v codex-exec

# Optional: if you also wrap codex-exec via ~/.zshrc, keep using it.
command -v codex-glm-a-exec || true
```

## Version Family Guard (Required)

Both `codex` and `codex-exec` must report an `-lts` version:

```bash
codex-glm-a --version
codex-glm-a --version | rg --fixed-strings "-lts"

codex-exec --version
codex-exec --version | rg --fixed-strings "-lts"
```

## Basic Functionality

Help/usage:

```bash
codex-glm-a --help
codex-glm-a exec --help
codex-exec --help
```

Non-interactive sanity (no secrets):

```bash
codex-exec --json "print working directory and list files"
codex-exec --json "create a file named hello.txt with content 'hello' and then read it"
```

File operations:

```bash
tmpdir="$(mktemp -d)"
cd "$tmpdir"
printf "a\nb\nc\n" > a.txt
codex-exec --json "count lines in a.txt and write the count to out.txt"
cat out.txt
```

## Update Banner Sanity (Optional)

The updater should not suggest a jump to non-LTS tags.

```bash
codex-glm-a --search --help >/dev/null 2>&1 || true
```

If you see: `0.80.x-lts -> 0.96.0` (or any non-`-lts`), that is a bug.
