# LTS Plan Mode

LTS backports `/plan` to the `0.80.x` codebase as a persistent mode switch.

## Commands

- `/plan`
  - enter planning mode
- `/plan <request>`
  - enter planning mode and submit the request immediately
- `/code`
  - exit planning mode and return to normal coding behavior

## Behavior

- planning mode persists across turns until `/code`
- it changes runtime developer instructions instead of sending fake planning text as a user prompt
- it is designed to coexist with the LTS goal of keeping `/chat` stable on third-party providers
- entering `/plan` does not silently change the provider or model

## Release expectations

LTS `/plan` is release-blocking and must be verified on:

- `codex-qwen35`
- `codex-glm5`

The required suite steps live in the LTS Core and Extended checklists.
