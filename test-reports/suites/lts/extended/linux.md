# LTS Extended Suite (Linux x64)

Broad regression suite for Linux x64 after Core passes. This suite is meant to cover nearly the full operator-visible surface of the LTS CLI.

## LTS-EXT-LNX-001 Complete slash command matrix

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
- every command opens the correct UI or action path
- commands that need special context fail gracefully instead of being ignored

Note:

- `/quit` and `/exit` should be tested only at the end of a session
- debug-only commands hidden outside debug builds are not release blockers

## LTS-EXT-LNX-002 `model`, `approvals`, and status surfaces

Inside one session:

1. run `/model` and inspect model list / reasoning UI
2. run `/approvals` and inspect policy options
3. run `/status`

Expected:

- configuration surfaces render cleanly
- no broken input focus or frozen modal
- the session remains usable afterwards

## LTS-EXT-LNX-003 Session lifecycle

In a disposable workspace:

1. start a session and send one prompt
2. run `/new`
3. send a different prompt
4. run `/resume`
5. reopen the earlier session if available
6. run `/logout` at the end only if this machine is meant to test auth behavior

Expected:

- `/new` creates a fresh conversation
- `/resume` can reopen an existing saved conversation
- `/logout` behaves explicitly and does not corrupt the install

## LTS-EXT-LNX-004 `init`, `mention`, and `skills`

In a disposable workspace:

1. run `/init`
2. confirm `AGENTS.md` is created
3. run `/mention` and attach a file from the workspace
4. run `/skills` and verify the list/UI appears
5. ask the model to use the mentioned file in its answer

Expected:

- `AGENTS.md` creation works and does not overwrite silently
- file mention integrates into the input flow
- skills UI is reachable and non-broken

## LTS-EXT-LNX-005 Long-session `/compact`

Keep a single session alive across many turns with file reads, diffs, and normal replies. Then run:

```text
/compact
```

Expected:

- compaction completes
- the session remains interactive afterwards
- `/plan`, `/code`, `/diff`, and normal prompts still work after compaction

## LTS-EXT-LNX-006 Tool family: inspect/search/read

In a disposable repo with at least 8 to 10 files, ask for a task that requires:

- directory listing
- opening multiple files
- searching for a symbol or string
- summarizing the relevant implementation points

Suggested prompt:

`find where the app sets the port, read the relevant files, and summarize the flow without changing anything`

Expected:

- realistic inspect flow succeeds
- tool chaining remains stable
- no duplicate or orphaned tool-call error appears

## LTS-EXT-LNX-007 Tool family: shell execution and output capture

Ask for a task that clearly requires shell output, for example:

`print the current directory, show the active git branch if any, and summarize the result`

Expected:

- shell execution works
- the answer reflects the actual command output
- the session remains stable after the command

## LTS-EXT-LNX-008 Tool family: single-file edit

Ask for a safe edit in one file, for example:

`update README.md to add one short troubleshooting bullet and then show the result`

Expected:

- the file is edited correctly
- the final answer reflects the real on-disk change
- no patch parser issue appears

## LTS-EXT-LNX-009 Tool family: multi-file edit

Ask for a coordinated edit across at least two files, for example:

`change the exported constant in src/app.ts, update docs/notes.md to match, and summarize the diff`

Expected:

- multi-file mutation works end-to-end
- follow-up reads match the changes actually made

## LTS-EXT-LNX-010 Git workflows: `/diff` and `/review`

Validate both contexts:

1. outside a git repo, run `/diff` and `/review`
2. inside a git repo with a small change, run `/diff` and `/review`

Expected:

- outside git: explicit, non-crashing message
- inside git: meaningful diff/review output

## LTS-EXT-LNX-011 MCP surfaces

If MCP servers are configured on the machine:

1. run `/mcp`
2. ask the model to list MCP resources
3. ask it to read one MCP resource

Expected:

- MCP listing works
- resource listing/reading works when configured
- if MCP is unavailable, failure is explicit and graceful

## LTS-EXT-LNX-012 Web search behavior

If web search is enabled for the tested provider/config:

1. ask a clearly web-dependent question
2. ask a follow-up that reuses the fetched context

Expected:

- search works without malformed tool errors
- citations/links or fetched context remain coherent
- if web search is disabled, the failure mode is explicit

## LTS-EXT-LNX-013 Provider advisory matrix

If wrappers exist:

```bash
codex-deepseek
codex-kimi
```

Repeat:

- two-turn `/chat`
- `/plan` then `/code`
- one inspect/read task
- one edit task

Expected: advisory providers should remain functional, but failures here are advisory unless promoted to release gate.

## LTS-EXT-LNX-014 Failure-mode matrix

Explicitly provoke and record:

- `/diff` outside git
- `/review` outside git
- mention a missing file
- ask to read a missing file
- ask to edit a read-only or intentionally invalid target if safe to simulate
- ask for MCP use when MCP is not configured

Expected:

- failures are clear and recoverable
- the session survives and accepts the next prompt normally

## LTS-EXT-LNX-015 Package identity and update path

Verify the installed tree belongs to the LTS package and not upstream:

```bash
command -v codex-lts
whence -v codex-lts
readlink -f "$(command -v codex-lts)" 2>/dev/null || true
```

Expected:

- the tested binary resolves to the intended LTS tree
- nothing points at `@openai/codex`
