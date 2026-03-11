# LTS Extended Suite (Linux)

Broader regression suite for Linux x64 after Core passes.

## Slash command coverage

Inside `codex-lts`, verify:

- `/compact`
- `/diff`
- `/review`
- `/status`
- `/model`
- `/approvals`
- `/mcp`
- `/new`
- `/resume`
- `/init`

Expected: every command is recognized and produces sane UI behavior.

## `/compact` behavior

Use a longer conversation, then run:

```text
/compact
```

Expected: compaction runs without breaking the session or losing slash command functionality.

## `/diff` inside and outside git

1. run `codex-lts` in a temp directory outside a git repo and issue `/diff`
2. run `codex-lts` inside a git repo with a small local change and issue `/diff`

Expected:

- outside git: clear non-crashing message
- inside git: meaningful diff summary

## `/review` on disposable changes

Create a tiny change in a disposable git repo, then run:

```text
/review
```

Expected: review completes and focuses on issues/risks.

## Session lifecycle

Verify:

- `/new` starts a fresh chat
- `/resume` can reopen a saved chat
- `/init` can create `AGENTS.md` in a disposable workspace

## Mention / file reference flow

In a disposable workspace containing multiple files:

1. mention a file via slash/file picker
2. ask the model to inspect it

Expected: file mention works without corrupting the input flow.

## Provider advisory matrix

If wrappers exist, run:

```bash
codex-deepseek
codex-kimi
```

Repeat the Core `/chat`, `/plan`, and tool-mutation smoke.

## Packaging identity

Verify installed LTS package paths point at the intended package tree and not upstream `@openai/codex`.
