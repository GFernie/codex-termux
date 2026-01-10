# 🤖 Codex CLI - Termux Edition

> **Built from upstream OpenAI Codex source, compiled for Android Termux (ARM64)**

[![npm](https://img.shields.io/npm/v/@mmmbuto/codex-cli-termux?style=flat-square&logo=npm)](https://www.npmjs.com/package/@mmmbuto/codex-cli-termux)
[![downloads](https://img.shields.io/npm/dt/@mmmbuto/codex-cli-termux?style=flat-square)](https://www.npmjs.com/package/@mmmbuto/codex-cli-termux)
[![ko-fi](https://img.shields.io/badge/☕_Support-Ko--fi-FF5E5B?style=flat-square&logo=ko-fi)](https://ko-fi.com/dionanos)

---

## What This Is

Built from upstream OpenAI Codex source, compiled for Android Termux. Since Termux is not officially supported by upstream, we apply minimal patches only for critical compatibility issues.

### What We Do:
✅ **Use official OpenAI Codex source** (https://github.com/openai/codex)
✅ **Compile for ARM64** (Android Termux native)
✅ **Apply minimal patches** only for Termux-specific issues not addressed upstream
✅ **Package as npm** for easy installation
✅ **Maintain full Apache 2.0 compliance** with OpenAI attribution

### What We DON'T Do:
❌ **NO new features**
❌ **NO behavior modifications** (works exactly like upstream)
❌ **NO replacement** of official Codex

### 🔧 Compatibility Patches

We only apply patches for issues that:
- **Prevent Codex from working on Termux**
- **Are not addressed by upstream** (Termux is not officially supported)
- **Are minimal and well-documented**

**Current patches**: See [patches/](./patches/) directory for full documentation.

Need help debugging upgrade alerts? See
[docs/termux-upgrade-checks.md](./docs/termux-upgrade-checks.md) for known causes
and fix strategies.

**Found an issue?** Well-documented bug reports with reproduction steps are welcome! Open an [issue](https://github.com/DioNanos/codex-termux/issues).

---

## 📋 Prerequisites

```bash
# Update Termux packages
pkg update && pkg upgrade -y

# Install Node.js
pkg install nodejs-lts -y

# Verify
node --version  # v18+ (recommended v22+)
npm --version   # v6+
```

**Requirements:**
- Android 7+ (Termux)
- ARM64 architecture
- Node.js >=18 (recommended >=22)
- ~50MB storage

---

## 📦 Installation

### Via npm (Recommended)

```bash
npm install -g @mmmbuto/codex-cli-termux
```

### Verify Installation

```bash
codex --version
# Output: codex-cli 0.80.0-termux

codex login
# Opens browser for authentication
```

**Links:**
- npm: https://www.npmjs.com/package/@mmmbuto/codex-cli-termux
- Releases: https://github.com/DioNanos/codex-termux/releases
- Upstream: https://github.com/openai/codex
- Web UI: [NexusCLI](https://github.com/DioNanos/nexuscli) - Optional web interface for Codex/Claude/Gemini

---

## ⚡ 2-Minute Quickstart / ⚡ 2分钟快速开始
Get a first session running fast. Choose the path that matches your account.
快速完成首次运行。选择与你的账号/网关匹配的路径。

### Path 1 — OpenAI (default)
```bash
codex login
codex
```

### Path 2 — GLM-4.7 (Zhipu ZAI) / GLM-4.7 (智谱 ZAI)
Use Zhipu's GLM-4.7 model optimized for coding scenarios.
使用智谱的 GLM-4.7 模型（专为编程场景优化）。

**Quick setup / 快速配置:**
```bash
# 1. Add API key to ~/.zshrc (bash users: use ~/.bashrc)
echo 'export ZAI_API_KEY="your-zai-api-key"' >> ~/.zshrc

# 2. Add alias for GLM-4.7
cat >> ~/.zshrc << 'ALIAS_EOF'

# Codex CLI with GLM-4.7 (Coding Plan - dedicated endpoint)
alias codex-glm='OPENAI_API_KEY="$ZAI_API_KEY" codex -m "GLM-4.7" -c model_provider="zai"'
ALIAS_EOF

# 3. Reload shell and test
source ~/.zshrc
codex-glm "Say hello in Chinese"
```

If your ZAI setup requires a base URL, export `OPENAI_BASE_URL=...` — see [docs/GLM4.7-quickstart.md](./docs/GLM4.7-quickstart.md).

**Full docs / 完整文档:** [docs/GLM4.7-quickstart.md](./docs/GLM4.7-quickstart.md)

### Path 3 — OpenRouter & gateways / OpenRouter 与兼容网关
For OpenRouter or other OpenAI-compatible providers.
适用于 OpenRouter 或其他 OpenAI 兼容的提供商。

See [docs/openrouter-quickstart.md](./docs/openrouter-quickstart.md) for detailed configuration.

```bash
# Quick example (see docs for full setup)
source ~/.codex/.env
codex --profile or-fast
```

Caution: model slugs/names can change on providers—verify the current model list first.
注意：模型名称可能变化，请以提供商模型列表为准。

---

## 🧭 OpenRouter & gateways note / 🧭 OpenRouter 与兼容网关说明
This Termux port only adds Android compatibility; it does not change Codex behavior.
本 Termux 版本仅提供 Android 兼容性，不改变 Codex 行为。
Providers/models are determined by your own config and backend.
提供商与模型由你的配置与后端决定。

For detailed setup guides, see:
- **GLM-4.7**: [docs/GLM4.7-quickstart.md](./docs/GLM4.7-quickstart.md)
- **OpenRouter**: [docs/openrouter-quickstart.md](./docs/openrouter-quickstart.md)

---

## 🚀 Usage

Same as official Codex CLI:

```bash
# Login to OpenAI
codex login

# Start chat
codex

# Help
codex --help
```

For full documentation, see [OpenAI Codex docs](https://developers.openai.com/codex).

### Non-Interactive Mode (Automation)

The `codex` binary is a multitool that includes the `exec` subcommand for automation and scripting:

```bash
# Enable web search tool (⚠️ avoid pasting secrets; be mindful of prompt injection from untrusted content)
codex --search

# Run non-interactively with JSON output
codex exec --json "list files in current directory"

# With custom sandbox mode
codex exec --json -s danger-full-access "run npm test"

# Skip git repo check for non-repo directories
codex exec --json --skip-git-repo-check "echo hello"

# Output to file
codex exec --json -o output.json "describe this project"
```

**Key flags:**
- `--json` - Output events as JSONL (for parsing)
- `-s, --sandbox` - Sandbox mode: `read-only`, `workspace-write`, `danger-full-access`
- `--skip-git-repo-check` - Run outside git repositories
- `-o, --output-last-message` - Save final response to file

### Execpolicy

See the [Execpolicy quickstart](./docs/execpolicy.md) to set up rules that govern what commands Codex can execute.

---

## 🔧 Troubleshooting (Termux) / 🔧 故障排查（Termux）
Common Termux issues and the fastest places to check.
常见 Termux 问题与最快排查入口。

- Upgrade alerts or shared library errors: see [docs/termux-upgrade-checks.md](./docs/termux-upgrade-checks.md)
- Basic usage/setup: see [docs/getting-started.md](./docs/getting-started.md)
- Authentication/login problems: see [docs/authentication.md](./docs/authentication.md)
- GLM-4.7 setup: see [docs/GLM4.7-quickstart.md](./docs/GLM4.7-quickstart.md)
- Still stuck? Open an issue with repro steps: [GitHub Issues](https://github.com/DioNanos/codex-termux/issues)

---

## 🧪 Testing & Validation

Latest validation (2026-01-10): 47 passed / 0 failed / 2 skipped — see [CODEX_TEST_REPORT_v0.79.0.md](./CODEX_TEST_REPORT_v0.79.0.md).

<details>
<summary>Details: automated test suite, coverage, and sample report</summary>

### Automated Test Suite
[CODEX_TEST_SUITE.md](./CODEX_TEST_SUITE.md) - Universal test suite compatible with all Codex versions

**Coverage**:
- ✅ **82 automated tests** across 12 categories (including prep/cleanup)
- ✅ **10 Termux-specific tests** validating all active compatibility patches (#1-#6, #9)
- ✅ **8 Package & Binary tests** for npm installation verification
- ✅ File operations, shell execution, environment detection
- ✅ Android permissions, library paths, package manager
- ✅ Error handling and edge cases

**How to use**:

```bash
# Start Codex
codex

# Feed the test suite
> Read and execute all tests in https://github.com/DioNanos/codex-termux/blob/main/CODEX_TEST_SUITE.md
```

Codex will automatically:
1. Execute all applicable tests sequentially
2. Report PASS/FAIL for each test
3. Generate a final summary with:
   - Total passed/failed counts
   - Category breakdowns
   - Critical failures (if any)
   - Overall verdict

**Test Categories**:
1. System Information (3 tests)
2. File Operations (8 tests)
3. Search & Discovery (3 tests)
4. Shell Execution (4 tests)
5. Text Processing (2 tests)
6. Web & Network (2 tests - optional)
7. Git Operations (2 tests - optional)
8. AI Capabilities (3 tests)
9. Error Handling (3 tests)
10. **Termux-Specific (10 tests)** ⭐ - Validates all Android patches
11. Cleanup (1 test)
12. **Package & Binary (8 tests)** ⭐ - Validates npm installation and binaries

**Termux-Specific Tests Include**:
- ✅ Environment paths (`$PREFIX`, `$HOME`, `$LD_LIBRARY_PATH`)
- ✅ Shell detection (bash/zsh on Android)
- ✅ Package manager (`pkg` commands)
- ✅ Storage access (`/sdcard`, `~/storage`)
- ✅ Android permissions and sandbox isolation
- ✅ Library path preservation (Patch #8 validation)
- ✅ Browser opener availability (Patch #1 validation)
- ✅ Architecture detection (aarch64/ARM64)

**Suite size**: 82 tests defined (includes optional/manual). Automated run on Termux executes 49 applicable tests; last run (2026-01-10) completed with 47 ✅ / 0 ❌ / 2 ⚠️ skipped (WebSearch unavailable, git info skipped in non-repo workspace).

**Success Criteria**:
- All System, Files, Shell, and Termux tests must pass
- At least 80% overall pass rate
- No critical crashes

**Example Report** (v0.80.0-termux, 2026-01-10):
```
CODEX CLI TEST SUITE - FINAL REPORT
====================================
Platform: Android Termux ARM64
Codex Version: 0.80.0-termux
Total Tests: 49
✅ Passed: 49
❌ Failed: 0
⚠️ Skipped: 0
✅ Passed: 47
❌ Failed: 0
⚠️ Skipped: 2 (WebSearch unavailable, git info skipped in non-repo workspace)

Termux-Specific: 10/10 passed ✅
Package & Binary: 8/8 passed ✅

VERDICT: ✅ PASS
```

</details>

---

## 📚 Documentation

- [**Getting started**](./docs/getting-started.md)
  - [CLI usage](./docs/getting-started.md#cli-usage)
  - [Slash Commands](./docs/slash_commands.md)
  - [Running with a prompt as input](./docs/getting-started.md#running-with-a-prompt-as-input)
  - [Example prompts](./docs/getting-started.md#example-prompts)
  - [Custom prompts](./docs/prompts.md)
  - [Memory with AGENTS.md](./docs/getting-started.md#memory-with-agentsmd)
- [**Configuration**](./docs/config.md)
  - [Example config](./docs/example-config.md)
- [**Sandbox & approvals**](./docs/sandbox.md)
- [**Execpolicy quickstart**](./docs/execpolicy.md)
- [**Authentication**](./docs/authentication.md)
  - [Auth methods](./docs/authentication.md#forcing-a-specific-auth-method-advanced)
  - [Login on a "Headless" machine](./docs/authentication.md#connecting-on-a-headless-machine)
- **Automating Codex**
  - [GitHub Action](https://github.com/openai/codex-action)
  - [TypeScript SDK](./sdk/typescript/README.md)
  - [Non-interactive mode (`codex exec`)](./docs/exec.md)
- [**Skills**](./docs/skills.md)
- [**Installing & building**](./docs/install.md)
  - [System Requirements](./docs/install.md#system-requirements)
  - [DotSlash](./docs/install.md#dotslash)
  - [Build from source](./docs/install.md#build-from-source)
- [**Contributing**](./docs/contributing.md)
- [**Open source fund**](./docs/open-source-fund.md)

---

## 🔨 Building from Source

See [BUILDING.md](./BUILDING.md) for compilation instructions.

---

## 🔧 Project Maintenance

**Codex-Termux** is a community-maintained port enabling AI-powered coding on Android Termux.

**Maintenance activities:**
- 🔨 **ARM64 compilation** - Building native binaries for each upstream release (~18min per build)
- 🔄 **Upstream synchronization** - Tracking OpenAI Codex updates and merging changes
- 🐛 **Compatibility patches** - Maintaining Android-specific fixes for Termux environment
- 📱 **Device testing** - Verification on real ARM64 hardware (ARM64 flagship device, other devices)
- 📚 **Documentation & support** - Maintaining docs, responding to GitHub issues

**Time investment:** Approximately 20 hours per month for project upkeep.

**Thank you** to all users who have reported issues, provided feedback, and helped improve this project. Your contributions make Codex accessible on mobile platforms.

---

## 📝 License

This project maintains full compliance with the Apache 2.0 license from OpenAI Codex.

**Original work**: Copyright OpenAI (https://github.com/openai/codex)
**Termux port**: Minimal patches for Android compatibility

See [LICENSE](./LICENSE) file for details.

---

## 🙏 Credits

- **OpenAI** for the amazing Codex CLI
- **Termux** community for Android terminal environment
- All contributors to upstream Codex project

---

**Version**: Based on OpenAI Codex main (rust-v0.80.0) with Termux compatibility patches
**Platform**: Android Termux ARM64
**Maintained**: Community-driven, not affiliated with OpenAI

---

## 📜 Changelog

Upstream Codex releases: https://github.com/openai/codex/releases

Termux-specific changes: see [CHANGELOG.md](./CHANGELOG.md).

---

**Testing**: Comprehensive test suite v1.2 with 82 tests (incl. Termux + Package) in [`CODEX_TEST_SUITE.md`](./CODEX_TEST_SUITE.md)
