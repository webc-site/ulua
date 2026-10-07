#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR=$(git rev-parse --show-toplevel 2>/dev/null || (DIR=$(realpath "$0") && DIR=${DIR%/*} && echo "${DIR%/*/*}"))
cd "$ROOT_DIR"

PROFILE="release"
CARGO_FLAG="--release"

for arg in "$@"; do
  case "$arg" in
    --debug)
      PROFILE="debug"
      CARGO_FLAG=""
      ;;
    --release)
      PROFILE="release"
      CARGO_FLAG="--release"
      ;;
  esac
done

echo "==> 检查 Rust wasm32 编译目标..."
if ! rustup target list --installed | grep -q "wasm32-unknown-unknown"; then
  echo "==> 自动安装 wasm32-unknown-unknown 目标..."
  rustup target add wasm32-unknown-unknown
fi

EXPECTED_BINDGEN_VER=$(awk '$1 == "name" && $3 == "\"wasm-bindgen\"" { getline; print $3 }' Cargo.lock | tr -d '"' | head -n 1)
EXPECTED_BINDGEN_VER="${EXPECTED_BINDGEN_VER:-0.2.128}"

if ! command -v wasm-bindgen >/dev/null 2>&1; then
  echo "==> 未检测到 wasm-bindgen，正在自动安装 v$EXPECTED_BINDGEN_VER..."
  cargo install wasm-bindgen-cli --version "$EXPECTED_BINDGEN_VER"
else
  CURRENT_VER=$(wasm-bindgen --version 2>&1 | awk '{print $2}')
  if [ "$CURRENT_VER" != "$EXPECTED_BINDGEN_VER" ]; then
    echo "==> wasm-bindgen 版本 ($CURRENT_VER) 与 Cargo.lock ($EXPECTED_BINDGEN_VER) 不一致，正在同步安装..."
    cargo install wasm-bindgen-cli --version "$EXPECTED_BINDGEN_VER" --force
  fi
fi

echo "==> 编译 ulua-web ($PROFILE)..."
if [ -n "$CARGO_FLAG" ]; then
  cargo build -p ulua-web --target wasm32-unknown-unknown --features wasm $CARGO_FLAG
else
  cargo build -p ulua-web --target wasm32-unknown-unknown --features wasm
fi

TARGET_DIR="$(cargo metadata --format-version 1 | grep -o '"target_directory":"[^"]*"' | head -n 1 | cut -d'"' -f4)"
TARGET_DIR="${TARGET_DIR:-$ROOT_DIR/target}"
WASM_SOURCE="$TARGET_DIR/wasm32-unknown-unknown/$PROFILE/ulua_web.wasm"
OUT_DIR="$ROOT_DIR/website/public/pkg"

mkdir -p "$OUT_DIR"

echo "==> 执行 wasm-bindgen 生成前端产物..."
wasm-bindgen --target web --out-dir "$OUT_DIR" "$WASM_SOURCE"

if [ "$PROFILE" = "release" ] && command -v wasm-opt >/dev/null 2>&1; then
  echo "==> 使用 wasm-opt 优化 WebAssembly 体积..."
  wasm-opt -O3 "$OUT_DIR/ulua_web_bg.wasm" -o "$OUT_DIR/ulua_web_bg.wasm"
fi

echo "==> wasm 构建完成: $OUT_DIR"
