# LTS Context and Auto-Compact

LTS now ships curated defaults for the main coding model families used in this fork.

## How context is chosen

The effective context budget comes from this order:

1. explicit user override in config/profile
2. known-provider curated model metadata
3. conservative model-family fallback
4. provider-reported value when available at runtime

Provider metadata is not treated as fully reliable across third-party gateways, so LTS keeps curated static defaults.

## Known-provider exact defaults

### Alibaba Coding Plan

LTS follows the current official Alibaba Coding Plan model catalog for the built-in `alibaba-coding` provider.

| Model | Context window | Auto-compact |
| --- | ---: | ---: |
| `qwen3.5-plus` | `1000000` | `750000` |
| `qwen3-coder-plus` | `1000000` | `750000` |
| `qwen3-coder-next` | `262144` | `196608` |
| `qwen3-max-2026-01-23` | `262144` | `196608` |
| `glm-5` | `202752` | `152064` |
| `glm-4.7` | `202752` | `152064` |
| `kimi-k2.5` | `262144` | `196608` |
| `MiniMax-M2.5` | `204800` | `153600` |

### Z.AI direct GLM defaults

Built-in `zai-coding` presets use these curated direct values:

| Model family | Context window | Auto-compact |
| --- | ---: | ---: |
| `glm-5` | `200000` | `150000` |
| `glm-4.7` | `200000` | `150000` |
| `glm-4.6` | `200000` | `150000` |
| `glm-4.5*` | `128000` | `96000` |

### DeepSeek direct defaults

- `deepseek` direct profiles stay on `128000 / 96000`
- reasoning variants stay on `128000 / 80000`

## Conservative compatibility defaults

### Generic or routed providers

When the provider is generic or routed and LTS does not have a trusted exact table for that provider/model pair, it falls back to conservative family defaults:

- recognizable Qwen/GLM/Kimi/DeepSeek coding families: `128000 / 96000`
- thinking/reasoner variants: `128000 / 80000`
- fully unknown models: generic fallback continues to apply

This is why `openrouter` built-in compatibility profiles remain conservative unless the user overrides them.

Official provider docs override older local notes if values differ.

## Manual overrides

Users can still override these values with:

- `model_context_window`
- `model_auto_compact_token_limit`

User overrides always win over the built-in LTS defaults.
