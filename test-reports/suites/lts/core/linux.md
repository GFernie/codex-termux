# LTS Core Suite (Linux x64)

Release-blocking checklist for Linux x64. This suite is intentionally strict: it should prove that the installed LTS binary can chat, plan, inspect a workspace, mutate files, survive normal operator flows, and stay on the LTS line.

## Required providers

Run all provider-dependent checks with:

- `codex-qwen35`
- `codex-glm5`

If one provider fails, Core fails.

## LTS-CORE-LNX-001 Version, wrapper, and package identity

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

- every command resolves
- every version string is `-lts`
- wrappers point to the intended LTS install tree, not upstream `@openai/codex`

## LTS-CORE-LNX-002 Built-in profile discovery

```bash
codex-lts --profile qwen35-coding --version
codex-lts --profile qwen35-plan --version
codex-lts --profile glm5-coding --version
codex-lts --profile glm5-plan --version
codex-lts --profile glm47-zai-coding --version
```

Expected: profile lookup succeeds without local config edits.

## LTS-CORE-LNX-003 CLI help and feature sanity

```bash
codex-lts --help
codex-lts exec --help
codex-lts-exec --help
codex-lts features list
```

Expected:

- help output renders normally
- `tui2` is enabled for the tested LTS wrapper

## LTS-CORE-LNX-004 Non-interactive exec sanity

```bash
codex-lts-exec --json "print working directory and list files"
codex-lts-exec --json "create hello.txt with content 'hello from linux core' and then read it"
```

Expected:

- valid JSON output
- no invalid tool-call or malformed arguments errors
- the second task actually creates and reads the file

## LTS-CORE-LNX-005 Slash palette baseline

Run:

```bash
codex-lts
```

Inside the TUI:

1. type `/`
2. verify the command popup includes:
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

- commands are visible and selectable
- `/plan` is present as a real command
- no popup corruption or input glitches appear

## LTS-CORE-LNX-006 `/chat` responsiveness

Run both:

```bash
codex-qwen35
codex-glm5
```

In each TUI session:

1. ask `say hello, print current directory, then stop`
2. ask `now list up to five files here and stop`

Expected:

- first reply arrives normally
- second turn also works in the same session
- no silent blank-line no-op behavior
- no JSON, tool-call, or role warnings

## LTS-CORE-LNX-007 `/plan` and `/code` persistence

Inside both `codex-qwen35` and `codex-glm5`:

1. run `/plan`
2. ask `create a plan to add a troubleshooting section for linux install`
3. ask a second planning follow-up
4. run `/code`
5. ask `now implement a tiny markdown note in a disposable file`
6. run `/plan add one more rollout step for verification`

Expected:

- `/plan` is recognized
- planning persists across multiple turns until `/code`
- `/code` restores normal coding behavior
- no fake user-visible prompt dump appears
- no `developer role` style provider error appears

## LTS-CORE-LNX-008 Automatic workspace exploration

```bash
tmpdir="$(mktemp -d)"
cd "$tmpdir"
mkdir -p src docs
printf 'hello world\n' > README.md
printf 'export const answer = 41;\n' > src/app.ts
printf 'troubleshooting placeholder\n' > docs/notes.md
codex-lts
```

Inside the TUI, ask in one prompt:

`inspect this workspace, list files, read the relevant files, find where answer is defined, and summarize what should change`

Expected:

- the agent completes a realistic inspect/search/read flow
- file listing and file reading work
- search/find behavior is sane
- no stuck tool-call state appears

## LTS-CORE-LNX-009 Automatic file mutation round-trip

Reuse the disposable workspace from the previous test. Ask:

`change src/app.ts so answer becomes 42, add a short note to docs/notes.md, then show me what changed`

Expected:

- multi-file mutation succeeds
- the result is coherent
- the final answer reflects actual file changes
- no malformed patch or argument warning appears

## LTS-CORE-LNX-010 Git-aware `/diff` and `/review`

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

- `/diff` shows a meaningful summary
- `/review` completes and focuses on issues or risk
- neither command crashes outside the normal coding flow

## LTS-CORE-LNX-011 Updater and channel sanity

```bash
codex-lts --version
env | rg '^CODEX_MANAGED_BY_NPM=' || true
cat ~/.config/codex/version.json 2>/dev/null || true
```

Expected:

- nothing suggests switching from `-lts` to latest/upstream
- release metadata, if present, remains aligned to LTS

## LTS-CORE-LNX-012 Linux-specific environment capture

```bash
uname -a
id
pwd
node --version
npm --version
file "$(command -v node)"
```

Expected: capture enough environment details to make failures reproducible in the release report.
