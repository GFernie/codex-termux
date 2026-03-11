# LTS Core Suite (macOS arm64)

Release-blocking checklist for macOS arm64. This suite validates the final npm package/artifact behavior on Apple Silicon and should be run against the GitHub Actions-produced macOS binary/package.

## Required providers

Run all provider-dependent checks with:

- `codex-qwen35`
- `codex-glm5`

If one provider fails, Core fails.

## LTS-CORE-MAC-001 Version, wrapper, and package identity

```bash
command -v codex-lts
command -v codex-lts-exec
command -v codex-qwen35
command -v codex-glm5
whence -v codex-lts
whence -v codex-qwen35
whence -v codex-glm5
codex-lts --version
codex-lts-exec --version
codex-qwen35 --version
codex-glm5 --version
codex-lts --version | rg --fixed-strings "-lts"
codex-lts-exec --version | rg --fixed-strings "-lts"
```

Expected:

- installed binary is the LTS build
- nothing resolves to upstream `@openai/codex`

## LTS-CORE-MAC-002 Built-in profile discovery

```bash
codex-lts --profile qwen35-coding --version
codex-lts --profile qwen35-plan --version
codex-lts --profile glm5-coding --version
codex-lts --profile glm5-plan --version
codex-lts --profile glm47-zai-coding --version
```

Expected: built-in profiles resolve without config edits.

## LTS-CORE-MAC-003 CLI help and feature sanity

```bash
codex-lts --help
codex-lts exec --help
codex-lts-exec --help
codex-lts features list
```

Expected:

- help renders correctly
- `tui2` is enabled for the tested LTS wrapper

## LTS-CORE-MAC-004 Non-interactive exec sanity

```bash
codex-lts-exec --json "print working directory and list files"
codex-lts-exec --json "create hello.txt with content 'hello from mac core' and then read it"
```

Expected:

- valid JSON output
- no malformed tool-call or invalid-arguments error

## LTS-CORE-MAC-005 Slash palette baseline

Run:

```bash
codex-lts
```

Inside the TUI:

1. type `/`
2. verify the popup includes:
   - `/model`
   - `/approvals`
   - `/setup-elevated-sandbox`
   - `/skills`
   - `/review`
   - `/new`
   - `/resume`
   - `/init`
   - `/compact`
   - `/plan`
   - `/code`
   - `/diff`
   - `/mention`
   - `/status`
   - `/mcp`
   - `/logout`
   - `/feedback`

Expected: all visible commands appear and the UI remains stable.

## LTS-CORE-MAC-006 `/chat` responsiveness

Run both:

```bash
codex-qwen35
codex-glm5
```

In each TUI session:

1. ask `say hello, print current directory, then stop`
2. ask `now list a few files here and stop`

Expected:

- no hang
- no blank-line no-op behavior
- no protocol warnings

## LTS-CORE-MAC-007 `/plan` and `/code`

Inside both `codex-qwen35` and `codex-glm5`:

1. run `/plan`
2. ask `plan how to add a troubleshooting note for macOS install`
3. ask a second planning follow-up
4. run `/code`
5. ask `now make a tiny markdown edit in a disposable file`

Expected:

- persistent Plan mode until `/code`
- no provider error involving the `developer` role

## LTS-CORE-MAC-008 Automatic workspace exploration

```bash
tmpdir="$(mktemp -d)"
cd "$tmpdir"
mkdir -p src docs
printf 'hello mac\n' > README.md
printf 'export const answer = 41;\n' > src/app.ts
printf 'apple silicon notes\n' > docs/notes.md
codex-lts
```

Inside the TUI, ask:

`inspect this workspace, list files, read the relevant files, find where answer is defined, and summarize what should change`

Expected: inspect/search/read flow works cleanly.

## LTS-CORE-MAC-009 Automatic file mutation round-trip

Reuse the disposable workspace. Ask:

`change src/app.ts so answer becomes 42, add a short note to docs/notes.md, then show me what changed`

Expected:

- multi-file mutation succeeds
- on-disk files match the reply

## LTS-CORE-MAC-010 Git-aware `/diff` and `/review`

```bash
tmprepo="$(mktemp -d)"
cd "$tmprepo"
git init
printf 'one\n' > demo.txt
git add demo.txt
git commit -m 'init'
printf 'two\n' >> demo.txt
codex-lts
```

Inside the TUI:

1. run `/diff`
2. run `/review`

Expected: diff/review work and remain stable in the session.

## LTS-CORE-MAC-011 Updater and package sanity

```bash
codex-lts --version
env | rg '^CODEX_MANAGED_BY_NPM=' || true
cat ~/.config/codex/version.json 2>/dev/null || true
```

Expected: updater remains on LTS and does not suggest upstream/latest.

## LTS-CORE-MAC-012 macOS-specific capture

```bash
uname -a
uname -m
sw_vers
node --version
npm --version
```

Expected: capture environment and confirm Apple Silicon target.
