# LTS Model Suite (Linux): Qwen 3.5

Profile under test:

- `qwen35-coding`

This suite is stricter on malformed tool behavior because Qwen is more sensitive to broad prompts.

## MQ35-LNX-001 Profile resolution

```bash
codex-lts --profile qwen35-coding --version
```

Expected:

- profile resolves without local config edits
- version remains `-lts`

## MQ35-LNX-002 Two-turn chat stability

Run:

```bash
codex-qwen35
```

Inside the TUI:

1. ask `say hello, print current directory, then stop`
2. ask `now list up to five files here and stop`

Expected:

- both turns answer normally
- no silent blank-line behavior
- no malformed tool-call warnings

## MQ35-LNX-003 Minimal shell and file smoke

In a fresh session ask:

`print the current directory, list files, create qwen-smoke.txt with content qwen smoke ok, read it back, then stop`

Expected:

- shell command path works
- file is created and read correctly

## MQ35-LNX-004 Inspect/search/read

Use a disposable workspace with multiple small files.

Ask:

`inspect this workspace, list files, read the relevant files, and summarize the workspace`

Expected:

- no tool-call derail
- no orphaned or duplicated tool-call state

## MQ35-LNX-005 Single-file edit

Ask:

`update README.md to add one short troubleshooting bullet and then show the result`

Expected:

- edit succeeds in full sandbox
- read-only failure mode is explicit if you intentionally test it there

## MQ35-LNX-006 Multi-file edit

Ask:

`change src/app.ts so answer becomes 42, update docs/notes.md to match, and summarize the diff`

Expected:

- Qwen completes the coordinated edit
- final answer reflects the actual changes

## MQ35-LNX-007 MCP memory argument discipline

Only if MCP memory is configured:

Ask:

`read one memory category, summarize it briefly, then stop`

Expected:

- no empty `memory_read()` or missing-category call
- if the tool fails, the error is explicit and recoverable

## MQ35-LNX-008 `/plan` and `/code`

Inside the same session:

1. run `/plan`
2. ask for a short implementation plan
3. run `/code`
4. ask for a tiny file edit

Expected:

- `/plan` works as persistent mode
- `/code` returns to normal execution
- no provider role error appears

## MQ35-LNX-009 Failure recovery

Trigger:

- `/diff` outside git
- missing-file read request
- optional MCP unavailable path

Expected:

- explicit error
- next prompt still works
