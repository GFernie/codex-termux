# LTS Extended Suite (Termux)

Broader regression suite for Termux after Core passes.

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

## Long-session compact pressure

Keep a session active long enough to push context usage upward, then run `/compact`.

Expected:

- compaction does not freeze the TUI
- post-compact turns still work
- `/plan` and `/code` still behave correctly afterwards

## Git-aware behavior

In and out of a git repo, verify `/diff` and `/review` stay sane and do not crash.

## Session lifecycle

Verify:

- `/new`
- `/resume`
- `/init`

in a disposable Termux workspace.

## Mention / file reference flow

Mention a file from the current workspace and ask for a summary or fix plan.

## Provider advisory matrix

If wrappers exist:

```bash
codex-deepseek
codex-kimi
```

Repeat `/chat`, `/plan`, and one tool-using request.

## Termux-specific sanity

Capture:

```bash
command -v termux-open-url || true
command -v termux-setup-storage || true
ls -la /data/data/com.termux/files/usr || true
```

Expected: environment is captured for regression comparison.
