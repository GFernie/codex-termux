# LTS Extended Suite (macOS arm64)

Broader regression suite for macOS arm64 after Core passes.

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

## `/compact`, `/diff`, and `/review`

Validate:

- `/compact` on a longer session
- `/diff` outside and inside a git repo
- `/review` on a disposable local diff

Expected: no UI or protocol regressions.

## Session lifecycle

Verify:

- `/new`
- `/resume`
- `/init`

in a disposable workspace.

## Mention / file reference flow

Mention a file and ask the model to reason about it.

## Provider advisory matrix

If wrappers exist:

```bash
codex-deepseek
codex-kimi
```

Repeat the Core `/chat`, `/plan`, and tool-mutation smoke.

## Package identity

Verify the installed binary belongs to the macOS arm64 LTS package/artifact and not upstream Homebrew or upstream npm.
