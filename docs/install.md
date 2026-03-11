## Install codex-termux

This repository exposes two release lines:

- `main`: latest Termux build
- `lts`: LTS build for Termux, Linux x64, and macOS arm64

Release-line details: [`release-lines.md`](./release-lines.md)

### LTS via npm

```bash
npm install -g @mmmbuto/codex-cli-lts
```

Verify:

```bash
codex --version
codex exec --help
```

### Latest via npm

```bash
npm install -g @mmmbuto/codex-cli-termux
```

### Built-In LTS Profiles

LTS includes built-in profiles for common Qwen/GLM coding setups:

```bash
codex --profile qwen35-coding
codex --profile glm5-coding
codex --profile glm47-zai-coding
```

These profiles expect the matching provider key to be exported:

- `ALIBABA_CODE_API_KEY`
- `ZAI_API_KEY`
- `OPENROUTER_API_KEY`
- `DEEPSEEK_API_KEY`

Preset details: [`lts-provider-presets.md`](./lts-provider-presets.md)

### macOS arm64 note

For LTS npm releases, the macOS arm64 binary is produced by GitHub Actions and then included in the final package. Local Linux builds are not the source for that artifact.

### Testing after install

Run the LTS Core suite first:

- Linux: [`../test-reports/suites/lts/core/linux.md`](../test-reports/suites/lts/core/linux.md)
- Termux: [`../test-reports/suites/lts/core/termux.md`](../test-reports/suites/lts/core/termux.md)
- macOS: [`../test-reports/suites/lts/core/macos.md`](../test-reports/suites/lts/core/macos.md)
