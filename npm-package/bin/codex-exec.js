#!/usr/bin/env node
import { spawnSync } from 'node:child_process';
import { existsSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));

function isTermux() {
  if (process.platform === 'android') return true;
  const prefix = process.env.PREFIX || '';
  if (prefix.includes('/com.termux/')) return true;
  if (process.env.TERMUX_VERSION) return true;
  return existsSync('/data/data/com.termux/files/usr');
}

function resolveBinary(name) {
  if (isTermux()) {
    return path.join(__dirname, 'android-arm64', name);
  }
  if (process.platform === 'linux' && process.arch === 'x64') {
    return path.join(__dirname, 'linux-x64', name);
  }
  if (process.platform === 'linux' && process.arch === 'arm64') {
    return path.join(__dirname, 'linux-arm64', name);
  }
  return null;
}

const bin = resolveBinary('codex-exec');
if (!bin || !existsSync(bin)) {
  console.error(`Unsupported platform/arch: ${process.platform}/${process.arch}.`);
  console.error('Supported: linux-x64, android-arm64 (Termux).');
  process.exit(1);
}

// Let the Rust updater logic detect that this binary is managed by an npm package,
// even on macOS where the global npm prefix is typically under /opt/homebrew.
const env = { ...process.env, CODEX_MANAGED_BY_NPM: '1' };
const result = spawnSync(bin, process.argv.slice(2), { stdio: 'inherit', env });
if (result.error) {
  console.error(result.error.message);
  process.exit(1);
}
process.exit(result.status ?? 1);
