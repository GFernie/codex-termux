# LTS Core Suite (macOS arm64)

Release-blocking checklist for macOS arm64.

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

Expected: installed binary is the LTS build, not upstream `@openai/codex`.

## LTS-CORE-002 Built-in profile discovery

```bash
codex-lts --profile qwen35-coding --version
codex-lts --profile glm5-coding --version
codex-lts --profile glm47-zai-coding --version
```

## LTS-CORE-003 Basic CLI sanity

```bash
codex-lts --help
codex-lts exec --help
codex-lts-exec --help
```

## LTS-CORE-004 Non-interactive exec sanity

```bash
codex-lts-exec --json "print working directory and list files"
codex-lts-exec --json "create hello.txt with content 'hello' and then read it"
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

Expected: no hang, no blank-line no-op behavior, no protocol warnings.

## LTS-CORE-006 `/plan` and `/code`

Inside both `codex-qwen35` and `codex-glm5`:

1. `/plan`
2. `plan how to add a troubleshooting note for macOS install`
3. second planning follow-up
4. `/code`
5. normal coding request

Expected: persistent Plan mode until `/code`.

## LTS-CORE-007 Tool mutation round-trip

```bash
tmpdir="$(mktemp -d)"
cd "$tmpdir"
codex-lts
```

Inside the TUI:

1. `list files in this directory`
2. `create hello.txt containing hello from mac lts`
3. `read hello.txt`
4. `rename hello.txt to hello-renamed.txt and show its contents`

## LTS-CORE-008 Updater channel sanity

```bash
codex-lts --version
env | rg '^CODEX_MANAGED_BY_NPM=' || true
cat ~/.config/codex/version.json 2>/dev/null || true
```

Expected: updater remains on LTS.
