#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR=$(git rev-parse --show-toplevel 2>/dev/null || (DIR=$(realpath "$0") && DIR=${DIR%/*} && echo "${DIR%/*}"))
WEBSITE_DIR="$ROOT_DIR/website"
cd "$WEBSITE_DIR"

PKG_WASM="$WEBSITE_DIR/public/pkg/ulua_web_bg.wasm"
PKG_JS="$WEBSITE_DIR/public/pkg/ulua_web.js"
FORCE_BUILD=0
FORCE_BENCH=0

for arg in "$@"; do
  case "$arg" in
    -b|--build)
      FORCE_BUILD=1
      ;;
    -B|--bench)
      FORCE_BENCH=1
      ;;
  esac
done

NEED_BUILD=0

if [ $FORCE_BUILD -eq 1 ]; then
  NEED_BUILD=1
elif [ ! -s "$PKG_WASM" ] || [ ! -s "$PKG_JS" ]; then
  echo "==> 未检测到 WebAssembly 引擎产物，正在自动触发构建..."
  NEED_BUILD=1
else
  NEWER_SOURCE=$(find "$ROOT_DIR/crates/ulua-web" -name "*.rs" -newer "$PKG_WASM" 2>/dev/null | head -n 1)
  if [ -n "$NEWER_SOURCE" ]; then
    echo "==> 检测到 crates/ulua-web 源码有修改，正在自动增量重构 WASM..."
    NEED_BUILD=1
  fi
fi

if [ $NEED_BUILD -eq 1 ]; then
  "$WEBSITE_DIR/wasm.sh"
fi

BENCH_DATA="$WEBSITE_DIR/src/lib/benchData.js"
if [ "${FORCE_BENCH:-0}" -eq 1 ] || [ ! -f "$BENCH_DATA" ]; then
  echo "==> 未检测到前端基准数据，正在自动生成 ($BENCH_DATA)..."
  if [ -f "$ROOT_DIR/benchmarks/results.json" ] && [ "${FORCE_BENCH:-0}" -eq 0 ]; then
    bun "$WEBSITE_DIR/scripts/benchConvert.js"
  elif [ -f "$ROOT_DIR/bench.sh" ]; then
    "$ROOT_DIR/bench.sh" --runs=1 || bun "$WEBSITE_DIR/scripts/benchConvert.js"
  else
    bun "$WEBSITE_DIR/scripts/benchConvert.js"
  fi
fi

echo "==> 启动网站本地开发服务器..."
exec bun run dev
