# LTS Model Suite (Linux): Qwen 3 Coder Next

Profile under test:

- `qwen3-coder-next`

Run with:

```bash
codex-qwen-coder-next
```

## MQCN-LNX-001 Profile resolution

```bash
codex-lts --profile qwen3-coder-next --version
```

Expected: profile resolves and stays on `-lts`.

Blocking rule:

- the session must actually run on `qwen3-coder-next`
- if the provider/model falls back to anything else, stop and record the suite as `BLOCKED`

## MQCN-LNX-002 Two-turn chat stability

Ask two short operational prompts in the same session.

Expected:

- both replies arrive normally
- no blank-line no-op behavior

## MQCN-LNX-003 Inspect/search/read

Ask for a realistic workspace summary that requires multiple file reads.

Expected:

- inspect flow succeeds
- summary is coherent

## MQCN-LNX-004 Single-file edit

Ask for a one-file markdown edit and verify the on-disk result.

Expected:

- edit completes correctly

## MQCN-LNX-005 Multi-file edit

Ask for a coordinated two-file change and verify the diff.

Expected:

- both files updated correctly

## MQCN-LNX-006 `/plan` and `/code`

Switch into `/plan`, ask for a plan, then `/code` and ask for a real edit.

Expected:

- mode switching works cleanly

## MQCN-LNX-007 Failure recovery

Run `/diff` outside git or ask for a missing file.

Expected:

- explicit error
- session remains usable

Final interpretation:

- generic CLI success on a fallback model does not count as qwen3-coder-next validation
