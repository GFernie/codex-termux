#!/bin/bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$ROOT_DIR"

echo "🔍 VERIFYING TERMUX PATCH SET..."
echo ""

pass() { echo "✅ PRESENT"; }
fail() { echo "❌ MISSING!"; exit 1; }

binary_has_origin_runpath() {
  local binary="$1"

  readelf -d "$binary" 2>/dev/null \
    | grep -E '(RUNPATH|RPATH)' \
    | grep -F '$ORIGIN' >/dev/null
}

is_elf_binary() {
  local binary="$1"

  readelf -h "$binary" >/dev/null 2>&1
}

detect_runtime_pair() {
  local search_roots=(
    "npm-package/bin"
    "codex-rs/target/debug"
    "codex-rs/target/release"
    "codex-rs/target/ci-test"
  )
  local root

  if [ -n "${CARGO_TARGET_DIR:-}" ]; then
    search_roots+=(
      "$CARGO_TARGET_DIR/debug"
      "$CARGO_TARGET_DIR/release"
      "$CARGO_TARGET_DIR/ci-test"
    )
  fi

  for root in "${search_roots[@]}"; do
    if [ -x "$root/codex.bin" ] && [ -x "$root/codex-exec.bin" ] \
      && is_elf_binary "$root/codex.bin" \
      && is_elf_binary "$root/codex-exec.bin"; then
      printf '%s\n%s\n' "$root/codex.bin" "$root/codex-exec.bin"
      return 0
    fi

    if [ -x "$root/codex" ] && [ -x "$root/codex-exec" ] \
      && is_elf_binary "$root/codex" \
      && is_elf_binary "$root/codex-exec"; then
      printf '%s\n%s\n' "$root/codex" "$root/codex-exec"
      return 0
    fi
  done

  return 1
}

# Patch #1
printf "Patch #1 (Browser Login): "
if grep -q "target_os.*android" codex-rs/login/src/server.rs \
  && grep -q "termux-open-url" codex-rs/login/src/server.rs; then
  pass
else
  fail
fi

# Patch #2
printf "Patch #2 (RAM Optimization): "
if grep -q "lto = false" codex-rs/Cargo.toml; then
  pass
else
  fail
fi

# Patch #4
printf "Patch #4 (Auto-Update URL): "
if grep -q "DioNanos/codex-termux" codex-rs/tui/src/updates.rs; then
  pass
else
  fail
fi

# Patch #5
printf "Patch #5 (Version Parser): "
if grep -q "split('-')" codex-rs/tui/src/updates.rs; then
  pass
else
  fail
fi

# Patch #6
printf "Patch #6 (NPM Package): "
if grep -q "@mmmbuto/codex-cli-termux" codex-rs/tui/src/update_action.rs; then
  pass
else
  fail
fi

# Patch #9
printf "Patch #9 (Auto-Update Exec): "
if grep -q "update_action = exit_info.update_action" codex-rs/cli/src/main.rs \
  && grep -q "run_update_action" codex-rs/cli/src/main.rs; then
  pass
else
  fail
fi

# Patch #10
printf "Patch #10 (Launcher + libc++ fallback): "
if grep -q 'exec "\$SCRIPT_DIR/codex.bin"' npm-package/bin/codex \
  && grep -q 'exec "\$SCRIPT_DIR/codex-exec.bin"' npm-package/bin/codex-exec \
  && grep -q 'LD_LIBRARY_PATH' npm-package/bin/codex \
  && grep -q 'LD_LIBRARY_PATH' npm-package/bin/codex-exec \
  && grep -q '"bin/codex.bin"' npm-package/package.json \
  && grep -q '"bin/codex-exec.bin"' npm-package/package.json \
  && grep -q 'link-arg=-Wl,-rpath,$ORIGIN' codex-rs/.cargo/config.toml; then
  if runtime_pair=$(detect_runtime_pair); then
    runtime_codex=$(printf '%s\n' "$runtime_pair" | sed -n '1p')
    runtime_codex_exec=$(printf '%s\n' "$runtime_pair" | sed -n '2p')

    if binary_has_origin_runpath "$runtime_codex" \
      && binary_has_origin_runpath "$runtime_codex_exec"; then
      echo "✅ PRESENT (runtime proof ok: $(basename "$runtime_codex"), $(basename "$runtime_codex_exec"))"
    else
      echo "❌ MISSING!"
      echo "  Runtime proof failed for:"
      echo "  - $runtime_codex"
      echo "  - $runtime_codex_exec"
      exit 1
    fi
  else
    echo "✅ PRESENT (source wiring ok; runtime proof skipped because no binary pair was found)"
  fi
else
  fail
fi

# Patch #11
printf "Patch #11 (Android no-voice policy): "
if grep -q "\[target.'cfg(target_os = \"android\")'.dependencies\]" codex-rs/cli/Cargo.toml \
  && grep -q 'codex-tui = { path = "../tui", default-features = false }' codex-rs/cli/Cargo.toml \
  && grep -q "\[target.'cfg(target_os = \"android\")'.dependencies\]" codex-rs/cloud-tasks/Cargo.toml \
  && grep -q 'codex-tui = { path = "../tui", default-features = false }' codex-rs/cloud-tasks/Cargo.toml; then
  pass
else
  fail
fi

# Patch #12
printf "Patch #12 (Dynamic subcommand routing): "
if grep -q "spawnSync(binaryPath" npm-package/bin/codex.js \
  && grep -q "detectSubcommands" npm-package/bin/codex.js \
  && grep -q "aliasesMatch" npm-package/bin/codex.js; then
  pass
else
  fail
fi

# Bazel/Toolchain patch set declared in MODULE.bazel
printf "Bazel declared patch files: "
DECLARED_PATCHES=$(grep -o "//patches:[^\" ]*\\.patch" MODULE.bazel | sed 's#//patches:##' | sort -u || true)
if [ -z "$DECLARED_PATCHES" ]; then
  fail
fi

MISSING_DECLARED=0
for patch in $DECLARED_PATCHES; do
  if [ ! -f "patches/$patch" ]; then
    echo ""
    echo "  ❌ Declared but missing: patches/$patch"
    MISSING_DECLARED=1
  fi
done
if [ "$MISSING_DECLARED" -ne 0 ]; then
  exit 1
fi
pass

# Informational: patch files not currently declared in MODULE.bazel
UNDECLARED=$(comm -23 \
  <(find patches -maxdepth 1 -type f -name "*.patch" -printf "%f\n" | sort) \
  <(printf "%s\n" "$DECLARED_PATCHES" | sort) || true)
if [ -n "$UNDECLARED" ]; then
  echo ""
  echo "ℹ️  Patch files present but not declared in MODULE.bazel:"
  printf "  - %s\n" $UNDECLARED
fi

echo ""
echo "🎉 ALL CRITICAL PATCHES VERIFIED!"
