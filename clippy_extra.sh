#!/usr/bin/env bash

set -euo pipefail

ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)

if [ -d "$ROOT/fuzz" ]; then
  echo "==> fuzz"
  (
    cd "$ROOT/fuzz"
    cargo +nightly fmt --all
    cargo +nightly clippy -q --tests --all-targets --all-features -- -D warnings -W clippy::absolute_paths
  )
fi

if [ -d "$ROOT/benchmarks/runner" ]; then
  echo "==> runner"
  (
    cd "$ROOT/benchmarks/runner"
    cargo +nightly fmt --all
    for backend in engine-luau engine-luajit engine-lua54; do
      cargo +nightly clippy -q --tests --all-targets --features "$backend,count-alloc" -- -D warnings -W clippy::absolute_paths
    done
  )
fi

echo "✅ 游离 workspace 通过"
