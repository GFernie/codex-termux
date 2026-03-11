# LTS Extended Suite (Termux / Android arm64)

Broader regression suite for Termux after Core passes. This suite should prove that the LTS build is not merely functional, but still comfortable to use from a real phone/tablet shell session.

## LTS-EXT-TMX-001 Complete slash command matrix

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

- each command is recognized
- the resulting UI is usable in Termux dimensions
- commands fail explicitly when context is missing

## LTS-EXT-TMX-002 Long-session `/compact`

Keep one session active across many turns, including file reads and one edit, then run `/compact`.

Expected:

- compaction does not freeze the TUI
- the next prompt works
- `/plan` and `/code` still work after compaction

## LTS-EXT-TMX-003 Session lifecycle

In a disposable workspace:

1. start a session and send one prompt
2. run `/new`
3. send a different prompt
4. run `/resume`
5. reopen the earlier session if available

Expected:

- session switching works
- no broken input state remains after resume

## LTS-EXT-TMX-004 `init`, `mention`, and `skills`

In a disposable workspace:

1. run `/init`
2. confirm `AGENTS.md` exists
3. use `/mention` on a local file
4. open `/skills`
5. ask the model to answer using the mentioned file

Expected:

- mobile terminal input remains stable
- file mention works
- `AGENTS.md` generation is sane

## LTS-EXT-TMX-005 Tool family: inspect/search/read

In a workspace with several files, ask for a task that forces:

- directory listing
- multiple file reads
- symbol or text search
- explanation only, no edits

Expected:

- realistic inspect flow works
- no duplicated or orphaned tool-call error

## LTS-EXT-TMX-006 Tool family: shell execution and output capture

Ask for a task that requires shell output, for example:

`print the current directory, show whether this is inside a git repo, and summarize the result`

Expected:

- shell execution works in Termux
- the answer reflects actual command output
- the session stays stable afterwards

## LTS-EXT-TMX-007 Tool family: single-file and multi-file edits

Run two tasks:

1. one-file doc edit
2. two-file coordinated code + docs edit

Expected:

- edits succeed on Termux filesystem paths
- follow-up reads confirm the actual changes

## LTS-EXT-TMX-008 Git-aware behavior

Check both contexts:

1. `/diff` outside a git repo
2. `/diff` and `/review` inside a disposable git repo with a tiny change

Expected:

- outside git: explicit message
- inside git: useful output
- no TUI corruption

## LTS-EXT-TMX-009 MCP surfaces

If MCP is configured on the device:

1. run `/mcp`
2. ask the model to list MCP resources
3. ask it to read one resource

Expected:

- MCP flows work
- if unavailable, the failure mode is explicit and recoverable

## LTS-EXT-TMX-010 Web search behavior

If web search is enabled:

1. ask a clearly web-dependent question
2. ask a follow-up that depends on the fetched context

Expected:

- web search works without malformed tool errors
- if disabled, that is stated clearly

## LTS-EXT-TMX-011 Advisory provider matrix

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

Expected: advisory providers remain functional enough for smoke validation.

## LTS-EXT-TMX-012 Failure-mode matrix

Explicitly provoke and record:

- `/diff` outside git
- `/review` outside git
- mention a missing file
- ask for a missing file
- ask for MCP use when MCP is unavailable

Expected:

- failures are clear
- the next prompt still works

## LTS-EXT-TMX-013 Termux ergonomics and latency notes

Record:

- whether first token feels delayed
- whether paste/input behaves correctly
- whether the session survives app backgrounding or terminal resize

Expected: no severe usability regression compared with the previous good LTS candidate.
