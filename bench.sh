#!/usr/bin/env bash

set -e
DIR=$(realpath "$0") && DIR=${DIR%/*}
cd "$DIR"

echo "=== 1. 运行纯 Rust 进程内性能对比测试 (解释执行与 JIT 双模式) ==="
cargo run --manifest-path benchmarks/runner/Cargo.toml --release -- "$@"

echo "=== 2. 转换数据为前端模块 (website/src/lib/benchData.js) ==="
if command -v bun >/dev/null 2>&1; then
  bun website/scripts/benchConvert.js
  echo "=== 3. 生成离线矢量图表 (benchmarks/*.svg) ==="
  bun website/scripts/benchSvg.js
  if [ -f hook/svg.js ]; then
    echo "=== 4. 优化与压缩 SVG 矢量图 ==="
    bun hook/svg.js benchmarks/benchmark*.svg
  fi
else
  echo "提示: 未检测到 bun 命令，跳过 website 数据转换与 SVG 生成"
fi

echo "✓ 性能对比评测、数据转换与 SVG 矢量图生成完毕！"
