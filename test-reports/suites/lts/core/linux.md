# LTS Core Suite (Linux)

Release-blocking checklist for Linux x64.

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

Expected: all required commands resolve and every version is `-lts`.

## LTS-CORE-002 Built-in profile discovery

```bash
codex-lts --profile qwen35-coding --version
codex-lts --profile glm5-coding --version
codex-lts --profile glm47-zai-coding --version
```

Expected: profile lookup succeeds without editing `~/.codex/config.toml`.

## LTS-CORE-003 Basic CLI sanity

```bash
codex-lts --help
codex-lts exec --help
codex-lts-exec --help
```

Expected: help output renders normally.

## LTS-CORE-004 Non-interactive exec sanity

```bash
codex-lts-exec --json "print working directory and list files"
codex-lts-exec --json "create hello.txt with content 'hello' and then read it"
```

Expected: valid JSON output, no protocol or tool-call errors.

## LTS-CORE-005 `/chat` responsiveness

Run both:

```bash
codex-qwen35
codex-glm5
```

In each TUI session:

1. Ask `say hello, print current directory, then stop`
2. Ask a second follow-up in the same chat

Expected:

- first reply arrives without silent blank-line no-op behavior
- second reply also arrives normally
- no tool-call/JSON/role warnings

## LTS-CORE-006 `/plan` and `/code`

Inside both `codex-qwen35` and `codex-glm5`:

1. run `/plan`
2. ask `create a plan to add a troubleshooting section`
3. ask a second planning follow-up
4. run `/code`
5. ask for a normal coding task
6. run `/plan add a small troubleshooting note for linux install`

Expected:

- `/plan` is recognized
- planning persists until `/code`
- normal coding mode returns after `/code`
- no fake prompt text is shown visibly

## LTS-CORE-007 Tool mutation round-trip

```bash
tmpdir="$(mktemp -d)"
cd "$tmpdir"
codex-lts
```

Inside the TUI:

1. `list files in this directory`
2. `create hello.txt containing hello from linux lts`
3. `read hello.txt`
4. `rename hello.txt to hello-renamed.txt and show its contents`

Expected: tool use and mutation work end-to-end without malformed tool-call warnings.

## LTS-CORE-008 Updater channel sanity

```bash
codex-lts --search --help >/dev/null 2>&1 || true
```

Expected: no suggested jump from `-lts` to a non-LTS version.
