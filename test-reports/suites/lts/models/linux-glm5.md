# LTS Model Suite (Linux): GLM-5

Profile under test:

- `glm5-coding`

Run this suite after Linux Core passes.

## MGLM-LNX-001 Profile resolution

```bash
codex-lts --profile glm5-coding --version
```

Expected:

- profile resolves without local config edits
- version remains `-lts`

## MGLM-LNX-002 Two-turn chat stability

Run:

```bash
codex-glm5
```

Inside the TUI:

1. ask `say hello, print current directory, then stop`
2. ask `now list up to five files here and stop`

Expected:

- both turns answer normally
- no blank-line stall
- no tool or role warnings

## MGLM-LNX-003 Inspect/search/read

Use a disposable workspace with `README.md`, `src/app.ts`, and `docs/notes.md`.

Ask:

`inspect this workspace, find where answer is defined, read the relevant files, and summarize what should change`

Expected:

- inspect and file-read flow works
- summary matches the real files

## MGLM-LNX-004 Shell execution

Ask:

`print the current directory, show the active git branch if any, and summarize the result`

Expected:

- shell execution succeeds
- answer reflects real command output

## MGLM-LNX-005 Single-file edit

Ask:

`update README.md to add one short troubleshooting bullet and then show the result`

Expected:

- edit succeeds
- result matches on-disk content

## MGLM-LNX-006 Multi-file edit

Ask:

`change src/app.ts so answer becomes 42, update docs/notes.md to match, and summarize the diff`

Expected:

- both files change correctly
- final summary reflects actual diff

## MGLM-LNX-007 `/plan` and `/code`

Inside the same session:

1. run `/plan`
2. ask for a short implementation plan
3. run `/code`
4. ask for a tiny file edit

Expected:

- `/plan` persists across turns
- `/code` restores normal execution mode

## MGLM-LNX-008 Failure recovery

Trigger:

- `/diff` outside git
- missing-file read request

Expected:

- explicit error
- session remains usable on the next prompt
