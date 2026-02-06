# Configuration

This repo ships repackaged builds of the upstream Codex CLI. Configuration is
provider-neutral and follows the upstream CLI conventions.

## Quick Setup

1. Set your API key:

```bash
export OPENAI_API_KEY="..."
```

2. Optional: point to an OpenAI-compatible endpoint:

```bash
export OPENAI_BASE_URL="https://your-gateway.example/v1"
```

3. Run:

```bash
codex
```

## Common Options

- Choose a model:

```bash
codex -m "<model>"
```

- Non-interactive (automation) mode:

```bash
codex exec --json "list files in current directory"
```

## Notes

- This repo avoids provider-specific guidance in `README.md`. Keep any provider
  details in your own environment and/or shell aliases.
- Upstream docs remain the reference for the full CLI surface area and auth
  flows.

