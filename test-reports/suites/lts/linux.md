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
- `codex-deepseek` (recommended)
- `codex-kimi` (recommended)

Verify commands resolve:

```bash
command -v codex-lts
command -v codex-lts-exec
command -v codex-qwen35
command -v codex-glm5
command -v codex-deepseek || true
command -v codex-kimi || true
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
codex-deepseek --version || true
codex-kimi --version || true
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

## Interactive TUI: `/chat` Stability (Required)

Start with a provider alias, preferably:

```bash
codex-qwen35
```

Then verify inside the TUI:

1. Type a normal request in `/chat` style such as:
   `say hello, state current directory, then stop`
2. Confirm the model replies without hanging for minutes, without silent blank lines,
   and without protocol/tool-call warnings.
3. Send a second follow-up message in the same session.
4. Confirm the second answer also arrives normally.

Repeat the same smoke with:

```bash
codex-glm5
codex-deepseek || true
codex-kimi || true
```

Record if any provider:

- stalls before first token
- prints warnings/errors about tool calls, roles, or invalid JSON
- stops responding after the first turn

## Interactive TUI: `/plan` and `/code` (Required)

Inside `codex-qwen35` and `codex-glm5`:

1. Run `/plan`
2. Ask: `create a plan to add a README section for troubleshooting`
3. Confirm the assistant stays in planning behavior and does not execute edits
4. Ask a second planning follow-up without re-running `/plan`
5. Confirm Plan mode persists across turns
6. Run `/code`
7. Ask the same kind of request again
8. Confirm normal coding behavior is restored

Also verify inline args:

```text
/plan add a small troubleshooting section for linux install issues
```

Expected:

- `/plan` is recognized
- planning mode persists until `/code`
- no fake prompt text is injected visibly into the chat
- no slash-command regressions on `/compact`, `/diff`, `/review`

## Tools and File Mutation (Required)

Use a disposable directory:

```bash
tmpdir="$(mktemp -d)"
cd "$tmpdir"
codex-lts
```

Inside the TUI, ask:

1. `list files in this directory`
2. `create hello.txt containing hello from lts`
3. `read hello.txt`
4. `rename hello.txt to hello-renamed.txt and show its contents`

Expected:

- tool use works end-to-end
- file mutation succeeds without malformed tool-call warnings
- resulting file contents are correct
- no `/chat` freeze after tool execution

## Update Banner Sanity (Optional)

The updater should not suggest a jump to non-LTS tags.

```bash
codex-lts --search --help >/dev/null 2>&1 || true
```

If you see: `0.80.x-lts -> 0.96.0` (or any non-`-lts`), that is a bug.
