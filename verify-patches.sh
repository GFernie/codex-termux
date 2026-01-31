#!/bin/bash
echo "🔍 VERIFYING ALL TERMUX PATCHES POST-MERGE..."
echo ""

# Patch #1
echo -n "Patch #1 (Browser Login): "
if grep -q "target_os.*android" codex-rs/login/src/server.rs && grep -q "termux-open-url" codex-rs/login/src/server.rs; then
    echo "✅ PRESENT"
else
    echo "❌ MISSING!"; exit 1
fi

# Patch #2
echo -n "Patch #2 (RAM Optimization): "
if grep -q "lto = false" codex-rs/Cargo.toml; then
    echo "✅ PRESENT"
else
    echo "❌ MISSING!"; exit 1
fi

# Patch #4
echo -n "Patch #4 (Auto-Update URL): "
if grep -q "DioNanos/codex-termux" codex-rs/tui/src/updates.rs; then
    echo "✅ PRESENT"
else
    echo "❌ MISSING!"; exit 1
fi

# Patch #5
echo -n "Patch #5 (Version Parser): "
if grep -q "split('-')" codex-rs/tui/src/updates.rs; then
    echo "✅ PRESENT"
else
    echo "❌ MISSING!"; exit 1
fi

# Patch #6 (LTS) - Updated for codex-cli-lts
echo -n "Patch #6 (LTS Package): "
if grep -q "@mmmbuto/codex-cli-lts" codex-rs/tui/src/update_action.rs && grep -q "ends_with(\"-lts\")" codex-rs/tui/src/updates.rs; then
    echo "✅ PRESENT"
else
    echo "❌ MISSING!"; exit 1
fi

# Patch #9
echo -n "Patch #9 (Auto-Update Exec): "
if grep -q "if let Some(action) = exit_info.update_action" codex-rs/tui/src/main.rs; then
    echo "✅ PRESENT"
else
    echo "❌ MISSING!"; exit 1
fi

echo ""
echo "🎉 ALL CRITICAL PATCHES VERIFIED!"
