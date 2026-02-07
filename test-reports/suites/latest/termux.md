# Latest Test Suite (Termux / Android ARM64)

Purpose: validate the Latest Termux-only build (`@mmmbuto/codex-cli-termux`) using
the global `codex` and `codex-exec` commands.

WARNING: This release may ship with incomplete re-validation. Run this suite
before relying on it in production.

## Install Guard (Required)

Confirm you are testing the Termux package (not upstream):

```bash
npm ls -g --depth=0 @mmmbuto/codex-cli-termux || true
```

Expected: installed version ends with `-termux` (example `0.98.0-termux`).

Confirm the commands you are running are the global ones:

```bash
command -v codex
command -v codex-exec
ls -la "$(command -v codex)" "$(command -v codex-exec)"
```

## Version Guard (Required)

The CLI should report the expected upstream semver line. Depending on upstream,
the `--version` output may be plain semver even when the npm/tag version uses
`-termux`.

```bash
codex --version
codex-exec --version
```

## Core Tests

Workspace:

```bash
rm -rf ~/codex-test-workspace
mkdir -p ~/codex-test-workspace
cd ~/codex-test-workspace
```

Help:

```bash
codex --help
codex exec --help
codex-exec --help
```

Non-interactive sanity:

```bash
codex-exec --json "print current directory and list files"
codex-exec --json "create hello.txt with content 'hello' and then read it"
```

Termux checks:

```bash
uname -a
echo "$PREFIX"
node --version
npm --version
command -v termux-open-url || true
```
