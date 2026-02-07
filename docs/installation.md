# Installation

## Quick Install

### Termux (Android ARM64)

```bash
# Update Termux packages and install Node.js
pkg update && pkg upgrade -y
pkg install nodejs-lts -y

# Latest (tracks upstream OpenAI)
npm install -g @mmmbuto/codex-cli-termux

# OR LTS (stable, /chat compatible)
npm install -g @mmmbuto/codex-cli-lts

# Verify
codex --version
codex login
```

**Requirements:** Android 7+, ARM64, Node.js >=18 (recommended v22+), ~50MB storage

---

### Linux (x64)

```bash
# Install Node.js (example for Debian/Ubuntu)
sudo apt-get update
sudo apt-get install -y nodejs npm

# LTS (Linux support)
npm install -g @mmmbuto/codex-cli-lts

# Verify
codex --version
codex login
```

**Requirements:** Linux x64, Node.js >=18 (recommended v22+), ~80MB storage

---

### macOS (arm64)

```bash
# Using Homebrew
brew install node

# Download and install LTS from GitHub releases
curl -L https://github.com/DioNanos/codex-termux/releases/latest/download/codex-cli-lts-macos-arm64.tar.gz -o codex-cli-lts-macos-arm64.tar.gz
tar -xzf codex-cli-lts-macos-arm64.tar.gz
sudo mv codex codex-exec /usr/local/bin/

# Verify
codex --version
codex login
```

**Requirements:** macOS arm64, Node.js >=18 (recommended v22+), ~100MB storage

---

## Package Information

### @mmmbuto/codex-cli-termux (Latest)

- **Focus**: Termux-only, tracks upstream OpenAI closely
- **Platform**: Android Termux (ARM64)
- **Updates**: Follows upstream OpenAI releases
- **Use case**: Latest features, bleeding-edge updates

### @mmmbuto/codex-cli-lts (LTS)

- **Focus**: Long-term support, stability, /chat compatibility
- **Base**: OpenAI Codex rust-v0.80.0
- **Platform**: Linux x64 + Android Termux (ARM64) + macOS arm64
- **Updates**: Minimal features + security patches only
- **Use case**: Production stability, /chat wire API support

---

## Troubleshooting

### Termux Issues

**Package manager errors:**
```bash
pkg update && pkg upgrade -y
```

**Node.js not found:**
```bash
pkg install nodejs-lts -y
```

### Linux Issues

**Permission denied on install:**
```bash
sudo npm install -g @mmmbuto/codex-cli-lts
```

### macOS Issues

**Binary not found after install:**
```bash
# Check installation location
which codex

# Manually add to PATH
echo 'export PATH="/usr/local/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

---

## Verification

After installation, verify everything works:

```bash
# Check version
codex --version

# Check binary
codex-exec --help

# Test login (opens browser)
codex login
```

## Links

- npm Latest: https://www.npmjs.org/package/@mmmbuto/codex-cli-termux
- npm LTS: https://www.npmjs.org/package/@mmmbuto/codex-cli-lts
- GitHub Releases: https://github.com/DioNanos/codex-termux/releases
- Upstream: https://github.com/openai/codex
- Web UI: [NexusCLI](https://github.com/DioNanos/nexuscli) - Optional web interface
