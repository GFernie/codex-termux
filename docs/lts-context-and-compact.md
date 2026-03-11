# LTS Context and Auto-Compact

LTS now ships curated defaults for the main coding model families used in this fork.

## How context is chosen

The effective context budget comes from this order:

1. explicit user override in config/profile
2. built-in model family metadata
3. provider-reported value when available at runtime

Provider metadata is not treated as fully reliable across third-party gateways, so LTS keeps static family defaults for the high-traffic coding models.

## Current curated defaults

### Qwen coding family

- nominal context window: `128000`
- default auto-compact threshold: `96000`

Applies to:

- `qwen3.5-plus`
- `qwen3.5`
- `qwen3-coder-plus`
- `qwen3-coder-next`
- `qwen3-coding`
- `qwen3-coding-next`

### GLM coding family

- nominal context window: `128000`
- default auto-compact threshold: `96000`

Applies to:

- `glm-5`
- `glm-4.7`
- `glm-4.6`
- `glm-4.5`

### DeepSeek and Kimi standard variants

- nominal context window: `128000`
- default auto-compact threshold: `96000`

### DeepSeek/Kimi thinking or reasoner variants

- nominal context window: `128000`
- conservative auto-compact threshold: `80000`
- tool compatibility remains conservative on purpose

## Manual overrides

Users can still override these values with:

- `model_context_window`
- `model_auto_compact_token_limit`

Built-in LTS profiles already include safe defaults for the main Qwen/GLM coding setups.
