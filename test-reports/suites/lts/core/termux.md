# LTS Core Suite (Termux / Android arm64)

Release-blocking checklist for Termux. The goal is not just correctness, but also real usability on-device: no no-op Enter key behavior, no long unexplained stalls, and no regressions in `/chat` or `/plan`.

## Required providers

Run all provider-dependent checks with:

- `codex-qwen35`
- `codex-glm5`

If one provider fails, Core fails.

## LTS-CORE-TMX-001 Version, wrapper, and package identity

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

- commands resolve inside Termux
- versions are `-lts`
- wrappers point to the intended LTS install

## LTS-CORE-TMX-002 Built-in profile discovery

```bash
codex-lts --profile qwen35-coding --version
codex-lts --profile qwen35-plan --version
codex-lts --profile glm5-coding --version
codex-lts --profile glm5-plan --version
codex-lts --profile glm47-zai-coding --version
```

Expected: built-in profiles resolve without local config edits.

## LTS-CORE-TMX-003 Termux environment capture

```bash
uname -a
echo "$PREFIX"
echo "$HOME"
node --version
npm --version
termux-info 2>/dev/null || true
```

Expected: capture environment details for the release report.

## LTS-CORE-TMX-004 CLI help and feature sanity

```bash
codex-lts --help
codex-lts exec --help
codex-lts-exec --help
codex-lts features list
```

Expected:

- help renders correctly in Termux
- `tui2` is enabled for the tested LTS wrapper

## LTS-CORE-TMX-005 Non-interactive exec sanity

```bash
codex-lts-exec --json "print current directory and list files"
codex-lts-exec --json "create hello.txt with content 'hello from termux core' and then read it"
```

Expected:

- valid JSON output
- no malformed tool-call or invalid-arguments error

## LTS-CORE-TMX-006 Slash palette baseline

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

Expected:

- all visible commands appear
- input remains stable on mobile terminal rendering

## LTS-CORE-TMX-007 `/chat` responsiveness

Run both:

```bash
codex-qwen35
codex-glm5
```

In each TUI session:

1. ask `say hello, print current directory, then stop`
2. ask `now list a few files here and stop`

Expected:

- no blank-line no-op after pressing Enter
- no unexplained multi-minute stall beyond normal provider latency
- no invalid JSON / tool-call / role warnings

## LTS-CORE-TMX-008 `/plan` and `/code`

Inside both `codex-qwen35` and `codex-glm5`:

1. run `/plan`
2. ask `plan how to add a troubleshooting note for termux install`
3. ask a second planning follow-up
4. run `/code`
5. ask `now make a tiny markdown edit in a disposable file`
6. run `/plan investigate startup delay on termux and propose fixes`

Expected:

- `/plan` is recognized
- planning persists until `/code`
- normal coding returns after `/code`
- no provider error mentioning `developer role`

## LTS-CORE-TMX-009 Automatic workspace exploration

```bash
tmpdir="$(mktemp -d)"
cd "$tmpdir"
mkdir -p src docs
printf 'hello termux\n' > README.md
printf 'export const answer = 41;\n' > src/app.ts
printf 'android notes\n' > docs/notes.md
codex-lts
```

Inside the TUI, ask:

`inspect this workspace, list files, read the relevant files, find where answer is defined, and summarize what should change`

Expected:

- the inspect/search/read flow works on-device
- the session stays responsive after tool usage

## LTS-CORE-TMX-010 Automatic file mutation round-trip

Reuse the disposable workspace. Ask:

`change src/app.ts so answer becomes 42, add a short note to docs/notes.md, then show me what changed`

Expected:

- multi-file mutation succeeds
- results match on-disk files
- no patch/tool regression appears

## LTS-CORE-TMX-011 Git-aware `/diff` and `/review`

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

Expected:

- both commands remain usable in Termux
- no crash or input corruption occurs

## LTS-CORE-TMX-012 Updater and Termux-specific sanity

```bash
codex-lts --version
cat ~/.config/codex/version.json 2>/dev/null || true
command -v termux-open-url || true
command -v termux-setup-storage || true
ls -la /data/data/com.termux/files/usr 2>/dev/null || true
```

Expected:

- no non-LTS update path is suggested
- enough Termux-specific context is captured for regression comparison
