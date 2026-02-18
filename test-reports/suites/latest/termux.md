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
# NOTE: Recent upstream builds can refuse to run outside a trusted directory.
# If you see: "Not inside a trusted directory and --skip-git-repo-check was not specified."
# rerun with --skip-git-repo-check (as below), or run inside a trusted git repo.
# NOTE: Default sandbox can be read-only; use workspace-write so file creation checks work.
codex-exec --sandbox workspace-write --skip-git-repo-check --json "print current directory and list files"
codex-exec --sandbox workspace-write --skip-git-repo-check --json "create hello.txt with content 'hello' and then read it"
```

## v0.104.0 Regression Guard (Android network policy stub)

Binary architecture guard:

```bash
file "$(command -v codex)"
file "$(command -v codex-exec)"
```

Expected:
- both binaries are Linux/Android ELF
- architecture is `aarch64`/`ARM64`

Network-path smoke (must not panic):

```bash
codex-exec --sandbox workspace-write --skip-git-repo-check --json \
  "run one network check with curl -I https://example.com and report the first HTTP status line only"
```

Expected:
- no crash/panic
- no errors referencing missing network policy symbols (for example
  `NetworkDecision::ask`, `NetworkDecision::deny`, or `BlockedRequest.decision`)
- command may succeed or be blocked by policy, but failure must be graceful

Maintainer-only compile guard (optional, from source repo):

```bash
cd ~/Dev/codex-termux/codex-rs
cargo check -p codex-network-proxy --target aarch64-linux-android
cargo check -p codex-core --target aarch64-linux-android
cargo check -p codex-cli --target aarch64-linux-android
```

Expected:
- all checks complete without compile errors

Termux checks:

```bash
uname -a
echo "$PREFIX"
node --version
npm --version
command -v termux-open-url || true
```
