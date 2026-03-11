# LTS Model Suite (Linux): Qwen 3 Coder Plus

Profile under test:

- `qwen3-coder-plus`

Run with:

```bash
codex-qwen-coder-plus
```

## MQCP-LNX-001 Profile resolution

```bash
codex-lts --profile qwen3-coder-plus --version
```

Expected: profile resolves and stays on `-lts`.

Blocking rule:

- the session must actually run on `qwen3-coder-plus`
- if the provider/model falls back to anything else, stop and record the suite as `BLOCKED`

## MQCP-LNX-002 Two-turn chat stability

Ask two short operational prompts in the same session.

Expected:

- both replies arrive normally
- no blank-line no-op behavior

## MQCP-LNX-003 Inspect/search/read

Ask for a realistic workspace summary that requires multiple file reads.

Expected:

- inspect flow succeeds
- summary is coherent

## MQCP-LNX-004 Single-file edit

Ask for a one-file markdown edit and verify the on-disk result.

Expected:

- edit completes correctly

## MQCP-LNX-005 Multi-file edit

Ask for a coordinated two-file change and verify the diff.

Expected:

- both files updated correctly

## MQCP-LNX-006 `/plan` and `/code`

Switch into `/plan`, ask for a plan, then `/code` and ask for a real edit.

Expected:

- mode switching works cleanly

## MQCP-LNX-007 Failure recovery

Run `/diff` outside git or ask for a missing file.

Expected:

- explicit error
- session remains usable

Final interpretation:

- generic CLI success on a fallback model does not count as qwen3-coder-plus validation
