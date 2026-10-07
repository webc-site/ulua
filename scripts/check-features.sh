#!/usr/bin/env bash
#
# check-features.sh — build the key Cargo feature combinations for ulua and
# report which combos compile. This is intentionally a shell script (NOT a
# #[test]) so the test suite never recursively invokes cargo.
#
# Combos checked:
#   1. default features (umbrella crate `ulua`)
#   2. --no-default-features (umbrella crate `ulua`)
#   3. the wasm crate for the wasm32 target: -p ulua-web --features wasm
#      --target wasm32-unknown-unknown   (skipped with a clear note if the
#      target toolchain is not installed)
#
# Exit code: 0 if every *attempted* combo built; 1 if any attempted combo failed.
# A skipped combo (missing wasm target) does not fail the script.

set -u

# Resolve the workspace root from this script's location (scripts/ lives at root).
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$ROOT" || exit 2

PASS=()
FAIL=()
SKIP=()

run_combo() {
    local label="$1"
    shift
    echo "=== building: $label ==="
    echo "    cargo $*"
    if cargo "$@" >/dev/null 2>&1; then
        echo "    -> OK"
        PASS+=("$label")
    else
        echo "    -> FAILED"
        FAIL+=("$label")
    fi
}

# 1. default features
run_combo "ulua (default features)" build -p ulua

# 2. no default features
run_combo "ulua (--no-default-features)" build -p ulua --no-default-features

# 3. wasm crate on the wasm target (only if the target is installed)
if rustup target list --installed 2>/dev/null | grep -q '^wasm32-unknown-unknown$'; then
    run_combo "ulua-web --features wasm (wasm32-unknown-unknown)" \
        build -p ulua-web --features wasm --target wasm32-unknown-unknown
else
    echo "=== skipping: ulua-web wasm build (wasm32-unknown-unknown target not installed) ==="
    SKIP+=("ulua-web wasm (target not installed)")
fi

echo
echo "================ feature-matrix summary ================"
for c in "${PASS[@]:-}"; do [ -n "$c" ] && echo "  PASS  $c"; done
for c in "${SKIP[@]:-}"; do [ -n "$c" ] && echo "  SKIP  $c"; done
for c in "${FAIL[@]:-}"; do [ -n "$c" ] && echo "  FAIL  $c"; done
echo "========================================================"

if [ "${#FAIL[@]}" -gt 0 ]; then
    exit 1
fi
exit 0
