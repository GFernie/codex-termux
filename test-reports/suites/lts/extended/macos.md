# LTS Extended Suite (macOS arm64)

Broader regression suite for macOS arm64 after Core passes. This suite should validate the packaged LTS experience from the GitHub Actions-produced macOS artifact.

## LTS-EXT-MAC-001 Complete slash command matrix

Inside `codex-lts`, verify each visible slash command individually:

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

- every command is recognized
- every related UI opens cleanly

## LTS-EXT-MAC-002 `model`, `approvals`, and status surfaces

Inside one session:

1. run `/model`
2. run `/approvals`
3. run `/status`

Expected: modals/panels render and return control cleanly.

## LTS-EXT-MAC-003 Session lifecycle

In a disposable workspace:

1. start a session and send one prompt
2. run `/new`
3. send another prompt
4. run `/resume`
5. reopen the earlier session if available
6. run `/logout` only if auth behavior is in scope

Expected:

- lifecycle flows work
- no broken session state remains

## LTS-EXT-MAC-004 `init`, `mention`, and `skills`

In a disposable workspace:

1. run `/init`
2. confirm `AGENTS.md` exists
3. use `/mention` on a workspace file
4. open `/skills`
5. ask the model to use the mentioned file in its answer

Expected: file mention and AGENTS flow work correctly.

## LTS-EXT-MAC-005 Long-session `/compact`

Create a longer session with multiple turns and file operations, then run `/compact`.

Expected:

- compaction succeeds
- later turns still work
- `/plan` and `/code` still behave correctly afterwards

## LTS-EXT-MAC-006 Tool family: inspect/search/read

In a workspace with several files, ask the model to:

- list files
- search for a symbol
- read the relevant files
- summarize findings without editing

Expected: realistic inspect workflow succeeds without tool-call regressions.

## LTS-EXT-MAC-007 Tool family: shell execution and output capture

Ask for a task that clearly requires shell output, for example:

`print the current directory, show the active git branch if any, and summarize the result`

Expected:

- shell execution works
- the answer reflects real command output
- the session remains stable afterwards

## LTS-EXT-MAC-008 Tool family: one-file and multi-file edits

Run:

1. one-file documentation edit
2. two-file coordinated code + docs edit

Expected:

- edits succeed
- follow-up reads match the real file contents

## LTS-EXT-MAC-009 Git-aware behavior

Check:

1. `/diff` outside git
2. `/diff` and `/review` inside a disposable git repo

Expected:

- outside git: explicit message
- inside git: meaningful output

## LTS-EXT-MAC-010 MCP surfaces

If MCP is configured:

1. run `/mcp`
2. ask to list MCP resources
3. ask to read one MCP resource

Expected:

- MCP flows work
- missing MCP config fails clearly

## LTS-EXT-MAC-011 Web search behavior

If web search is enabled:

1. ask a clearly web-dependent question
2. ask a follow-up relying on the fetched context

Expected:

- web search works without malformed tool errors
- disabled search is reported explicitly

## LTS-EXT-MAC-012 Advisory provider matrix

If wrappers exist:

```bash
codex-deepseek
codex-kimi
```

Repeat:

- two-turn `/chat`
- `/plan` then `/code`
- one inspect task
- one edit task

Expected: advisory providers remain usable for smoke validation.

## LTS-EXT-MAC-013 Package identity and artifact sanity

Verify the tested binary belongs to the LTS package/artifact:

```bash
command -v codex-lts
whence -v codex-lts
file "$(command -v codex-lts)" 2>/dev/null || true
```

Expected:

- binary path belongs to the intended LTS install
- nothing points at upstream Homebrew or upstream npm package
