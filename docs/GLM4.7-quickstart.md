# 🚀 GLM-4.7 Quickstart for Codex CLI / GLM-4.7 快速配置指南

Configure Codex CLI to use Zhipu ZAI's GLM-4.7 model for coding scenarios.
为 Codex CLI 配置智谱 ZAI 的 GLM-4.7 模型用于编程场景。

---

## 📋 Prerequisites / 前置条件

1. **ZAI API Key** - Get your key from https://platform.z.ai
2. **Codex CLI installed** - See [main README](../README.md#installation)
3. **API Key environment variable set**:

```bash
export ZAI_API_KEY="your-zai-api-key-here"
```

Add this to your `~/.zshrc` or `~/.bashrc` to persist:

```bash
echo 'export ZAI_API_KEY="your-zai-api-key-here"' >> ~/.zshrc
source ~/.zshrc
```

---

## 🔧 Method 1: Alias (Recommended) / 方法1：别名（推荐）

Add this to your `~/.zshrc`:

```bash
# Codex CLI with GLM-4.7 (Coding Plan - dedicated endpoint)
# Docs: https://docs.z.ai - coding endpoint for coding scenarios only
alias codex-glm='OPENAI_API_KEY="$ZAI_API_KEY" codex -m "GLM-4.7" -c model_provider="zai"'
```

Then reload your shell:

```bash
source ~/.zshrc
```

**Usage / 使用:**

```bash
# Run Codex with GLM-4.7
codex-glm

# Or pass a prompt directly
codex-glm "Help me write a Python script"
```

---

## 🔧 Method 2: Config File / 方法2：配置文件

Create or edit `~/.codex/config.toml`:

```toml
# Model provider configuration for Zhipu ZAI
[model_providers.zai]
name = "ZAI"
base_url = "https://api.z.ai/api/coding/paas/v4"
env_key = "OPENAI_API_KEY"
wire_api = "chat"
stream_idle_timeout_ms = 3000000

# Optional: Set GLM-4.7 as default model
# model = "GLM-4.7"
```

**Usage / 使用:**

```bash
# Set API key
export OPENAI_API_KEY="$ZAI_API_KEY"

# Run with GLM-4.7 and ZAI provider
codex -m "GLM-4.7" -c model_provider="zai"

# Or set as default in config and just run
codex
```

---

## 📝 Environment Variables Summary / 环境变量总结

| Variable | Value | Description |
|----------|-------|-------------|
| `ZAI_API_KEY` | Your ZAI API key | Primary key for GLM models |
| `OPENAI_API_KEY` | `$ZAI_API_KEY` | Codex CLI expects this var |

---

## 🧪 Verification / 验证

Test your setup:

```bash
# Using alias
codex-glm "Say hello in Chinese and English"

# Using config
codex -m "GLM-4.7" -c model_provider="zai" "What is 2+2?"
```

Expected output: Successful response from GLM-4.7.
预期结果：GLM-4.7 成功响应。

---

## 🔗 Resources / 资源链接

- **ZAI Platform**: https://platform.z.ai
- **ZAI Docs**: https://docs.z.ai
- **GLM-4.7 Model**: Optimized for coding scenarios
- **Codex CLI Docs**: https://github.com/openai/codex

---

## ⚠️ Important Notes / 重要提示

1. **Coding endpoint only** - GLM-4.7 via ZAI is optimized for coding tasks
   仅支持编程端点 - GLM-4.7 通过 ZAI 针对编程任务优化

2. **Timeout setting** - Set `stream_idle_timeout_ms` to 3000000 (50 min) for long-running tasks
   超时设置 - 设置 `stream_idle_timeout_ms` 为 3000000 (50分钟) 以支持长任务

3. **API Key security** - Never commit API keys to git
   API Key 安全 - 永远不要将 API Key 提交到 git

---

## 🆘 Troubleshooting / 故障排查

### Error: "API key not found"
```bash
# Check if env var is set
echo $OPENAI_API_KEY

# Should show your ZAI API key
# If empty, run: export OPENAI_API_KEY="$ZAI_API_KEY"
```

### Error: "Connection timeout"
- Check your network connection
- Verify the base URL in config: `https://api.z.ai/api/coding/paas/v4`

### Error: "Model not found"
- Verify model name is exactly `GLM-4.7`
- Check ZAI platform for current available models

---

**Version**: GLM-4.7 Quickstart v1.0
**Last Updated**: 2026-01-08
**Platform**: Android Termux ARM64
