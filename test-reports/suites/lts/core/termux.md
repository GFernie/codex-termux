# LTS Core Suite (Termux)

Release-blocking checklist for Android ARM64 / Termux.

## LTS-CORE-001 Version and wrapper identity

```bash
command -v codex-lts
command -v codex-lts-exec
command -v codex-qwen35
command -v codex-glm5
codex-lts --version
codex-lts --version | rg --fixed-strings "-lts"
codex-lts-exec --version
codex-lts-exec --version | rg --fixed-strings "-lts"
codex-qwen35 --version
codex-glm5 --version
```

## LTS-CORE-002 Built-in profile discovery

```bash
codex-lts --profile qwen35-coding --version
codex-lts --profile glm5-coding --version
codex-lts --profile glm47-zai-coding --version
```

## LTS-CORE-003 Termux environment

```bash
uname -a
echo "$PREFIX"
echo "$HOME"
node --version
npm --version
```

Expected: capture environment for the release report.

## LTS-CORE-004 Non-interactive exec sanity

```bash
codex-lts-exec --json "print current directory and list files"
codex-lts-exec --json "create a file hello.txt with content 'hello' and then read it"
```

## LTS-CORE-005 `/chat` responsiveness

Run both:

```bash
codex-qwen35
codex-glm5
```

In each TUI session:

1. ask `say hello, print current directory, then stop`
2. ask a second follow-up

Expected:

- no long blank-line stall after pressing Enter
- no multi-minute first-token delay beyond normal provider latency
- no invalid JSON / tool-call / role warnings

## LTS-CORE-006 `/plan` and `/code`

Inside both `codex-qwen35` and `codex-glm5`:

1. `/plan`
2. `plan how to add a troubleshooting note for termux install`
3. second planning follow-up
4. `/code`
5. normal coding request
6. `/plan investigate startup delay on termux and propose fixes`

Expected: `/plan` persists until `/code` and remains visible as a real slash command.

## LTS-CORE-007 Tool mutation round-trip

```bash
tmpdir="$(mktemp -d)"
cd "$tmpdir"
codex-lts
```

Inside the TUI:

1. `list files in this directory`
2. `create hello.txt containing hello from termux lts`
3. `read hello.txt`
4. `rename hello.txt to hello-renamed.txt and show its contents`

## LTS-CORE-008 Updater channel sanity

```bash
codex-lts --version
cat ~/.config/codex/version.json 2>/dev/null || true
```

Expected: no non-LTS update jump is suggested.
