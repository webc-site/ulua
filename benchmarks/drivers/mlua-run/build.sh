#!/bin/bash
# Build the three mlua backends (vendored C sources) into target/release/mlua-<backend>.
set -e
cd "$(dirname "$0")"
for feat in luau lua54 luajit; do
  echo "=== building mlua-run [$feat] ==="
  cargo build --release --no-default-features --features "$feat"
  cp target/release/mlua-run "target/release/mlua-$feat"
  echo "  -> target/release/mlua-$feat"
done
