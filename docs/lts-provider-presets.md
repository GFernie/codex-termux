# LTS Provider Presets

LTS includes curated provider definitions and ready-to-use profiles for the coding stacks used most in this fork.

Policy:

- known providers use curated provider-specific defaults
- generic providers stay on conservative compatibility defaults unless the user overrides them

## Built-in providers

| Provider ID | Purpose | Base URL | Env key | Wire API |
| --- | --- | --- | --- | --- |
| `alibaba-coding` | Alibaba Coding Plan / Qwen + GLM coding models | `https://coding-intl.dashscope.aliyuncs.com/v1` | `ALIBABA_CODE_API_KEY` | `chat` |
| `zai-coding` | Z.AI coding endpoint for GLM family | `https://api.z.ai/api/coding/paas/v4` | `ZAI_API_KEY` | `chat` |
| `openrouter` | Generic routed OpenAI-compatible provider | `https://openrouter.ai/api/v1` | `OPENROUTER_API_KEY` | `chat` |
| `deepseek` | Direct DeepSeek OpenAI-compatible endpoint | `https://api.deepseek.com/v1` | `DEEPSEEK_API_KEY` | `chat` |

## Built-in profiles

| Profile | Provider | Model | Intent |
| --- | --- | --- | --- |
| `qwen35-coding` | `alibaba-coding` | `qwen3.5-plus` | default Qwen coding |
| `qwen35-plan` | `alibaba-coding` | `qwen3.5-plus` | higher-reasoning planning default |
| `qwen3-coder-plus` | `alibaba-coding` | `qwen3-coder-plus` | coder-oriented Qwen |
| `qwen3-coder-next` | `alibaba-coding` | `qwen3-coder-next` | newer coder-oriented Qwen |
| `qwen3max-plan` | `alibaba-coding` | `qwen3-max-2026-01-23` | large-context Qwen planning |
| `glm5-coding` | `alibaba-coding` | `glm-5` | default GLM coding |
| `glm5-plan` | `alibaba-coding` | `glm-5` | higher-reasoning GLM planning |
| `glm47-coding` | `alibaba-coding` | `glm-4.7` | Alibaba GLM-4.7 coding |
| `kimi-k25-coding` | `alibaba-coding` | `kimi-k2.5` | Alibaba Kimi coding |
| `minimax-m25-coding` | `alibaba-coding` | `MiniMax-M2.5` | Alibaba MiniMax coding |
| `glm5-zai-coding` | `zai-coding` | `glm-5` | direct Z.AI GLM-5 |
| `glm47-zai-coding` | `zai-coding` | `glm-4.7` | direct Z.AI GLM-4.7 |
| `openrouter-qwen` | `openrouter` | `qwen/qwen3-coder-next` | routed Qwen coder profile |
| `deepseek-coding` | `deepseek` | `deepseek-v3.2` | direct DeepSeek coding |

## Example usage

```bash
export ALIBABA_CODE_API_KEY="..."
codex --profile qwen35-coding
```

```bash
export ZAI_API_KEY="..."
codex --profile glm47-zai-coding
```

```bash
export OPENROUTER_API_KEY="..."
codex --profile openrouter-qwen
```

## Notes

- these presets do not embed secrets or user-specific headers
- user-defined `model_providers` and `profiles` still override the shipped defaults
- `alibaba-coding` uses official Alibaba Coding Plan model windows where documented
- `zai-coding` uses curated direct GLM windows from current Z.AI docs
- `openrouter` stays conservative by default because routed model metadata is not trusted as uniformly stable in the LTS API-key flow
- context and compact defaults are documented in [`lts-context-and-compact.md`](./lts-context-and-compact.md)
