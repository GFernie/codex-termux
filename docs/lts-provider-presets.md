# LTS Provider Presets

LTS includes curated provider definitions and ready-to-use profiles for the coding stacks used most in this fork.

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
| `glm5-coding` | `alibaba-coding` | `glm-5` | default GLM coding |
| `glm5-plan` | `alibaba-coding` | `glm-5` | higher-reasoning GLM planning |
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
- context and compact defaults are documented in [`lts-context-and-compact.md`](./lts-context-and-compact.md)
