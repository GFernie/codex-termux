# OpenRouter Quickstart (Termux) / OpenRouter 快速开始 (Termux)
Minimal setup for OpenRouter or other OpenAI-compatible gateways on Termux.
Termux 上使用 OpenRouter/兼容网关的最小配置。

## 1) Add API key / 添加 API Key
Store your key in a local env file and keep permissions tight.
将密钥存放在本地 env 文件并限制权限。

```bash
mkdir -p ~/.codex
cat > ~/.codex/.env <<'ENVFILE'
export OPENROUTER_API_KEY="YOUR_KEY_HERE"
ENVFILE
chmod 600 ~/.codex/.env
```

## 2) Minimal config / 最小配置
Define a provider and two example profiles in `~/.codex/config.toml`.
在 `~/.codex/config.toml` 中定义提供商与两个示例 profile。

```toml
# ~/.codex/config.toml
[model_providers.openrouter]
name = "OpenRouter"
base_url = "https://openrouter.ai/api/v1"
env_key = "OPENROUTER_API_KEY"
wire_api = "responses"

[profiles.or-fast]
model_provider = "openrouter"
model = "<provider-model-slug>"

[profiles.or-accurate]
model_provider = "openrouter"
model = "<provider-model-slug>"
```

Replace the model slugs with values from your provider's model list.
If your gateway does not support Responses, use `wire_api = "chat"` instead.
If your gateway supports the Responses API, set `wire_api = "responses"`.

## 3) Run a profile / 运行配置
Load the env file, then run Codex with a profile.
加载 env 文件，然后用 profile 运行 Codex。

```bash
source ~/.codex/.env
codex --profile or-fast
```

Tip: add `source ~/.codex/.env` to your shell profile if you want it loaded automatically.

## Troubleshooting / 故障排查
Quick checks for common errors.
常见错误的快速检查。

- 401/403: API key missing or invalid; ensure `OPENROUTER_API_KEY` is exported and re-run `source ~/.codex/.env`.
- Model not found: the provider slug may have changed; pick a current model name and update `config.toml`.
- Permissions/path: ensure `~/.codex/.env` is `chmod 600` and `~/.codex/config.toml` is readable. If you set `CODEX_HOME`, place files under that directory instead.
