# Slash commands

Fork-specific slash command notes for the maintained LTS line.

For the upstream general overview, see the official OpenAI documentation:

- https://developers.openai.com/codex/cli/slash-commands

## LTS-relevant commands

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
- `/quit`
- `/exit`
- `/feedback`

## LTS `/plan`

On LTS, `/plan` is a persistent runtime mode:

- `/plan` switches to planning mode
- `/plan <request>` switches to planning mode and sends the request immediately
- `/code` returns to normal coding mode

Important differences from the earlier broken backport:

- no fake planning prompt is injected into visible chat text
- planning behavior persists across turns until `/code`
- the implementation updates runtime developer instructions instead of simulating plan mode in a user message

More detail: [`lts-plan-mode.md`](./lts-plan-mode.md)

## Release-blocking slash command checks

The LTS test suites explicitly validate:

- `/plan`
- `/code`
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

Test policy: [`lts-testing.md`](./lts-testing.md)
